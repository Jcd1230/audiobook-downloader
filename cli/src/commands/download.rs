use crate::commands::utils::{format_book_line, get_config_dir};
use crate::error::{CLIError, Result};
use tracing::{debug, info};

pub async fn download(
    query: Option<&str>,
    all: bool,
    no_cue: bool,
    no_folder: bool,
    filename: Option<String>,
) -> Result<()> {
    let auth_path = get_config_dir().join("auth.json");
    let token_data = std::fs::read_to_string(&auth_path).map_err(|_| CLIError::AuthExpired)?;

    let mut auth: audible_api::auth::AuthInfo =
        serde_json::from_str(&token_data).map_err(|e| anyhow::anyhow!(e))?;
    if auth.is_expired() {
        info!("Access token expired. Refreshing...");
        auth.refresh_access_token().await?;
        std::fs::write(
            &auth_path,
            serde_json::to_string_pretty(&auth).map_err(|e| anyhow::anyhow!(e))?,
        )?;
    }

    let client = audible_api::Client::new(auth);
    let library_file = get_config_dir().join("library.json");
    let mut state =
        crate::state::LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    let mut books_to_download = Vec::new();

    if let Some(q) = query {
        let matches = state.search(q);
        if matches.is_empty() {
            println!("No books found matching '{}'. Try running 'sync' first.", q);
            return Ok(());
        } else if matches.len() == 1 || all {
            books_to_download.extend(matches);
        } else {
            println!(
                "Found {} matching books. Please be more specific or use --all:",
                matches.len()
            );
            for book in matches {
                println!("{}", format_book_line(&book));
            }
            return Err(CLIError::Anyhow(anyhow::anyhow!(
                "Multiple books matched the query. Aborting."
            )));
        }
    } else if all {
        info!("Finding all missing books...");
        for book in state.books.values() {
            if book.status == crate::state::BookStatus::NotDownloaded {
                books_to_download.push(book.clone());
            }
        }
    } else {
        return Err(CLIError::Anyhow(anyhow::anyhow!(
            "Please specify a query (title or ID) or use --all."
        )));
    }

    if books_to_download.is_empty() {
        println!("No books to download.");
        return Ok(());
    }

    info!("Found {} books to download", books_to_download.len());

    let downloader = crate::download::Downloader::new();
    let decryptor = crate::media::FfmpegDecryptor::new();
    let download_dir = std::env::current_dir()?;

    info!("Fetching DRM activation bytes...");
    let activation_bytes = client.get_activation_bytes().await?;
    debug!("Activation bytes acquired: {}", activation_bytes);

    for mut book in books_to_download {
        if book.status != crate::state::BookStatus::NotDownloaded {
            debug!("Skipping {} (Already downloaded or decrypted)", book.title);
            continue;
        }

        info!("Requesting download URL for '{}'...", book.title);
        let url = client.get_aax_download_url(&book.id).await?;

        let safe_title = book.title.replace("/", "_").replace(":", " -");
        let safe_author = book.author.replace("/", "_");

        let template = filename
            .clone()
            .unwrap_or_else(|| "{author} - {title}".to_string());
        let mut final_name = template;
        final_name = final_name.replace("{title}", &safe_title);
        final_name = final_name.replace("{author}", &safe_author);
        final_name = final_name.replace("{asin}", &book.id);

        let safe_series = book
            .series_title
            .as_deref()
            .unwrap_or("")
            .replace("/", "_")
            .replace(":", " -");
        let safe_book_num = book
            .series_sequence
            .as_deref()
            .unwrap_or("")
            .replace("/", "_");

        final_name = final_name.replace("{series}", &safe_series);
        final_name = final_name.replace("{book_number}", &safe_book_num);
        final_name = final_name.trim().to_string();

        let dir_path = if no_folder {
            download_dir.clone()
        } else {
            let mut p = download_dir.clone();
            p.push(&final_name);
            p
        };

        std::fs::create_dir_all(&dir_path)?;

        let aax_file_name = format!("{}.aax", final_name);
        let m4b_file_name = format!("{}.m4b", final_name);

        let aax_path = dir_path.join(&aax_file_name);
        let m4b_path = dir_path.join(&m4b_file_name);

        println!("Downloading {}...", aax_file_name);
        downloader
            .download(&url, &aax_path)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        println!("Download complete! Decrypting to {}...", m4b_file_name);
        use crate::media::Decryptor;
        decryptor
            .decrypt(&aax_path, &m4b_path, &activation_bytes)
            .map_err(|e| anyhow::anyhow!(e))?;

        info!("Decryption of {} complete", safe_title);

        if !no_cue {
            debug!("Generating CUE file...");
            let cue_path = dir_path.join(format!("{}.cue", final_name));
            if let Err(e) = decryptor.extract_cue(
                &m4b_path,
                &cue_path,
                &book.title,
                &book.author,
                &m4b_file_name,
            ) {
                tracing::warn!("Failed to generate CUE file: {}", e);
            }
        }

        // Update state
        book.status = crate::state::BookStatus::Decrypted;
        state.upsert_book(book);
        state.save(&library_file).map_err(|e| anyhow::anyhow!(e))?;
    }

    Ok(())
}

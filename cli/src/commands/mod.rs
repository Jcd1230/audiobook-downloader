use crate::cli::Commands;

use std::path::PathBuf;

fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("audiobook-downloader");
    std::fs::create_dir_all(&path).unwrap();
    path
}

pub async fn handle(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Auth => auth().await,
        Commands::Sync => sync().await,
        Commands::List => list().await,
        Commands::Info { id } => info(&id).await,
        Commands::Download { id, all } => download(id.as_deref(), all).await,
        Commands::Config => config().await,
    }
}

async fn auth() -> anyhow::Result<()> {
    println!("Starting native Audible authentication flow...");
    let auth_info = audible_api::auth::login_with_browser().await?;
    
    let out_json = serde_json::to_string_pretty(&auth_info)?;
    let auth_path = get_config_dir().join("auth.json");
    std::fs::write(&auth_path, out_json)?;
    
    println!("Successfully authenticated! Credentials saved natively to {}", auth_path.display());
    Ok(())
}

async fn sync() -> anyhow::Result<()> {
    println!("Syncing library...");
    
    let auth_path = get_config_dir().join("auth.json");
    let token_data = std::fs::read_to_string(&auth_path)
        .map_err(|_| anyhow::anyhow!("Please run 'audiobook-downloader auth' first."))?;
        
    let mut auth: audible_api::auth::AuthInfo = serde_json::from_str(&token_data)?;

    if auth.is_expired() {
        println!("Access token expired. Refreshing...");
        auth.refresh_access_token().await?;
        
        let out_json = serde_json::to_string_pretty(&auth)?;
        std::fs::write(&auth_path, out_json)?;
        println!("Refreshed successfully!");
    }

    let client = audible_api::Client::new(auth);
    
    println!("Fetching activation bytes...");
    let act_bytes = client.get_activation_bytes().await?;
    println!("Activation Bytes: {}", act_bytes);

    println!("Fetching library...");
    let library = client.get_library().await?;
    
    let library_file = get_config_dir().join("library.json");
    let mut state = crate::state::LibraryState::load(&library_file)?;
    
    for item in library {
        let authors = item.authors.map(|a| a.into_iter().map(|c| c.name).collect::<Vec<_>>().join(", ")).unwrap_or_default();
        let narrators = item.narrators.map(|n| n.into_iter().map(|c| c.name).collect::<Vec<_>>().join(", ")).unwrap_or_default();
        let narrator_opt = if narrators.is_empty() { None } else { Some(narrators) };
        
        let book = crate::state::Book {
            id: item.asin,
            title: item.title,
            author: authors,
            narrator: narrator_opt,
            duration_seconds: item.runtime_length_min.map(|m| m * 60),
            status: crate::state::BookStatus::NotDownloaded,
        };
        state.upsert_book(book);
    }
    
    state.save(&library_file)?;
    println!("Successfully synced {} books to your local library state.", state.books.len());

    Ok(())
}

async fn list() -> anyhow::Result<()> {
    println!("Listing locally cached books...");
    Ok(())
}

async fn info(id: &str) -> anyhow::Result<()> {
    println!("Info for book {}", id);
    Ok(())
}

async fn download(id: Option<&str>, all: bool) -> anyhow::Result<()> {
    let auth_path = get_config_dir().join("auth.json");
    let token_data = std::fs::read_to_string(&auth_path)
        .map_err(|_| anyhow::anyhow!("Please run 'audiobook-downloader auth' first."))?;
        
    let mut auth: audible_api::auth::AuthInfo = serde_json::from_str(&token_data)?;
    if auth.is_expired() {
        auth.refresh_access_token().await?;
        std::fs::write(&auth_path, serde_json::to_string_pretty(&auth)?)?;
    }
    
    let client = audible_api::Client::new(auth);
    let library_file = get_config_dir().join("library.json");
    let mut state = crate::state::LibraryState::load(&library_file)?;
    
    let mut books_to_download = Vec::new();

    if all {
        println!("Finding all missing books...");
        for book in state.books.values() {
            if book.status == crate::state::BookStatus::NotDownloaded {
                books_to_download.push(book.clone());
            }
        }
    } else if let Some(book_id) = id {
        if let Some(book) = state.get_book(book_id) {
            books_to_download.push(book.clone());
        } else {
            anyhow::bail!("Book {} not found in library. Did you run sync first?", book_id);
        }
    } else {
        anyhow::bail!("Please specify a book ID or use --all.");
    }
    
    if books_to_download.is_empty() {
        println!("No books to download.");
        return Ok(());
    }
    
    println!("Found {} books to download.", books_to_download.len());
    
    let downloader = crate::download::Downloader::new();
    let decryptor = crate::media::FfmpegDecryptor::new();
    let download_dir = std::env::current_dir()?; // We can make this configurable later
    
    // Fetch activation bytes once for the batch
    println!("Fetching DRM activation bytes...");
    let activation_bytes = client.get_activation_bytes().await?;
    println!("Activation bytes acquired: {}", activation_bytes);
    
    for mut book in books_to_download {
        if book.status != crate::state::BookStatus::NotDownloaded {
            println!("Skipping {} (Already downloaded or decrypted)", book.title);
            continue;
        }

        println!("Requesting download URL for '{}'...", book.title);
        let url = client.get_aax_download_url(&book.id).await?;
        
        let safe_title = book.title.replace("/", "_").replace(":", " -");
        let safe_author = book.author.replace("/", "_");
        let aax_file_name = format!("{} - {}.aax", safe_author, safe_title);
        let m4b_file_name = format!("{} - {}.m4b", safe_author, safe_title);
        
        let aax_path = download_dir.join(&aax_file_name);
        let m4b_path = download_dir.join(&m4b_file_name);
        
        println!("Downloading {}...", aax_file_name);
        downloader.download(&url, &aax_path).await?;
        
        println!("Download complete! Decrypting to {}...", m4b_file_name);
        use crate::media::Decryptor;
        decryptor.decrypt(&aax_path, &m4b_path, &activation_bytes)?;
        
        println!("Decryption of {} complete!", safe_title);
        
        // Update state
        book.status = crate::state::BookStatus::Decrypted;
        state.upsert_book(book);
        state.save(&library_file)?;
    }

    Ok(())
}

async fn config() -> anyhow::Result<()> {
    println!("Config management...");
    Ok(())
}

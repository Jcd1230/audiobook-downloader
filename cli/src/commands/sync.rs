use crate::commands::utils::get_config_dir;
use crate::error::{CLIError, Result};
use tracing::{debug, info};

pub async fn sync() -> Result<()> {
    info!("Syncing library...");

    let auth_path = get_config_dir().join("auth.json");
    let token_data = std::fs::read_to_string(&auth_path).map_err(|_| CLIError::AuthExpired)?;

    let mut auth: audible_api::auth::AuthInfo =
        serde_json::from_str(&token_data).map_err(|e| anyhow::anyhow!(e))?;

    if auth.is_expired() {
        info!("Access token expired. Refreshing...");
        auth.refresh_access_token().await?;

        let out_json = serde_json::to_string_pretty(&auth).map_err(|e| anyhow::anyhow!(e))?;
        std::fs::write(&auth_path, out_json)?;
        debug!("Refreshed successfully!");
    }

    let client = audible_api::Client::new(auth);

    info!("Fetching activation bytes...");
    let act_bytes = client.get_activation_bytes().await?;
    debug!("Activation Bytes: {}", act_bytes);

    info!("Fetching library...");
    let library = client.get_library().await?;

    let library_file = get_config_dir().join("library.json");
    let mut state =
        crate::state::LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    for item in library {
        let authors = item
            .authors
            .map(|a| a.into_iter().map(|c| c.name).collect::<Vec<_>>().join(", "))
            .unwrap_or_default();
        let narrators = item
            .narrators
            .map(|n| n.into_iter().map(|c| c.name).collect::<Vec<_>>().join(", "))
            .unwrap_or_default();
        let narrator_opt = if narrators.is_empty() {
            None
        } else {
            Some(narrators)
        };

        let (series_title, series_sequence) = if let Some(series_list) = item.series {
            if let Some(first_series) = series_list.into_iter().next() {
                (first_series.title, first_series.sequence)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let book = crate::state::Book {
            id: item.asin,
            title: item.title,
            author: authors,
            narrator: narrator_opt,
            series_title,
            series_sequence,
            duration_seconds: item.runtime_length_min.map(|m| m * 60),
            status: crate::state::BookStatus::NotDownloaded,
        };
        state.upsert_book(book);
    }

    state.save(&library_file).map_err(|e| anyhow::anyhow!(e))?;
    println!(
        "Successfully synced {} books to your local library state.",
        state.books.len()
    );

    Ok(())
}

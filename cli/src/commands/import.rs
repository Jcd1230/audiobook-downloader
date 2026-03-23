use crate::commands::utils::get_config_dir;
use crate::config::Config;
use crate::error::{Result, CLIError};
use crate::state::LibraryState;
use regex::Regex;
use std::collections::HashSet;
use tracing::{info, debug};
use walkdir::WalkDir;

pub async fn import(config: Config) -> Result<()> {
    let library_path = config.library_path.ok_or_else(|| {
        anyhow::anyhow!("library_path not set. Run 'audiobook-downloader config set library_path <PATH>' first.")
    })?;

    let library_file = get_config_dir().join("library.json");
    let mut state = LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    if state.books.is_empty() {
        return Err(CLIError::EmptyLibrary);
    }

    info!("Scanning directory: {}", library_path);
    
    // Regex for ASIN: [B0...] or [ISBN-10]
    let asin_re = Regex::new(r"\[([A-Z0-9]{10})\]").unwrap();
    let mut found_asins = HashSet::new();

    for entry in WalkDir::new(&library_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "m4b") {
            let path_str = entry.path().to_string_lossy();
            if let Some(caps) = asin_re.captures(&path_str) {
                let asin = caps.get(1).unwrap().as_str();
                found_asins.insert(asin.to_string());
                debug!("Found ASIN {} in path: {}", asin, path_str);
            }
        }
    }

    let mut updated_count = 0;
    println!("Found {} unique ASINs in filesystem. Updating database...", found_asins.len());

    for asin in found_asins {
        if let Some(book) = state.books.get_mut(&asin) {
            if book.status != crate::state::BookStatus::Decrypted {
                book.status = crate::state::BookStatus::Decrypted;
                updated_count += 1;
                debug!("Marked book '{}' ({}) as Decrypted", book.title, book.id);
            }
        }
    }

    if updated_count > 0 {
        state.save(&library_file).map_err(|e| anyhow::anyhow!(e))?;
    }

    println!("Import complete!");
    println!("  - New books marked as decrypted: {}", updated_count);

    Ok(())
}

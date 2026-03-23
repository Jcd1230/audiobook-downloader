use crate::commands::utils::get_config_dir;
use crate::config::Config;
use crate::error::{CLIError, Result};
use crate::state::LibraryState;
use regex::Regex;
use std::collections::HashSet;
use tracing::{debug, info};
use walkdir::WalkDir;

pub async fn import(mut config: Config, path: Option<String>) -> Result<()> {
    let mut auto_save_config = false;
    let library_path = if let Some(p) = path {
        p
    } else if let Some(ref p) = config.library_path {
        p.clone()
    } else {
        debug!("library_path not set, falling back to current directory");
        ".".to_string()
    };

    let library_file = get_config_dir().join("library.json");
    let mut state = LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    if state.books.is_empty() {
        return Err(CLIError::EmptyLibrary);
    }

    let absolute_path = std::fs::canonicalize(&library_path)
        .map_err(|e| anyhow::anyhow!("Failed to resolve path '{}': {}", library_path, e))?;
    let path_str = absolute_path.to_string_lossy().to_string();

    info!("Scanning directory: {}", path_str);

    // Regex for ASIN: [B0...] or [ISBN-10]
    let asin_re = Regex::new(r"\[([A-Z0-9]{10})\]").unwrap();
    let mut found_asins = HashSet::new();

    for entry in WalkDir::new(&path_str).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "m4b") {
            let entry_path_str = entry.path().to_string_lossy();
            if let Some(caps) = asin_re.captures(&entry_path_str) {
                let asin = caps.get(1).unwrap().as_str();
                found_asins.insert(asin.to_string());
                debug!("Found ASIN {} in path: {}", asin, entry_path_str);
            }
        }
    }

    let mut updated_count = 0;
    println!(
        "Found {} unique ASINs in filesystem. Updating database...",
        found_asins.len()
    );

    for asin in &found_asins {
        if let Some(book) = state.books.get_mut(asin) {
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

    if !found_asins.is_empty() && config.library_path.is_none() {
        auto_save_config = true;
    }

    if auto_save_config {
        config.library_path = Some(path_str.clone());
        config.save().map_err(|e| anyhow::anyhow!(e))?;
        println!(
            "✨ Detected books in {}. Saving this as your default library_path.",
            path_str
        );
    }

    println!("Import complete!");
    println!("  - New books marked as decrypted: {}", updated_count);

    Ok(())
}

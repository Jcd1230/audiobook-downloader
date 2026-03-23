use crate::commands::utils::{format_book_line, get_config_dir};
use crate::error::{CLIError, Result};
use tracing::info;

pub async fn list(json: bool) -> Result<()> {
    let library_file = get_config_dir().join("library.json");
    let state = crate::state::LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    let mut books: Vec<_> = state.books.values().collect();
    books.sort_by(|a, b| {
        let title_a = a.title.to_lowercase();
        let title_b = b.title.to_lowercase();
        title_a.cmp(&title_b).then_with(|| a.id.cmp(&b.id))
    });

    if json {
        if books.is_empty() {
            eprintln!("No books in local library. Run 'sync' first.");
            println!("[]");
        } else {
            println!(
                "{}",
                serde_json::to_string_pretty(&books).map_err(|e| anyhow::anyhow!(e))?
            );
        }
    } else {
        info!("Listing locally cached books...");
        if books.is_empty() {
            return Err(CLIError::EmptyLibrary);
        } else {
            println!("Found {} books in local library:", books.len());
            for book in books {
                println!("{}", format_book_line(book));
            }
        }
    }
    Ok(())
}

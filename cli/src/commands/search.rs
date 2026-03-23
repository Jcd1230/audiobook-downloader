use crate::commands::utils::{format_book_line, get_config_dir};
use crate::error::Result;
use tracing::info;

pub async fn search(query: &str, _yes: bool) -> Result<()> {
    let library_file = get_config_dir().join("library.json");
    let state = crate::state::LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    let results = state.search(query);

    if results.is_empty() {
        println!("No books found matching '{}'", query);
    } else {
        info!("Found {} matching books", results.len());
        for book in results {
            println!("{}", format_book_line(&book));
        }
    }
    Ok(())
}

use crate::commands::utils::{format_book_line, get_config_dir};
use crate::error::Result;
use tracing::info;

pub async fn search(query: &str, yes: bool) -> Result<()> {
    let library_file = get_config_dir().join("library.json");
    let state = crate::state::LibraryState::load(&library_file).map_err(|e| anyhow::anyhow!(e))?;

    let results = state.search(query);

    if results.is_empty() {
        println!("No books found matching '{}'", query);
    } else {
        println!("Found {} matching books:", results.len());
        for book in &results {
            println!("{}", format_book_line(book));
        }

        if results.len() > 1 && !yes {
            use inquire::Select;
            let options = results.iter().map(|b| format!("{} ({})", b.title, b.id)).collect::<Vec<_>>();
            let ans = Select::new("Select a book to view info:", options).prompt();

            match ans {
                Ok(choice) => {
                    let index = results.iter().position(|b| format!("{} ({})", b.title, b.id) == choice).unwrap();
                    let book = &results[index];
                    println!("\nDetailed Info for: {}", book.title);
                    println!("ASIN: {}", book.id);
                    println!("Author: {}", book.author);
                    if let Some(ref n) = book.narrator { println!("Narrator: {}", n); }
                    if let Some(ref s) = book.series_title { println!("Series: {} #{}", s, book.series_sequence.as_deref().unwrap_or("?")); }
                    if let Some(d) = book.duration_seconds { println!("Duration: {}h {}m", d/3600, (d%3600)/60); }
                    println!("Status: {:?}", book.status);
                }
                Err(_) => println!("Selection cancelled."),
            }
        }
    }
    Ok(())
}


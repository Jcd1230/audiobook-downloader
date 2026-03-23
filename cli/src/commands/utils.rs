use colored::*;
use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("audiobook-downloader");
    let _ = std::fs::create_dir_all(&path);
    path
}

pub fn format_book_line(book: &crate::state::Book) -> String {
    let status_str = match book.status {
        crate::state::BookStatus::NotDownloaded => "[NotDownloaded]",
        crate::state::BookStatus::Downloading => "[Downloading  ]",
        crate::state::BookStatus::Downloaded => "[Downloaded   ]",
        crate::state::BookStatus::Decrypted => "[Decrypted    ]",
    };

    let status_label = match book.status {
        crate::state::BookStatus::NotDownloaded => status_str.dimmed(),
        crate::state::BookStatus::Downloading => status_str.blue(),
        crate::state::BookStatus::Downloaded => status_str.cyan(),
        crate::state::BookStatus::Decrypted => status_str.green(),
    };

    let title = book.title.bold().white();

    let mut line = format!("{} {}", status_label, title);

    if !book.author.is_empty() {
        line.push_str(&format!(
            " {} {}",
            "·".dimmed(),
            book.author.italic().dimmed()
        ));
    }

    line.push_str(&format!(" {}", format!("({})", book.id).dimmed()));

    line
}

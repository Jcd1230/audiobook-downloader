use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryState {
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub narrator: Option<String>,
    pub duration_seconds: Option<u64>,
    pub status: BookStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BookStatus {
    NotDownloaded,
    Downloading,
    Downloaded,    // Has DRM
    Decrypted,     // Ready to play
}

impl Default for LibraryState {
    fn default() -> Self {
        Self { books: Vec::new() }
    }
}

impl LibraryState {
    /// Loads the library state from a JSON file. Returns a default state if the file doesn't exist.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let data = fs::read_to_string(path)?;
        let state: Self = serde_json::from_str(&data)?;
        Ok(state)
    }

    /// Saves the current library state to a JSON file.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Adds or updates a book in the library.
    pub fn upsert_book(&mut self, book: Book) {
        if let Some(existing) = self.books.iter_mut().find(|b| b.id == book.id) {
            *existing = book;
        } else {
            self.books.push(book);
        }
    }

    /// Retrieves a book by ID.
    pub fn get_book(&self, id: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.id == id)
    }
}

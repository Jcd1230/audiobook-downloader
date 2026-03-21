use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryState {
    pub books: HashMap<String, Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub narrator: Option<String>,
    pub series_title: Option<String>,
    pub series_sequence: Option<String>,
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
        Self { books: HashMap::new() }
    }
}

impl LibraryState {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        let state: Self = serde_json::from_str(&data)?;
        Ok(state)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn upsert_book(&mut self, mut book: Book) {
        if let Some(existing) = self.books.get(&book.id) {
            // Do not override a downloaded status just because we did a sync!
            book.status = existing.status.clone();
        }
        self.books.insert(book.id.clone(), book);
    }

    pub fn get_book(&self, id: &str) -> Option<&Book> {
        self.books.get(id)
    }
}

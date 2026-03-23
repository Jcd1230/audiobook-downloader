use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    Downloaded, // Has DRM
    Decrypted,  // Ready to play
    Unavailable, // License denied
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

    pub fn search(&self, query: &str) -> Vec<Book> {
        let lower_query = query.to_lowercase();
        let mut exact_id_matches = Vec::new();
        let mut exact_title_matches = Vec::new();
        let mut substring_matches = Vec::new();

        for book in self.books.values() {
            if book.id.to_lowercase() == lower_query {
                exact_id_matches.push(book.clone());
            } else if book.title.to_lowercase() == lower_query {
                exact_title_matches.push(book.clone());
            } else if book.title.to_lowercase().contains(&lower_query) {
                substring_matches.push(book.clone());
            }
        }

        if !exact_id_matches.is_empty() {
            exact_id_matches
        } else if !exact_title_matches.is_empty() {
            exact_title_matches
        } else {
            substring_matches
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_book(id: &str, title: &str, author: &str) -> Book {
        Book {
            id: id.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            narrator: None,
            series_title: None,
            series_sequence: None,
            duration_seconds: None,
            status: BookStatus::NotDownloaded,
        }
    }

    #[test]
    fn test_search_exact_id() {
        let mut books = HashMap::new();
        books.insert(
            "B01".to_string(),
            create_test_book("B01", "Book One", "Author A"),
        );
        books.insert(
            "B02".to_string(),
            create_test_book("B02", "Book Two", "Author B"),
        );
        let state = LibraryState { books };

        let results = state.search("B01");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "B01");
    }

    #[test]
    fn test_search_exact_title() {
        let mut books = HashMap::new();
        books.insert(
            "B01".to_string(),
            create_test_book("B01", "Book One", "Author A"),
        );
        books.insert(
            "B02".to_string(),
            create_test_book("B02", "Book Two", "Author B"),
        );
        let state = LibraryState { books };

        let results = state.search("Book Two");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Book Two");
    }

    #[test]
    fn test_search_substring() {
        let mut books = HashMap::new();
        books.insert(
            "B01".to_string(),
            create_test_book("B01", "Rust Programming", "Author A"),
        );
        books.insert(
            "B02".to_string(),
            create_test_book("B02", "Go Programming", "Author B"),
        );
        let state = LibraryState { books };

        let results = state.search("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "B01");
    }
}

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A client for communicating with the Audiobook provider's API.
pub struct ApiClient {
    client: reqwest::Client,
    auth_token: Option<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            auth_token: None,
        }
    }

    /// Set the authentication token for future requests.
    pub fn set_auth_token(&mut self, token: String) {
        self.auth_token = Some(token);
    }

    /// Authenticate using username and password (stub).
    pub async fn authenticate(&mut self, _username: &str, _password: &str) -> Result<()> {
        println!("[API] Authenticating...");
        // TODO: Implement actual auth flow
        self.auth_token = Some("dummy_token".to_string());
        Ok(())
    }

    /// Sync library by fetching all owned books (stub).
    pub async fn fetch_library(&self) -> Result<Vec<crate::state::Book>> {
        println!("[API] Fetching library metadata...");
        // TODO: Implement actual metadata fetch
        
        let dummy_book = crate::state::Book {
            id: "B012345678".to_string(),
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik & Carol Nichols".to_string(),
            narrator: Some("Community".to_string()),
            series_title: None,
            series_sequence: None,
            duration_seconds: Some(36000),
            status: crate::state::BookStatus::NotDownloaded,
        };
        
        Ok(vec![dummy_book])
    }
}

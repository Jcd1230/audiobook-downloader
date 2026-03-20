use anyhow::Result;
use std::path::Path;

/// A struct that manages concurrent chunked downloading for large audiobooks.
pub struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Downloads the file from `url` into `output_path`.
    /// 
    /// This is currently a stub for chunked downloading.
    pub async fn download(&self, url: &str, output_path: &Path) -> Result<()> {
        println!("Starting download from {} to {:?}", url, output_path);
        
        let mut response = self.client.get(url).send().await?.error_for_status()?;
        
        let mut file = tokio::fs::File::create(output_path).await?;
        
        while let Some(chunk) = response.chunk().await? {
            tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
        }
        
        Ok(())
    }
}

use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::ProgressBar;
use reqwest::Client;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub struct Downloader {
    client: Client,
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

impl Downloader {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Audible/671 CFNetwork/1240.0.4 Darwin/20.6.0")
            .build()
            .unwrap_or_default();
        Self { client }
    }

    pub async fn download_with_pb(
        &self,
        url: &str,
        output_path: &Path,
        pb: ProgressBar,
    ) -> Result<()> {
        let response = self.client.get(url).send().await?.error_for_status()?;

        let total_size = response.content_length();
        if let Some(size) = total_size {
            pb.set_length(size);
        }

        let part_path = output_path.with_extension(format!(
            "{}.part",
            output_path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
        ));

        let mut file = tokio::fs::File::create(&part_path)
            .await
            .context("Failed to create part file")?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Error while downloading chunk")?;
            file.write_all(&chunk)
                .await
                .context("Error while writing chunk")?;

            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");

        // Atomically rename .part to final
        tokio::fs::rename(&part_path, output_path)
            .await
            .context("Failed to rename part file to final path")?;

        Ok(())
    }
}

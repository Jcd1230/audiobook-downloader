use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::{info, debug, warn};
use async_trait::async_trait;

#[async_trait]
pub trait Decryptor {
    async fn decrypt(&self, input: &Path, output: &Path, activation_bytes: &str, book: &crate::state::Book) -> Result<()>;
    fn extract_cue(
        &self,
        input: &Path,
        output: &Path,
        title: &str,
        author: &str,
        m4b_filename: &str,
    ) -> Result<()>;
}

pub struct FfmpegDecryptor;

impl Default for FfmpegDecryptor {
    fn default() -> Self {
        Self::new()
    }
}

impl FfmpegDecryptor {
    pub fn new() -> Self {
        Self
    }

    async fn download_cover(&self, url: &str) -> Result<tempfile::NamedTempFile> {
        debug!("Downloading cover art from: {}", url);
        let resp = reqwest::get(url).await?.error_for_status()?;
        let bytes = resp.bytes().await?;
        
        let temp = tempfile::NamedTempFile::new()?;
        let mut file = tokio::fs::File::from_std(temp.reopen()?);
        use tokio::io::AsyncWriteExt;
        file.write_all(&bytes).await?;
        Ok(temp)
    }
}

#[async_trait]
impl Decryptor for FfmpegDecryptor {
    async fn decrypt(&self, input: &Path, output: &Path, activation_bytes: &str, book: &crate::state::Book) -> Result<()> {
        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y")
           .arg("-activation_bytes")
           .arg(activation_bytes);

        // Input cover if available
        let mut cover_file = None;
        if let Some(ref cover_url) = book.cover_url {
            match self.download_cover(cover_url).await {
                Ok(temp) => {
                    cmd.arg("-i").arg(temp.path());
                    cover_file = Some(temp);
                }
                Err(e) => {
                    warn!("Failed to download cover art: {}. Proceeding without it.", e);
                }
            }
        }

        cmd.arg("-i").arg(input);
        
        // Metadata
        cmd.arg("-metadata").arg(format!("title={}", book.title))
           .arg("-metadata").arg(format!("artist={}", book.author))
           .arg("-metadata").arg(format!("comment={}", book.id));
        
        if let Some(ref series) = book.series_title {
            cmd.arg("-metadata").arg(format!("album={}", series));
        }

        if cover_file.is_some() {
            // Map the second input (the image) to the first video stream and dispose it as a picture
            cmd.arg("-map").arg("1:0")
               .arg("-map").arg("0:0")
               .arg("-disposition:v:0").arg("attached_pic");
        }

        cmd.arg("-c").arg("copy")
           .arg(output);

        debug!("Running ffmpeg: {:?}", cmd);
        let status = cmd.status()
            .context("Failed to spawn ffmpeg. Is it installed and in your PATH?")?;

        if !status.success() {
            anyhow::bail!(
                "ffmpeg failed to decrypt the audiobook with status: {}",
                status
            );
        }

        // Clean up the original encrypted .aax file
        std::fs::remove_file(input)
            .context("Failed to clean up original .aax file after decryption")?;

        Ok(())
    }

    fn extract_cue(
        &self,
        input: &Path,
        output: &Path,
        title: &str,
        author: &str,
        m4b_filename: &str,
    ) -> Result<()> {
        let output_ffprobe = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_chapters")
            .arg("-print_format")
            .arg("json")
            .arg(input)
            .output()
            .context("Failed to spawn ffprobe to generate CUE. Is it installed?")?;

        if !output_ffprobe.status.success() {
            anyhow::bail!("ffprobe failed with status: {}", output_ffprobe.status);
        }

        let stdout_str = String::from_utf8_lossy(&output_ffprobe.stdout);
        let parsed: serde_json::Value =
            serde_json::from_str(&stdout_str).context("Failed to parse ffprobe json output")?;

        let mut cue_data = String::new();
        cue_data.push_str(&format!("TITLE \"{}\"\n", title));
        cue_data.push_str(&format!("PERFORMER \"{}\"\n", author));
        cue_data.push_str(&format!("FILE \"{}\" MP4\n", m4b_filename));

        if let Some(chapters) = parsed["chapters"].as_array() {
            for (i, chapter) in chapters.iter().enumerate() {
                let track_num = i + 1;
                let fallback_title = format!("Chapter {}", track_num);
                let ch_title = chapter["tags"]["title"].as_str().unwrap_or(&fallback_title);

                // CUE supports MM:SS:FF. ffprobe start_time is seconds as string
                let start_time_str = chapter["start_time"].as_str().unwrap_or("0.000000");
                let start_time_f: f64 = start_time_str.parse().unwrap_or(0.0);

                let mins = (start_time_f / 60.0).floor() as u64;
                let secs = (start_time_f % 60.0).floor() as u64;
                // Frames are typically 1/75 of a second for CUE formatting.
                let frames = ((start_time_f.fract() * 75.0).round()) as u64;

                cue_data.push_str(&format!("  TRACK {:0>2} AUDIO\n", track_num));
                cue_data.push_str(&format!("    TITLE \"{}\"\n", ch_title));
                cue_data.push_str(&format!(
                    "    INDEX 01 {:0>2}:{:0>2}:{:0>2}\n",
                    mins, secs, frames
                ));
            }
        }

        std::fs::write(output, cue_data).context("Failed to write .cue file")?;

        Ok(())
    }
}

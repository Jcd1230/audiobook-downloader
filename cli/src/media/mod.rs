use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub trait Decryptor {
    fn decrypt(&self, input: &Path, output: &Path, activation_bytes: &str) -> Result<()>;
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
}

impl Decryptor for FfmpegDecryptor {
    fn decrypt(&self, input: &Path, output: &Path, activation_bytes: &str) -> Result<()> {
        let status = Command::new("ffmpeg")
            .arg("-y") // Overwrite output files without asking
            .arg("-activation_bytes")
            .arg(activation_bytes)
            .arg("-i")
            .arg(input)
            .arg("-c")
            .arg("copy")
            .arg(output)
            .status()
            .context("Failed to spawn ffmpeg. Is it installed and in your PATH?")?;

        if !status.success() {
            anyhow::bail!("ffmpeg failed to decrypt the audiobook with status: {}", status);
        }
        
        // Clean up the original encrypted .aax file
        std::fs::remove_file(input).context("Failed to clean up original .aax file after decryption")?;

        Ok(())
    }
}

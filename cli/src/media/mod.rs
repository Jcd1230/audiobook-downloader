use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// A trait for decrypting audiobook files.
pub trait Decryptor {
    /// Decrypts a file from `input_path` to `output_path`.
    fn decrypt(&self, input_path: &Path, output_path: &Path, activation_bytes: &str) -> Result<()>;
}

/// Uses an external `ffmpeg` process to decrypt `.aax` / `.aaxc` files.
pub struct FfmpegDecryptor;

impl Decryptor for FfmpegDecryptor {
    fn decrypt(&self, input_path: &Path, output_path: &Path, activation_bytes: &str) -> Result<()> {
        // Run ffmpeg -activation_bytes <BYTES> -i <IN> -c copy <OUT>
        let status = Command::new("ffmpeg")
            .arg("-activation_bytes")
            .arg(activation_bytes)
            .arg("-i")
            .arg(input_path)
            .arg("-c")
            .arg("copy")
            .arg(output_path)
            .status()?;

        if !status.success() {
            anyhow::bail!("ffmpeg failed to decrypt the file");
        }

        Ok(())
    }
}

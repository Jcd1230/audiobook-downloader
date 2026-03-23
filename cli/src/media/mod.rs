use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub trait Decryptor {
    fn decrypt(&self, input: &Path, output: &Path, activation_bytes: &str) -> Result<()>;
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

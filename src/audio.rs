use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tracing::{info, debug, warn};
use std::ffi::OsStr;

/// Supported video/audio formats for input
const SUPPORTED_FORMATS: &[&str] = &[
    "mp4", "avi", "mov", "mkv", "wmv", "flv", "webm", "ogv", "3gp", "m4v",
    "vob", "ts", "mpg", "mpeg", "mp3", "wav", "flac", "aac", "ogg", "m4a"
];

/// Extracts audio from a video file and returns the path to the extracted audio file
pub async fn extract_audio(video_path: &Path) -> Result<PathBuf> {
    // Validate input file exists
    if !video_path.exists() {
        anyhow::bail!("Input file does not exist: {:?}", video_path);
    }

    // Check if format is supported (optional warning, FFmpeg will try anyway)
    if let Some(extension) = video_path.extension().and_then(|e| e.to_str()) {
        if !SUPPORTED_FORMATS.contains(&extension.to_lowercase().as_str()) {
            warn!("File extension '{}' is not in common supported formats list, but FFmpeg will attempt to process it", extension);
        }
    }

    let output_path = create_temp_audio_path(video_path);
    
    debug!("Extracting audio from {:?} to {:?}", video_path, output_path);
    
    // Use ffmpeg to extract audio from video
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg("-vn") // No video
        .arg("-acodec")
        .arg("pcm_s16le") // 16-bit PCM (compatible with Whisper)
        .arg("-ar")
        .arg("16000") // 16kHz sample rate (optimal for Whisper)
        .arg("-ac")
        .arg("1") // Mono (Whisper works best with mono)
        .arg("-y") // Overwrite output file if it exists
        .arg("-hide_banner") // Reduce FFmpeg output verbosity
        .arg("-loglevel")
        .arg("error") // Only show errors
        .arg(&output_path)
        .output()
        .await
        .context("Failed to execute ffmpeg command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("FFmpeg failed to extract audio: {}", stderr);
    }

    // Verify output file was created and has content
    if !output_path.exists() {
        anyhow::bail!("Audio extraction failed: output file was not created");
    }

    let metadata = tokio::fs::metadata(&output_path)
        .await
        .context("Failed to read audio file metadata")?;
    
    if metadata.len() == 0 {
        anyhow::bail!("Audio extraction failed: output file is empty");
    }

    info!("✅ Audio extraction completed successfully ({} bytes)", metadata.len());
    Ok(output_path)
}

/// Creates a temporary path for the extracted audio file
fn create_temp_audio_path(video_path: &Path) -> PathBuf {
    let stem = video_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("audio_extract");
    
    let temp_dir = std::env::temp_dir();
    let audio_filename = format!("{stem}_{}.wav", chrono::Utc::now().timestamp());
    
    temp_dir.join(audio_filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_audio_path_creation() {
        let video_path = PathBuf::from("/path/to/wayne_dyer_video.mp4");
        let audio_path = create_temp_audio_path(&video_path);
        
        assert!(audio_path.file_name().unwrap().to_str().unwrap().starts_with("wayne_dyer_video_"));
        assert!(audio_path.extension().unwrap() == "wav");
    }
}

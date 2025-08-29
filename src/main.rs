use anyhow::{Context, Result};
use clap::{Arg, Command};
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber;

mod audio;
mod transcription;
mod output;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let matches = Command::new("wayne-transcriber")
        .version("0.1.0")
        .author("Wayne Dyer Video Transcriber")
        .about("Transcribes Wayne Dyer videos using OpenAI Whisper")
        .arg(
            Arg::new("input")
                .help("Input video file path")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file path (optional, defaults to input filename with .txt extension)"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Output format: txt, srt, vtt")
                .default_value("txt"),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Whisper model size: tiny, base, small, medium, large")
                .default_value("base"),
        )
        .get_matches();

    let input_path = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_path = match matches.get_one::<String>("output") {
        Some(path) => PathBuf::from(path),
        None => {
            let mut path = input_path.clone();
            path.set_extension("txt");
            path
        }
    };
    let format = matches.get_one::<String>("format").unwrap();
    let model_size = matches.get_one::<String>("model").unwrap();

    info!("Starting Wayne Dyer video transcription...");
    info!("Input: {:?}", input_path);
    info!("Output: {:?}", output_path);
    info!("Format: {}", format);
    info!("Model: {}", model_size);

    // Step 1: Extract audio from video
    info!("Extracting audio from video...");
    let audio_path = audio::extract_audio(&input_path)
        .await
        .context("Failed to extract audio from video")?;

    // Step 2: Transcribe audio using Whisper
    info!("Transcribing audio with Whisper...");
    let transcription = transcription::transcribe_audio(&audio_path, model_size)
        .await
        .context("Failed to transcribe audio")?;

    // Step 3: Save transcription in desired format
    info!("Saving transcription to file...");
    output::save_transcription(&transcription, &output_path, format)
        .await
        .context("Failed to save transcription")?;

    info!("✅ Transcription completed successfully!");
    info!("Output saved to: {:?}", output_path);

    // Cleanup temporary audio file
    if let Err(e) = tokio::fs::remove_file(&audio_path).await {
        warn!("Could not clean up temporary audio file: {}", e);
    }

    Ok(())
}

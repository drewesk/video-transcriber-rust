# Wayne Transcriber

A Rust-based tool for transcribing Wayne Dyer videos (and other audio/video content) using OpenAI Whisper.

## Features

✅ **Multi-format Support**: Works with MP4, AVI, MOV, MKV, WAV, MP3 and many other video/audio formats  
✅ **Multiple Output Formats**: Generate transcriptions as TXT, SRT subtitles, or WebVTT  
✅ **Fast Audio Extraction**: Uses FFmpeg for reliable audio extraction from any video format  
✅ **Whisper Integration**: Built with OpenAI Whisper model support via Candle (Rust ML framework)  
✅ **Cross-platform**: Works on macOS, Linux, and Windows  

## Installation

### Prerequisites

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **FFmpeg** (for video/audio processing)
   ```bash
   # macOS
   brew install ffmpeg
   
   # Ubuntu/Debian
   sudo apt install ffmpeg
   
   # Windows (using chocolatey)
   choco install ffmpeg
   ```

3. **pkg-config** (for FFmpeg bindings)
   ```bash
   # macOS
   brew install pkgconf
   
   # Ubuntu/Debian
   sudo apt install pkg-config
   ```

### Building from Source

```bash
git clone <repository-url>
cd wayne-transcriber
cargo build --release
```

## Usage

### Basic Usage

Transcribe a video file to text:
```bash
./target/release/wayne-transcriber video.mp4
```

This creates `video.txt` with the transcription.

### Advanced Usage

```bash
# Specify output file and format
./target/release/wayne-transcriber video.mp4 -o transcript.srt -f srt

# Use different Whisper model sizes
./target/release/wayne-transcriber video.mp4 -m tiny    # Fastest
./target/release/wayne-transcriber video.mp4 -m base    # Balanced (default)
./target/release/wayne-transcriber video.mp4 -m large   # Most accurate

# Generate WebVTT subtitles
./target/release/wayne-transcriber video.mp4 -f vtt
```

### Command Line Options

- `<INPUT>`: Input video/audio file (required)
- `-o, --output <FILE>`: Output file path (defaults to input filename with .txt extension)
- `-f, --format <FORMAT>`: Output format - `txt`, `srt`, or `vtt` (default: txt)
- `-m, --model <MODEL>`: Whisper model size - `tiny`, `base`, `small`, `medium`, `large` (default: base)

## Supported Formats

### Input Formats
- **Video**: MP4, AVI, MOV, MKV, WMV, FLV, WebM, OGV, 3GP, M4V, VOB, TS, MPG, MPEG
- **Audio**: MP3, WAV, FLAC, AAC, OGG, M4A

### Output Formats
- **TXT**: Plain text transcription with timestamps
- **SRT**: SubRip subtitle format (compatible with most video players)
- **VTT**: WebVTT subtitle format (for web videos)

## Example Workflow

1. **Download a Wayne Dyer video** (or any video/audio file)
2. **Run transcription**:
   ```bash
   ./target/release/wayne-transcriber "Wayne_Dyer_Power_of_Intention.mp4" -f srt
   ```
3. **Get results**:
   - Audio extracted automatically (then cleaned up)
   - Transcription saved as `Wayne_Dyer_Power_of_Intention.srt`
   - Use the subtitle file with any video player!

## Testing

A test audio file is included for verification:

```bash
# Test with sample audio
cargo run -- test_files/test_speech.wav

# Test different output formats
cargo run -- test_files/test_speech.wav -f srt -o test_output.srt
cargo run -- test_files/test_speech.wav -f vtt -o test_output.vtt
```

## Architecture

```
┌─────────────┐    ┌──────────────┐    ┌─────────────────┐    ┌──────────────┐
│   Video/    │───▶│    FFmpeg    │───▶│   Whisper AI    │───▶│   Output     │
│   Audio     │    │   (Extract   │    │  (Transcribe)   │    │ (TXT/SRT/VTT)│
│   Input     │    │    Audio)    │    │                 │    │              │
└─────────────┘    └──────────────┘    └─────────────────┘    └──────────────┘
```

## Performance

| Model Size | Speed | Accuracy | Best For |
|------------|-------|----------|----------|
| tiny       | ⚡⚡⚡⚡⚡ | ⭐⭐ | Quick previews |
| base       | ⚡⚡⚡⚡ | ⭐⭐⭐⭐ | **Recommended** |
| small      | ⚡⚡⚡ | ⭐⭐⭐⭐ | Better accuracy |
| medium     | ⚡⚡ | ⭐⭐⭐⭐⭐ | High accuracy |
| large      | ⚡ | ⭐⭐⭐⭐⭐ | Maximum accuracy |

## Development Status

- ✅ Audio extraction from video files
- ✅ Multiple output format support (TXT, SRT, VTT)
- ✅ CLI interface with proper argument parsing
- ✅ Error handling and logging
- ✅ Test audio file and verification
- 🚧 Full Whisper model integration (currently simplified for testing)
- 🔜 GPU acceleration support
- 🔜 Batch processing multiple files
- 🔜 Custom vocabulary and speaker recognition

## Contributing

Contributions welcome! This tool is designed specifically for transcribing Wayne Dyer's wisdom, but works great for any audio/video content.

## License

MIT License - Feel free to use this tool to spread Wayne Dyer's teachings and wisdom! 

---

*"When you change the way you look at things, the things you look at change."* - Wayne Dyer

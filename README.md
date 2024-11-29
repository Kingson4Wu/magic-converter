# Magic Converter

A versatile tool for converting MTS video files to MP4 format, featuring both command-line (CLI) and graphical user interface (GUI) modes.

## Features

- Convert single MTS files to MP4
- Batch convert all MTS files in a directory
- Preserve original video quality using FFmpeg's copy mode
- Choose between CLI or GUI interface
- Optional output directory specification

## Prerequisites

- FFmpeg installed on your system
  - For macOS: `brew install ffmpeg`
  - For Linux: `sudo apt-get install ffmpeg`
  - For Windows: Download from [FFmpeg official website](https://ffmpeg.org/download.html)

## Installation

### Option 1: Download Pre-built Binaries (Recommended)

1. Go to the [Releases](https://github.com/yourusername/magic-converter/releases) page
2. Download the archive for your operating system:
   - Windows: `magic-converter-windows-amd64.tar.gz`
   - macOS: `magic-converter-macos-amd64.tar.gz`
   - Linux: `magic-converter-linux-amd64.tar.gz`
3. Extract the archive to get both CLI and GUI executables
4. (Optional) Add the extracted directory to your system's PATH

### Option 2: Build from Source (Requires Rust)

If you want to build from source, you'll need Rust installed on your system.

1. Install Rust from [https://rustup.rs/](https://rustup.rs/)

2. Clone the repository:
```bash
git clone https://github.com/yourusername/magic-converter.git
cd magic-converter
```

3. Build the project:
```bash
# Build both CLI and GUI versions
cargo build --release

# Build only CLI version
cargo build --release --bin magic-converter-cli

# Build only GUI version
cargo build --release --bin magic-converter-gui
```

The executables will be available in `target/release/`:
- CLI version: `magic-converter-cli`
- GUI version: `magic-converter-gui`

## Usage

### GUI Version

To launch the GUI interface:

```bash
# Using cargo
cargo run --bin magic-converter-gui

# Or using the built executable
./target/release/magic-converter-gui
```

The GUI provides:
- Input field for source file/directory path
- Optional output directory field
- Two conversion options:
  - Convert Single File: For converting individual MTS files
  - Convert Directory: For batch converting all MTS files in a directory
- Status message area showing conversion progress and results

### CLI Version

For command-line usage:

```bash
# Using cargo
cargo run --bin magic-converter-cli -- [COMMAND] [OPTIONS]

# Or using the built executable
./target/release/magic-converter-cli [COMMAND] [OPTIONS]
```

Available commands:

#### Converting a Single File

```bash
# Basic usage with default output directory (same as input)
magic-converter-cli convert -i input.mts

# Specify custom output directory
magic-converter-cli convert -i input.mts -o /path/to/output/directory
```

#### Converting Multiple Files in a Directory

```bash
# Basic usage with default output directory (same as input)
magic-converter-cli convert-dir -i /path/to/input/directory

# Specify custom output directory
magic-converter-cli convert-dir -i /path/to/input/directory -o /path/to/output/directory
```

#### Help Commands

For general help:
```bash
magic-converter-cli --help
```

For specific command help:
```bash
magic-converter-cli convert --help
magic-converter-cli convert-dir --help
```

## Technical Details

- Uses FFmpeg for video conversion
- Implements H.264 video stream processing
- Maintains original video and audio quality
- Efficient processing with stream copying when possible
- Built with Rust for performance and safety
- GUI implemented using the Iced framework
- CLI implemented using the Clap framework

## Project Structure

- `src/bin/cli.rs`: CLI binary entry point
- `src/bin/gui.rs`: GUI binary entry point
- `src/command.rs`: CLI command definitions
- `src/service.rs`: Core business logic
- `src/video.rs`: Video conversion implementation
- `src/gui_widget.rs`: GUI widget implementation
- `src/lib.rs`: Library interface and module exports

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [FFmpeg](https://ffmpeg.org/) for video processing
- [Iced](https://iced.rs/) for GUI framework
- [Clap](https://clap.rs/) for CLI argument parsing

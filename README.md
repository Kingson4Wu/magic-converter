# Magic Converter

A versatile tool for converting MTS video files to MP4 format, featuring both command-line (CLI) and graphical user interface (GUI) modes.

## Features

- Convert single MTS files to MP4
- Batch convert all MTS files in a directory
- Preserve original video quality using FFmpeg's copy mode
- User-friendly GUI interface
- Flexible command-line interface
- Optional output directory specification

## Prerequisites

- Rust (latest stable version)
- FFmpeg installed on your system
  - For macOS: `brew install ffmpeg`
  - For Linux: `sudo apt-get install ffmpeg`
  - For Windows: Download from [FFmpeg official website](https://ffmpeg.org/download.html)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/magic-converter.git
cd magic-converter
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available in `target/release/magic-converter`

## Usage

The tool can be used in two modes: Command Line Interface (CLI) or Graphical User Interface (GUI).

### GUI Mode

To launch the GUI interface, simply run the program without any arguments:

```bash
cargo run
```

The GUI provides:
- Input field for source file/directory path
- Optional output directory field
- Two conversion options:
  - Convert Single File: For converting individual MTS files
  - Convert Directory: For batch converting all MTS files in a directory
- Status message area showing conversion progress and results

### CLI Mode

For command-line usage, the following commands are available:

#### Converting a Single File

```bash
# Basic usage with default output directory (same as input)
cargo run -- convert -i input.mts

# Specify custom output directory
cargo run -- convert -i input.mts -o /path/to/output/directory
```

#### Converting Multiple Files in a Directory

```bash
# Basic usage with default output directory (same as input)
cargo run -- convert-dir -i /path/to/input/directory

# Specify custom output directory
cargo run -- convert-dir -i /path/to/input/directory -o /path/to/output/directory
```

#### Help Commands

For general help:
```bash
cargo run -- --help
```

For specific command help:
```bash
# Help for single file conversion
cargo run -- convert --help

# Help for directory conversion
cargo run -- convert-dir --help
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

- `src/main.rs`: Application entry point, handles CLI/GUI mode switching
- `src/command.rs`: CLI command definitions
- `src/service.rs`: Core business logic
- `src/video.rs`: Video conversion implementation
- `src/gui.rs`: GUI implementation
- `src/lib.rs`: Library interface and module exports

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

[MIT License](LICENSE)

## Acknowledgments

- [FFmpeg](https://ffmpeg.org/) for video processing
- [Iced](https://iced.rs/) for GUI framework
- [Clap](https://clap.rs/) for CLI argument parsing

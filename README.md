# Magic Converter

A command-line tool for converting MTS video files to MP4 format using FFmpeg.

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

The tool provides two main commands:

### Converting a Single File

To convert a single MTS file to MP4:

```bash
# Basic usage with default output directory (same as input)
cargo run -- convert -i input.mts

# Specify custom output directory
cargo run -- convert -i input.mts -o /path/to/output/directory
```

### Converting Multiple Files in a Directory

To convert all MTS files in a directory:

```bash
# Basic usage with default output directory (same as input)
cargo run -- convert-dir -i /path/to/input/directory

# Specify custom output directory
cargo run -- convert-dir -i /path/to/input/directory -o /path/to/output/directory
```

### Help Commands

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

## Features

- Convert single MTS files to MP4
- Batch convert all MTS files in a directory
- Preserve original video quality using FFmpeg's copy mode
- Optional output directory specification
- User-friendly command-line interface

## Technical Details

- Uses FFmpeg for video conversion
- Implements H.264 video stream processing
- Maintains original video and audio quality
- Efficient processing with stream copying when possible

## License

[MIT License](LICENSE)

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

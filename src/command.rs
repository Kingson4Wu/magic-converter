use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "magic-converter")]
#[command(author = "Kingson Wu")]
#[command(version = "1.0")]
#[command(about = "A tool for converting MTS video files to MP4 format", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a single MTS file to MP4
    Convert {
        /// Input MTS file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory (optional, defaults to input file's directory)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert all MTS files in a directory
    ConvertDir {
        /// Input directory containing MTS files
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for converted files
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
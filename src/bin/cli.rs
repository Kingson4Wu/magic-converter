use std::path::PathBuf;
use clap::{Parser, Subcommand};
use magic_converter::ConverterService;
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a single MTS file to MP4
    File {
        /// Input MTS file path
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert all MTS files in a directory to MP4
    Directory {
        /// Input directory path
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let service = ConverterService::new();

    // Create a progress callback for CLI
    let progress_callback = Arc::new(|progress: f32| {
        print!("\rProgress: {}%", (progress * 100.0) as i32);
        if progress >= 1.0 {
            println!();
        }
    });

    match cli.command {
        Commands::File { input, output } => {
            println!("Converting file: {}", input.display());
            match service.convert_file(&input, output.as_deref(), Some(progress_callback)) {
                Ok(_) => println!("Conversion completed successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Directory { input, output } => {
            println!("Converting directory: {}", input.display());
            match service.convert_directory(&input, output.as_deref(), Some(progress_callback)) {
                Ok(_) => println!("Directory conversion completed successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

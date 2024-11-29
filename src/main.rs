use clap::Parser;
use iced::{Settings, Sandbox};
use magic_converter::{Cli, Commands, ConverterService, ConverterGui};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if any command line arguments are provided
    if env::args().len() > 1 {
        // CLI mode
        let cli = Cli::parse();
        let service = ConverterService::new();

        match cli.command {
            Commands::Convert { input, output } => {
                service.convert_file(&input, output.as_deref())
            },
            Commands::ConvertDir { input, output } => {
                service.convert_directory(&input, output.as_deref())
            }
        }
    } else {
        // GUI mode
        ConverterGui::run(Settings::default())?;
        Ok(())
    }
}
use clap::Parser;
use magic_converter::{Cli, Commands, ConverterService};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
}
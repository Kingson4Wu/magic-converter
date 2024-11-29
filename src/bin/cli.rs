use clap::Parser;
use magic_converter::command::{Cli, Commands};
use magic_converter::ConverterService;

fn main() {
    let cli = Cli::parse();
    let service = ConverterService::new();

    match cli.command {
        Commands::Convert { input, output } => {
            match service.convert_file(&input, output.as_deref()) {
                Ok(_) => println!("Successfully converted {}", input.display()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::ConvertDir { input, output } => {
            match service.convert_directory(&input, output.as_deref()) {
                Ok(_) => println!("Successfully converted directory {}", input.display()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

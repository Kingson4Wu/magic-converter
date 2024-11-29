use std::path::Path;
use crate::video::{convert_mts_to_mp4, convert_mts_files_in_directory};

#[derive(Debug)]
pub struct ConverterService;

impl ConverterService {
    pub fn new() -> Self {
        ConverterService
    }

    pub fn convert_file(&self, input: &Path, output: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting file conversion: {}", input.display());
        let result = convert_mts_to_mp4(input, output);
        match &result {
            Ok(_) => println!("File conversion completed: {}", input.display()),
            Err(e) => println!("File conversion failed: {}", e),
        }
        result
    }

    pub fn convert_directory(&self, input: &Path, output: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting directory conversion: {}", input.display());
        let result = convert_mts_files_in_directory(input, output);
        match &result {
            Ok(_) => println!("Directory conversion completed: {}", input.display()),
            Err(e) => println!("Directory conversion failed: {}", e),
        }
        result
    }
}
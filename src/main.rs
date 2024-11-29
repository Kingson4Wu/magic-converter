use std::path::Path;
use std::env;

use magic_converter::{convert_mts_to_mp4, convert_mts_files_in_directory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => {
            // 单个文件转换
            let input_path = Path::new(&args[1]);
            convert_mts_to_mp4(input_path, None)
        },
        3 => {
            // 指定输入目录和输出目录
            let input_dir = Path::new(&args[1]);
            let output_dir = Path::new(&args[2]);
            convert_mts_files_in_directory(input_dir, Some(output_dir))
        },
        _ => {
            println!("使用方法:");
            println!("1. 转换单个文件: cargo run <input.mts>");
            println!("2. 转换目录中的所有MTS文件: cargo run <input_directory> <output_directory>");
            Ok(())
        }
    }
}
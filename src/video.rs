use std::process::Command;
use std::path::Path;
use std::fs;

pub fn convert_mts_to_mp4(input_path: &Path, output_dir: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    // If no output directory specified, use input file's directory
    let output_dir = output_dir.unwrap_or_else(|| input_path.parent().unwrap());

    // Generate output file path
    let output_filename = input_path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("converted");
    
    let output_path = output_dir.join(format!("{}.mp4", output_filename));

    // Use ffmpeg for conversion
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c")
        .arg("copy")  // Direct stream copy for faster conversion
        .arg("-bsf:v")
        .arg("h264_mp4toannexb")  // Process H.264 video stream
        .arg(output_path.to_str().unwrap())
        .status()?;

    if status.success() {
        println!("Successfully converted {} to MP4", input_path.display());
        Ok(())
    } else {
        Err("Video conversion failed".into())
    }
}

pub fn convert_mts_files_in_directory(input_dir: &Path, output_dir: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through all .mts files in directory
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Check file extension
        if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("mts")) {
            convert_mts_to_mp4(&path, output_dir)?;
        }
    }
    
    Ok(())
}
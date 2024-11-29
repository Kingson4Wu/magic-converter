use std::process::{Command, Stdio};
use std::path::Path;
use std::fs;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

pub type ProgressCallback = Arc<dyn Fn(f32) + Send + Sync + 'static>;

pub fn convert_mts_to_mp4(
    input_path: &Path,
    output_dir: Option<&Path>,
    progress_callback: Option<ProgressCallback>
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = output_dir.unwrap_or_else(|| input_path.parent().unwrap());
    let output_filename = input_path.file_stem().unwrap_or_default().to_str().unwrap_or("converted");
    let output_path = output_dir.join(format!("{}.mp4", output_filename));

    let mut cmd = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c")
        .arg("copy")
        .arg("-bsf:v")
        .arg("h264_mp4toannexb")
        .arg("-progress")
        .arg("-")
        .arg(output_path.to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(callback) = progress_callback {
        if let Some(stdout) = cmd.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut total_frames = 1000;

            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("frame=") {
                        if let Ok(current_frame) = line[6..].trim().parse::<i32>() {
                            let progress = (current_frame as f32) / (total_frames as f32);
                            callback(progress.min(1.0));
                        }
                    } else if line.starts_with("total_frames=") {
                        if let Ok(frames) = line[13..].trim().parse::<i32>() {
                            total_frames = frames;
                        }
                    }
                }
            }
            callback(1.0);
        }
    }

    let status = cmd.wait()?;

    if status.success() {
        println!("Successfully converted {} to MP4", input_path.display());
        Ok(())
    } else {
        Err("Video conversion failed".into())
    }
}

pub fn convert_mts_files_in_directory(
    input_dir: &Path,
    output_dir: Option<&Path>,
    progress_callback: Option<ProgressCallback>
) -> Result<(), Box<dyn std::error::Error>> {
    let total_files = fs::read_dir(input_dir)?
        .filter(|entry| {
            entry.as_ref()
                .map(|e| e.path().extension().map_or(false, |ext| ext.eq_ignore_ascii_case("mts")))
                .unwrap_or(false)
        })
        .count();

    if total_files == 0 {
        return Ok(());
    }

    let mut completed_files = 0;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("mts")) {
            if let Some(ref callback) = progress_callback {
                let callback = Arc::clone(callback);
                let completed = completed_files;
                let total = total_files;
                
                // Create a new thread-safe callback for this file
                let file_callback: ProgressCallback = {
                    let callback = Arc::clone(&callback);
                    Arc::new(move |file_progress| {
                        let overall_progress = (completed as f32 + file_progress) / (total as f32);
                        callback(overall_progress);
                    })
                };
                
                convert_mts_to_mp4(&path, output_dir, Some(file_callback))?;
            } else {
                convert_mts_to_mp4(&path, output_dir, None)?;
            }
            completed_files += 1;
        }
    }
    
    Ok(())
}
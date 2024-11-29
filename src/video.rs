use std::process::Command;
use std::path::Path;
use std::fs;

pub fn convert_mts_to_mp4(input_path: &Path, output_dir: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    // 如果没有指定输出目录，使用输入文件的目录
    let output_dir = output_dir.unwrap_or_else(|| input_path.parent().unwrap());

    // 生成输出文件路径
    let output_filename = input_path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("converted");
    
    let output_path = output_dir.join(format!("{}.mp4", output_filename));

    // 使用ffmpeg进行转换
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c")
        .arg("copy")  // 直接复制编解码器，加快转换速度
        .arg("-bsf:v")
        .arg("h264_mp4toannexb")  // 处理H.264视频流
        .arg(output_path.to_str().unwrap())
        .status()?;

    if status.success() {
        println!("成功将 {} 转换为 MP4", input_path.display());
        Ok(())
    } else {
        Err("视频转换失败".into())
    }
}

pub fn convert_mts_files_in_directory(input_dir: &Path, output_dir: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    // 遍历目录中的所有.mts文件
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // 检查文件扩展名
        if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("mts")) {
            convert_mts_to_mp4(&path, output_dir)?;
        }
    }
    
    Ok(())
}
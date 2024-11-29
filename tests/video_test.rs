use magic_converter::video::{convert_mts_files_in_directory, convert_mts_to_mp4};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tempfile::{tempdir, TempDir};
use std::process::Command;

// Helper struct for test setup
struct TestSetup {
    input_dir: TempDir,
    output_dir: TempDir,
}

impl TestSetup {
    fn new() -> Self {
        Self {
            input_dir: tempdir().unwrap(),
            output_dir: tempdir().unwrap(),
        }
    }

    fn create_dummy_mts_file(&self, name: &str) -> PathBuf {
        let path = self.input_dir.path().join(format!("{}.mts", name));
        // Create a minimal valid MTS file structure
        let mut file = File::create(&path).unwrap();
        // Write some dummy MTS content - this won't be a valid MTS file
        // but is sufficient for testing the file handling logic
        file.write_all(b"HDMV").unwrap();
        file.write_all(&[0u8; 192]).unwrap();
        path
    }

    fn create_non_mts_file(&self, name: &str, extension: &str) -> PathBuf {
        let path = self.input_dir.path().join(format!("{}.{}", name, extension));
        fs::write(&path, b"dummy content").unwrap();
        path
    }
}

// Helper function to check if ffmpeg is installed
fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .is_ok()
}

#[test]
fn test_empty_directory() {
    let setup = TestSetup::new();
    let result = convert_mts_files_in_directory(
        setup.input_dir.path(),
        Some(setup.output_dir.path()),
        None
    );
    assert!(result.is_ok(), "Should handle empty directory gracefully");
    assert!(fs::read_dir(setup.output_dir.path()).unwrap().count() == 0);
}

#[test]
fn test_no_mts_files() {
    let setup = TestSetup::new();
    
    // Create various non-MTS files
    setup.create_non_mts_file("test1", "txt");
    setup.create_non_mts_file("test2", "mp4");
    setup.create_non_mts_file("test3", "avi");

    let result = convert_mts_files_in_directory(
        setup.input_dir.path(),
        Some(setup.output_dir.path()),
        None
    );
    assert!(result.is_ok(), "Should handle directory with no MTS files");
    assert!(fs::read_dir(setup.output_dir.path()).unwrap().count() == 0);
}

#[test]
fn test_progress_callback() {
    if !is_ffmpeg_available() {
        println!("Skipping progress callback test - ffmpeg not available");
        return;
    }

    let setup = TestSetup::new();
    
    // Create test MTS files
    setup.create_dummy_mts_file("test1");

    let progress_counter = Arc::new(AtomicUsize::new(0));
    let progress_clone = Arc::clone(&progress_counter);
    
    let callback = Arc::new(move |progress: f32| {
        assert!(progress >= 0.0 && progress <= 1.0, "Progress should be between 0 and 1");
        progress_clone.fetch_add(1, Ordering::SeqCst);
    });

    let _result = convert_mts_files_in_directory(
        setup.input_dir.path(),
        Some(setup.output_dir.path()),
        Some(callback)
    );

    // Give some time for the callback to be called
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let progress_count = progress_counter.load(Ordering::SeqCst);
    assert!(progress_count > 0, "Progress callback should have been called at least once");
}

#[test]
fn test_invalid_input_path() {
    let setup = TestSetup::new();
    let invalid_path = setup.input_dir.path().join("nonexistent.mts");
    
    let result = convert_mts_to_mp4(
        &invalid_path,
        Some(setup.output_dir.path()),
        None
    );
    assert!(result.is_err(), "Should error on invalid input file");
}

// Integration test that only runs if ffmpeg is available
#[test]
fn test_actual_conversion() {
    if !is_ffmpeg_available() {
        println!("Skipping actual conversion test - ffmpeg not available");
        return;
    }

    let setup = TestSetup::new();
    let test_file = setup.create_dummy_mts_file("test_conversion");
    
    let result = convert_mts_to_mp4(
        &test_file,
        Some(setup.output_dir.path()),
        None
    );
    
    // Since our dummy MTS file isn't actually valid, we expect an error
    assert!(result.is_err(), "Should fail with invalid MTS content");
}

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use tempfile::{tempdir, TempDir};
use smartui::error::Result;

pub fn create_test_file(content: &str) -> Result<(TempDir, PathBuf)> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", content)?;
    Ok((dir, file_path))
}

pub fn create_test_code_file(language: &str, content: &str) -> Result<(TempDir, PathBuf)> {
    let dir = tempdir()?;
    let extension = match language.to_lowercase().as_str() {
        "rust" => "rs",
        "python" => "py",
        "javascript" => "js",
        _ => "txt",
    };
    let file_path = dir.path().join(format!("test.{}", extension));
    let mut file = File::create(&file_path)?;
    writeln!(file, "{}", content)?;
    Ok((dir, file_path))
}

pub fn setup_test_environment() -> Result<()> {
    std::env::set_var("GEMINI_API_KEY", "test_key");
    Ok(())
}

pub fn cleanup_test_environment() {
    std::env::remove_var("GEMINI_API_KEY");
} 
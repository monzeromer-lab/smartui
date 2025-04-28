use std::path::PathBuf;
use std::io::{self, Read};
use crate::error::{GmError, Result};
use crate::gemini::GeminiClientTrait;

pub async fn execute<T: GeminiClientTrait>(client: &T, file: Option<PathBuf>, stdin: bool) -> Result<()> {
    let text = if stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else if let Some(path) = file {
        std::fs::read_to_string(path)?
    } else {
        return Err(GmError::InvalidInput("Either --stdin or a file path must be provided".to_string()));
    };

    let prompt = format!(
        "Please provide a concise summary of the following text:\n\n{}",
        text
    );

    let summary = client.generate_content(prompt).await?;
    println!("{}", summary);

    Ok(())
}
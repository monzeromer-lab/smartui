use crate::error::{GmError, Result};
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;
use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ConvertFormatCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ConvertFormatCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ConvertFormatCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let convert_prompt = format!(
            "Convert the following data to the specified format:\n\n{}",
            prompt
        );
        
        self.client.generate_content(convert_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(
    client: T,
    file: Option<PathBuf>,
    stdin: bool,
    from: String,
    to: String,
) -> Result<()> {
    let convert_command = ConvertFormatCommand::new(client);
    let data = if stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else if let Some(path) = file {
        fs::read_to_string(path)?
    } else {
        return Err(GmError::InvalidInput("Either --stdin or a file path must be provided".to_string()));
    };

    let prompt = format!("Convert from {} to {}:\n\n{}", from, to, data);
    let converted = convert_command.generate_content(&prompt).await?;
    println!("{}", converted);
    Ok(())
}
 
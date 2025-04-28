use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;
use std::path::PathBuf;
use std::fs;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ExplainCodeCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ExplainCodeCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ExplainCodeCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let explanation_prompt = format!(
            "Explain the following code in detail:\n\n{}",
            prompt
        );
        
        self.client.generate_content(explanation_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(
    client: T,
    file: PathBuf,
    language: Option<String>,
) -> Result<()> {
    let code = fs::read_to_string(&file)?;
    let explain_command = ExplainCodeCommand::new(client);
    
    let prompt = if let Some(lang) = language {
        format!("Explain this {} code:\n\n{}", lang, code)
    } else {
        format!("Explain this code:\n\n{}", code)
    };
    
    let explanation = explain_command.generate_content(&prompt).await?;
    println!("{}", explanation);
    Ok(())
}

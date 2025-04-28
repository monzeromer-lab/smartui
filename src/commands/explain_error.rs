use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ExplainErrorCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ExplainErrorCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ExplainErrorCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let error_prompt = format!(
            "Explain the following error message and suggest possible solutions:\n\n{}",
            prompt
        );
        
        self.client.generate_content(error_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, error: String, stdin: bool) -> Result<()> {
    let explain_command = ExplainErrorCommand::new(client);
    let prompt = if stdin {
        error
    } else {
        format!("Error: {}", error)
    };
    let explanation = explain_command.generate_content(&prompt).await?;
    println!("{}", explanation);
    Ok(())
}

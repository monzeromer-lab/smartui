use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ExplainAlgorithmCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ExplainAlgorithmCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ExplainAlgorithmCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let algorithm_prompt = format!(
            "Explain the following algorithm in detail:\n\n{}",
            prompt
        );
        
        self.client.generate_content(algorithm_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, algorithm: String) -> Result<()> {
    let explain_command = ExplainAlgorithmCommand::new(client);
    let explanation = explain_command.generate_content(&algorithm).await?;
    println!("{}", explanation);
    Ok(())
}

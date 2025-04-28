use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ExplainCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ExplainCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ExplainCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let explanation_prompt = format!(
            "Explain the following code in detail, including its purpose, how it works, and any important implementation details:\n\n{}",
            prompt
        );
        
        self.client.generate_content(explanation_prompt).await
    }
}

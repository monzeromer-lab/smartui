use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct TranslateCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> TranslateCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for TranslateCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let translation_prompt = format!(
            "Translate the following text:\n\n{}",
            prompt
        );
        
        self.client.generate_content(translation_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, text: String, to: String) -> Result<()> {
    let translate_command = TranslateCommand::new(client);
    let prompt = format!("Translate to {}: {}", to, text);
    let translation = translate_command.generate_content(&prompt).await?;
    println!("{}", translation);
    Ok(())
} 
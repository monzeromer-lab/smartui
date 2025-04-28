use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct RephraseCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> RephraseCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for RephraseCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let rephrase_prompt = format!(
            "Rephrase the following text in a different way:\n\n{}",
            prompt
        );
        
        self.client.generate_content(rephrase_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, text: String) -> Result<()> {
    let rephrase_command = RephraseCommand::new(client);
    let rephrased = rephrase_command.generate_content(&text).await?;
    println!("{}", rephrased);
    Ok(())
}

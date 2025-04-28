use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct SuggestNameCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> SuggestNameCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for SuggestNameCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let name_prompt = format!(
            "Suggest a name for the following description:\n\n{}",
            prompt
        );
        
        self.client.generate_content(name_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, description: String) -> Result<()> {
    let suggest_command = SuggestNameCommand::new(client);
    let suggestions = suggest_command.generate_content(&description).await?;
    println!("{}", suggestions);
    Ok(())
}
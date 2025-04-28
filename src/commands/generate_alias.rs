use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct GenerateAliasCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> GenerateAliasCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for GenerateAliasCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let alias_prompt = format!(
            "Generate a shell alias for the following command:\n\n{}",
            prompt
        );
        
        self.client.generate_content(alias_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, command: String) -> Result<()> {
    let generate_command = GenerateAliasCommand::new(client);
    let alias = generate_command.generate_content(&command).await?;
    println!("{}", alias);
    Ok(())
}
 
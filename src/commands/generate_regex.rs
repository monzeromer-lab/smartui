use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct GenerateRegexCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> GenerateRegexCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for GenerateRegexCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let regex_prompt = format!(
            "Generate a regular expression for the following pattern:\n\n{}",
            prompt
        );
        
        self.client.generate_content(regex_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, pattern: String) -> Result<()> {
    let generate_command = GenerateRegexCommand::new(client);
    let regex = generate_command.generate_content(&pattern).await?;
    println!("{}", regex);
    Ok(())
}

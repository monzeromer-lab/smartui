use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct FindCommandCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> FindCommandCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for FindCommandCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let command_prompt = format!(
            "Find a command for the following task:\n\n{}",
            prompt
        );
        
        self.client.generate_content(command_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, task: String) -> Result<()> {
    let find_command = FindCommandCommand::new(client);
    let command = find_command.generate_content(&task).await?;
    println!("{}", command);
    Ok(())
}

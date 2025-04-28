use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct BrainstormCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> BrainstormCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for BrainstormCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let brainstorm_prompt = format!(
            "Brainstorm ideas for the following topic:\n\n{}",
            prompt
        );
        
        self.client.generate_content(brainstorm_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, topic: String) -> Result<()> {
    let brainstorm_command = BrainstormCommand::new(client);
    let ideas = brainstorm_command.generate_content(&topic).await?;
    println!("{}", ideas);
    Ok(())
}

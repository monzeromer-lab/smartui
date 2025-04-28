use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct FixTyposCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> FixTyposCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for FixTyposCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let fix_typos_prompt = format!(
            "Fix any typos in the following text:\n\n{}",
            prompt
        );
        
        self.client.generate_content(fix_typos_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, text: String) -> Result<()> {
    let fix_typos_command = FixTyposCommand::new(client);
    let corrected = fix_typos_command.generate_content(&text).await?;
    println!("{}", corrected);
    Ok(())
}

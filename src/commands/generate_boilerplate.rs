use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct GenerateBoilerplateCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> GenerateBoilerplateCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for GenerateBoilerplateCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let boilerplate_prompt = format!(
            "Generate boilerplate code for the following description:\n\n{}",
            prompt
        );
        
        self.client.generate_content(boilerplate_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, description: String) -> Result<()> {
    let generate_command = GenerateBoilerplateCommand::new(client);
    let boilerplate = generate_command.generate_content(&description).await?;
    println!("{}", boilerplate);
    Ok(())
}

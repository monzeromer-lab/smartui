use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct GenerateTestCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> GenerateTestCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for GenerateTestCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let test_prompt = format!(
            "Generate test cases for the following code:\n\n{}",
            prompt
        );
        
        self.client.generate_content(test_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(
    client: T,
    code: String,
    language: Option<String>,
) -> Result<()> {
    let generate_command = GenerateTestCommand::new(client);
    let prompt = if let Some(lang) = language {
        format!("Language: {}\n\nCode:\n{}", lang, code)
    } else {
        code
    };
    let tests = generate_command.generate_content(&prompt).await?;
    println!("{}", tests);
    Ok(())
}
 
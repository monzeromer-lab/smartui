use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct DefineCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> DefineCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for DefineCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let definition_prompt = format!(
            "Define the following term:\n\n{}",
            prompt
        );
        
        self.client.generate_content(definition_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, term: String, context: Option<String>) -> Result<()> {
    let define_command = DefineCommand::new(client);
    let prompt = if let Some(context) = context {
        format!("Define '{}' in the context of {}:", term, context)
    } else {
        format!("Define '{}':", term)
    };
    let definition = define_command.generate_content(&prompt).await?;
    println!("{}", definition);
    Ok(())
}

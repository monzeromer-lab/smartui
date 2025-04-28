use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ExplainConceptCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ExplainConceptCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ExplainConceptCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let concept_prompt = format!(
            "Explain the following concept in detail:\n\n{}",
            prompt
        );
        
        self.client.generate_content(concept_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(
    client: T,
    concept: String,
    field: Option<String>,
) -> Result<()> {
    let explain_command = ExplainConceptCommand::new(client);
    let prompt = if let Some(field) = field {
        format!("Explain the concept of '{}' in the field of {}:", concept, field)
    } else {
        format!("Explain the concept of '{}':", concept)
    };
    let explanation = explain_command.generate_content(&prompt).await?;
    println!("{}", explanation);
    Ok(())
}

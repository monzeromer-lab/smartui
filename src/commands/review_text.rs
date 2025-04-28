use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ReviewTextCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ReviewTextCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ReviewTextCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let review_prompt = format!(
            "Review the following text and provide feedback:\n\n{}",
            prompt
        );
        
        self.client.generate_content(review_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(
    client: T,
    text: String,
    _stdin: bool,
    criteria: Option<String>,
) -> Result<()> {
    let review_command = ReviewTextCommand::new(client);
    let prompt = if let Some(criteria) = criteria {
        format!("Review criteria: {}\n\nText to review:\n{}", criteria, text)
    } else {
        text
    };
    let review = review_command.generate_content(&prompt).await?;
    println!("{}", review);
    Ok(())
}

use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct AnalyzeSentimentCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> AnalyzeSentimentCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for AnalyzeSentimentCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let sentiment_prompt = format!(
            "Analyze the sentiment of the following text:\n\n{}",
            prompt
        );
        
        self.client.generate_content(sentiment_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, text: String, stdin: bool) -> Result<()> {
    let analyze_command = AnalyzeSentimentCommand::new(client);
    let prompt = if stdin {
        text
    } else {
        format!("Analyze sentiment: {}", text)
    };
    let analysis = analyze_command.generate_content(&prompt).await?;
    println!("{}", analysis);
    Ok(())
}
 
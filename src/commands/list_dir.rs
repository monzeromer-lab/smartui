use crate::error::Result;
use crate::gemini::GeminiClientTrait;
use async_trait::async_trait;
use std::path::PathBuf;
use std::fs;

#[async_trait]
pub trait GenerateContent {
    async fn generate_content(&self, prompt: &str) -> Result<String>;
}

pub struct ListDirCommand<T: GeminiClientTrait> {
    client: T,
}

impl<T: GeminiClientTrait> ListDirCommand<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<T: GeminiClientTrait + Send + Sync> GenerateContent for ListDirCommand<T> {
    async fn generate_content(&self, prompt: &str) -> Result<String> {
        let list_prompt = format!(
            "Analyze and describe the contents of this directory:\n\n{}",
            prompt
        );
        
        self.client.generate_content(list_prompt).await
    }
}

pub async fn execute<T: GeminiClientTrait + Send + Sync>(client: T, path: Option<PathBuf>) -> Result<()> {
    let current_dir = path.unwrap_or_else(|| PathBuf::from("."));
    let entries = fs::read_dir(&current_dir)?;
    
    let mut contents = String::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();
            let is_dir = path.is_dir();
            contents.push_str(&format!("{} {}\n", if is_dir { "[DIR]" } else { "[FILE]" }, name));
        }
    }
    
    let list_command = ListDirCommand::new(client);
    let analysis = list_command.generate_content(&contents).await?;
    println!("{}", analysis);
    Ok(())
}

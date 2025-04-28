use serde::{Deserialize, Serialize};
use std::env;
use crate::error::{GmError, Result};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

#[async_trait]
pub trait GeminiClientTrait {
    async fn generate_content(&self, prompt: String) -> Result<String>;
}

pub struct RealGeminiClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl RealGeminiClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| GmError::ApiKeyNotFound)?;

        Ok(Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string(),
        })
    }
}

#[async_trait]
impl GeminiClientTrait for RealGeminiClient {
    async fn generate_content(&self, prompt: String) -> Result<String> {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt,
                }],
            }],
        };

        let response = self.client
            .post(&self.base_url)
            .query(&[("key", &self.api_key)])
            .json(&request)
            .send()
            .await
            .map_err(GmError::GeminiApi)?;

        if !response.status().is_success() {
            return Err(GmError::InvalidResponse(format!(
                "API returned status code: {}",
                response.status()
            )));
        }

        let response_body: GeminiResponse = response
            .json()
            .await
            .map_err(GmError::GeminiApi)?;

        if let Some(candidate) = response_body.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                return Ok(part.text.clone());
            }
        }

        Err(GmError::InvalidResponse("No content in response".to_string()))
    }
}

#[async_trait]
impl GeminiClientTrait for &RealGeminiClient {
    async fn generate_content(&self, prompt: String) -> Result<String> {
        (*self).generate_content(prompt).await
    }
}

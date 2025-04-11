use dotenv::dotenv;
use std::env;

// pub mod anthropic;
pub mod gemini;
pub mod openai;

#[derive(Debug)]
pub enum AIProvider {
    OpenAI,
    Gemini,
    // Anthropic,
}

impl AIProvider {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();
        let provider = env::var("AI_PROVIDER")
            .map_err(|_| "AI_PROVIDER environment variable not set".to_string())?;

        match provider.to_lowercase().as_str() {
            "openai" => Ok(AIProvider::OpenAI),
            "gemini" => Ok(AIProvider::Gemini),
            // "anthropic" => Ok(AIProvider::Anthropic),
            _ => Err(format!("Unsupported AI provider: {}", provider)),
        }
    }
}

pub async fn get_ai_response(prompt: &str) -> Result<String, String> {
    let provider = AIProvider::from_env()?;
    let model = env::var("MODEL").map_err(|_| "MODEL environment variable not set".to_string())?;
    let api_key =
        env::var("API_KEY").map_err(|_| "API_KEY environment variable not set".to_string())?;

    match provider {
        AIProvider::OpenAI => openai::get_response(prompt, &model, &api_key).await,
        AIProvider::Gemini => gemini::get_response(prompt, &model, &api_key).await,
        // AIProvider::Anthropic => anthropic::get_response(prompt, &model, &api_key).await,
    }
}

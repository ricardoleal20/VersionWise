use reqwest::Client;
use serde_json::json;

/// Makes a request to the OpenAI API to generate a response based on the given prompt.
///
/// # Arguments
///
/// * `prompt` - The text prompt to send to the API
/// * `model` - The OpenAI model to use (e.g., "gpt-3.5-turbo")
/// * `api_key` - The OpenAI API key for authentication
///
/// # Returns
///
/// A `Result` containing either:
/// * `Ok(String)` - The generated response text
/// * `Err(String)` - An error message if the request fails
///
/// # Examples
///
/// ```rust
/// let response = get_response("Hello, how are you?", "gpt-3.5-turbo", "your-api-key").await?;
/// ```
pub async fn get_response(prompt: &str, model: &str, api_key: &str) -> Result<String, String> {
    // Initialize the HTTP client
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    // Build and send the request to OpenAI's API
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7  // Controls randomness in the response
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    // Parse the JSON response
    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Extract the generated text from the response
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "Failed to extract content from response".to_string())?;

    Ok(content.to_string())
}

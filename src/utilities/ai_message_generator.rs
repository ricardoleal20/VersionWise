use dotenvy::dotenv;
use std::env;
/// AI-based message generator for changesets
/// This module provides functionality to generate changeset messages
/// based on git changes using AI assistance.
use std::process::Command;
use std::str;

// Import the AI modules
use crate::utilities::ai_calls::{gemini, openai};

/// Configuration for the AI message generator
#[derive(Debug, Clone)]
pub struct AIConfig {
    /// The API key for the AI service
    pub api_key: String,
    /// The model to use for generation
    pub model: String,
    /// The AI provider to use (openai or gemini)
    pub provider: String,
}

impl AIConfig {
    /// Builds a new AIConfig from environment variables
    ///
    /// # Returns
    ///
    /// A new AIConfig instance
    ///
    /// # Panics
    ///
    /// Panics if any of the required environment variables are not set:
    /// - AI_PROVIDER
    /// - API_KEY
    /// - MODEL
    pub fn build() -> Self {
        // Load environment variables from .env file
        dotenv().ok();

        // Get provider from env
        let provider = env::var("AI_PROVIDER")
            .expect("AI_PROVIDER must be set in .env file")
            .to_lowercase();

        // Validate provider
        if provider != "openai" && provider != "gemini" {
            panic!("AI_PROVIDER must be either 'openai' or 'gemini'");
        }

        // Get API key from env
        let api_key = env::var("API_KEY").expect("API_KEY must be set in .env file");

        // Get model from env
        let model = env::var("MODEL").expect("MODEL must be set in .env file");

        Self {
            api_key,
            model,
            provider,
        }
    }
}

/// Get git diff for the specified file
///
/// This function will return the diff of the given file if it's tracked by git
/// or None if the file isn't tracked or there's no git repository.
pub fn get_git_diff(file_path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["diff", "HEAD", file_path])
        .output()
        .ok()?;

    if output.status.success() {
        let diff = str::from_utf8(&output.stdout).ok()?.to_string();
        if !diff.is_empty() {
            return Some(diff);
        }
    }

    // If we didn't get a diff from HEAD, try to get from staging
    let output = Command::new("git")
        .args(["diff", "--staged", file_path])
        .output()
        .ok()?;

    if output.status.success() {
        let diff = str::from_utf8(&output.stdout).ok()?.to_string();
        if !diff.is_empty() {
            return Some(diff);
        }
    }

    None
}

/// Extract summary information from git diff
///
/// Returns a tuple with:
/// 1. Files changed count
/// 2. Lines added count  
/// 3. Lines removed count
/// 4. A summary of the main changes
pub fn extract_diff_summary(diff: &str) -> (u32, u32, u32, String) {
    let mut files_changed = 0;
    let mut lines_added = 0;
    let mut lines_removed = 0;
    let mut summary = String::new();

    // Count files, added and removed lines
    for line in diff.lines() {
        if line.starts_with("+++") || line.starts_with("---") {
            if !line.ends_with("/dev/null") {
                files_changed += 1;
            }
        } else if line.starts_with('+') && !line.starts_with("+++") {
            lines_added += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            lines_removed += 1;
        }

        // Extract key lines for summary (function definitions, class declarations, etc.)
        if line.starts_with('+')
            && (line.contains("fn ")
                || line.contains("class ")
                || line.contains("def ")
                || line.contains("struct ")
                || line.contains("impl "))
        {
            summary.push_str(&line[1..]); // Skip the '+' at the beginning
            summary.push('\n');
        }
    }

    // Handle empty summary
    if summary.is_empty() {
        summary = format!(
            "{} files changed, {} insertions(+), {} deletions(-)",
            files_changed / 2,
            lines_added,
            lines_removed
        );
    }

    (files_changed / 2, lines_added, lines_removed, summary)
}

/// Generate a message using the configured AI provider
///
/// # Arguments
///
/// * `change_type` - The type of change (MAJOR, MINOR, PATCH)
/// * `tag` - The tag describing the change
/// * `module` - The module being changed
/// * `diff_summary` - A summary of the changes
/// * `config` - The AI configuration to use
///
/// # Returns
///
/// A `Result` containing either:
/// * `Ok(String)` - The generated message
/// * `Err(String)` - An error message if generation fails
pub async fn generate_message_with_ai(
    change_type: &str,
    tag: &str,
    module: &str,
    diff_summary: &str,
    config: &AIConfig,
) -> Result<String, String> {
    // Construct the prompt for the AI
    let prompt = format!(
        "Generate a changeset message that follows the conventional commit format for a semantic version change. \
        Details:\n\
        - Change Type: {}\n\
        - Tag: {}\n\
        - Module: {}\n\
        - Changes: {}\n\n\
        The message should:\n\
        1. Start with the tag name followed by a colon and space\n\
        2. Use present tense\n\
        3. Be specific about what changed\n\
        4. Not exceed one line\n\
        5. Not include the module name if it's already implied\n\
        Example format: 'Feature: add user authentication system based on Format'\n\
        Return ONLY the message, no additional text.",
        change_type, tag, module, diff_summary
    );

    // Call the appropriate AI provider and clean the response
    let response = match config.provider.to_lowercase().as_str() {
        "openai" => openai::get_response(&prompt, &config.model, &config.api_key).await,
        "gemini" => gemini::get_response(&prompt, &config.model, &config.api_key).await,
        _ => Err(format!("Unsupported AI provider: {}", config.provider)),
    }?;

    // Clean the response: trim whitespace and ensure single line
    Ok(response
        .trim() // Remove leading/trailing whitespace and newlines
        .lines() // Split into lines
        .next() // Take first line only
        .unwrap_or("") // Default to empty string if no lines
        .trim() // Final trim to ensure no whitespace
        .to_string())
}

/// Main function to generate a message based on changes in a module
///
/// This is the function that should be called from the changeset creation process.
///
/// # Arguments
///
/// * `change_type` - The type of change (MAJOR, MINOR, PATCH)
/// * `tag` - The tag describing the change
/// * `module` - The module being changed
/// * `config` - The AI configuration to use
///
/// # Returns
///
/// A `Result` containing either:
/// * `Ok(String)` - The generated message
/// * `Err(String)` - An error message if generation fails
pub async fn generate_ai_message(
    change_type: &str,
    tag: &str,
    module: &str,
    config: &AIConfig,
) -> Result<String, String> {
    // Check if module is a file path that can be analyzed
    let diff = if !module.is_empty() {
        get_git_diff(module)
    } else {
        None
    };

    // If we have a diff, analyze it and generate a message
    if let Some(diff_content) = diff {
        let (_files, _added, _removed, summary) = extract_diff_summary(&diff_content);
        generate_message_with_ai(change_type, tag, module, &summary, config).await
    } else {
        // Fallback if no diff is available
        generate_message_with_ai(change_type, tag, module, "recent changes", config).await
    }
}

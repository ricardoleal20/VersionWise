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
    /// Whether AI generation is enabled
    pub enabled: bool,
    /// The API key for the AI service (if needed)
    pub api_key: Option<String>,
    /// The model to use for generation
    pub model: String,
    /// The AI provider to use (openai or gemini)
    pub provider: String,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_key: None,
            model: "gpt-3.5-turbo".to_string(),
            provider: "openai".to_string(),
        }
    }
}

/// Get git diff for the specified file
///
/// This function will return the diff of the given file if it's tracked by git
/// or None if the file isn't tracked or there's no git repository.
pub fn get_git_diff(file_path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(&["diff", "HEAD", file_path])
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
        .args(&["diff", "--staged", file_path])
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
        "Generate a concise changeset message for a {} change with tag '{}' in module '{}'. \
        The changes include: {}. The message should be clear, professional, and follow \
        conventional commit message format.",
        change_type, tag, module, diff_summary
    );

    // Get the API key, returning an error if not configured
    let api_key = config
        .api_key
        .as_ref()
        .ok_or_else(|| "API key not configured".to_string())?;

    // Call the appropriate AI provider
    match config.provider.to_lowercase().as_str() {
        "openai" => openai::get_response(&prompt, &config.model, api_key).await,
        "gemini" => gemini::get_response(&prompt, &config.model, api_key).await,
        _ => Err(format!("Unsupported AI provider: {}", config.provider)),
    }
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

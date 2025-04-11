/// AI-based message generator for changesets
/// This module provides functionality to generate changeset messages
/// based on git changes using AI assistance.
use std::process::Command;
use std::str;

/// Configuration for the AI message generator
#[derive(Debug, Clone)]
pub struct AIConfig {
    /// Whether AI generation is enabled
    pub enabled: bool,
    /// The API key for the AI service (if needed)
    pub api_key: Option<String>,
    /// The model to use for generation
    pub model: String,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_key: None,
            model: "default".to_string(),
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

/// Mock function for generating messages with AI
///
/// In a real implementation, this would call an AI API with the diff information
/// to generate an appropriate changeset message.
pub fn generate_message_with_ai(
    change_type: &str,
    tag: &str,
    module: &str,
    diff_summary: &str,
) -> String {
    // This is a mock implementation for now
    // In a real version, this would call an external AI API

    match (change_type, tag) {
        ("MAJOR", "Remove") => format!(
            "Remove functionality in {} based on changes: {}",
            module, diff_summary
        ),
        ("MAJOR", "Rename") => format!(
            "Rename components in {} according to new convention",
            module
        ),
        ("MAJOR", "I/O") => format!(
            "Change input/output interface in {} to improve usability",
            module
        ),
        ("MAJOR", "Behavior") => format!("Change behavior of {} to fix critical issues", module),

        ("MINOR", "Feature") => format!(
            "Add new feature to {} that implements {}",
            module, diff_summary
        ),
        ("MINOR", "Add") => format!(
            "Add functionality to {} to support {}",
            module, diff_summary
        ),
        ("MINOR", "I/O") => format!(
            "Add optional parameters to {} for extended functionality",
            module
        ),
        ("MINOR", "Deprecated") => format!(
            "Mark {} as deprecated, to be removed in future version",
            module
        ),

        ("PATCH", "Refactor") => format!("Refactor {} to improve code quality", module),
        ("PATCH", "Bug") => format!("Fix bug in {} where {}", module, diff_summary),
        ("PATCH", "Optimization") => format!("Optimize {} to improve performance", module),
        ("PATCH", "Tests") => format!("Add tests for {} to ensure reliability", module),
        ("PATCH", "Patch") => format!("Update {} to handle edge cases", module),

        _ => format!("Update {} with various improvements", module),
    }
}

/// Main function to generate a message based on changes in a module
///
/// This is the function that should be called from the changeset creation process.
pub fn generate_ai_message(change_type: &str, tag: &str, module: &str) -> String {
    // Check if module is a file path that can be analyzed
    let diff = if !module.is_empty() {
        get_git_diff(module)
    } else {
        None
    };

    // If we have a diff, analyze it and generate a message
    if let Some(diff_content) = diff {
        let (_files, _added, _removed, summary) = extract_diff_summary(&diff_content);
        generate_message_with_ai(change_type, tag, module, &summary)
    } else {
        // Fallback if no diff is available
        generate_message_with_ai(change_type, tag, module, "recent changes")
    }
}

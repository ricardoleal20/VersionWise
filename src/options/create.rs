/// ================================ ///
///         OPTIONS :: Create        ///
/// ================================ ///
/// For this, we'll follow the next path:
/// * P1: Set the changeset name (If not specified, would be randomly chosen)
/// * P2: Select the type of versioning change (major, minor, patch)
/// * P3: Search for the available modules in the package. If not found, let them write their own module name
/// * P4: Write the message to add in the changeset
use colored::*;
use fake::faker::lorem::en::Word;
use fake::Fake;
use regex::Regex;
use requestty::{prompt, prompt_one, Answer, Question};
use std::fs;
use std::path::Path;
use std::process::Command;
// Local imports
use crate::options::Changeset;
use crate::utilities::{
    create_changeset_folder, find_version, generate_ai_message, write_changeset_file,
};

/// Detect modules in the project by scanning files
fn detect_modules() -> Vec<String> {
    let mut modules = Vec::new();

    // Add common directories to scan
    let directories = vec!["src", "tests", "lib", "app"];

    for dir in directories {
        // Skip if directory doesn't exist
        if !Path::new(dir).exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(path) = entry.path().to_str() {
                            // Add the file path as a module
                            modules.push(path.to_string());
                        }
                    }
                }
            }
        }
    }

    // Add "Other" option to let user input custom module
    modules.push("Other (specify manually)".to_string());

    modules
}

/// Get default message template based on change type and tag
fn get_message_template(change_type: &str, tag: &str) -> String {
    match (change_type, tag) {
        ("MAJOR", "Remove") => "Remove ... functionality because ...".to_string(),
        ("MAJOR", "Rename") => "Rename ... to ... to better reflect ...".to_string(),
        ("MAJOR", "I/O") => "Change ... input/output to ...".to_string(),
        ("MAJOR", "Behavior") => "Change behavior of ... to ...".to_string(),

        ("MINOR", "Feature") => "Add ... feature that allows ...".to_string(),
        ("MINOR", "Add") => "Add ... functionality to ...".to_string(),
        ("MINOR", "I/O") => "Include optional ... parameter to ...".to_string(),
        ("MINOR", "Deprecated") => {
            "Mark ... as deprecated, to be removed in version ...".to_string()
        }

        ("PATCH", "Refactor") => "Refactor ... to improve ...".to_string(),
        ("PATCH", "Bug") => "Fix ... bug where ...".to_string(),
        ("PATCH", "Optimization") => "Optimize ... to improve performance by ...".to_string(),
        ("PATCH", "Tests") => "Add tests for ... to verify ...".to_string(),
        ("PATCH", "Patch") => "Update ... to handle ...".to_string(),

        _ => "".to_string(),
    }
}

/// Get changed files from git
fn get_git_changed_files() -> Vec<String> {
    let mut changed_files = Vec::new();

    // Try to get modified files from git
    let output = Command::new("git")
        .args(&["diff", "--name-only", "HEAD"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let git_output = String::from_utf8_lossy(&output.stdout);
            for line in git_output.lines() {
                if !line.is_empty() {
                    changed_files.push(line.to_string());
                }
            }
        }
    }

    // Add "Other" option at the end
    if !changed_files.is_empty() {
        changed_files.push("Other (specify manually)".to_string());
    }

    changed_files
}

/// Select tags depending on the change type
fn select_tags(change_type: &str) -> Vec<String> {
    let available_tags: Vec<String>;
    // Based on the change type representation, select the tags.
    if change_type == "MAJOR" {
        available_tags = vec![
            String::from("⚰️  Remove: Removed features."),
            String::from("🚚 Rename: Renamed features."),
            String::from("✏️  I/O: Changing input/output of features."),
            String::from("💥 Behavior: Changing features behavior."),
        ];
    } else if change_type == "MINOR" {
        available_tags = vec![
            String::from("✨ Feature: New feature."),
            String::from("➕ Add: Add functionality to existing feature."),
            String::from("✏️  I/O: Include optional input/output to a feature."),
            String::from("🗑️  Deprecated: Deprecated features."),
        ];
    } else {
        available_tags = vec![
            String::from("♻️  Refactor: Refactor of existing code."),
            String::from("🐛 Bug: Fix a bug."),
            String::from("⚡️ Optimization: Simple optimization of code."),
            String::from("🧪 Tests: Include or update tests."),
            String::from("🩹 Patch: Include or delete logs, catch errors or related things."),
        ];
    }
    // Return the selected tags
    available_tags
}

/// Create the question to set the tag
fn set_tag(change_type: &str) -> String {
    // Get the available tags
    let available_tags = select_tags(change_type);
    // Create the question
    let tag_question = Question::select("tag")
        .message("Select the tag for this change")
        .choices(available_tags)
        .build();
    // Perform the question
    let results = prompt_one(tag_question);
    let result = match results {
        // Check and receive the OK, if there's any
        Ok(x) => x,
        Err(_) => panic!("There's something wrong selecting the tag for the changeset"),
    };
    // And, at the end, just receive the answer
    let mut tag = result.as_list_item().unwrap().text.as_str();
    // And now, clean the tag
    let re = Regex::new(r"([A-Za-z]+):").unwrap();
    if let Some(capture) = re.captures(&tag) {
        if let Some(matched) = capture.get(1) {
            tag = matched.as_str();
        }
    }
    tag.to_string()
}

/// Create the questions
fn create_questions(default_name: &str) -> Vec<Question<'static>> {
    //* P1
    let changeset_name: Question<'_> = Question::input("name")
        .message("Write the Changeset name")
        .default(default_name)
        .build();
    //* P2
    let change_type: Question<'_> = Question::select("change_type")
        .message("Select the change type that is most adequate to these changes")
        .choices(vec![
            "💥 MAJOR: Most of the time related to breaking changes.",
            "✨ MINOR: New features that keep backwards compatibility.",
            "🩹 PATCH: Refactors, bugs, fixes and small changes.",
        ])
        .build();

    // Return the initial questions - module and message will be asked later
    // after we know the change type and tag
    let questions = vec![changeset_name, change_type];

    // Return the questions
    questions
}

/// Ask for module based on git changes and auto-detected modules
fn ask_for_module() -> String {
    // First try to get git changed files
    let git_modules = get_git_changed_files();

    let module_question: Question;

    if !git_modules.is_empty() {
        // If we have git changes, use those
        module_question = Question::select("module")
            .message("Select the module/file that has changed")
            .choices(git_modules)
            .build();
    } else {
        // Otherwise use auto-detected modules or let user specify
        let detected_modules = detect_modules();

        if detected_modules.len() > 1 {
            // More than just "Other"
            module_question = Question::select("module")
                .message("Select the module that has changed")
                .choices(detected_modules)
                .build();
        } else {
            // Fall back to text input if no modules detected
            module_question = Question::input("module")
                .message("Write the module/class/function name that has changed (optional)")
                .default("")
                .build();
        }
    }

    // Get the answer
    let result = prompt_one(module_question).expect("Error getting module input");

    // Handle the result based on whether it was a select or input
    let module = match result {
        Answer::ListItem(item) => {
            if item.text == "Other (specify manually)" {
                // If "Other" was selected, ask for manual input
                let custom_module = Question::input("custom_module")
                    .message("Enter the custom module name")
                    .build();
                let custom_result = prompt_one(custom_module).expect("Error getting custom module");
                custom_result.as_string().unwrap().to_string()
            } else {
                item.text
            }
        }
        Answer::String(input) => input,
        _ => "".to_string(),
    };

    module
}

/// Ask for message generation method (AI, template, manual)
fn ask_for_message_method() -> String {
    let method_question = Question::select("message_method")
        .message("How would you like to create your changeset message?")
        .choices(vec![
            "Generate with AI based on detected changes",
            "Use message template",
            "Write message from scratch",
        ])
        .build();

    let result = prompt_one(method_question).expect("Error selecting message method");
    result.as_list_item().unwrap().text.to_string()
}

/// Ask for the message with template suggestions
fn ask_for_message(change_type: &str, tag: &str, module: &str) -> String {
    // First, ask which method to use
    let method = ask_for_message_method();

    if method.contains("Generate with AI") {
        // Generate a message with AI
        println!("Analyzing changes and generating message...");
        let ai_message = generate_ai_message(change_type, tag, module);

        // Ask if user wants to edit the generated message
        let edit_question = Question::confirm("edit_message")
            .message(format!(
                "AI generated message: \n\"{}\"\n\nWould you like to edit this message?",
                ai_message
            ))
            .default(false)
            .build();

        let edit_result = prompt_one(edit_question).expect("Error asking to edit message");

        if edit_result.as_bool().unwrap() {
            // User wants to edit the message
            let edit_message_question = Question::input("edited_message")
                .message("Edit the message:")
                .default(&ai_message)
                .build();

            let edited_result = prompt_one(edit_message_question).expect("Error editing message");
            edited_result.as_string().unwrap().to_string()
        } else {
            // Use the AI message as is
            ai_message
        }
    } else if method.contains("Use message template") {
        // Use template approach
        let template = get_message_template(change_type, tag);

        let message_question = Question::input("message")
            .message("Write the message for the change")
            .default(&template)
            .build();

        let result = prompt_one(message_question).expect("Error getting message input");
        let message = result.as_string().unwrap();

        if message.is_empty() || message == template {
            panic!("There was no message for the changeset or template was not modified. You need to add a personalized message.");
        }

        message.to_string()
    } else {
        // Write from scratch
        let message_question = Question::input("message")
            .message("Write the message for the change")
            .default("")
            .build();

        let result = prompt_one(message_question).expect("Error getting message input");
        let message = result.as_string().unwrap();

        if message.is_empty() {
            panic!("There was no message for the changeset. You need to add a message.");
        }

        message.to_string()
    }
}

/// Display a summary and confirm before saving
fn confirm_changeset(changeset: &Changeset) -> bool {
    println!("\n{}", "Changeset Summary:".bold());
    println!("Name: {}.toml", changeset.name);
    println!("Type: {}", changeset.change);
    println!("Tag: {}", changeset.tag);

    if !changeset.modules.is_empty() {
        println!("Module: {}", changeset.modules);
    }

    println!("Message: {}", changeset.message);
    println!("Version: {}\n", changeset.version);

    let confirm_question = Question::confirm("confirm")
        .message("Do you want to save this changeset?")
        .default(true)
        .build();

    let result = prompt_one(confirm_question).expect("Error getting confirmation");
    result.as_bool().unwrap()
}

fn process_answers() -> Changeset {
    // Generate the default name
    let default_name = "Leave it blank for a random name";
    // Process the initial results (name and change type)
    let results = prompt(create_questions(default_name));
    // Get the results
    let result = match results {
        // Check and receive the OK, if there's any
        Ok(x) => x,
        Err(_) => panic!("There's something wrong creating the changeset"),
    };

    // Get the name
    let mut name = result.get("name").unwrap().as_string().unwrap();
    if name == default_name || name.trim().is_empty() {
        name = Word().fake();
    }

    // Get the change type
    let mut change = result
        .get("change_type")
        .unwrap()
        .as_list_item()
        .unwrap()
        .text
        .as_str();
    // Instance the regex to search for the word
    let re = Regex::new(r"\b(MAJOR|MINOR|PATCH)\b").unwrap();
    // get the change
    if let Some(capture) = re.captures(change) {
        if let Some(matched) = capture.get(1) {
            change = matched.as_str();
        }
    }

    // Get the tag (now that we know the change type)
    let tag = set_tag(change);

    // Get the module (with git and auto-detection)
    let module = ask_for_module();

    // Get the message (with AI, templates, or manual input)
    let message = ask_for_message(change, &tag, &module);

    // Create the changeset
    let changeset = Changeset {
        name: name.into(),
        change: change.into(),
        modules: module,
        tag,
        message,
        // Set the version. For that, find the latest version in the changelog
        version: find_version(),
    };

    // Return the changeset only if confirmed
    if confirm_changeset(&changeset) {
        changeset
    } else {
        println!("Changeset creation cancelled.");
        std::process::exit(0);
    }
}

pub fn create_changesets() {
    // Process the results
    let changeset: Changeset = process_answers();
    // Then, start creating the Changeset file in the changeset function
    // Let's see if the folder exists. If not, create it
    create_changeset_folder();
    // Once you have created the folder, create the changeset
    write_changeset_file(&changeset);
    // Once you have created it, print a confirmation message
    println!(
        "\n Changeset `{}.toml` has been created! 🎉",
        changeset.name.green()
    );
}

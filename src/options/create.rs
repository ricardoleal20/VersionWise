/// ================================ ///
///         OPTIONS :: Create        ///
/// ================================ ///
/// For this, we'll follow the next path:
/// * P1: Set the changeset name (If not specified, would be randomly chosen)
/// * P2: Select the type of versioning change (major, minor, patch)
/// * P3: Search for the available modules in the package. If not found, let them write their own module name
/// * P4: Write the message to add in the changeset
use fake::faker::lorem::en::Word;
use fake::Fake;
use regex::Regex;
use requestty::{prompt, prompt_one, Question};
// Local imports
use crate::options::Changeset;
use crate::utilities::{create_changeset_folder, write_changeset_file};

/// Select tags depending on the change type
fn select_tags(change_type: &str) -> Vec<String> {
    let available_tags: Vec<String>;
    // Based on the change type representation, select the tags.
    if change_type == "MAJOR" {
        available_tags = vec![
            String::from("⚰️ Remove: Removed features."),
            String::from("🚚 Rename: Renamed features."),
            String::from("✏️ I/O: Changing input/output of features."),
            String::from("💥 Behavior: Changing features behavior."),
        ];
    } else if change_type == "MINOR" {
        available_tags = vec![
            String::from("✨ Feature: New feature."),
            String::from("➕ Add: Add functionality to existing feature."),
            String::from("✏️ I/O: Include optional input/output to a feature."),
            String::from("🗑️ Deprecated: Deprecated features."),
        ];
    } else {
        available_tags = vec![
            String::from("♻️ Refactor: Refactor of existing code."),
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
        .message("Select the change type that most adequate to this changes")
        .choices(vec![
            "💥 MAJOR: Most of the time related to breaking changes.",
            "✨ MINOR: New features that keep backwards compatibility.",
            "🩹 PATCH: Refactors, bugs, fixes and small changes.",
        ])
        .build();
    //* P3
    let module_with_change: Question<'_> = Question::input("module")
        .message("Write the module/class/function name that has suffers changes (optional)")
        .default("")
        .build();
    //* P4
    let message_for_change: Question<'_> = Question::input("message")
        .message("Write the message for the change")
        .default("")
        .build();
    // Return the questions
    let questions = vec![
        changeset_name,
        change_type,
        module_with_change,
        message_for_change,
    ];
    // Return the questions
    questions
}

fn process_answers() -> Changeset {
    // Generate the default name
    let default_name = "Leave it black for a random name";
    // Process the results
    let results = prompt(create_questions(default_name));
    // Get the results
    let result = match results {
        // Check and receive the OK, if there's any
        Ok(x) => x,
        Err(_) => panic!("There's something wrong creating the changeset"),
    };
    // Get the values
    // ** Name
    let mut name = result.get("name").unwrap().as_string().unwrap();
    if name == default_name {
        name = Word().fake();
    }
    // Check if there's a name
    // ** Change type
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
    // ** Module with change
    let module = result.get("module").unwrap().as_string().unwrap();
    // ** message for change
    let message = result.get("message").unwrap().as_string().unwrap();
    if message.is_empty() {
        panic!("There was no message for the changeset. You need to add one.")
    }
    // Return the results
    Changeset {
        name: name.into(),
        change: change.into(),
        module: module.into(),
        tag: set_tag(change),
        message: message.into(),
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
    println!("\n Changeset `{}.md` has been created! 🎉", changeset.name);
}

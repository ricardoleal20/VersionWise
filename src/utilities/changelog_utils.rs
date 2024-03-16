use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader, Write};
// Local imports
use crate::options::Changeset;

/// Function to open the Changeset in case that exists
pub fn open_changelog() -> Vec<String> {
    // Open the Changeset file in case that exist
    let file = fs::File::open("CHANGELOG.md")
        .expect("Error opening CHANGELOG.md. Ensure that you have one already.");
    let reader = BufReader::new(file);

    // Create the content structure
    let mut content: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(line_content) = line {
            content.push(line_content);
        }
    }
    // And return it
    content
}

pub fn create_changelog(content: Vec<String>) {
    // Crea un nuevo archivo CHANGELOG.md
    let mut file = fs::File::create("CHANGELOG.md").expect("Error creating the CHANGELOG.md");

    // Write the entire CHANGELOG content
    writeln!(file, "{}", content.join("\n")).expect("Error when writing the CHANGELOG.md");

    // If everything's cool, then write the successful message
    println!("The CHANGELOG.md has been updated!");
}

pub fn new_changelog_entry(changesets: Vec<Changeset>) -> Vec<String> {
    // First, get a list of printed tags to avoid read the same tag twice
    let mut printed_tags: HashSet<&String> = HashSet::new();
    // Create a mutable for the content written
    let mut content: Vec<String> = Vec::new();
    for changeset in changesets.iter() {
        // Evaluate if this tag has been written
        if printed_tags.contains(&changeset.tag) {
            continue;
        }
        // Write the tag first
        content.push(format!("\n### {}\n\n", changeset.tag));
        // Filter for all the same tags
        for nested_changeset in changesets.iter().filter(|c| c.tag == changeset.tag) {
            // Then, write all the changes
            if nested_changeset.module == "" {
                content.push(format!("- {}.\n", nested_changeset.message));
            } else {
                content.push(format!(
                    "- {}: {}.\n",
                    nested_changeset.module, nested_changeset.message
                ));
            }
        }
        // And at the end, write this tag on the read ones
        printed_tags.insert(&changeset.tag);
    }
    // And at the end, return the content list
    content
}

// ================================ ///
//      UTILITIES :: Changeset      ///
// ================================ ///
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
// Local imports
use crate::options::Changeset;

/// From a file content, process it and return it to the Changeset structure
fn parse_changeset(file_name: &str, file_content: &str) -> Option<Changeset> {
    // Get the lines
    let lines = file_content.lines();

    // Extract the lines
    let remaining_lines: Vec<&str> = lines.collect();

    // Get the data from the changeset
    if remaining_lines.len() >= 3 {
        let mut module_parts = remaining_lines[1].split(":");
        let change = module_parts.next().unwrap().to_string();
        let tag = module_parts.next().unwrap().to_string();
        // And add the module and message
        let mut module_message = remaining_lines[3].split(":");
        // Get the strings comming from the module message next element
        let mut module = match module_message.next() {
            Some(module_str) => module_str.trim().to_string(),
            None => String::new(), // Include a new empty string for the next scenario
        };
        let message = match module_message.next() {
            Some(message_str) => message_str.trim().to_string(),
            None => {
                // If there's no second element, clone the module one
                module.clone()
            }
        };
        // If the module and the message are the same, make the module an empty value
        if module == message {
            module = "".to_string()
        }
        // Create it
        Some(Changeset::new(
            file_name.to_string().split(".md").collect(),
            change,
            module,
            tag,
            message,
        ))
    } else {
        None
    }
}

/// Process a file from a file path and from there, process and get the changesets
fn process_file(file_name: &str, file_path: &PathBuf) -> io::Result<Changeset> {
    // Process the file content
    let file_content = fs::read_to_string(file_path)?;
    // Let's see if we can create a changeset from here
    if let Some(changeset) = parse_changeset(file_name, &file_content) {
        Ok(changeset)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid file format",
        ))
    }
}

pub fn get_current_changesets() -> Vec<Changeset> {
    // Initialize the changesets
    let mut changesets = vec![];
    // Get the directory where we can find the changesets
    let changeset_dir = Path::new(".changesets");

    // Iterate over all the entries in there
    if let Ok(entries) = fs::read_dir(changeset_dir) {
        // Check every entry on the entries
        for entry in entries {
            // Get the file
            let dir_entry = entry.unwrap();
            // Get the file path and file name
            let file_name = dir_entry.file_name();
            let file_path = &dir_entry.path();
            // Process. if the filepath is a file and it's extension is .md, then process
            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "md") {
                match process_file(file_name.to_str().unwrap(), file_path) {
                    Ok(changeset) => {
                        changesets.push(changeset);
                    }
                    Err(err) => {
                        println!("Error processing file {:?}: {}", &file_path, err);
                    }
                }
            }
        }
    }
    // Sort them
    changesets.sort();
    // At the end, return the changesets
    changesets
}

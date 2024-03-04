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
        let change = remaining_lines[1].trim().to_string();
        let mut module = remaining_lines[3].split(":").collect();
        let message = remaining_lines[3..].join("\n").trim().to_string();
        // If module and message are equal, then they didn't have a module
        if module == message {
            module = format!("")
        }

        // Create it
        Some(Changeset::new(
            file_name.to_string().split(".md").collect(),
            change,
            module,
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
    // At the end, return the changesets
    changesets
}

/// Methods for write, delete and create Changesets
use std::fs;
use std::io::Write;
// Local imports
use crate::options::Changeset;

/// Create the changeset directory on the root project
pub fn create_changeset_folder() {
    // Check to see if the folder `.changesets` exists
    if !fs::metadata(".changesets/").is_ok() {
        // If it doesn't exist, create it
        match fs::create_dir(".changesets/") {
            Ok(_) => {}
            Err(_) => {
                panic!("There was an error creating the `.changesets/` directory.")
            }
        }
    }
}

/// Write a changeset file from a Changeset structure
pub fn write_changeset_file(changeset: &Changeset) {
    // Write the Changeset file from the object obtained
    // First, obtain the file name
    let filename = format!(".changesets/{}.md", &changeset.name);
    // Then, start generating the message
    let mut message = String::new();

    // Initialize the separator
    let separator = "-".repeat(35) + "\n";
    // Start adding it a line of 10 `-`
    message.push_str(&separator);
    message.push_str(&format!("\t{}\n", &changeset.change));
    message.push_str(&separator);
    // Write the message and the module, if it exists
    if !changeset.module.is_empty() {
        message.push_str(&format!("`{}`: ", &changeset.module));
    }
    message.push_str(&changeset.message);
    // Then, create of the file
    let file: Result<fs::File, std::io::Error> = fs::File::create(filename);

    match file {
        Ok(mut file) => match file.write_all(message.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                panic!("There's an error writing the changeset.")
            }
        },
        Err(_) => {
            println!("There was an error creating a Changeset.");
        }
    }
}

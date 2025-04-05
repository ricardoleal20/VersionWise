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
    let filename = format!(".changesets/{}.toml", &changeset.name);
    // Then, start generating the message
    let mut toml_content = String::new();

    // Write [changeset] section - match the RawChangeset structure
    toml_content.push_str("[changeset]\n");
    // Remove any tab character that might be in the change field
    let clean_change = changeset.change.trim_start().trim_end();
    toml_content.push_str(&format!("change_type = \"{}\"\n", clean_change));
    toml_content.push_str(&format!("tag = \"{}\"\n", &changeset.tag));
    toml_content.push_str(&format!("version = \"{}\"\n", &changeset.version));
    toml_content.push_str("\n");

    // Write [changes] section - match the RawChangeset structure
    toml_content.push_str("[changes]\n");

    // Check if modules exists and format as array
    if !changeset.modules.is_empty() {
        toml_content.push_str(&format!(
            "modules = [\"{}\"]",
            &changeset.modules.replace(", ", "\", \"")
        ));
    } else {
        toml_content.push_str("modules = []");
    }
    toml_content.push_str("\n");

    // Add description (matches the RawChangeset structure)
    toml_content.push_str(&format!("description = \"{}\"\n", &changeset.message));

    // Then, create the file
    let file: Result<fs::File, std::io::Error> = fs::File::create(filename);

    match file {
        Ok(mut file) => match file.write_all(toml_content.as_bytes()) {
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

// ================================ ///
//      UTILITIES :: Changeset      ///
// ================================ ///
use std::fs;
use std::io;
use std::path::Path;
// Local imports
use crate::options::Changeset;
use crate::utilities::changeset_structures::RawChangeset;

/// From a file content, process it and return the Changeset structure
fn parse_changeset(file_name: &str) -> Option<Changeset> {
    // We try to read the file
    let file_content = fs::read_to_string(file_name).ok()?;
    // We parse the TOML content into a RawChangeset structure
    let raw_changeset: RawChangeset = toml::from_str(&file_content).ok()?;
    // Then, we process the modules
    let modules = raw_changeset.changes.modules.join(", ");

    // And, at the end, we create the Changeset structure! Easy peasy!
    Some(Changeset::new(
        file_name.to_string(),
        raw_changeset.changeset.change_type,
        modules,
        raw_changeset.changeset.module,
        raw_changeset.changes.description,
        raw_changeset.changeset.version,
    ))
}

/// Process a file from a file path and from there, process and get the changesets
fn process_file(file_name: &str) -> io::Result<Changeset> {
    // Let's see if we can create a changeset from here
    if let Some(changeset) = parse_changeset(file_name) {
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
                match process_file(file_name.to_str().unwrap()) {
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

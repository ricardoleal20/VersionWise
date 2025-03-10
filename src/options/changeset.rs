/// Just write the Changeset structure
use std::cmp::Ordering;

fn update_version(change: &String, version: String) -> String {
    let mut new_version = String::new();
    // We're going to update the version based on the change type
    match change.as_str() {
        "MAJOR" => new_version = format!("{}.0.0", version.split('.').nth(0).unwrap()),
        "MINOR" => {
            new_version = format!(
                "{}.{}.0",
                version.split('.').nth(0).unwrap(),
                version.split('.').nth(1).unwrap()
            )
        }
        "PATCH" => {
            new_version = format!(
                "{}.{}.{}",
                version.split('.').nth(0).unwrap(),
                version.split('.').nth(1).unwrap(),
                version.split('.').nth(2).unwrap()
            )
        }
        _ => {}
    }
    // Return the new version
    new_version
}

/// Changeset structure, including all the necessary fields
/// to process and create the new CHANGELOG.md
#[derive(Debug, Eq)]
pub struct Changeset {
    pub name: String,
    pub change: String,
    pub module: String,
    pub tag: String,
    pub message: String,
    pub version: String,
}

impl Changeset {
    pub fn new(
        name: String,
        change: String,
        module: String,
        tag: String,
        message: String,
        version: String,
    ) -> Changeset {
        // Get the new and updated version!
        let new_version = update_version(&change, version);
        // Initialize the changeset with the new version and the normal
        // methods
        Changeset {
            name,
            change,
            module,
            tag,
            message,
            version: new_version,
        }
    }

    /// Method to update the version based on the change type
    pub fn updated_version(&mut self) -> String {
        // Initialize the version parts to define the newest version!
        let mut version_parts: Vec<String> =
            self.version.split('.').map(|s| s.to_string()).collect();
        // Get the change in string
        let change: &str = &self.change;
        match change {
            "MAJOR" => {
                // If it is a major change, we need to increment the major version
                // and reset the minor and patch versions to 0
                version_parts[0] = (version_parts[0].parse::<u32>().unwrap() + 1).to_string();
                version_parts[1] = "0".to_string();
                version_parts[2] = "0".to_string();
            }
            "MINOR" => {
                // If it is a minor change, we need to increment the minor version
                // and reset the patch version to 0
                version_parts[1] = (version_parts[1].parse::<u32>().unwrap() + 1).to_string();
                version_parts[2] = "0".to_string();
            }
            "PATCH" => {
                // If it is a patch change, we need to increment the patch version
                version_parts[2] = (version_parts[2].parse::<u32>().unwrap() + 1).to_string();
            }
            _ => {}
        }
        // Return the updated version as response. This would be the parameter used
        // to update the version in the CHANGELOG.md
        version_parts.join(".")
    }
}

// Implement the PartialEq to compare changesets between them
impl PartialEq for Changeset {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.change == other.change
            && self.module == other.module
            && self.tag == other.tag
            && self.message == other.message
            && self.version == other.version
    }
}

// Implement a PartialOrd method to sort the classes
impl PartialOrd for Changeset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Changeset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

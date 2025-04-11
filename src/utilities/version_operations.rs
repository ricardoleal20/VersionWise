use regex::Regex;

/// Calculates the next version based on the current version and change type
///
/// # Arguments
///
/// * `current_version` - The current version string (e.g., "0.1.0")
/// * `change_type` - The type of change ("MAJOR", "MINOR", or "PATCH")
///
/// # Returns
///
/// The next version string
///
/// # Examples
///
/// ```
/// let next_version = calculate_next_version("0.1.0", "MINOR");
/// assert_eq!(next_version, "0.2.0");
/// ```
pub fn calculate_next_version(current_version: &str, change_type: &str) -> String {
    // Parse the current version
    let re = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();
    let caps = re
        .captures(current_version)
        .expect("Invalid version format. Expected format: MAJOR.MINOR.PATCH");

    let major: u32 = caps[1].parse().unwrap();
    let minor: u32 = caps[2].parse().unwrap();
    let patch: u32 = caps[3].parse().unwrap();

    // Calculate the next version based on change type
    match change_type {
        "MAJOR" => format!("{}.0.0", major + 1),
        "MINOR" => format!("{}.{}.0", major, minor + 1),
        "PATCH" => format!("{}.{}.{}", major, minor, patch + 1),
        _ => panic!("Invalid change type. Must be MAJOR, MINOR, or PATCH"),
    }
}

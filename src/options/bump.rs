/// ================================ ///
///         OPTIONS :: Bump          ///
/// ================================ ///
// Local imports
use crate::utilities::{
    create_changelog, get_current_changesets, new_changelog_entry, open_changelog,
};

pub fn bump_version() {
    // First, get the changesets
    let changesets = get_current_changesets();
    // From here, parse the changesets as the new Changelog entry
    let new_entry = new_changelog_entry(changesets);
    // Now, read the current CHANGESET file
    let mut content = open_changelog();
    // Find the index where it is the start of versions
    let start_of_versions_index = content
        .iter()
        .position(|line| line.starts_with("## ["))
        .unwrap_or(content.len());
    // Append the new entry and the content
    content.insert(start_of_versions_index, new_entry.join(""));

    println!("{}", content.join("\n"));
    // Then, by last, write the content
    //create_changelog(content)
}

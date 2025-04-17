/// Define some structures for changeset data, so it's easier for us
/// to deserialize and work with changesets.
use serde::Deserialize;

/// Represents a raw changeset with detailed information.
/// Such as the changeset information and the detail. This is a
/// raw representation of a changeset, before creating the normal
/// structure.
#[derive(Debug, Deserialize)]
pub struct RawChangeset {
    pub changeset: ChangesetInfo,
    pub changes: ChangeDetails,
}

/// Represents the information of a changeset. Which change_type
/// does include, the module, and the version.
#[derive(Debug, Deserialize)]
pub struct ChangesetInfo {
    pub change_type: String,
    pub tag: String,
    pub version: String,
}

/// Represents the details of a changeset. Which modules are affected
/// and the description of the changes.
#[derive(Debug, Deserialize)]
pub struct ChangeDetails {
    pub modules: Vec<String>,
    pub description: String,
}

/// Just write the Changeset structure
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Changeset {
    pub name: String,
    pub change: String,
    pub module: String,
    pub tag: String,
    pub message: String,
}

impl Changeset {
    pub fn new(
        name: String,
        change: String,
        module: String,
        tag: String,
        message: String,
    ) -> Changeset {
        Changeset {
            name,
            change,
            module,
            tag,
            message,
        }
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

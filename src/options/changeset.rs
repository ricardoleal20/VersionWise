/// Just write the Changeset structure

#[derive(Debug)]
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

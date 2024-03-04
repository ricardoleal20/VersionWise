/// ================================ ///
///         OPTIONS :: List          ///
/// ================================ ///
// Local imports
use crate::utilities::get_current_changesets;

pub fn list_changesets() {
    // Get the changesets and list them
    let changesets = get_current_changesets();
    // Process them
    println!("{:?}", changesets);
}

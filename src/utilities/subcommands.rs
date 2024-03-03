/// ================================ ///
///      UTILITIES :: SubCommand     ///
/// ================================ ///
use clap::{App, Arg, SubCommand};

// create a type APP to avoid write it on every command
type APP = App<'static, 'static>;

fn add_create_subcommand() -> APP {
    // Here, create the subcommand `create`
    let create_subcommand: App<'_, '_> =
        SubCommand::with_name("create").about("Create a new Changeset");
    // Return the subcommand
    create_subcommand
}

fn add_list_subcommand() -> APP {
    // Here, create the subcommand `list`
    let list_subcommand: App<'_, '_> = SubCommand::with_name("list")
        .about("List all the current Changesets")
        .arg(
            Arg::with_name("preview")
                .short("p")
                .long("preview")
                .required(false)
                .help("Preview the addition to the Changelog, showing all the changesets and the new version to bump."),
        ).help("List the current changesets and a short description from them.");
    // Return the subcommand
    list_subcommand
}

fn add_bump_subcommand() -> APP {
    // Here, create the subcommand `bump`
    let bump_subcommand: App<'_, '_> =
        SubCommand::with_name("bump").about("Bump the new version using the pending changesets").help("Using all the current changesets that we have locally, and delete them after update the changelog and the version of the package.");
    // Return the subcommand
    bump_subcommand
}

/// Create and append the subcommands  for the CLI application
pub fn create_subcommands() -> (APP, APP, APP) {
    // Create the `create`` subcommand
    let create = add_create_subcommand();
    // Create the `list` subcommand
    let list = add_list_subcommand();
    // Create the `bump` subcommand
    let bump = add_bump_subcommand();
    // Return the commands
    (create, list, bump)
}

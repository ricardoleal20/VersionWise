/// ================================ ///
///      UTILITIES :: SubCommand     ///
/// ================================ ///
use clap::{App, SubCommand};

// create a type APP to avoid write it on every command
type CLIApp = App<'static, 'static>;

fn add_create_subcommand() -> CLIApp {
    // Here, create the subcommand `create`
    let create_subcommand: CLIApp = SubCommand::with_name("create")
        .about("Create a new Changeset")
        .help("With a bunch of options, create the new changeset for a set of development");
    // Return the subcommand
    create_subcommand
}

fn add_list_subcommand() -> CLIApp {
    // Here, create the subcommand `list`
    let list_subcommand: CLIApp = SubCommand::with_name("list")
        .about("List all the current Changesets")
        .help("List the current changesets and a short description from them. Doesn't require arguments.");
    // Return the subcommand
    list_subcommand
}

fn add_bump_subcommand() -> CLIApp {
    // Here, create the subcommand `bump`
    let bump_subcommand: CLIApp =
        SubCommand::with_name("bump")
        .about("Bump the new version using the pending changesets")
        .help("Using all the current changesets that we have locally, and delete them after update the changelog and the version of the package.
        
It is also going to delete all the current files in the `.changesets` folder (to restart the process)");
    // Return the subcommand
    bump_subcommand
}

/// Create and append the subcommands  for the CLI application
pub fn create_subcommands() -> (CLIApp, CLIApp, CLIApp) {
    // Create the `create`` subcommand
    let create = add_create_subcommand();
    // Create the `list` subcommand
    let list = add_list_subcommand();
    // Create the `bump` subcommand
    let bump = add_bump_subcommand();
    // Create the extra commands
    // Return the commands
    (create, list, bump)
}

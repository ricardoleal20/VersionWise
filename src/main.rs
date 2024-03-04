/// Here' we'll only import the CLI structure
/// inside of main so it can be used when it's called
use clap::{App, ArgMatches};
// Local imports
mod options;
mod utilities;
// Use the methods from the modules
use options::{bump_version, create_changesets, list_changesets};
use utilities::create_subcommands;

fn main() {
    // Create the methods
    let (create, list, bump) = create_subcommands();
    // Instance the App
    let app: App<'_, '_> = App::new("SemPyVer :: Project management with Changesets")
        .subcommand(create)
        .subcommand(list)
        .subcommand(bump);
    // Add the methods to the app method
    // Search for the matches
    let matches: ArgMatches<'_> = app.get_matches();
    // Search for the matches
    match matches.subcommand() {
        // Create
        ("create", Some(_)) => {
            // Instance the app method
            create_changesets()
        }
        // List
        ("list", Some(_)) => {
            // Instance the app method
            list_changesets()
        }
        // Bump
        ("bump", Some(_)) => {
            // Instance the app method
            bump_version()
        }
        _ => {
            // Manejar casos inesperados o mostrar ayuda por defecto
            println!("{}", matches.usage());
        }
    }
}

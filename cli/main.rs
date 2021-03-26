mod commands;

use clap::{App, Arg, SubCommand};

fn main() {
    const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let matches = App::new("catsh")
        .version(APP_VERSION)
        .author(APP_AUTHORS.replace(":", "\n").as_str())
        .subcommand(
            SubCommand::with_name("run")
                .version(APP_VERSION)
                .about("Run shell script.")
                .arg(
                    Arg::with_name("file")
                        .value_name("FILE")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(new_args)) => commands::run::run_command(new_args),
        (&_, _) => println!("Command not found"),
    }
}

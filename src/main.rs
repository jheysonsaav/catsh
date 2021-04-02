// Copyright (C) Stellar authors. All right reserved.
mod commands;
mod prompt;
mod utils;

use std::env;

use clap::{App, Arg, SubCommand};
use utils::{
    dirs,
    logs::{Log, LogLevel},
};

fn main() {
    dirs::StellarDirs::load().verify();

    let app_version: String =
        env::var("CARGO_PKG_VERSION").unwrap_or(String::from("1.0.0"));
    let app_authors: String = env::var("CARGO_PKG_AUTHORS")
        .unwrap_or(String::from("Jheyson Saavedra"));

    let matches = App::new("stellar")
        .version(app_version.as_str())
        .author(app_authors.replace(":", "\n").as_str())
        .subcommand(
            SubCommand::with_name("run")
                .version(app_version.as_str())
                .about("Run shell script.")
                .arg(
                    Arg::with_name("file")
                        .value_name("FILE")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("shell")
                .version(app_version.as_str())
                .about("Run shell.")
                .arg(
                    Arg::with_name("private")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(args)) => commands::run::run_command(args),
        ("shell", Some(args)) => commands::shell::shell_command(args),
        (&_, _) => {
            Log::new(LogLevel::Error, 0, "Command not found").show();
        }
    }
}

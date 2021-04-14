// Copyright (C) Stellar authors. All right reserved.
mod commands;

use std::env;

use clap::{App, Arg, SubCommand};
use stellar_core::{
    dirs,
    logs::{Log, LogLevel},
};

fn main() {
    dirs::StellarDirs::load().verify();

    let app_version: String =
        env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| String::from("1.0.0"));
    let app_authors: String = env::var("CARGO_PKG_AUTHORS")
        .unwrap_or_else(|_| String::from("Jheyson Saavedra"));

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
            SubCommand::with_name("start")
                .version(app_version.as_str())
                .about("Start new stellar session.")
                .arg(
                    Arg::with_name("private")
                        .required(false)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("color")
                        .required(false)
                        .default_value("auto")
                        .possible_values(&["auto", "always", "never"])
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(args)) => commands::run::run_command(args),
        ("start", Some(args)) => commands::start::start_command(args),
        (&_, _) => {
            Log::new(LogLevel::Error, 0, "Command not found").show();
        }
    }
}

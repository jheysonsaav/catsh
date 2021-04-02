// Copyright (C) stellar authors. All right reserved.
mod commands;

use clap::{App, Arg, SubCommand};
use stellar_core::{
    dirs,
    logs::{Log, LogLevel},
};

#[cfg(windows)]
use stellar_core::colors;

fn main() {
    dirs::StellarDirs::load().verify();

    #[cfg(windows)]
    colors::enable_ansi();

    const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    let matches = App::new("stellar")
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
        .subcommand(
            SubCommand::with_name("shell")
                .version(APP_VERSION)
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
        ("", Some(args)) => commands::shell::shell_command(args),
        (&_, _) => {
            Log::new(LogLevel::Error, 0, "Command not found").show();
        }
    }
}

// Copyright (C) Stellar authors. All right reserved.
use clap::ArgMatches;

pub(crate) fn run_command(args: &ArgMatches) {
    if let Some(run_file) = args.value_of("file") {
        // TODO: create a parser for shell scripting and execute
        println!("Running: {}", run_file);
    }
}

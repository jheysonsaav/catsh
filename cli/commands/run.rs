// Copyright (C) Catsh authors. All right reserved.
use clap::ArgMatches;

pub(crate) fn run_command(args: &ArgMatches) {
    if let Some(run_file) = args.value_of("file") {
        println!("Running: {}", run_file);
    }
}

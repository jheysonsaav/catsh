use clap::ArgMatches;
use stellar_core::prompt::Prompt;

pub(crate) fn start_command(args: &ArgMatches) {
    let _ = Prompt::new(args.is_present("private"));
}

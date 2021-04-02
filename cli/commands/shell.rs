use clap::ArgMatches;
use stellar_core::prompt::Prompt;

pub(crate) fn shell_command(args: &ArgMatches) {
    Prompt::new(args.is_present("private"));
}

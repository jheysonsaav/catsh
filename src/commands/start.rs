use crate::prompt::Prompt;
use clap::ArgMatches;

pub(crate) fn start_command(args: &ArgMatches) {
    Prompt::new(args.is_present("private"));
}

use catsh_core::prompt::Prompt;
use clap::ArgMatches;

pub(crate) fn start_command(args: &ArgMatches) {
    let _ = Prompt::new(args.is_present("private"));
}

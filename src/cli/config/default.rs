//! Create default configuration file

use crate::cli::{ArgSet, CliResult};
use clap::{App, Arg, SubCommand};

/// Get the `default` subcommand
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    let command = SubCommand::with_name("default")
        .about("Save config file with default values")
        .arg(Arg::with_name("force")
            .help("Do not prompt if config file already exists")
            .long("force")
            .short("f"));

    command
}

/// Save the default configuration
pub fn run<'a>(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `default` subcommand");

    let force = args.sub.is_present("force");

    crate::cli::config::save_default_config(args.global, force)?;

    Ok(())
}

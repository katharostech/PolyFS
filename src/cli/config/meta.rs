use crate::cli::config::ArgSet;
use crate::cli::CliResult;
use clap::{App, SubCommand};

// Backends
mod sqlite;

/// Run `meta` subcommand
pub fn run<'a>(_args: ArgSet) -> CliResult<()> {
    log::debug!("Running `meta` subcommand");

    Ok(())
}

/// Get CLI for the `meta` subcommand
#[rustfmt::skip]
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("meta")
        .about("Configure metadata store")
        .long_about(
"Configure the metadata store. Each subcommand allows you to configure a \
different supported metadata backend. Only one backend can be used at a time. \
If a backend is configured it will replace any previous backend configuration."
        )
        .subcommand(sqlite::get_cli())
}

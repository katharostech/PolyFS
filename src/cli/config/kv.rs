//! Key-value store configuration subcommand

use crate::cli::config::{ArgSet, CliResult};
use clap::{App, SubCommand};

// Backends
mod sqlite;

/// Run `kv` subcommand
pub fn run<'a>(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `kv` subcommand");

    match args.sub.subcommand() {
        ("sqlite", Some(sub)) => sqlite::run(ArgSet {
            global: args.global,
            sub,
        })?,
        _ => panic!(
            "Unimplemented command or failure to show help message when lacking a subcommand."
        ),
    }

    Ok(())
}

/// Get CLI for the `kv` subcommand
#[rustfmt::skip]
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("kv")
        .about("Configure key-value store")
        .long_about(
"Configure the key-value store. Each subcommand allows you to configure a \
different supported key-value backend. Only one backend can be used at a time. \
If a backend is configured it will replace any previous backend configuration."
        )
        .subcommand(sqlite::get_cli())
}

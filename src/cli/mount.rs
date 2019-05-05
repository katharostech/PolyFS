//! PolyFS `mount` subcommand

use crate::cli::{ArgSet, CliResult};
use clap::{App, Arg, SubCommand};

/// Get CLI for the `config` subcommand
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("mount")
        .about("Mount the filesystem")
        .arg(Arg::with_name("read_only")
            .long("read-only")
            .short("r")
            .help("Mount the filesystem as read-only"))
        .arg(Arg::with_name("mountpoint")
                .help("location to mount the filesystem")
                .required(true))
}

pub fn run(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `mount` subcommand");

    Ok(())
}

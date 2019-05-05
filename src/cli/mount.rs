//! PolyFS `mount` subcommand

use crate::PolyfsResult;
use crate::cli::ArgSet;
use clap::{App, Arg, SubCommand};

/// Get CLI for the `mount` subcommand
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

/// Run `mount` subcommand
pub fn run(_args: ArgSet) -> PolyfsResult<()> {
    log::debug!("Running `mount` subcommand");

    Ok(())
}

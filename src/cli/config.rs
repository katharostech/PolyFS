use crate::cli::{ArgSet, CliResult};
use clap::{App, SubCommand};

pub mod kv;
pub mod meta;
pub mod yaml;

/// Run `config` subcommand
pub fn run<'a>(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `config` subcommand");

    match args.sub.subcommand() {
        ("kv", Some(sub)) => kv::run(ArgSet {
            global: args.global,
            sub,
        })?,
        ("meta", Some(sub)) => meta::run(ArgSet {
            global: args.global,
            sub,
        })?,
        _ => panic!(
            "Unimplemented command or failure to show help message when lacking a subcommand."
        ),
    }

    Ok(())
}

/// Get CLI for the `config` subcommand
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("config")
        .about("Create or update PolyFS config file")
        .subcommand(kv::get_cli())
        .subcommand(meta::get_cli())
}

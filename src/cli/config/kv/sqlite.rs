use crate::cli::config::{self, ArgSet};
use crate::cli::CliResult;
use clap::{App, Arg, SubCommand};
use yaml_rust::{Yaml};

/// Run `sqlite` subcommand
pub fn run<'a>(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `sqlite` subcommand");
    let _config_file: Yaml = config::yaml::get_config(args.global)?;

    Ok(())
}

/// Get CLI for the `sqlite` subcommand
#[rustfmt::skip]
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sqlite")
        .about("Configure Sqlite backend")
        .arg(Arg::with_name("db")
            .help("Path to the Sqlite database")
            .required(true))
}

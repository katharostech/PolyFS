use crate::cli::config::ArgSet;
use crate::cli::CliResult;
use crate::app::config::*;

use clap::{App, Arg, SubCommand};

/// Run `sqlite` subcommand
pub fn run<'a>(_args: ArgSet) -> CliResult<()> {
    log::debug!("Running `sqlite` subcommand");
    // let _config_file: Yaml = config::yaml::get_config(args.global)?;

    let config = AppConfig {
        key_value: KvBackend::sqlite(
            SqliteConfig {
                db: String::from("kv.db")
            }
        )
    };

    println!("{}", serde_yaml::to_string(&config).unwrap());

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

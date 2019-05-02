use crate::cli::{ArgSet, CliResult};
use crate::cli::config::{load_config, save_config};
use crate::app::config::KvBackend;
use crate::app::backends::keyvalue::sqlite::SqliteConfig;

use clap::{App, Arg, SubCommand};

/// Run `sqlite` subcommand
pub fn run<'a>(args: ArgSet) -> CliResult<()> {
    log::debug!("Running `sqlite` subcommand");
    let mut config = load_config(args.global)?;

    config.backends.key_value = KvBackend::Sqlite(SqliteConfig {
        db: String::from(args.sub.value_of("db").expect("Couldn't load db argument"))
    });

    save_config(args.global, &config)?;

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

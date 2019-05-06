//! The `config kv/meta sqlite` subcommand

use crate::PolyfsResult;
use crate::cli::ArgSet;
use crate::cli::config::{load_config, save_config};
use crate::app::config::{KvBackend, MetaBackend};
use crate::app::backends::BackendType;
use crate::app::backends::dual::sqlite::{SqliteConfig, SqliteDb};

use clap::{App, Arg, ArgGroup, SubCommand};

/// Run `sqlite` subcommand
pub fn run<'a>(args: ArgSet, backend: BackendType) -> PolyfsResult<()> {
    log::debug!("Running `sqlite` subcommand");
    let mut config = load_config(args.global)?;

    let sqlite_config = SqliteConfig {
        db: match args.sub.value_of("db_file") {
            Some(file) => SqliteDb::File(file.into()),
            None => {
                if !args.sub.is_present("in_memory") {
                    panic!("db_file not specified, but in_memory not present");
                } else {
                    SqliteDb::InMemory
                }
            }
        }
    };

    match backend {
        BackendType::KeyValue => config.backends.key_value = KvBackend::Sqlite(sqlite_config),
        BackendType::Metadata => config.backends.metadata = MetaBackend::Sqlite(sqlite_config),
    }

    save_config(args.global, &config)?;

    Ok(())
}

/// Get CLI for the `sqlite` subcommand
#[rustfmt::skip]
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sqlite")
        .about("Configure Sqlite backend")
        .arg(Arg::with_name("db_file")
            .long("db")
            .short("-f")
            .value_name("db-file")
            .help("Path to the Sqlite database"))
        .arg(Arg::with_name("in_memory")
            .long("in-memory")
            .short("m")
            .help("Use an in-memory Sqlite database for testing"))
        .group(ArgGroup::with_name("db")
            .args(&["db_file", "in_memory"])
            .required(true))
}

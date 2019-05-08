//! PolyFS `mount` subcommand

use crate::try_to;
use crate::PolyfsResult;
use crate::cli::ArgSet;
use crate::cli::config::load_config;
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
pub fn run(args: ArgSet) -> PolyfsResult<()> {
    log::debug!("Running `mount` subcommand");

    use crate::app::backends::keyvalue::{KeyValueStore, KeyValueError};
    use crate::app::backends::dual::sqlite::SqliteKvStore;
    use crate::app::config::KvBackend;

    let config = load_config(args.global)?;

    let kv_store:SqliteKvStore;
    match config.backends.key_value {
        KvBackend::Sqlite(sqlite_config) => {
            kv_store = SqliteKvStore::new(sqlite_config)?;
        }
    }

    // TODO: Convert this error
    match kv_store.get("test") {
        Ok(value) => println!("{}", value),
        Err(KeyValueError::KeyNotFound) => println!("Key not found"),
        Err(KeyValueError::DatabaseError(e)) => try_to!(Err(e), "Couldn't connect to database"),
    }

    Ok(())
}

//! PolyFS `mount` subcommand

use crate::cli::config::load_config;
use crate::cli::ArgSet;
use crate::PolyfsResult;
use clap::{App, Arg, SubCommand};

/// Get CLI for the `mount` subcommand
pub fn get_cli<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("mount")
        .about("Mount the filesystem")
        .arg(
            Arg::with_name("read_only")
                .long("read-only")
                .short("r")
                .help("Mount the filesystem as read-only"),
        )
        .arg(
            Arg::with_name("mountpoint")
                .help("location to mount the filesystem")
                .required(true),
        )
}

/// Run `mount` subcommand
pub fn run(args: ArgSet) -> PolyfsResult<()> {
    log::debug!("Running `mount` subcommand");

    use crate::try_to;
    use crate::app::backends::dual::sqlite::SqliteKvStore;
    use crate::app::backends::keyvalue::KeyValueStore;
    use crate::app::config::KvBackend;

    let config = load_config(args.global)?;

    let kv_store;
    match config.backends.key_value {
        KvBackend::Sqlite(sqlite_config) => {
            kv_store = SqliteKvStore::new(sqlite_config)?;
        }
    }

    try_to!(kv_store.set("hello", "world"), "Couldn't set key");
    try_to!(kv_store.set("dan", "haws"), "Couldn't set key");

    Ok(())
}

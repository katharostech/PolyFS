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

    use crate::app::backends::dual::sqlite::SqliteKvStore;
    use crate::app::backends::dual::sqlite::SqliteMetaStore;
    use crate::app::config::{KvBackend, MetaBackend};
    use crate::app::filesystem::PolyfsFilesystem;

    let mountpoint = args
        .sub
        .value_of("mountpoint")
        .expect("Could not load mountpoint arg");
    let config = load_config(args.global)?;

    let kv_store;
    match config.backends.key_value {
        KvBackend::Sqlite(sqlite_config) => {
            kv_store = SqliteKvStore::new(sqlite_config)?;
        }
    }

    let meta_store;
    match config.backends.metadata {
        MetaBackend::Sqlite(sqlite_config) => {
            meta_store = SqliteMetaStore::new(sqlite_config);
        }
    }

    use std::ffi::OsStr;
    let fuse_args: &[&OsStr] = &[&OsStr::new("-o"), &OsStr::new("auto_unmount")];
    let filesystem = PolyfsFilesystem::new(kv_store, meta_store);
    crate::try_to!(
        fuse::mount(filesystem, &mountpoint, fuse_args),
        "Could not mount filesystem"
    );

    Ok(())
}

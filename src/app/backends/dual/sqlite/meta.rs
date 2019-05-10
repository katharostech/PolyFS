//! Metadata store implementation for Sqlite
pub mod meta_schema;

mod types;
use self::types::*;

use super::SqliteConfig;
use crate::app::backends::metadata::{MetadataStore, MetadataResult, MetadataError};

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::embed_migrations;

embed_migrations!("src/app/backends/dual/sqlite/meta/meta-migrations");

/// Metadata store implementation for Sqlite
pub struct SqliteMetaStore {}

impl SqliteMetaStore {
    pub fn new(_config: SqliteConfig) -> SqliteMetaStore {
        SqliteMetaStore {}
    }
}

impl MetadataStore for SqliteMetaStore {
    fn get_file_attr_in_dir(&self, parent: u64, name: &str) -> MetadataResult<fuse::FileAttr> {
        panic!("Not implemented");
    }

    fn get_file_attr(&self, ino: u64) -> MetadataResult<fuse::FileAttr> {
        panic!("Not implemented");
    }
}

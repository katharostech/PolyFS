//! Metadata store implementation for Sqlite

use super::SqliteConfig;
use crate::app::backends::metadata::MetadataStore;

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
    fn dummy(&self) {
        println!("Filesystem init!");
    }
}

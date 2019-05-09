//! Metadata store implementation for Sqlite

use super::SqliteConfig;
use crate::app::backends::metadata::MetadataStore;

/// Metadata store implementation for Sqlite
pub struct SqliteMetaStore {}

impl SqliteMetaStore {
    pub fn new(_config: SqliteConfig) -> SqliteMetaStore {
        SqliteMetaStore {}
    }
}

impl MetadataStore for SqliteMetaStore {
    fn dummy(&self) {
        println!("SqliteMetaStore.dummy()");
    }
}

//! Sqlite storage backend

use serde::{Serialize, Deserialize};

/// Sqlite database configuation structure
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct SqliteConfig {
    pub db: SqliteDb
}

/// Sqlite database type
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum SqliteDb {
    /// An in-memory Sqlite database for testing
    InMemory,
    /// A temporary database that will be cleaned up on exit
    Temporary,
    /// An Sqlite database file
    #[serde(rename = "file")]
    File(String),
}

impl Default for SqliteDb {
    fn default() -> SqliteDb {
        SqliteDb::Temporary
    }
}

pub mod kv;
pub use self::kv::SqliteKvStore;

pub mod meta;
pub use self::meta::SqliteMetaStore;


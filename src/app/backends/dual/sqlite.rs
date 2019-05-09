//! Sqlite storage backend

use serde::{Serialize, Deserialize};

/// Sqlite database configuation structure
#[derive(Serialize, Deserialize, Debug)]
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
    /// An Sqlite database file
    #[serde(rename = "file")]
    File(String),
}

pub mod kv;
pub use self::kv::SqliteKvStore;

pub mod meta;
pub use self::meta::SqliteMetaStore;


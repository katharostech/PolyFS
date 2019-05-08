use crate::app::backends::keyvalue::{KeyValueResult, KeyValueStore};
use crate::{try_to, PolyfsResult};

use super::{SqliteConfig, SqliteDb};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use diesel_migrations::embed_migrations;

embed_migrations!("src/app/backends/dual/sqlite/kv-migrations");

#[derive(Queryable)]
pub struct KvPair {
    pub key: String,
    pub value: String,
}

pub struct SqliteKvStore {
    config: SqliteConfig,
    conn: SqliteConnection,
}

use std::fmt;

impl fmt::Debug for SqliteKvStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SqliteKvStore {{ config: {:#?} }}", self.config)
    }
}

impl SqliteKvStore {
    pub fn new(config: SqliteConfig) -> PolyfsResult<SqliteKvStore> {
        let db_path = match &config.db {
            SqliteDb::InMemory => ":memory:".into(),
            SqliteDb::File(file) => file.clone(),
        };

        let conn = try_to!(
            SqliteConnection::establish(&db_path),
            "Could not connect to database for KV store"
        );

        try_to!(embedded_migrations::run(&conn), "Could not run migrations");

        Ok(SqliteKvStore { config, conn })
    }
}

impl KeyValueStore for SqliteKvStore {
    fn get(&self, key: &str) -> KeyValueResult<String> {
        use super::kv_schema::kv_store;

        // TODO: Don't use expect, use try_to or match
        let results: Vec<KvPair> = kv_store::table
            .filter(kv_store::key.eq(key))
            .load::<KvPair>(&self.conn)
            .expect("Could not connect to database");

        if results.len() > 0 {
            Ok(results[0].value.clone())
        } else {
            Ok("Key not found".into())
        }
    }
}

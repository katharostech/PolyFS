//! Sqlite key-value store implementation

use super::{SqliteConfig, SqliteDb};
use crate::app::keyvalue::{KeyValueError, KeyValueResult, KeyValueStore};
use crate::{PolyfsResult, try_to};

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::embed_migrations;

mod kv_schema;
use self::kv_schema::kv_store;

embed_migrations!("src/app/backends/sqlite/kv/kv-migrations");

/// A Queryable and Insertable KeyValue pair
#[derive(Queryable, Insertable)]
#[table_name = "kv_store"]
pub struct KvPair {
    /// The key in the KV pair
    pub key: Vec<u8>,
    /// The value in the KV pair
    pub value: Vec<u8>,
}

/// A Sqlite backed implementation of `KeyValueStore`
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
    /// Instantiate a Sqlite KV store
    pub fn new(config: SqliteConfig) -> PolyfsResult<SqliteKvStore> {
        let db_path = match &config.db {
            SqliteDb::InMemory => ":memory:".into(),
            SqliteDb::Temporary => "".into(),
            SqliteDb::File(file) => file.clone(),
        };

        let conn = try_to!(
            SqliteConnection::establish(&db_path),
            "Could not connect to database for KV store"
        );

        // TODO: Migrations should not be run without warning the user to backup
        // their database first
        try_to!(
            embedded_migrations::run(&conn),
            "Could not run database migrations"
        );

        Ok(SqliteKvStore { config, conn })
    }
}

impl KeyValueStore for SqliteKvStore {
    fn get(&self, key: Vec<u8>) -> KeyValueResult<Option<Vec<u8>>> {
        match kv_store::table
            .filter(kv_store::key.eq(key))
            .get_result::<KvPair>(&self.conn)
        {
            Ok(kv_pair) => Ok(Some(kv_pair.value)),
            Err(DieselError::NotFound) => Ok(None),
            Err(other_error) => Err(KeyValueError::DatabaseError(other_error)),
        }
    }

    fn set(&self, key: Vec<u8>, value: Vec<u8>) -> KeyValueResult<()> {
        diesel::replace_into(kv_store::table)
            .values(KvPair {
                key,
                value,
            })
            .execute(&self.conn)?;

        Ok(())
    }

    fn delete(&self, key: Vec<u8>) -> KeyValueResult<()> {
        diesel::delete(kv_store::table.filter(kv_store::key.eq(key))).execute(&self.conn)?;

        Ok(())
    }

    fn list(&self) -> KeyValueResult<Vec<Vec<u8>>> {
        Ok(kv_store::table
            .select(kv_store::key)
            .load::<Vec<u8>>(&self.conn)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::keyvalue::KeyValueStore;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    const DB_CONFIG: SqliteConfig = SqliteConfig {
        db: SqliteDb::InMemory,
    };

    #[test]
    fn set_and_get() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        // Set a couple values then get them
        kv_store.set(b"hello".to_vec().to_vec(), "world".as_bytes().to_vec())?;
        kv_store.set(b"goodbye".to_vec(), "later".as_bytes().to_vec())?;
        assert_eq!(kv_store.get(b"hello".to_vec())?.unwrap(), "world".as_bytes());
        assert_eq!(kv_store.get(b"goodbye".to_vec())?.unwrap(), "later".as_bytes());

        Ok(())
    }

    #[test]
    fn set_and_update_key() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        kv_store.set(b"hello".to_vec(), "world".as_bytes().to_vec())?;
        assert_eq!(kv_store.get(b"hello".to_vec())?.unwrap(), "world".as_bytes());
        kv_store.set(b"hello".to_vec(), "mister".as_bytes().to_vec())?;
        assert_eq!(kv_store.get(b"hello".to_vec())?.unwrap(), "mister".as_bytes());

        Ok(())
    }

    #[test]
    fn get_nothing() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        // Get a non-existant value
        assert_eq!(kv_store.get(b"none".to_vec())?, None);

        Ok(())
    }

    #[test]
    fn delete_key() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        // Set a value and make sure it is set
        kv_store.set(b"hello".to_vec(), "world".as_bytes().to_vec())?;
        assert_eq!(kv_store.get(b"hello".to_vec())?.unwrap(), "world".as_bytes());

        // Delete a value and make sure it is none afterwards
        kv_store.delete(b"hello".to_vec())?;
        assert_eq!(kv_store.get(b"hello".to_vec())?, None);

        Ok(())
    }

    #[test]
    fn list_keys() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        kv_store.set(b"hello".to_vec(), "world".as_bytes().to_vec())?;
        kv_store.set(b"goodbye".to_vec(), "world".as_bytes().to_vec())?;

        assert_eq!(kv_store.list()?.sort(), vec!["hello", "goodbye"].sort());

        Ok(())
    }
}

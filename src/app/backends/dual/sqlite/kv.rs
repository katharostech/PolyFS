use crate::app::backends::keyvalue::{KeyValueError, KeyValueResult, KeyValueStore};
use crate::{PolyfsResult, try_to};

use super::{SqliteConfig, SqliteDb};

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sqlite::SqliteConnection;

use diesel_migrations::embed_migrations;

embed_migrations!("src/app/backends/dual/sqlite/kv-migrations");

use super::kv_schema::kv_store;

#[derive(Queryable, Insertable)]
#[table_name = "kv_store"]
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

        try_to!(
            embedded_migrations::run(&conn),
            "Could not run database migrations"
        );

        Ok(SqliteKvStore { config, conn })
    }
}

impl KeyValueStore for SqliteKvStore {
    fn get(&self, key: &str) -> KeyValueResult<Option<String>> {
        match kv_store::table
            .filter(kv_store::key.eq(key))
            .get_result::<KvPair>(&self.conn)
        {
            Ok(kv_pair) => Ok(Some(kv_pair.value)),
            Err(DieselError::NotFound) => Ok(None),
            Err(other_error) => Err(KeyValueError::DatabaseError(other_error)),
        }
    }

    fn set(&self, key: &str, value: &str) -> KeyValueResult<()> {
        diesel::replace_into(kv_store::table)
            .values(KvPair {
                key: key.into(),
                value: value.into(),
            })
            .execute(&self.conn)?;

        Ok(())
    }

    fn delete(&self, key: &str) -> KeyValueResult<()> {
        diesel::delete(kv_store::table.filter(kv_store::key.eq(key))).execute(&self.conn)?;

        Ok(())
    }

    fn list(&self) -> KeyValueResult<Vec<String>> {
        Ok(kv_store::table
            .select(kv_store::key)
            .load::<String>(&self.conn)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::backends::keyvalue::KeyValueStore;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    const DB_CONFIG: SqliteConfig = SqliteConfig {
        db: SqliteDb::InMemory,
    };

    #[test]
    fn set_and_get() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        // Set a couple values then get them
        kv_store.set("hello", "world")?;
        kv_store.set("goodbye", "later")?;
        assert_eq!(kv_store.get("hello")?.unwrap(), "world");
        assert_eq!(kv_store.get("goodbye")?.unwrap(), "later");

        Ok(())
    }

    #[test]
    fn set_and_update_key() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        kv_store.set("hello", "world")?;
        assert_eq!(kv_store.get("hello")?.unwrap(), "world");
        kv_store.set("hello", "mister")?;
        assert_eq!(kv_store.get("hello")?.unwrap(), "mister");

        Ok(())
    }

    #[test]
    fn get_nothing() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        // Get a non-existant value
        assert_eq!(kv_store.get("none")?, None);

        Ok(())
    }

    #[test]
    fn delete_key() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        // Set a value and make sure it is set
        kv_store.set("hello", "world")?;
        assert_eq!(kv_store.get("hello")?.unwrap(), "world");

        // Delete a value and make sure it is none afterwards
        kv_store.delete("hello")?;
        assert_eq!(kv_store.get("hello")?, None);

        Ok(())
    }

    #[test]
    fn list_keys() -> TestResult {
        let kv_store = SqliteKvStore::new(DB_CONFIG)?;

        kv_store.set("hello", "world")?;
        kv_store.set("goodbye", "world")?;

        assert_eq!(kv_store.list()?.sort(), vec!["hello", "goodbye"].sort());

        Ok(())
    }
}

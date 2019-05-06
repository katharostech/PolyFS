use serde::{Serialize, Deserialize};
use crate::app::backends::dual::sqlite::{SqliteConfig, SqliteDb};

/// Get the default PolyFS configuration
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            backends: BackendConfig {
                key_value: KvBackend::Sqlite(SqliteConfig {
                    db: SqliteDb::File("kv.sqlite3".into()),
                }),
                metadata: MetaBackend::Sqlite(SqliteConfig {
                    db: SqliteDb::File("meta.sqlite3".into())
                })
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub backends: BackendConfig
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BackendConfig {
    pub key_value: KvBackend,
    pub metadata: MetaBackend,
}

//
// Key-Value Backends
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum KvBackend {
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig)
}

//
// Metadata Backends
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum MetaBackend {
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig)
}

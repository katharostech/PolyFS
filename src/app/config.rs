use serde::{Serialize, Deserialize};

/// Get the default PolyFS configuration
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            backends: BackendConfig {
                key_value: KvBackend::Sqlite(SqliteKvConfig {
                    db: String::from("kv.db"),
                }),
                metadata: MetaBackend::Sqlite(SqliteMetaConfig {
                    db: String::from("meta.db"),
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

use crate::app::backends::keyvalue::sqlite::SqliteConfig as SqliteKvConfig;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum KvBackend {
    #[serde(rename = "sqlite")]
    Sqlite(SqliteKvConfig)
}

//
// Metadata Backends
//

use crate::app::backends::metadata::sqlite::SqliteConfig as SqliteMetaConfig;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum MetaBackend {
    #[serde(rename = "sqlite")]
    Sqlite(SqliteMetaConfig)
}

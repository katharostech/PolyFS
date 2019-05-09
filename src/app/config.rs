//! Module containing aspects of the global application configuration

use serde::{Serialize, Deserialize};
use crate::app::backends::dual::sqlite::SqliteConfig;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub backends: BackendConfig
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

impl Default for KvBackend {
    fn default() -> KvBackend {
        KvBackend::Sqlite(SqliteConfig::default())
    }
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

impl Default for MetaBackend {
    fn default() -> MetaBackend {
        MetaBackend::Sqlite(SqliteConfig::default())
    }
}

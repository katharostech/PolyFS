//! Module containing aspects of the global application configuration

use serde::{Serialize, Deserialize};
use crate::app::backends::sqlite::SqliteConfig;

/// Application config
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    /// Storage backend configuration
    pub backend: Backend
}

/// A supported storage backend with its config
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all="snake_case")]
pub enum Backend {
    /// Sqlite backend config
    Sqlite(SqliteConfig)
}

impl Default for Backend {
    fn default() -> Backend {
        Backend::Sqlite(SqliteConfig::default())
    }
}

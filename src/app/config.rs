//! Module containing aspects of the global application configuration

use serde::{Serialize, Deserialize};
use crate::app::backends::sqlite::SqliteConfig;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub backend: Backend
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Backend {
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig)
}

impl Default for Backend {
    fn default() -> Backend {
        Backend::Sqlite(SqliteConfig::default())
    }
}

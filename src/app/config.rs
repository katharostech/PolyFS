use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub key_value: KvBackend
}

use crate::app::backends::keyvalue::sqlite::SqliteConfig;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum KvBackend {
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig)
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub key_value: KvBackend
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum KvBackend {
    sqlite(SqliteConfig)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SqliteConfig {
    pub db: String
}

/// The result of a KeyValueStore operation
pub type KeyValueResult<T> = Result<T, KeyValueError>;

/// An error resulting from a KeyValue store operation
#[derive(Debug)]
pub enum KeyValueError {
    /// A diesel error returned as a result of the operation.
    DatabaseError(diesel::result::Error)
}

use std::fmt;
impl fmt::Display for KeyValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            KeyValueError::DatabaseError(error) => write!(f, "DatabaseError: {}", error)
        }
    }
}

impl std::error::Error for KeyValueError {}

impl From<diesel::result::Error> for KeyValueError {
    fn from(error: diesel::result::Error) -> Self {
        KeyValueError::DatabaseError(error)
    }
}

/// A key value store
pub trait KeyValueStore {
    /// Get the value of a key
    fn get(&self, key: &str) -> KeyValueResult<Option<String>>;
    /// Set the value of a key
    fn set(&self, key: &str, value: &str) -> KeyValueResult<()>;
    /// Delete a key and its value
    fn delete(&self, key: &str) -> KeyValueResult<()>;
    /// List all keys in the store
    fn list(&self) -> KeyValueResult<Vec<String>>;
}

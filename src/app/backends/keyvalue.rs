/// Key value store result type
pub type KeyValueResult<T> = Result<T, KeyValueError>;

/// The key value error type
#[derive(Debug)]
pub enum KeyValueError {
    /// Failure to connect to database
    ConnectionError(Box<dyn std::error::Error>),
    /// Key does not exist
    KeyNotFound
}

/// A key value store
pub trait KeyValueStore {
    /// Get the value of a key
    fn get(&self, key: &str) -> KeyValueResult<String>;
    // fn set(&self, key: &str) -> PolyfsResult<()>;
    // fn list(&self) -> PolyfsResult<&[&str]>;
}

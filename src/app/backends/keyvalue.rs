use diesel::result::Error as DieselError;

/// Key value store result type
pub type KeyValueResult<T> = Result<T, KeyValueError>;

/// The key value error type
#[derive(Debug)]
pub enum KeyValueError {
    /// Failure to connect to database
    DatabaseError(DieselError),
    /// Key does not exist
    KeyNotFound
}

impl std::convert::From<DieselError> for KeyValueError {
    fn from(error: DieselError) -> Self {
        KeyValueError::DatabaseError(error)
    }
}

/// A key value store
pub trait KeyValueStore {
    /// Get the value of a key
    fn get(&self, key: &str) -> KeyValueResult<String>;
    // fn set(&self, key: &str) -> PolyfsResult<()>;
    // fn list(&self) -> PolyfsResult<&[&str]>;
}

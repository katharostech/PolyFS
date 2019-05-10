//! Module containing backends and types specific to metadata stores

use fuse::FileAttr;

/// The result of a MetadataStore operation
pub type MetadataResult<T> = Result<T, MetadataError>;

/// An error resulting from a KeyValue store operation
#[derive(Debug)]
pub enum MetadataError {
    /// A diesel error returned as a result of the operation.
    DatabaseError(diesel::result::Error),
}

use std::fmt;
impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MetadataError::DatabaseError(error) => write!(f, "DatabaseError: {}", error),
        }
    }
}

impl std::error::Error for MetadataError {}

impl From<diesel::result::Error> for MetadataError {
    fn from(error: diesel::result::Error) -> Self {
        MetadataError::DatabaseError(error)
    }
}

/// A filesystem Metadata store trait
pub trait MetadataStore {
    /// Get the file attributes of a file with `name` in the directory with an
    /// ino of `parent`
    fn get_file_attr_in_dir(&self, parent: u64, name: &str) -> MetadataResult<FileAttr>;
    /// Get the file attributes of the file with the given `ino`
    fn get_file_attr(&self, ino: u64) -> MetadataResult<FileAttr>;
}

//! Module containing the different filesystem backends

pub mod dual;
pub mod keyvalue;
pub mod metadata;

/// A type of storage that a backend can provide
pub enum BackendType {
    /// A key-value store
    KeyValue,
    /// A metadata store
    Metadata
}

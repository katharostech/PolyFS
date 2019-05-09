//! Module containing backends and types specific to metadata stores

/// A filesystem Metadata store trait
pub trait MetadataStore {
    fn dummy(&self);
}

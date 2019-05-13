//! The PolyFS FUSE filesystem implemented on top of the key-value and metadata
//! storage backends

use crate::app::keyvalue::KeyValueStore;

use fuse::{Filesystem, ReplyEntry, Request};
use std::ffi::OsStr;
use time::Timespec;

/// The PolyFS filesystem implementation
pub struct PolyfsFilesystem<KvStore: KeyValueStore> {
    kv_store: KvStore,
}

impl<KvStore: KeyValueStore> PolyfsFilesystem<KvStore> {
    pub fn new(kv_store: KvStore) -> PolyfsFilesystem<KvStore> {
        PolyfsFilesystem { kv_store }
    }
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl<KvStore> Filesystem for PolyfsFilesystem<KvStore>
where
    KvStore: KeyValueStore,
{
}

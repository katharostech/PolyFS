//! The PolyFS FUSE filesystem implemented on top of the key-value and metadata
//! storage backends

use crate::app::backends::keyvalue::KeyValueStore;
use crate::app::backends::metadata::MetadataStore;

use fuse::{Filesystem, ReplyEntry, Request};
use std::ffi::OsStr;
use time::Timespec;

/// The PolyFS filesystem implementation
pub struct PolyfsFilesystem<KvStore: KeyValueStore, MetaStore: MetadataStore> {
    kv_store: KvStore,
    meta_store: MetaStore,
}

impl<KvStore: KeyValueStore, MetaStore: MetadataStore> PolyfsFilesystem<KvStore, MetaStore> {
    pub fn new(kv_store: KvStore, meta_store: MetaStore) -> PolyfsFilesystem<KvStore, MetaStore> {
        PolyfsFilesystem {
            kv_store,
            meta_store,
        }
    }
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl<KvStore, MetaStore> Filesystem for PolyfsFilesystem<KvStore, MetaStore>
where
    KvStore: KeyValueStore,
    MetaStore: MetadataStore,
{
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        reply.entry(
            &TTL,
            &self
                .meta_store
                .get_file_attr_in_dir(
                    parent,
                    name.to_str()
                        .expect("Attempt to lookup non-unicode filepath"),
                )
                .unwrap(),
            0,
        )
    }
}

//! The PolyFS FUSE filesystem implemented on top of the key-value and metadata
//! storage backends

use crate::app::backends::keyvalue::KeyValueStore;
use crate::app::backends::metadata::MetadataStore;

// use fuse_mt::{FilesystemMT, RequestInfo, ResultEmpty};
use fuse::{Filesystem, Request};

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

impl<KvStore, MetaStore> Filesystem for PolyfsFilesystem<KvStore, MetaStore>
where
    KvStore: KeyValueStore,
    MetaStore: MetadataStore,
{
    fn init(&mut self, _req: &Request) -> Result<(), i32> {
        self.meta_store.dummy();       
        self.kv_store.set("hello", "world".as_bytes().to_vec()).expect("Testing ignore me");

        Ok(())
    }
}

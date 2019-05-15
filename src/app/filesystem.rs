//! The PolyFS FUSE filesystem implemented on top of the key-value and metadata
//! storage backends

use crate::app::keyvalue::KeyValueStore;

use bincode::{deserialize, serialize};
use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyDirectory, ReplyEmpty, ReplyEntry, Request,
};
use libc::ENOENT;
use log::{debug, trace};
use std::convert::TryInto;
use std::ffi::OsStr;
use time::Timespec;

mod types;
use self::types::*;

/// The PolyFS filesystem implementation
pub struct PolyfsFilesystem<KvStore: KeyValueStore> {
    kv_store: KvStore,
}

impl<KvStore: KeyValueStore> PolyfsFilesystem<KvStore> {
    /// Create a filesystem instance backed by the provided `KeyValueStore`
    pub fn new(kv_store: KvStore) -> PolyfsFilesystem<KvStore> {
        PolyfsFilesystem { kv_store }
    }

    /// Get an inode id that isn't used by any existing node
    ///
    /// The implementation involves generating a random ino and checking to see
    /// whether or not it exists.
    ///
    /// TODO: This will be very unperformant as soon as the number of inodes
    /// approaches the maxiumum number of inodes, but I'm not sure if that will
    /// ever happen. Either way we should do this differently later.
    fn get_available_ino(&self) -> u64 {
        let is_used = |ino| {
            let key = KvQuery::FileAttributes(ino).get_key();
            match self.kv_store.get(key).unwrap() {
                Some(_) => true,
                None => false,
            }
        };

        let mut ino;
        loop {
            ino = rand::random::<u64>();

            if !is_used(ino) {
                return ino;
            }
        }
    }

    fn create_file(
        &self,
        file_type: FileType,
        req: &Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        _rdev: Option<u32>,
        reply: ReplyEntry,
    ) {
        let ino = self.get_available_ino();

        let created_time = time::get_time();

        let attributes = FileAttr {
            ino,
            size: 0,
            blocks: 0,
            atime: created_time.clone(),
            mtime: created_time.clone(),
            ctime: created_time.clone(),
            crtime: created_time.clone(),
            kind: file_type,
            perm: mode as u16,
            nlink: 1,
            uid: req.uid(),
            gid: req.gid(),
            rdev: 0,
            flags: 0,
        };

        // Insert file attributes
        let key = KvQuery::FileAttributes(ino).get_key();
        self.kv_store
            .set(key, serialize(&SerdeFileAttr(attributes)).unwrap())
            .unwrap();

        // Insert file record
        let key = KvQuery::Files(parent, name).get_key();
        self.kv_store.set(key, ino.to_le_bytes().to_vec()).unwrap();

        // Update inode children record
        let key = KvQuery::InodeChildren(parent).get_key();
        let data: Vec<u8>;
        let mut child_list = match self.kv_store.get(key.clone()).unwrap() {
            Some(bytes) => {
                data = bytes;
                deserialize::<Vec<(u64, SerdeFileType, &str)>>(&data).unwrap()
            }
            None => vec![],
        };

        child_list.push((ino, SerdeFileType(attributes.kind), name.to_str().unwrap()));

        self.kv_store
            .set(key, serialize(&child_list).unwrap())
            .unwrap();

        reply.entry(&TTL, &attributes, 0);
    }

    fn remove_file(&self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let key = KvQuery::Files(parent, name).get_key();
        let ino = match self.kv_store.get(key.clone()).unwrap() {
            Some(data) => u64::from_le_bytes(data.as_slice().try_into().unwrap()),
            None => {
                reply.error(ENOENT);
                return;
            }
        };

        // Remove file record
        self.kv_store.delete(key).unwrap();

        // Remove entry from `inode_children`
        let key = KvQuery::InodeChildren(parent).get_key();
        let mut data: Vec<u8> = Vec::new();
        let mut dir_children = self
            .kv_store
            .get(key.clone())
            .unwrap()
            .map_or(vec![], |bytes| {
                data = bytes;
                deserialize::<Vec<(u64, SerdeFileType, &str)>>(&data).unwrap()
            });

        for (index, (item_ino, _, _)) in dir_children.iter().enumerate() {
            if *item_ino == ino {
                dir_children.remove(index);
                break;
            }
        }

        self.kv_store
            .set(key, serialize(&dir_children).unwrap())
            .unwrap();

        // Delete file attributes
        self.kv_store
            .delete(KvQuery::FileAttributes(ino).get_key())
            .unwrap();

        reply.ok();
    }
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };
const DEFAULT_TIME: Timespec = Timespec { sec: 0, nsec: 0 };

impl<KvStore> Filesystem for PolyfsFilesystem<KvStore>
where
    KvStore: KeyValueStore,
{
    fn init(&mut self, _req: &Request) -> Result<(), i32> {
        println!("Starting up FUSE filesystem");

        // Insert the attributes for the mountpoint directory
        let key = KvQuery::FileAttributes(1).get_key();

        if let None = self.kv_store.get(key.clone()).unwrap() {
            self.kv_store
                .set(
                    key,
                    serialize(&SerdeFileAttr(FileAttr {
                        ino: 2,
                        size: 13,
                        blocks: 1,
                        atime: DEFAULT_TIME,
                        mtime: DEFAULT_TIME,
                        ctime: DEFAULT_TIME,
                        crtime: DEFAULT_TIME,
                        kind: FileType::Directory,
                        perm: 0o777,
                        nlink: 1,
                        uid: 1001,
                        gid: 1001,
                        rdev: 0,
                        flags: 0,
                    }))
                    .unwrap(),
                )
                .unwrap();
        }

        Ok(())
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        debug!(
            "Lookup: parent({}), name({})",
            parent,
            name.to_str().unwrap()
        );
        // Get inode of requested file
        let key = KvQuery::Files(parent, name).get_key();
        let ino = match self.kv_store.get(key).unwrap() {
            Some(data) => {
                let ino = u64::from_le_bytes(
                    data.as_slice()
                        .try_into()
                        .expect("Could not decode data from database"),
                );

                debug!("    Found: ino({})", ino);

                ino
            }
            None => {
                debug!("    Not found: ENOENT");
                reply.error(ENOENT);
                return;
            }
        };

        debug!("    Found: ino({})", ino);

        // Get file attributes using inode
        let key = KvQuery::FileAttributes(ino).get_key();
        match self.kv_store.get(key).unwrap() {
            Some(data) => {
                let attributes: FileAttr = deserialize::<SerdeFileAttr>(data.as_slice()).unwrap().0;

                trace!("    Attr: {:#?}", attributes);
                reply.entry(&TTL, &attributes, 0);
                return;
            }
            None => {
                debug!("    Attributes not found for ino!");
                reply.error(ENOENT);
                return;
            }
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        debug!("Get attr: ino({})", ino);
        let key = KvQuery::FileAttributes(ino).get_key();
        match self.kv_store.get(key).unwrap() {
            Some(data) => {
                let attributes: FileAttr = deserialize::<SerdeFileAttr>(data.as_slice()).unwrap().0;

                debug!("    Found attr");
                trace!("        {:#?}", attributes);

                reply.attr(&TTL, &attributes);
                return;
            }
            None => {
                debug!("    Not found: ENOENT");

                reply.error(ENOENT);
                return;
            }
        }
    }

    fn setattr(
        &mut self,
        _req: &Request,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Timespec>,
        mtime: Option<Timespec>,
        _fh: Option<u64>,
        crtime: Option<Timespec>,
        chgtime: Option<Timespec>,
        _bkuptime: Option<Timespec>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let key = KvQuery::FileAttributes(ino).get_key();

        // Get attributes
        let mut attributes = match self.kv_store.get(key.clone()).unwrap() {
            Some(data) => deserialize::<SerdeFileAttr>(data.as_slice()).unwrap().0,
            None => {
                reply.error(ENOENT);
                return;
            }
        };

        // Patch attributes
        if let Some(value) = mode {
            attributes.perm = value as u16;
        }
        if let Some(value) = uid {
            attributes.uid = value;
        }
        if let Some(value) = gid {
            attributes.gid = value;
        }
        if let Some(value) = size {
            attributes.size = value;
        }
        if let Some(value) = atime {
            attributes.atime = value;
        }
        if let Some(value) = mtime {
            attributes.mtime = value;
        }
        // TODO: Handle fh
        if let Some(value) = crtime {
            attributes.crtime = value;
        }
        if let Some(value) = chgtime {
            attributes.mtime = value;
        }
        // TODO: Handle bkuptime
        if let Some(value) = flags {
            attributes.flags = value;
        }

        // Set attributes
        self.kv_store
            .set(key, serialize(&SerdeFileAttr(attributes)).unwrap())
            .unwrap();

        reply.attr(&TTL, &attributes)
    }

    fn unlink(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        self.remove_file(req, parent, name, reply);
    }

    fn rmdir(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        self.remove_file(req, parent, name, reply);
    }

    fn mknod(
        &mut self,
        req: &Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        self.create_file(
            FileType::RegularFile,
            req,
            parent,
            name,
            mode,
            Some(rdev),
            reply,
        );
    }

    fn mkdir(&mut self, req: &Request, parent: u64, name: &OsStr, mode: u32, reply: ReplyEntry) {
        self.create_file(FileType::Directory, req, parent, name, mode, None, reply);
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        debug!("Read dir: ino({}), offset({})", ino, offset);
        let key = KvQuery::InodeChildren(ino).get_key();
        let mut data: Vec<u8> = Vec::new();
        let mut children = self.kv_store.get(key).unwrap().map_or(vec![], |bytes| {
            data = bytes;
            deserialize::<Vec<(u64, SerdeFileType, &str)>>(&data).unwrap()
        });

        // TODO: Inserting these records may cause vector to realocate which
        // could be inefficient
        children.insert(0, (ino, SerdeFileType(FileType::Directory), ".."));
        children.insert(0, (ino, SerdeFileType(FileType::Directory), "."));

        for (index, (ino, SerdeFileType(file_type), filename)) in
            &mut children.split_off(offset as usize).iter().enumerate()
        {
            trace!(
                "    {:?}",
                (*ino, index as i64 + offset + 1, *file_type, filename)
            );
            if reply.add(*ino, index as i64 + offset + 1, *file_type, filename) {
                break;
            }
        }
        debug!("    Done");
        reply.ok();
    }
}

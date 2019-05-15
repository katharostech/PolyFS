use fuse::{FileAttr, FileType};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use time::Timespec;

/// Represents a query for a virtual table in the KV store
///
/// `KvQuery` implements `Into<Vec<u8>>` to allow using the query as the key
/// when accessing the KV store
pub enum KvQuery<'a> {
    /// Query file attributes by ino
    FileAttributes(u64),
    /// Query file inode by parent ino and filename
    Files(u64, &'a OsStr),
    /// Query inode children by ino
    InodeChildren(u64),
}

impl<'a> KvQuery<'a> {
    /// Generate a `Vec<u8>` key for representing the query as a key in the KV
    /// store
    pub fn get_key(self) -> Vec<u8> {
        let prefix = match &self {
            KvQuery::FileAttributes(_) => 0u8,
            KvQuery::Files(_, _) => 1u8,
            KvQuery::InodeChildren(_) => 2u8,
        };

        match self {
            KvQuery::FileAttributes(ino) => {
                let mut vec = vec![prefix];
                vec.extend_from_slice(&u64::to_le_bytes(ino));

                vec
            }
            KvQuery::Files(ino, filename) => {
                let mut vec = vec![prefix];
                vec.extend_from_slice(&u64::to_le_bytes(ino));
                vec.extend_from_slice(
                    filename
                        .to_str()
                        .expect("Error parsing bytes in filename")
                        .as_bytes(),
                );

                vec
            }
            KvQuery::InodeChildren(ino) => {
                let mut vec = vec![prefix];
                vec.extend_from_slice(&u64::to_le_bytes(ino));

                vec
            }
        }
    }
}

/// A serializable wrapper for `fuse::FileAttr`
#[derive(Serialize, Deserialize)]
pub struct SerdeFileAttr(#[serde(with = "FileAttrDef")] pub FileAttr);

#[derive(Serialize, Deserialize)]
#[serde(remote = "FileAttr")]
struct FileAttrDef {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    #[serde(with = "TimespecDef")]
    pub atime: Timespec,
    #[serde(with = "TimespecDef")]
    pub mtime: Timespec,
    #[serde(with = "TimespecDef")]
    pub ctime: Timespec,
    #[serde(with = "TimespecDef")]
    pub crtime: Timespec,
    #[serde(with = "FileTypeDef")]
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub flags: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Timespec")]
struct TimespecDef {
    pub sec: i64,
    pub nsec: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SerdeFileType(#[serde(with = "FileTypeDef")] pub FileType);

#[derive(Serialize, Deserialize)]
#[serde(remote = "FileType")]
enum FileTypeDef {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
}

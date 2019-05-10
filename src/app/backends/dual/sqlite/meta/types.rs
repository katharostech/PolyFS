use std::convert::TryInto;

use time::Timespec;
use fuse::FileType;

use super::meta_schema::file_attributes;

#[derive(Queryable, Insertable)]
#[table_name = "file_attributes"]
pub struct MetaFileAttr {
    pub ino: Vec<u8>,
    pub size: Vec<u8>,
    pub blocks: Vec<u8>,
    pub atime: Vec<u8>,
    pub mtime: Vec<u8>,
    pub ctime: Vec<u8>,
    pub crtime: Vec<u8>,
    pub kind: String,
    pub perm: Vec<u8>,
    pub nlink: Vec<u8>,
    pub uid: Vec<u8>,
    pub gid: Vec<u8>,
    pub rdev: Vec<u8>,
    pub flags: Vec<u8>,
}

impl Into<fuse::FileAttr> for MetaFileAttr {
    fn into(self) -> fuse::FileAttr {
        fuse::FileAttr {
            ino: *DecodedU64::from(self.ino),
            size: *DecodedU64::from(self.size),
            blocks: *DecodedU64::from(self.blocks),
            atime: *DecodedTimespec::from(self.atime),
            mtime: *DecodedTimespec::from(self.mtime),
            ctime: *DecodedTimespec::from(self.ctime),
            crtime: *DecodedTimespec::from(self.crtime),
            kind: *DecodedFileType::from(self.kind),
            perm: *DecodedU16::from(self.perm),
            nlink: *DecodedU32::from(self.nlink),
            uid: *DecodedU32::from(self.uid),
            gid: *DecodedU32::from(self.gid),
            rdev: *DecodedU32::from(self.rdev),
            flags: *DecodedU32::from(self.flags),
        }
    }
}

// Wrapper types used for conversions

pub struct FileAttrEncoder(fuse::FileAttr);

impl Into<MetaFileAttr> for FileAttrEncoder {
    fn into(self) -> MetaFileAttr {
        MetaFileAttr {
            ino: self.0.ino.to_le_bytes().to_vec(),
            size: self.0.size.to_le_bytes().to_vec(),
            blocks: self.0.blocks.to_le_bytes().to_vec(),
            atime: DecodedTimespec(self.0.atime).into(),
            mtime: DecodedTimespec(self.0.mtime).into(),
            ctime: DecodedTimespec(self.0.ctime).into(),
            crtime: DecodedTimespec(self.0.crtime).into(),
            kind: DecodedFileType(self.0.kind).into(),
            perm: self.0.perm.to_le_bytes().to_vec(),
            nlink: self.0.nlink.to_le_bytes().to_vec(),
            uid: self.0.uid.to_le_bytes().to_vec(),
            gid: self.0.gid.to_le_bytes().to_vec(),
            rdev: self.0.rdev.to_le_bytes().to_vec(),
            flags: self.0.flags.to_le_bytes().to_vec(),
        }
    }
}

struct DecodedFileType(FileType);

impl From<String> for DecodedFileType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "NamedPipe" => DecodedFileType(FileType::NamedPipe),
            "CharDevice" => DecodedFileType(FileType::NamedPipe),
            "BlockDevice" => DecodedFileType(FileType::BlockDevice),
            "Directory" => DecodedFileType(FileType::Directory),
            "RegularFile" => DecodedFileType(FileType::RegularFile),
            "Symlink" => DecodedFileType(FileType::Symlink),
            "Socket" => DecodedFileType(FileType::Socket),
            _ => panic!("Error decoding file type from database")
        }
    }
}

impl Into<String> for DecodedFileType {
    fn into(self) -> String {
        match &self {
            DecodedFileType(FileType::NamedPipe) => "NamedPipe",
            DecodedFileType(FileType::CharDevice) => "CharDevice",
            DecodedFileType(FileType::BlockDevice) => "BlockDevice",
            DecodedFileType(FileType::Directory) => "Directory",
            DecodedFileType(FileType::RegularFile) => "NamedPipe",
            DecodedFileType(FileType::Symlink) => "Symlink",
            DecodedFileType(FileType::Socket) => "Socket",
        }.into()
    }
}

impl std::ops::Deref for DecodedFileType {
    type Target = FileType;

    fn deref(&self) -> &FileType {
        &self.0
    }
}

struct DecodedTimespec(Timespec);

impl From<Vec<u8>> for DecodedTimespec {
    fn from(v: Vec<u8>) -> Self {
        let (sec, nsec) = v.split_at(std::mem::size_of::<i64>());

        DecodedTimespec(Timespec {
            sec: i64::from_le_bytes(sec.try_into().expect("File attributes in database corrupted")),
            nsec: i32::from_le_bytes(nsec.try_into().expect("File attributes in database corrupted")),
        })
    }
}

impl Into<Vec<u8>> for DecodedTimespec {
    fn into(self) -> Vec<u8> {
        let mut v: [u8; 12] = [0; 12];
        let mut i = 0;

        for byte in &self.0.sec.to_le_bytes() {
            v[i] = *byte;
            i += 1;
        }

        for byte in &self.0.nsec.to_le_bytes() {
            v[i] = *byte;
            i += 1;
        }

        v.to_vec()
    }
}

impl std::ops::Deref for DecodedTimespec {
    type Target = Timespec;

    fn deref(&self) -> &Timespec {
        &self.0
    }
}

struct DecodedU64(u64);

impl From<Vec<u8>> for DecodedU64 {
    fn from(v: Vec<u8>) -> Self {
        DecodedU64(u64::from_le_bytes(v.as_slice().try_into().expect("File attributes in database corrupted")))
    }
}

impl std::ops::Deref for DecodedU64 {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

struct DecodedU32(u32);

impl From<Vec<u8>> for DecodedU32 {
    fn from(v: Vec<u8>) -> Self {
        DecodedU32(u32::from_le_bytes(v.as_slice().try_into().expect("File attributes in database corrupted")))
    }
}

impl std::ops::Deref for DecodedU32 {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

struct DecodedU16(u16);

impl From<Vec<u8>> for DecodedU16 {
    fn from(v: Vec<u8>) -> Self {
        DecodedU16(u16::from_le_bytes(v.as_slice().try_into().expect("File attributes in database corrupted")))
    }
}

impl std::ops::Deref for DecodedU16 {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.0
    }
}

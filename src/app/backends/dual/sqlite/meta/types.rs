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
            ino: decode_u64(self.ino),
            size: decode_u64(self.size),
            blocks: decode_u64(self.blocks),
            atime: decode_timespec(self.atime),
            mtime: decode_timespec(self.mtime),
            ctime: decode_timespec(self.ctime),
            crtime: decode_timespec(self.crtime),
            kind: decode_file_type(&self.kind),
            perm: decode_u16(self.perm),
            nlink: decode_u32(self.nlink),
            uid: decode_u32(self.uid),
            gid: decode_u32(self.gid),
            rdev: decode_u32(self.rdev),
            flags: decode_u32(self.flags),
        }
    }
}

impl From<fuse::FileAttr> for MetaFileAttr {
    fn from(f: fuse::FileAttr) -> MetaFileAttr {
        MetaFileAttr {
            ino: f.ino.to_le_bytes().to_vec(),
            size: f.size.to_le_bytes().to_vec(),
            blocks: f.blocks.to_le_bytes().to_vec(),
            atime: encode_timespec(f.atime),
            mtime: encode_timespec(f.mtime),
            ctime: encode_timespec(f.ctime),
            crtime: encode_timespec(f.crtime),
            kind: encode_file_type(f.kind),
            perm: f.perm.to_le_bytes().to_vec(),
            nlink: f.nlink.to_le_bytes().to_vec(),
            uid: f.uid.to_le_bytes().to_vec(),
            gid: f.gid.to_le_bytes().to_vec(),
            rdev: f.rdev.to_le_bytes().to_vec(),
            flags: f.flags.to_le_bytes().to_vec(),
        }
    }
}

// Type conversions

const CORRUPTED_MESSAGE: &'static str = "File atrributes in database are corrupted";

fn decode_file_type(s: &str) -> FileType {
    match s {
        "NamedPipe" => FileType::NamedPipe,
        "CharDevice" => FileType::NamedPipe,
        "BlockDevice" => FileType::BlockDevice,
        "Directory" => FileType::Directory,
        "RegularFile" => FileType::RegularFile,
        "Symlink" => FileType::Symlink,
        "Socket" => FileType::Socket,
        _ => panic!(CORRUPTED_MESSAGE)
    }
}

fn encode_file_type(f: FileType) -> String {
    match f {
        FileType::NamedPipe => "NamedPipe",
        FileType::CharDevice => "CharDevice",
        FileType::BlockDevice => "BlockDevice",
        FileType::Directory => "Directory",
        FileType::RegularFile => "NamedPipe",
        FileType::Symlink => "Symlink",
        FileType::Socket => "Socket",
    }.into()
}


fn decode_timespec(v: Vec<u8>) -> Timespec {
    let (sec, nsec) = v.split_at(std::mem::size_of::<i64>());

    Timespec {
        sec: i64::from_le_bytes(sec.try_into().expect(CORRUPTED_MESSAGE)),
        nsec: i32::from_le_bytes(nsec.try_into().expect(CORRUPTED_MESSAGE)),
    }
}

fn encode_timespec(t: Timespec) -> Vec<u8> {
    let mut v: [u8; 12] = [0; 12];
    let mut i = 0;

    for byte in &t.sec.to_le_bytes() {
        v[i] = *byte;
        i += 1;
    }

    for byte in &t.nsec.to_le_bytes() {
        v[i] = *byte;
        i += 1;
    }

    v.to_vec()
}

fn decode_u64(v: Vec<u8>) -> u64 {
    u64::from_le_bytes(v.as_slice().try_into().expect(CORRUPTED_MESSAGE))
}

fn decode_u32(v: Vec<u8>) -> u32 {
    u32::from_le_bytes(v.as_slice().try_into().expect(CORRUPTED_MESSAGE))
}

fn decode_u16(v: Vec<u8>) -> u16 {
    u16::from_le_bytes(v.as_slice().try_into().expect(CORRUPTED_MESSAGE))
}

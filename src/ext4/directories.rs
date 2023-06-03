use super::LoadAble;
use bitflags::bitflags;

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct DirEntry {
    /// Number of the inode that this directory entry points to.
    pub inode: u32,

    /// Length of this directory entry. Must be a multiple of 4.
    pub rec_len: u16,

    /// Length of the file name.
    pub name_len: u16,

    /// File name.
    pub name: [u8; 255],
}
impl LoadAble for DirEntry {}

bitflags! {
    #[derive(Debug)]
    pub struct FileType:u8 {
        const Ext4FtUnknown = 0;
        const Ext4FtRegFile = 1;
        const Ext4FtDir = 2;
        const Ext4FtChrdev = 3;
        const Ext4FtBlkdev = 4;
        const Ext4FtFifo = 5;
        const Ext4FtSock = 6;
        const Ext4FtSymlink = 7;
        const Ext4FtMax = 8;
        const Ext4FtDirCsum = 0xDE;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct DirEntry2 {
    /// Number of the inode that this directory entry points to.
    pub inode: u32,

    /// Length of this directory entry.
    pub rec_len: u16,

    /// Length of the file name.
    pub name_len: u8,

    /// File type code
    pub file_type: FileType,

    /// File name.
    pub name: [u8; 255],
}
impl LoadAble for DirEntry2 {}
impl DirEntry2 {
    pub fn to_char(&self) -> String {
        std::str::from_utf8(&self.name[0..self.name_len as usize])
            .unwrap()
            .to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct DirEntryHash {
    /// The hash of the directory name
    pub hash: u32,

    /// The minor hash of the directory name
    pub minor_hash: u32,
}
impl LoadAble for DirEntryHash {}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct DxEntry {
    pub hash: u32,
    pub block: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct DxRoot {
    // Dot
    pub dotinode: u32,
    pub dotrec_len: u16,
    pub dotname_len: u8,
    pub dotfile_type: FileType,
    pub dotname: [u8; 4],

    // DotDot
    pub dotdotinode: u32,
    pub dotdotrec_len: u16,
    pub dotdotname_len: u8,
    pub dotdotfile_type: FileType,
    pub dotdotname: [u8; 4],

    // dx_root_info
    pub reserved_zero: u32,
    pub hash_version: u8,
    pub info_length: u8,
    pub indirect_levels: u8,
    pub unused_flags: u8,

    // limits
    pub limit: u16,
    pub count: u16,
    pub block: u32,
    // pub entries: [DxEntry],
}
impl LoadAble for DxEntry {}
impl LoadAble for DxRoot {}

use super::loadable::LoadAble;

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Ext4DirEntry {
    /// Number of the inode that this directory entry points to.
    pub inode: u32,

    /// Length of this directory entry. Must be a multiple of 4.
    pub rec_len: u16,

    /// Length of the file name.
    pub name_len: u16,

    /// File name.
    pub name: [char; 255],
}
impl LoadAble for Ext4DirEntry {}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
pub enum FileType {
    Ext4FtUnknown = 0,
    Ext4FtRegFile = 1,
    Ext4FtDir = 2,
    Ext4FtChrdev = 3,
    Ext4FtBlkdev = 4,
    Ext4FtFifo = 5,
    Ext4FtSock = 6,
    Ext4FtSymlink = 7,
    Ext4FtMax = 8,
    Ext4FtDirCsum = 0xDE,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Ext4DirEntry2 {
    /// Number of the inode that this directory entry points to.
    pub inode: u32,

    /// Length of this directory entry.
    pub rec_len: u16,

    /// Length of the file name.
    pub name_len: u8,

    /// File type code
    pub file_type: FileType,

    /// File name.
    pub name: [char; 255],
}
impl LoadAble for Ext4DirEntry2 {}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Ext4DirEntryHash {
    /// The hash of the directory name
    pub hash: u32,

    /// The minor hash of the directory name
    pub minor_hash: u32,
}
impl LoadAble for Ext4DirEntryHash {}

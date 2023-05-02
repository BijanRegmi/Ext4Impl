use super::loadable::LoadAble;
use bitflags::bitflags;

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

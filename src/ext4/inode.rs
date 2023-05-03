use super::LoadAble;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct FileMode: u16 {
        /// Others may execute
        const SIxoth = 0x1;
        /// Others may write
        const SIwoth = 0x2;
        /// Others may read
        const SIroth = 0x4;
        /// Group members may execute
        const SIxgrp = 0x8;
        /// Group members may write
        const SIwgrp = 0x10;
        /// Group members may read
        const SIrgrp = 0x20;
        /// Owner may execute
        const SIxusr = 0x40;
        /// Owner may write
        const SIwusr = 0x80;
        /// Owner may read
        const SIrusr = 0x100;
        /// Sticky bit
        const SIsvtx = 0x200;
        /// Set GID
        const SIsgid = 0x400;
        /// Set UID
        const SIsuid = 0x800;
        /// FIFO
        const SIfifo = 0x1000;
        /// Character device
        const SIfchr = 0x2000;
        /// Directory
        const SIfdir = 0x4000;
        /// Block device
        const SIfblk = 0x6000;
        /// Regular file
        const SIfreg = 0x8000;
        /// Symbolic link
        const SIflnk = 0xA000;
        /// Socket
        const SIfsock = 0xC000;
    }

    #[derive(Debug)]
    pub struct IFlags: u32 {
        /// Secure deletion
        const Ext4SecrmFl = 0x00000001;
        /// Undelete
        const Ext4UnrmFl = 0x00000002;
        /// Compress file
        const Ext4ComprFl = 0x00000004;
        /// Synchronous updates
        const Ext4SyncFl = 0x00000008;
        /// Immutable file
        const Ext4ImmutableFl = 0x00000010;
        /// writes to file may only append
        const Ext4AppendFl = 0x00000020;
        /// do not dump file
        const Ext4NodumpFl = 0x00000040;
        /// do not update atime
        const Ext4NoatimeFl = 0x00000080;
        ///
        const Ext4DirtyFl = 0x00000100;
        /// One or more compressed clusters
        const Ext4ComprblkFl = 0x00000200;
        /// Don't compress
        const Ext4NocomprFl = 0x00000400;
        /// encrypted file
        const Ext4EncryptFl = 0x00000800;
        /// hash-indexed directory
        const Ext4IndexFl = 0x00001000;
        /// AFS directory
        const Ext4ImagicFl = 0x00002000;
        /// file data should be journaled
        const Ext4JournalDataFl = 0x00004000;
        /// file tail should not be merged
        const Ext4NotailFl = 0x00008000;
        /// dirsync behaviour (directories only)
        const Ext4DirsyncFl = 0x00010000;
        /// Top of directory hierarchies
        const Ext4TopdirFl = 0x00020000;
        /// Set to each huge file
        const Ext4HugeFileFl = 0x00040000;
        /// Inode uses extents
        const Ext4ExtentsFl = 0x00080000;
        /// Verity protected inode
        const Ext4VerityFl = 0x00100000;
        /// Inode used for large EA
        const Ext4EaInodeFl = 0x00200000;
        /// Inode is DAX
        const Ext4DaxFl = 0x02000000;
        /// Inode has inline data.
        const Ext4InlineDataFl = 0x10000000;
        /// Create with parents projid
        const Ext4ProjinheritFl = 0x20000000;
        /// Casefolded directory
        const Ext4CasefoldFl = 0x40000000;
        /// reserved for ext4 lib
        const Ext4ReservedFl = 0x80000000;
        /// User modifiable flags
        const Ext4FlUserModifiable = 0x604BC0FF;
        /// User visible flags
        const Ext4FlUserVisible = 0x705BDFFF;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Inode {
    /// File mode.
    pub i_mode: FileMode,

    /// Lower 16-bits of Owner UID.
    pub i_uid: u16,

    /// Lower 32-bits of size in bytes.
    pub i_size_lo: u32,

    /// Last access time, in seconds since the epoch.
    /// However, if the EA_INODE inode flag is set, this inode
    /// stores an extended attribute value and this field contains
    /// the checksum of the value.
    pub i_atime: u32,

    /// Last inode change time, in seconds since the epoch.
    /// However, if the EA_INODE inode flag is set, this inode
    /// stores an extended attribute value and this field contains
    /// the lower 32 bits of the attribute value’s reference count.
    pub i_ctime: u32,

    /// Last data modification time, in seconds since the epoch.
    /// However, if the EA_INODE inode flag is set, this inode
    /// stores an extended attribute value and this field contains
    /// the number of the inode that owns the extended attribute.
    pub i_mtime: u32,

    /// Deletion Time, in seconds since the epoch.
    pub i_dtime: u32,

    /// Lower 16-bits of GID.
    pub i_gid: u16,

    /// Hard link count. Normally, ext4 does not permit an inode
    /// to have more than 65,000 hard links. This applies to files
    /// as well as directories, which means that there cannot be more
    /// than 64,998 subdirectories in a directory (each subdirectory’s ‘..’
    /// entry counts as a hard link, as does the ‘.’ entry in the directory
    /// itself). With the DIR_NLINK feature enabled, ext4 supports more than
    /// 64,998 subdirectories by setting this field to 1 to indicate that
    /// the number of hard links is not known.
    pub i_links_count: u16,

    /// Lower 32-bits of “block” count. If the huge_file feature flag
    /// is not set on the filesystem, the file consumes i_blocks_lo
    /// 512-byte blocks on disk. If huge_file is set and EXT4_HUGE_FILE_FL
    /// is NOT set in inode.i_flags, then the file consumes
    /// i_blocks_lo + (i_blocks_hi << 32) 512-byte blocks on disk. If
    /// huge_file is set and EXT4_HUGE_FILE_FL IS set in inode.i_flags,
    /// then this file consumes (i_blocks_lo + i_blocks_hi << 32) filesystem blocks on disk.
    pub i_blocks_lo: u32,

    /// Inode flags.
    pub i_flags: IFlags,

    // i_osd1 | Linux
    /// Inode version. However, if the EA_INODE inode flag is set, this
    /// inode stores an extended attribute value and this field contains
    /// the upper 32 bits of the attribute value’s reference count.
    pub l_i_version: u32,

    /// Block map or extent tree. See the section “The Contents of inode.i_block”.
    pub i_block: [u16; 15 * 2],

    /// File version (for NFS).
    pub i_generation: u32,

    /// Lower 32-bits of extended attribute block.
    /// ACLs are of course one of many possible extended attributes;
    /// I think the name of this field is a result of
    /// the first use of extended attributes being for ACLs.
    pub i_file_acl_lo: u32,

    /// Upper 32-bits of file/directory size.
    /// In ext2/3 this field was named i_dir_acl,
    /// though it was usually set to zero and never used.
    pub i_size_high: u32,

    /// (Obsolete) fragment address.
    pub i_obso_faddr: u32,

    // i_osd2 | Linux
    /// Upper 16-bits of the block count.
    pub l_i_blocks_high: u16,
    /// Upper 16-bits of the extended attribute block
    /// (historically, the file ACL location).
    pub l_i_file_acl_high: u16,
    /// Upper 16-bits of the Owner UID.
    pub l_i_uid_high: u16,
    /// Upper 16-bits of the GID.
    pub l_i_gid_high: u16,
    /// Lower 16-bits of the inode checksum.
    pub l_i_checksum_lo: u16,
    /// Unused.
    pub l_i_reserved: u16,

    /// Size of this inode - 128.
    /// Alternately, the size of the extended inode fields beyond
    /// the original ext2 inode, including this field.
    pub i_extra_isize: u16,

    /// Upper 16-bits of the inode checksum.
    pub i_checksum_hi: u16,

    /// Extra change time bits. This provides sub-second precision. See Inode Timestamps section.
    pub i_ctime_extra: u32,

    /// Extra modification time bits. This provides sub-second precision.
    pub i_mtime_extra: u32,

    /// Extra access time bits. This provides sub-second precision.
    pub i_atime_extra: u32,

    /// File creation time, in seconds since the epoch.
    pub i_crtime: u32,

    /// Extra file creation time bits. This provides sub-second precision.
    pub i_crtime_extra: u32,

    /// Upper 32-bits for version number.
    pub i_version_hi: u32,

    /// Project ID.
    pub i_projid: u32,
}

impl LoadAble for Inode {}

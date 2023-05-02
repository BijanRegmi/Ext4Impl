use bitflags::bitflags;

use super::loadable::LoadAble;

bitflags! {
    #[derive(Debug)]
    pub struct Ext4Os: u32 {
        const LINUX = 0;
        const HURD = 1;
        const MASIX = 2;
        const FREEBSD = 3;
        const LITES = 4;
    }

    #[derive(Debug)]
    pub struct Ext4CompatibleFeatures: u32{
        const DIR_PREALLOC = 0x0001;
        const IMAGIC_INODES = 0x0002;
        const HAS_JOURNAL = 0x0004;
        const EXT_ATTR = 0x0008;
        const RESIZE_INODE = 0x0010;
        const DIR_INDEX = 0x0020;
        const SPARSE_SUPER2 = 0x0200;
        const FAST_COMMIT = 0x0400;
        const STABLE_INODES = 0x0800;
        const ORPHAN_FILE = 0x1000;
    }

    #[derive(Debug)]
    pub struct Ext4IncompatibleFeatures: u32 {
        const COMPRESSION = 0x0001;
        const FILETYPE = 0x0002;
        const RECOVER = 0x0004;
        const JOURNAL_DEV = 0x0008;
        const META_BG = 0x0010;
        const EXTENTS = 0x0040;
        const _64BIT = 0x0080;
        const MMP = 0x0100;
        const FLEX_BG = 0x0200;
        const EA_INODE = 0x0400;
        const DIRDATA = 0x1000;
        const CSUM_SEED = 0x2000;
        const LARGEDIR = 0x4000;
        const INLINE_DATA = 0x8000;
        const ENCRYPT = 0x10000;
        const CASEFOLD = 0x20000;
    }

    #[derive(Debug)]
    pub struct Ext4ROCompatibleFeatures: u32 {
        const SPARSE_SUPER = 0x0001;
        const LARGE_FILE = 0x0002;
        const BTREE_DIR = 0x0004;
        const HUGE_FILE = 0x0008;
        const GDT_CSUM = 0x0010;
        const DIR_NLINK = 0x0020;
        const EXTRA_ISIZE = 0x0040;
        const QUOTA = 0x0100;
        const BIGALLOC = 0x0200;
        const METADATA_CSUM = 0x0400;
        const READONLY = 0x1000;
        const PROJECT = 0x2000;
        const VERITY = 0x8000;
        const ORPHAN_PRESENT = 0x10000;
    }

    #[derive(Debug)]
    pub struct DxHash: u8 {
        const LEGACY = 0;
        const HALF_MD4 = 1;
        const TEA = 2;
        const LEGACY_UNSIGNED = 3;
        const HALF_MD4_UNSIGNED = 4;
        const TEA_UNSIGNED = 5;
        const SIPHASH = 6;
    }

    /// Default mount options
    #[derive(Debug)]
    pub struct Ext4Defm: u32 {
        const DEBUG = 0x0001;
        const BSDGROUPS = 0x0002;
        const XATTR_USER = 0x0004;
        const ACL = 0x0008;
        const UID16 = 0x0010;
        const JMODE = 0x0060;
        const JMODE_DATA = 0x0020;
        const JMODE_ORDERED = 0x0040;
        const JMODE_WBACK = 0x0060;
        const NOBARRIER = 0x0100;
        const BLOCK_VALIDITY = 0x0200;
        const DISCARD = 0x0400;
        const NODELALLOC = 0x0800;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Ext4SuperBlock {
    /// Total inode count
    pub s_inodes_count: u32,
    /// Total block count.
    pub s_blocks_count_lo: u32,
    /// This number of blocks can only be allocated by the super-user.
    pub s_r_blocks_count_lo: u32,
    /// Free block count.
    pub s_free_blocks_count_lo: u32,
    /// Free inode count.
    pub s_free_inodes_count: u32,
    /// First data block. This must be at least 1 for 1k-block filesystems
    /// and is typically 0 for all other block sizes.
    pub s_first_data_block: u32,
    /// Block size is 2 ^ (10 + s_log_block_size).
    pub s_log_block_size: u32,
    /// Cluster size is 2 ^ (10 + s_log_cluster_size) blocks if bigalloc is enabled.
    /// Otherwise s_log_cluster_size must equal s_log_block_size.
    pub s_log_cluster_size: u32,
    /// Blocks per group.
    pub s_blocks_per_group: u32,
    /// Clusters per group, if bigalloc is enabled.
    /// Otherwise s_clusters_per_group must equal s_blocks_per_group.
    pub s_clusters_per_group: u32,
    /// Inodes per group.
    pub s_inodes_per_group: u32,
    /// Mount time, in seconds since the epoch.
    pub s_mtime: u32,
    /// Write time, in seconds since the epoch.
    pub s_wtime: u32,
    /// Number of mounts since the last fsck.
    pub s_mnt_count: u16,
    /// Number of mounts beyond which a fsck is needed.
    pub s_max_mnt_count: u16,
    /// Magic signature, 0xEF53
    pub s_magic: u16,
    /// File system state.
    /// 0x1 -> Cleanly unmounted
    /// 0x2 -> Errors detected
    /// 0x4 -> Orphans being recovered
    pub s_state: u16,
    /// Behaviour when detecting errors.
    /// 1 -> Continue
    /// 2 -> Remount read only
    /// 3 -> Panic
    pub s_errors: u16,
    /// Minor revision level.
    pub s_minor_rev_level: u16,
    /// Time of last check, in seconds since the epoch.
    pub s_lastcheck: u32,
    /// Maximum time between checks, in seconds.
    pub s_checkinterval: u32,
    /// Creator OS.
    pub s_creator_os: Ext4Os,
    /// Revision level.
    /// 0 -> Original format
    /// 1 -> v2 format w/ dynamic inode sizes
    pub s_rev_level: u32,
    /// Default uid for reserved blocks.
    pub s_def_resuid: u16,
    /// Default gid for reserved blocks.
    pub s_def_resgid: u16,
    /// First non-reserved inode.
    pub s_first_ino: u32,
    /// Size of inode structure, in bytes.
    pub s_inode_size: u16,
    /// Block group # of this superblock.
    pub s_block_group_nr: u16,
    /// Compatible feature set flags.
    /// Kernel can still read/write this fs even if it doesn’t understand a flag;
    /// fsck should not do that.
    pub s_feature_compat: Ext4CompatibleFeatures,
    /// Incompatible feature set.
    /// If the kernel or fsck doesn’t understand one of these bits,
    /// it should stop.
    pub s_feature_incompat: Ext4IncompatibleFeatures,
    /// Readonly-compatible feature set.
    /// If the kernel doesn’t understand one of these bits,
    /// it can still mount read-only.
    pub s_feature_ro_compat: Ext4ROCompatibleFeatures,
    /// 128-bit UUID for volume.
    pub s_uuid: [u8; 16],
    /// Volume label.
    pub s_volume_name: [u8; 16],
    /// Directory where filesystem was last mounted.
    pub s_last_mounted: [u8; 64],
    /// For compression (Not used in e2fsprogs/Linux)
    pub s_algorithm_usage_bitmap: u32,
    /// #. of blocks to try to preallocate for … files? (Not used in e2fsprogs/Linux)
    pub s_prealloc_blocks: u8,
    /// #. of blocks to preallocate for directories. (Not used in e2fsprogs/Linux)
    pub s_prealloc_dir_blocks: u8,
    ///Number of reserved GDT entries for future filesystem expansion.
    pub s_reserved_gdt_blocks: u16,

    // Journalling support is valid only if EXT4_FEATURE_COMPAT_HAS_JOURNAL is set.
    /// UUID of journal superblock
    pub s_journal_uuid: [u8; 16],
    /// inode number of journal file.
    pub s_journal_inum: u32,
    /// Device number of journal file, if the external journal feature flag is set.
    pub s_journal_dev: u32,
    /// Start of list of orphaned inodes to delete.
    pub s_last_orphan: u32,
    /// HTREE hash seed.
    pub s_hash_seed: [u32; 4],
    /// Default hash algorithm to use for directory hashes.
    pub s_def_hash_version: DxHash,
    /// If this value is 0 or EXT3_JNL_BACKUP_BLOCKS (1), then the s_jnl_blocks
    /// field contains a duplicate copy of the inode’s i_block[] array and i_size.
    pub s_jnl_backup_type: u8,
    /// Size of group descriptors, in bytes, if the 64bit incompat feature flag is set.
    pub s_desc_size: u16,
    /// Default mount options.
    pub s_default_mount_opts: Ext4Defm,
    /// First metablock block group, if the meta_bg feature is enabled.
    pub s_first_meta_bg: u32,
    /// When the filesystem was created, in seconds since the epoch.
    pub s_mkfs_time: u32,
    /// Backup copy of the journal inode’s i_block[] array in the first 15 elements
    /// and i_size_high and i_size in the 16th and 17th elements, respectively.
    pub s_jnl_blocks: [u32; 17],

    // 64bit support is valid only if EXT4_FEATURE_COMPAT_64BIT is set.
    /// High 32-bits of the block count.
    pub s_blocks_count_hi: u32,
    /// High 32-bits of the reserved block count.
    pub s_r_blocks_count_hi: u32,
    /// High 32-bits of the free block count.
    pub s_free_blocks_count_hi: u32,
    /// All inodes have at least # bytes.
    pub s_min_extra_isize: u16,
    /// New inodes should reserve # bytes.
    pub s_want_extra_isize: u16,
    /// Miscellaneous flags. 
    pub s_flags: u32,
    /// RAID stride. This is the number of logical blocks read from or written to
    /// the disk before moving to the next disk. This affects the placement of
    /// filesystem metadata, which will hopefully make RAID storage faster.
    pub s_raid_stride: u16,
    /// #. seconds to wait in multi-mount prevention (MMP) checking. In theory,
    /// MMP is a mechanism to record in the superblock which host and device have mounted
    /// the filesystem, in order to prevent multiple mounts. This feature does not seem to be implemented…
    pub s_mmp_update_interval: u16,
    /// Block # for multi-mount protection data.
    pub s_mmp_block: u64,
    /// RAID stripe width. This is the number of logical blocks read from or written
    /// to the disk before coming back to the current disk. This is used by the block
    /// allocator to try to reduce the number of read-modify-write operations in a RAID5/6.
    pub s_raid_stripe_width: u32,
    /// Size of a flexible block group is 2 ^ s_log_groups_per_flex.
    pub s_log_groups_per_flex: u8,
    /// Metadata checksum algorithm type. The only valid value is 1 (crc32c).
    pub s_checksum_type: u8,
    /// Versioning level for encryption
    pub s_encryption_level: u8,
    /// Padding to next 32bits
    pub s_reserved_pad: u8,
    /// Number of KiB written to this filesystem over its lifetime.
    pub s_kbytes_written: u64,
    /// inode number of active snapshot.
    /// (Not used in e2fsprogs/Linux.)
    pub s_snapshot_inum: u32,
    /// Sequential ID of active snapshot.
    /// (Not used in e2fsprogs/Linux.)
    pub s_snapshot_id: u32,
    /// Number of blocks reserved for active snapshot’s future use.
    /// (Not used in e2fsprogs/Linux.)
    pub s_snapshot_r_blocks_count: u64,
    /// inode number of the head of the on-disk snapshot list.
    /// (Not used in e2fsprogs/Linux.)
    pub s_snapshot_list: u32,
    /// Number of errors seen.
    pub s_error_count: u32,
    /// First time an error happened, in seconds since the epoch.
    pub s_first_error_time: u32,
    /// inode involved in first error.
    pub s_first_error_ino: u32,
    /// Number of block involved of first error.
    pub s_first_error_block: u64,
    /// Name of function where the error happened.
    pub s_first_error_func: [u8; 32],
    /// Line number where error happened.
    pub s_first_error_line: u32,
    /// Time of most recent error, in seconds since the epoch.
    pub s_last_error_time: u32,
    /// inode involved in most recent error.
    pub s_last_error_ino: u32,
    /// Line number where most recent error happened.
    pub s_last_error_line: u32,
    /// Number of block involved in most recent error.
    pub s_last_error_block: u64,
    /// Name of function where the most recent error happened.
    pub s_last_error_func: [u8; 32],
    /// ASCIIZ string of mount options.
    pub s_mount_opts: [u8; 64],
    /// Inode number of user quota file.
    pub s_usr_quota_inum: u32,
    /// Inode number of group quota file.
    pub s_grp_quota_inum: u32,
    /// Overhead blocks/clusters in fs.
    /// (Huh? This field is always zero, which means that the kernel calculates it dynamically.)
    pub s_overhead_clusters: u32,
    /// Block groups containing superblock backups (if sparse_super2)
    pub s_backup_bgs: [u32; 2],
    /// Encryption algorithms in use. There can be up to four algorithms in use at any time;
    /// valid algorithm codes are given in the super_encrypt table below.
    pub s_encrypt_algos: [u8; 4],
    /// Salt for the string2key algorithm for encryption.
    pub s_encrypt_pw_salt: [u8; 16],
    /// Inode number of lost+found
    pub s_lpf_ino: u32,
    /// Inode that tracks project quotas.
    pub s_prj_quota_inum: u32,
    /// Checksum seed used for metadata_csum calculations.
    /// This value is crc32c(~0, $orig_fs_uuid).
    pub s_checksum_seed: u32,
    /// Upper 8 bits of the s_wtime field.
    pub s_wtime_hi: u8,
    /// Upper 8 bits of the s_mtime field.
    pub s_mtime_hi: u8,
    /// Upper 8 bits of the s_mkfs_time field.
    pub s_mkfs_time_hi: u8,
    /// Upper 8 bits of the s_lastcheck field.
    pub s_lastcheck_hi: u8,
    /// Upper 8 bits of the s_first_error_time field.
    pub s_first_error_time_hi: u8,
    /// Upper 8 bits of the s_last_error_time field.
    pub s_last_error_time_hi: u8,
    /// Error code of first error
    pub s_first_error_errcode: u8,
    /// Error code of last error
    pub s_last_error_errcode: u8,
    /// Filename charset encoding.
    pub s_encoding: u16,
    /// Filename charset encoding flags.
    pub s_encoding_flags: u16,
    /// Orphan file inode number.
    pub s_orphan_file_inum: u32,
    /// Padding to the end of the block.
    pub s_reserved: [u32; 94],
    /// Superblock checksum.   
    pub s_checksum: u32,
}
impl LoadAble for Ext4SuperBlock {}

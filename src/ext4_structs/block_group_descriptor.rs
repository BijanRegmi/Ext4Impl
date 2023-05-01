use super::loadable::LoadAble;

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Ext4GroupDesc {
    /// Lower 32-bits of location of block bitmap.
    pub bg_block_bitmap_lo: u32,

    /// Lower 32-bits of location of inode bitmap.
    pub bg_inode_bitmap_lo: u32,

    /// Lower 32-bits of location of inode table.
    pub bg_inode_table_lo: u32,

    /// Lower 16-bits of free block count.
    pub bg_free_blocks_count_lo: u16,

    /// Lower 16-bits of free inode count.
    pub bg_free_inodes_count_lo: u16,

    /// Lower 16-bits of directory count.
    pub bg_used_dirs_count_lo: u16,

    /// Lower 32-bits of location of snapshot exclusion bitmap.
    pub bg_exclude_bitmap_lo: u32,

    /// Lower 16-bits of the block bitmap checksum.
    pub bg_block_bitmap_csum_lo: u16,

    /// Lower 16-bits of the inode bitmap checksum.
    pub bg_inode_bitmap_csum_lo: u16,

    /// Lower 16-bits of unused inode count. If set, we needn’t scan past the (sb.s_inodes_per_group - gdt.bg_itable_unused) th entry in the inode table for this group.
    pub bg_itable_unused_lo: u16,

    /// Group descriptor checksum; crc16(sb_uuid+group_num+bg_desc) if the RO_COMPAT_GDT_CSUM feature is set, or crc32c(sb_uuid+group_num+bg_desc) & 0xFFFF if the RO_COMPAT_METADATA_CSUM feature is set. The bg_checksum field in bg_desc is skipped when calculating crc16 checksum, and set to zero if crc32c checksum is used.
    pub bg_checksum: u16,

    /// Upper 32-bits of location of block bitmap.
    pub bg_block_bitmap_hi: u32,

    /// Upper 32-bits of location of inodes bitmap.
    pub bg_inode_bitmap_hi: u32,

    /// Upper 32-bits of location of inodes table.
    pub bg_inode_table_hi: u32,

    /// Upper 16-bits of free block count.
    pub bg_free_blocks_count_hi: u16,

    /// Upper 16-bits of free inode count.
    pub bg_free_inodes_count_hi: u16,

    /// Upper 16-bits of directory count.
    pub bg_used_dirs_count_hi: u16,

    /// Upper 16-bits of unused inode count.
    pub bg_itable_unused_hi: u16,

    /// Upper 32-bits of location of snapshot exclusion bitmap.
    pub bg_exclude_bitmap_hi: u32,

    /// Upper 16-bits of the block bitmap checksum.
    pub bg_block_bitmap_csum_hi: u16,

    /// Upper 16-bits of the inode bitmap checksum.
    pub bg_inode_bitmap_csum_hi: u16,

    /// Padding to 64 bytes.
    pub bg_reserved: u32,

    /// Block group flags.
    /// Value   Description
    /// 0x1     inode table and bitmap are not initialized (EXT4_BG_INODE_UNINIT).
    /// 0x2     block bitmap is not initialized (EXT4_BG_BLOCK_UNINIT).
    /// 0x3     inode table is zeroed (EXT4_BG_INODE_ZEROED).
    pub bg_flags: u16,
}

impl LoadAble for Ext4GroupDesc {}

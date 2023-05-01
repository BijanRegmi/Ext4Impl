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

    pub bg_flags: u16,
    pub bg_exclude_bitmap_lo: u32,
    pub bg_block_bitmap_csum_lo: u16,
    pub bg_inode_bitmap_csum_lo: u16,
    pub bg_itable_unused_lo: u16,
    pub bg_checksum: u16,
    pub bg_block_bitmap_hi: u32,
    pub bg_inode_bitmap_hi: u32,
    pub bg_inode_table_hi: u32,
    pub bg_free_blocks_count_hi: u16,
    pub bg_free_inodes_count_hi: u16,
    pub bg_used_dirs_count_hi: u16,
    pub bg_itable_unused_hi: u16,
    pub bg_exclude_bitmap_hi: u32,
    pub bg_block_bitmap_csum_hi: u16,
    pub bg_inode_bitmap_csum_hi: u16,
    pub bg_reserved: u32,
}

impl LoadAble for Ext4GroupDesc {}

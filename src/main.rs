mod ext4_structs;
use dbg_hex::dbg_hex;
use ext4_structs::LoadAble;

use std::{fs::File, mem::size_of};

fn main() -> std::io::Result<()> {
    let mut file = File::open("../iso/sda1.img")?;
    let sb = ext4_structs::Ext4SuperBlock::from_file_offset(&mut file, 0x400)?;
    let gd = ext4_structs::Ext4GroupDesc::from_file_offset(&mut file, 0x400 + 1024)?;

    let block_size: u64 = (2 as u64).pow(10 + sb.s_log_block_size) as u64;

    let inode_table_address: u64 =
        ((gd.bg_inode_table_hi as u64) << 8) + (gd.bg_inode_table_lo as u64) * block_size;

    let inode_num = 2;
    let inode_index_in_table = (inode_num - 1) % sb.s_inodes_per_group;
    let inode_offset_in_table = inode_index_in_table * sb.s_inode_size as u32;
    let inode_address = inode_table_address + (inode_offset_in_table as u64);
    let i = ext4_structs::Ext4Inode::from_file_offset(&mut file, inode_address)?;
    dbg!(sb.s_blocks_per_group, 8 * block_size);
    dbg_hex!(
        block_size,
        inode_table_address,
        inode_num,
        inode_index_in_table,
        inode_offset_in_table,
        inode_address,
        i
    );
    Ok(())
}

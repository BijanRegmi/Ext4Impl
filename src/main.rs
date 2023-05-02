mod disk;
mod ext4_structs;
use dbg_hex::dbg_hex;

fn main() -> std::io::Result<()> {
    let _sda1 = "../iso/sda1.img";
    let _nvme = "/dev/nvme0n1p3";
    let mut d = disk::Disk::new(_nvme);

    let i2 = d.get_inode(26);
    dbg!(i2);

    // let inode_table_address: u64 =
    //     ((gd.bg_inode_table_hi as u64) << 8) + (gd.bg_inode_table_lo as u64) * block_size;
    //     left shift hi by 32 bits then or with lo
    //
    // let inode_num = 2;
    // let inode_index_in_table = (inode_num - 1) % sb.s_inodes_per_group;
    // let inode_offset_in_table = inode_index_in_table * sb.s_inode_size as u32;
    // let inode_address = inode_table_address + (inode_offset_in_table as u64);
    // let i = ext4_structs::Ext4Inode::from_file_offset(&mut file, inode_address)?;
    Ok(())
}

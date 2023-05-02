use std::io::{Read, Seek};

use crate::ext4_structs::{self, Ext4GroupDesc, Ext4Inode, LoadAble};

pub struct Disk {
    file: std::fs::File,
    pub super_block: ext4_structs::Ext4SuperBlock,
    pub block_size: u64,
    pub groups_per_flex: u32,
}

impl Disk {
    pub fn new(path: &str) -> Self {
        let mut f = std::fs::File::open(path).expect("Failed to open file.");

        let sb = ext4_structs::Ext4SuperBlock::from_file_offset(&mut f, 0x400)
            .expect("Failed to read superblock");
        let bs: u64 = (2 as u64).pow(10 + sb.s_log_block_size);
        let gpf: u32 = (1 as u32) << sb.s_log_groups_per_flex;

        Disk {
            file: f,
            super_block: sb,
            block_size: bs,
            groups_per_flex: gpf,
        }
    }

    pub fn read_block(&mut self, block_num: u64) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; self.block_size as usize];
        self.file
            .seek(std::io::SeekFrom::Start(block_num * self.block_size))?;
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn get_group_desc(&mut self, group_num: u32) -> Ext4GroupDesc {
        let primary_group_in_flex = group_num - (group_num % self.groups_per_flex);
        let block_no = primary_group_in_flex * self.super_block.s_blocks_per_group
            + self.super_block.s_first_data_block
            + 1;
        ext4_structs::Ext4GroupDesc::from_file_offset(
            &mut self.file,
            (block_no as u64) * (self.block_size as u64),
        )
        .expect("Cannot read group desc")
    }

    pub fn block_group_has_superblock(&self, bg_num: u32) -> bool {
        if bg_num == 0 {
            return true;
        } else if self
            .super_block
            .s_feature_compat
            .contains(ext4_structs::Ext4CompatibleFeatures::SPARSE_SUPER2)
        {
            unimplemented!("Check s_backup_bgs")
        } else if bg_num <= 1 || !self.super_block.has_sparse_super_feature() {
            true
        } else if bg_num & 0x1 == 0 {
            false
        } else {
            fn test_root(mut a: u32, b: u32) -> bool {
                loop {
                    if a < b {
                        return false;
                    } else if a == b {
                        return true;
                    } else if (a % b) != 0 {
                        return false;
                    }
                    a = a / b;
                }
            }

            test_root(bg_num, 3) || test_root(bg_num, 5) || test_root(bg_num, 7)
        }
    }

    pub fn get_inode(&mut self, inode_num: u32) -> Ext4Inode {
        // Block group that an inode lives in
        let bg = (inode_num - 1) / self.super_block.s_inodes_per_group;
        // Get group desc of bg block number
        let gdesc = self.get_group_desc(bg);
        let offset_within_flex = bg % self.groups_per_flex;
        let inode_table_address = (gdesc.bg_inode_table_hi as u64) << 32
            | (gdesc.bg_inode_table_lo as u64) + offset_within_flex as u64;
        let inode_index_in_table = (inode_num - 1) % self.super_block.s_inodes_per_group;
        let inode_offset_in_table = inode_index_in_table * self.super_block.s_inode_size as u32;
        let inode_address = inode_table_address * self.block_size + inode_offset_in_table as u64;
        ext4_structs::Ext4Inode::from_file_offset(&mut self.file, inode_address)
            .expect("Failed to get inode")
    }
}

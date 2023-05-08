use std::io::{Read, Seek};
use std::mem::transmute;

use dbg_hex::dbg_hex;

use crate::ext4;
use crate::ext4::LoadAble;

pub struct Disk {
    file: std::fs::File,
    pub super_block: ext4::structs::SuperBlock,
    pub block_size: u32,
    pub groups_per_flex: u16,
}

impl Disk {
    pub fn new(path: &str) -> Self {
        let mut f = std::fs::File::open(path).expect("Failed to open file.");

        let sb = ext4::structs::SuperBlock::from_file_offset(&mut f, 0x400)
            .expect("Failed to read superblock");
        let bs: u32 = (2 as u32).pow(10 + sb.s_log_block_size);
        let gpf: u16 = (1 as u16) << sb.s_log_groups_per_flex;

        dbg!(&sb.s_log_groups_per_flex, bs, gpf);

        Disk {
            file: f,
            super_block: sb,
            block_size: bs,
            groups_per_flex: gpf,
        }
    }

    #[allow(dead_code)]
    pub fn read_block(&mut self, block_num: u64) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; self.block_size as usize];
        self.file
            .seek(std::io::SeekFrom::Start(block_num * self.block_size as u64))?;
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn get_itable_blk_num(&mut self, group_num: u32) -> u64 {
        let offset_in_flex = group_num as u32 % self.groups_per_flex as u32;
        let has_redundant_copy = self.block_group_has_redundant_copy(group_num);
        let blk_no: u64;
        if has_redundant_copy {
            let gd_blk_no = group_num as u64 * self.super_block.s_blocks_per_group as u64
                + 1 // skip superblock
                + self.super_block.s_first_data_block as u64;
            let blk = self.read_block(gd_blk_no).unwrap();
            let gd = ext4::structs::GroupDesc::from_buffer(&blk, 0);
            blk_no = ((gd.bg_inode_table_hi as u64) << 32) | gd.bg_inode_table_lo as u64;
        } else {
            let is_primary_bg_in_flex = offset_in_flex == 0;
            if is_primary_bg_in_flex {
                blk_no = (1 + 1) * self.groups_per_flex as u64
                    + self.super_block.s_first_data_block as u64;
            } else {
                let primary_flex_grp = group_num - offset_in_flex;
                if self.block_group_has_redundant_copy(primary_flex_grp) {
                    let gd_blk_no = primary_flex_grp as u64 * self.super_block.s_blocks_per_group as u64
                        + 1 // skip superblock
                        + self.super_block.s_first_data_block as u64;
                    let blk = self.read_block(gd_blk_no).unwrap();
                    let gd = ext4::structs::GroupDesc::from_buffer(&blk, 0);
                    blk_no = ((gd.bg_inode_table_hi as u64) << 32) | gd.bg_inode_table_lo as u64;
                } else {
                    blk_no = (1 + 1) * self.groups_per_flex as u64
                        + self.super_block.s_first_data_block as u64;
                }
            }
        }
        group_num as u64 * self.super_block.s_blocks_per_group as u64
            + blk_no
            + offset_in_flex as u64 * self.super_block.s_inodes_per_group as u64
    }

    #[allow(dead_code)]
    pub fn block_group_has_redundant_copy(&self, bg_num: u32) -> bool {
        if bg_num == 0 {
            true
        } else if self
            .super_block
            .s_feature_compat
            .contains(ext4::flags::superblock::CompatibleFeatures::SPARSE_SUPER2)
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

    fn get_inode(&mut self, inode_num: u32) -> ext4::structs::Inode {
        let inode_group_num = (inode_num - 1) / self.super_block.s_inodes_per_group;
        let inode_table_blk_num = self.get_itable_blk_num(inode_group_num);

        let inode_index_in_table = (inode_num - 1) % self.super_block.s_inodes_per_group;
        // This is not in blocks as s_inode_size is in bytes
        let inode_offset_in_table = inode_index_in_table * self.super_block.s_inode_size as u32;

        let inode_address =
            inode_table_blk_num * self.block_size as u64 + inode_offset_in_table as u64;

        ext4::structs::Inode::from_file_offset(&mut self.file, inode_address)
            .expect("Failed to get inode")
    }

    pub fn read_dir(&mut self, inode_num: u32) {
        let inode = self.get_inode(inode_num);
        dbg!(&inode);
        if inode
            .i_flags
            .contains(ext4::flags::inode::IFlags::Ext4IndexFl)
        {
            unimplemented!("Hashed Tree Directory");
        }

        let extents = self.get_extents(&inode);
        if extents.len() > 1 {
            unimplemented!();
        }

        let e = &extents[0];
        let blk_no: u64 = ((e.ee_start_hi as u64) << 32) | e.ee_start_lo as u64;
        let block = self.read_block(blk_no).unwrap();

        let mut offset = 0;
        let mut entries = Vec::<ext4::structs::dir::Entry2>::new();

        while offset < block.len() {
            let de = ext4::structs::dir::Entry2::from_buffer(&block, offset);
            if de.inode == 0 {
                break;
            }
            offset = offset + de.rec_len as usize;
            entries.push(de);
        }

        println!("Reading dir with inode_num {inode_num}");
        for ele in entries {
            let s = std::str::from_utf8(&ele.name[0..ele.name_len as usize]).unwrap();
            println!("{:>10}: {}", ele.inode, s);
        }
    }

    fn get_extents(&mut self, inode: &ext4::structs::Inode) -> Vec<ext4::structs::extent::Extent> {
        let buf = inode.i_block.to_vec();
        let eh = ext4::structs::extent::Header::from_buffer(&buf, 0);
        assert_eq!(eh.eh_magic, 0xf30a);

        let mut extents = Vec::<ext4::structs::extent::Extent>::new();
        if eh.eh_depth == 0 {
            for i in 0..eh.eh_entries as usize {
                extents.push(ext4::structs::extent::Extent::from_buffer(
                    &buf,
                    (i + 1) * 12,
                ));
            }
        } else {
            unimplemented!("Internal node encountered")
        }
        extents
    }
}

mod directories;
mod extents;
mod group_desc;
mod inode;
mod loadable;
mod superblock;

pub mod structs {
    pub use crate::ext4::group_desc::GroupDesc;
    pub use crate::ext4::inode::Inode;
    pub use crate::ext4::superblock::SuperBlock;
    pub mod dir {
        pub use crate::ext4::directories::{
            DirEntry as Entry, DirEntry2 as Entry2, DirEntryHash as EntryHash, DxEntry, DxRoot,
        };
    }
    pub mod extent {
        pub use crate::ext4::extents::{
            Extent, ExtentHeader as Header, ExtentIdx as Idx, ExtentTail as Tail,
        };
    }
}

pub mod flags {
    pub mod superblock {
        pub use crate::ext4::superblock::{
            CompatibleFeatures, Ext4Defm, IncompatibleFeatures, ROCompatibleFeatures, OS,
        };
    }
    pub mod group_desc {
        pub use crate::ext4::group_desc::GroupFlags;
    }
    pub mod inode {
        pub use crate::ext4::inode::{FileMode, IFlags};
    }
    pub mod dir {
        pub use crate::ext4::directories::FileType;
    }
}

pub use self::loadable::LoadAble;

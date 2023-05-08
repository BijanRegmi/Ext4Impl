use dbg_hex::dbg_hex;

mod disk;
mod ext4;

use ext4::LoadAble;

fn main() -> std::io::Result<()> {
    let _sda1 = "../iso/sda1.img";
    let _nvme = "/dev/nvme0n1p3";
    let _pen = "../iso/pen.img";

    let mut d = disk::Disk::new(_nvme);

    let inode = match std::env::args().nth(1) {
        Some(x) => x.parse().unwrap(),
        None => 2,
    };

    d.read_dir(inode);

    Ok(())
}

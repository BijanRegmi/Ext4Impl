mod disk;
mod ext4;

fn main() -> std::io::Result<()> {
    let _sda1 = "../iso/sda1.img";
    let _nvme = "/dev/nvme0n1p3";
    let mut d = disk::Disk::new(_sda1);

    let i2 = d.get_inode(2);
    dbg!(i2);
    Ok(())
}

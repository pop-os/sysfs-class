extern crate sysfs_class;
use sysfs_class::{Block, SysClass};
use std::io;

fn main() -> io::Result<()> {
    for block in Block::all()? {
        if block.has_device() {
            println!("Path: {:?}", block.path());
            println!("  Model: {:?}", block.device_model());
            println!("  Vendor: {:?}", block.device_vendor());
            println!("  Rev: {:?}", block.device_rev());
            println!("  Children: {:#?}", block.children());
        }
    }

    Ok(())
}
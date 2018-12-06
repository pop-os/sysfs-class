extern crate sysfs_class;
use sysfs_class::{PciDevice, PciDriver, SysClass};
use std::io;

fn main() -> io::Result<()> {
    for dev in PciDevice::all()? {
        println!("PCI Device: {}", dev.id());
    }

    for dev in PciDriver::iter() {
        let dev = dev?;
        println!("PCI Driver: {}", dev.id());
    }

    Ok(())
}

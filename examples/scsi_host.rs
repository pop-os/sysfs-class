extern crate sysfs_class;
use sysfs_class::{ScsiHost, SysClass};
use std::io;

fn main() -> io::Result<()> {
    for host in ScsiHost::all()? {
        println!(
            "{} has power management policy \"{:?}\"",
            host.id(),
            host.link_power_management_policy()
        );
    }

    Ok(())
}

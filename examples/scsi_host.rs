use std::io;
use sysfs_class::{ScsiHost, SysClass};

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

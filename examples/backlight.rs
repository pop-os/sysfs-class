use std::io;
use sysfs_class::{Backlight, Brightness, SysClass};

fn main() -> io::Result<()> {
    for dev in Backlight::all()? {
        println!(
            "{} brightness: {} / {}",
            dev.id(),
            dev.brightness().unwrap(),
            dev.max_brightness().unwrap()
        );
    }

    Ok(())
}

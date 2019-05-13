extern crate sysfs_class;
use sysfs_class::{Brightness, Leds, SysClass};
use std::io;

fn main() -> io::Result<()> {
    for dev in Leds::all()? {
        println!(
            "{} brightness: {} / {}",
            dev.id(),
            dev.brightness().unwrap(),
            dev.max_brightness().unwrap()
        );
    }

    Ok(())
}

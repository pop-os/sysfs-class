use std::io;
use sysfs_class::{Net, SysClass};

fn main() -> io::Result<()> {
    for dev in Net::iter() {
        let dev = dev?;

        println!("{}: {}", dev.id(), dev.address().unwrap());
        println!("    MTU: {}", dev.mtu().unwrap());
        println!("    Duplex: {:?}", dev.duplex());

        let statistics = dev.statistics();
        println!(
            "    RX: {} MiB",
            statistics.rx_bytes().unwrap() / (1024 * 1024)
        );
        println!(
            "    TX: {} MiB",
            statistics.tx_bytes().unwrap() / (1024 * 1024)
        );
    }

    Ok(())
}

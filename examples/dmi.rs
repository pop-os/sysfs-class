extern crate sysfs_class;
use sysfs_class::DmiId;

fn main() {
    let dmi = DmiId::default();
    println!(
        "BIOS Date: {:?}\n\
        BIOS Vendor: {:?}\n\
        BIOS Version: {:?}\n\
        Board Asset Tag: {:?}\n\
        Board Name: {:?}\n\
        Board Serial: {:?}\n\
        Board Vendor: {:?}\n\
        Board Version: {:?}\n\
        Chassis Asset Tag: {:?}\n\
        Chassis Name: {:?}\n\
        Chassis Serial: {:?}\n\
        Chassis Vendor: {:?}\n\
        Chassis Version: {:?}\n\
        Product Name: {:?}\n\
        Product Serial: {:?}\n\
        Product SKU: {:?}\n\
        Product UUID: {:?}\n\
        Product Version: {:?}\n\
        Sys Vendor: {:?}",
        dmi.bios_date(),
        dmi.bios_vendor(),
        dmi.bios_version(),
        dmi.board_asset_tag(),
        dmi.board_name(),
        dmi.board_serial(),
        dmi.board_vendor(),
        dmi.board_version(),
        dmi.chassis_asset_tag(),
        dmi.chassis_name(),
        dmi.chassis_serial(),
        dmi.chassis_vendor(),
        dmi.chassis_version(),
        dmi.product_name(),
        dmi.product_serial(),
        dmi.product_sku(),
        dmi.product_uuid(),
        dmi.product_version(),
        dmi.sys_vendor()
    );
}
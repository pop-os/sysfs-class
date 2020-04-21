use crate::SysClass;
use std::io::Result;
use std::path::{Path, PathBuf};

const BASE_PATH: &str = "/sys/class/dmi/id";

/// Provides BIOS, Board, Chassis, Product, & Vendor identifiers
#[derive(Clone)]
pub struct DmiId {
    path: &'static Path,
}

impl Default for DmiId {
    fn default() -> Self {
        Self {
            path: Path::new(BASE_PATH),
        }
    }
}

impl SysClass for DmiId {
    fn class() -> &'static str {
        "dmi/id"
    }

    unsafe fn from_path_unchecked(_path: PathBuf) -> Self {
        Self::default()
    }

    fn path(&self) -> &Path {
        self.path
    }
}

impl DmiId {
    method!(bios_date read_file String);

    method!(bios_vendor read_file String);

    method!(bios_version read_file String);

    method!(board_asset_tag read_file String);

    method!(board_name read_file String);

    method!(board_serial read_file String);

    method!(board_vendor read_file String);

    method!(board_version read_file String);

    method!(chassis_asset_tag read_file String);

    method!(chassis_name read_file String);

    method!(chassis_serial read_file String);

    method!(chassis_vendor read_file String);

    method!(chassis_version read_file String);

    method!(modalias read_file String);

    method!(product_family read_file String);

    method!(product_name read_file String);

    method!(product_serial read_file String);

    method!(product_sku read_file String);

    method!(product_uuid read_file String);

    method!(product_version read_file String);

    method!(sys_vendor read_file String);
}

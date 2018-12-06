use std::fs;
use std::io::{self, Result};
use std::path::{Path, PathBuf};
use SysClass;
use RuntimePM;
use RuntimePowerManagement;

#[derive(Clone)]
pub struct PciDriver {
    path: PathBuf
}

impl SysClass for PciDriver {
    fn base() -> &'static str {
        "bus"
    }

    fn class() -> &'static str {
        "pci/drivers"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl PciDriver {
    pub unsafe fn bind(&self, device: &PciDevice) -> io::Result<()> {
        self.write_file("bind", device.id())
    }

    pub unsafe fn unbind(&self, device: &PciDevice) -> io::Result<()> {
        self.write_file("unbind", device.id())
    }
}


#[derive(Clone)]
pub struct PciDevice {
    path: PathBuf
}

impl SysClass for PciDevice {
    fn base() -> &'static str {
        "bus"
    }

    fn class() -> &'static str {
        "pci/devices"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl PciDevice {
    method!(class parse_file u32);
    method!(device parse_file u16);
    method!(revision parse_file u8);
    method!(vendor parse_file u16);

    pub fn driver(&self) -> io::Result<PciDriver> {
        fs::canonicalize(self.path.join("driver"))
            .map(|path| PciDriver { path })
    }

    pub unsafe fn remove(&self) -> io::Result<()> {
        self.write_file("remove", "1")
    }
}

impl RuntimePM for PciDevice {
    fn set_runtime_pm(&self, state: RuntimePowerManagement) -> io::Result<()> {
        self.write_file("power/control", <&'static str>::from(state))
    }
}

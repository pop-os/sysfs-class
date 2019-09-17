use std::fs;
use std::io;
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

macro_rules! pci_devices {
    ($( fn $file:tt -> $out:tt; )*) => {
        $(
            pub fn $file(&self) -> io::Result<$out> {
                let v = self.read_file(stringify!($file))?;
                $out::from_str_radix(v[2..].trim(), 16).map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{}", err)
                    )
                })
            }
        )*
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
    pci_devices! {
        fn class -> u32;
        fn device -> u16;
        fn revision -> u8;
        fn subsystem_device -> u16;
        fn subsystem_vendor -> u16;
        fn vendor -> u16;
    }

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

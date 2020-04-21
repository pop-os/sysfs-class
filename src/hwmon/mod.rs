use crate::SysClass;
use std::io::Result;
use std::path::{Path, PathBuf};

pub use self::fan::HwMonFan;
mod fan;

pub use self::pwm::HwMonPwm;
mod pwm;

pub use self::temp::HwMonTemp;
mod temp;

/// A hardware monitoring device in /sys/class/hwmon
#[derive(Clone)]
pub struct HwMon {
    path: PathBuf,
}

impl SysClass for HwMon {
    fn class() -> &'static str {
        "hwmon"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl HwMon {
    pub fn name(&self) -> Result<String> {
        self.trim_file("name")
    }

    pub fn fan<'a>(&'a self, id: u64) -> Result<HwMonFan<'a>> {
        HwMonFan::new(self, id)
    }

    pub fn pwm<'a>(&'a self, id: u64) -> Result<HwMonPwm<'a>> {
        HwMonPwm::new(self, id)
    }

    pub fn temp<'a>(&'a self, id: u64) -> Result<HwMonTemp<'a>> {
        HwMonTemp::new(self, id)
    }
}

use std::io::Result;

use {HwMon, SysClass};
use util::{parse_file, trim_file};

pub struct HwMonFan<'a> {
    hwmon: &'a HwMon,
    id: u64,
}

impl<'a> HwMonFan<'a> {
    pub fn new(hwmon: &'a HwMon, id: u64) -> Result<Self> {
        let s = Self {
            hwmon,
            id
        };

        s.input()?;

        Ok(s)
    }

    pub fn label(&self) -> Result<String> {
        trim_file(self.hwmon.path().join(&format!("fan{}_label", self.id)))
    }

    pub fn input(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("fan{}_input", self.id)))
    }

    pub fn min(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("fan{}_min", self.id)))
    }

    pub fn max(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("fan{}_max", self.id)))
    }

    pub fn target(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("fan{}_target", self.id)))
    }

    pub fn div(&self) -> Result<u8> {
        parse_file(self.hwmon.path().join(&format!("fan{}_div", self.id)))
    }

    pub fn pulses(&self) -> Result<u8> {
        parse_file(self.hwmon.path().join(&format!("fan{}_pulses", self.id)))
    }
}

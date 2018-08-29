use std::io::Result;

use {HwMon, SysClass};
use util::{parse_file, trim_file};

pub struct HwMonTemp<'a> {
    hwmon: &'a HwMon,
    id: u64,
}

impl<'a> HwMonTemp<'a> {
    pub fn new(hwmon: &'a HwMon, id: u64) -> Result<Self> {
        let s = Self {
            hwmon,
            id
        };

        s.input()?;

        Ok(s)
    }

    pub fn label(&self) -> Result<String> {
        trim_file(self.hwmon.path().join(&format!("temp{}_label", self.id)))
    }

    pub fn input(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("temp{}_input", self.id)))
    }

    pub fn lcrit(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("temp{}_lcrit", self.id)))
    }

    pub fn min(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("temp{}_min", self.id)))
    }

    pub fn max(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("temp{}_max", self.id)))
    }

    pub fn crit(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("temp{}_crit", self.id)))
    }

    pub fn emergency(&self) -> Result<u32> {
        parse_file(self.hwmon.path().join(&format!("temp{}_emergency", self.id)))
    }
}

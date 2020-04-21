use crate::{HwMon, SysClass};
use std::io::Result;

pub struct HwMonTemp<'a> {
    hwmon: &'a HwMon,
    id: u64,
}

impl<'a> HwMonTemp<'a> {
    pub fn new(hwmon: &'a HwMon, id: u64) -> Result<Self> {
        let s = Self { hwmon, id };

        s.input()?;

        Ok(s)
    }

    pub fn label(&self) -> Result<String> {
        self.hwmon.trim_file(&format!("temp{}_label", self.id))
    }

    pub fn input(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("temp{}_input", self.id))
    }

    pub fn lcrit(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("temp{}_lcrit", self.id))
    }

    pub fn min(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("temp{}_min", self.id))
    }

    pub fn max(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("temp{}_max", self.id))
    }

    pub fn crit(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("temp{}_crit", self.id))
    }

    pub fn emergency(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("temp{}_emergency", self.id))
    }
}

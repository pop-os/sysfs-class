use crate::{HwMon, SysClass};
use std::io::Result;

pub struct HwMonFan<'a> {
    hwmon: &'a HwMon,
    id: u64,
}

impl<'a> HwMonFan<'a> {
    pub fn new(hwmon: &'a HwMon, id: u64) -> Result<Self> {
        let s = Self { hwmon, id };

        s.input()?;

        Ok(s)
    }

    pub fn label(&self) -> Result<String> {
        self.hwmon.trim_file(&format!("fan{}_label", self.id))
    }

    pub fn input(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("fan{}_input", self.id))
    }

    pub fn min(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("fan{}_min", self.id))
    }

    pub fn max(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("fan{}_max", self.id))
    }

    pub fn target(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("fan{}_target", self.id))
    }

    pub fn div(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("fan{}_div", self.id))
    }

    pub fn pulses(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("fan{}_pulses", self.id))
    }
}

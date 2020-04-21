use crate::{HwMon, SysClass};
use std::io::Result;

pub struct HwMonPwm<'a> {
    hwmon: &'a HwMon,
    id: u64,
}

impl<'a> HwMonPwm<'a> {
    pub fn new(hwmon: &'a HwMon, id: u64) -> Result<Self> {
        let s = Self { hwmon, id };

        s.input()?;

        Ok(s)
    }

    pub fn input(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("pwm{}", self.id))
    }

    pub fn min(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("pwm{}_min", self.id))
    }

    pub fn max(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("pwm{}_max", self.id))
    }

    pub fn freq(&self) -> Result<u32> {
        self.hwmon.parse_file(&format!("pwm{}_freq", self.id))
    }

    pub fn enable(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("pwm{}_enable", self.id))
    }

    pub fn mode(&self) -> Result<u8> {
        self.hwmon.parse_file(&format!("pwm{}_mode", self.id))
    }
}

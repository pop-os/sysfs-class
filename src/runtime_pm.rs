use std::io;

/// Control whether a device uses, or does not use, runtime power management.
pub enum RuntimePowerManagement {
    On,
    Off,
}

impl From<RuntimePowerManagement> for &'static str {
    fn from(pm: RuntimePowerManagement) -> &'static str {
        match pm {
            RuntimePowerManagement::On => "auto",
            RuntimePowerManagement::Off => "on",
        }
    }
}

pub trait RuntimePM {
    fn set_runtime_pm(&self, state: RuntimePowerManagement) -> io::Result<()>;
}

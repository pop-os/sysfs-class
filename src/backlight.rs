use crate::{Brightness, SysClass};
use std::io::Result;
use std::path::{Path, PathBuf};

/// Fetch and modify brightness values of backlight controls.
#[derive(Clone)]
pub struct Backlight {
    path: PathBuf,
}

impl SysClass for Backlight {
    fn class() -> &'static str {
        "backlight"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Backlight {
    method!(actual_brightness parse_file u64);

    method!(bl_power parse_file u64);

    method!("type", type_ trim_file String);
}

impl Brightness for Backlight {}

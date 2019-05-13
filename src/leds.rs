use std::io::Result;
use std::path::{Path, PathBuf};
use {Brightness, SysClass};

/// Fetch and modify brightness values of LED controllers.
#[derive(Clone)]
pub struct Leds {
    path: PathBuf
}

impl SysClass for Leds {
    fn class() -> &'static str {
        "leds"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Leds {
    /// Filters backlights to only include keyboard backlights
    pub fn iter_keyboards() -> impl Iterator<Item = Result<Self>> where Self: 'static {
        Self::iter().filter(move |object| {
            object.as_ref().ok().map_or(true, |o| o.id().contains("kbd_backlight"))
        })
    }
}

impl Brightness for Leds {}

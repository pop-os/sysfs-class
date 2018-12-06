use std::io::{self, Result};
use std::path::{Path, PathBuf};
use SysClass;

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
    method!(brightness parse_file u64);
    set_method!("brightness", set_brightness u64);

    method!(max_brightness parse_file u64);

    /// Sets the `new` brightness level if it is less than the current brightness.
    ///
    /// Returns the brightness level that was set at the time of exiting the function.
    pub fn set_if_lower(&mut self, new: u64) -> io::Result<u64> {
        let max_brightness = self.max_brightness()?;
        let current = self.brightness()?;
        let new = max_brightness * new / 100;
        if new < current {
            self.set_brightness(new)?;
            Ok(new)
        } else {
            Ok(current)
        }
    }

    /// Filters backlights to only include keyboard backlights
    pub fn keyboard_backlights() -> Box<Iterator<Item = Result<Self>>> where Self: 'static {
        Box::new(Self::iter().filter(|object| {
            object.as_ref().ok().map_or(true, |o| o.id().contains("kbd_backlight"))
        }))
    }
}

use std::io::Result;
use SysClass;

pub trait Brightness: SysClass {
    trait_method!(brightness parse_file u64);

    trait_method!(max_brightness parse_file u64);

    set_trait_method!("brightness", set_brightness u64);

    /// Sets the `new` brightness level if it is less than the current brightness.
    ///
    /// Returns the brightness level that was set at the time of exiting the function.
    fn set_if_lower_than(&self, percent: u64) -> Result<()> {
        let max_brightness = self.max_brightness()?;
        let current = self.brightness()?;

        let new = max_brightness * percent / 100;
        if new < current {
            self.set_brightness(new)
        } else {
            Ok(())
        }
    }
}

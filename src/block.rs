use std::io::Result;
use std::path::{Path, PathBuf};

use SysClass;

pub type SlaveIter = Box<Iterator<Item = Result<PathBuf>>>;

/// A block device in /sys/class/block
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Block {
    path: PathBuf
}

impl SysClass for Block {
    fn class() -> &'static str {
        "block"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Block {
    pub fn has_device(&self) -> bool {
        self.path.join("device").exists()
    }

    pub fn children(&self) -> Result<Vec<Self>> {
        let mut children = Block::all()?.into_iter()
            .filter(|x| {
                x.parent_device().map_or(false, |parent| parent.path() == self.path)
            })
            .collect::<Vec<_>>();
        children.sort_unstable();
        Ok(children)
    }

    pub fn parent_device(&self) -> Option<Block> {
        self.partition().ok().and_then(|partition| {
            let path = self.path().to_str()?;
            let pos = path.len() - partition as usize / 10 - 1;
            let path = Path::new(path.split_at(pos).0).to_path_buf();
            Some(unsafe { Block::from_path_unchecked(path) })
        })
    }

    method!(dev read_file String);

    method!(partition parse_file u8);

    method!(ro parse_file u8);

    method!(removable parse_file u8);

    /// Logical devices have their parent device(s) listed here.
    /// 
    /// For example:
    /// 
    /// - dm-4 has a slave of dm-0
    /// - dm-0 has a slave of sda3
    /// - sda3 does not have any slaves
    pub fn slaves(&self) -> Option<Result<SlaveIter>> {
        let slaves_path = self.path.join("slaves");
        if slaves_path.exists() {
            let res: Result<SlaveIter> = match slaves_path.read_dir() {
                Ok(iter) => Ok(Box::new(iter.map(|entry| Ok(entry?.path())))),
                Err(why) => Err(why)
            };

            Some(res)
        } else {
            None
        }
    }

    method!(start parse_file u64);

    method!(size parse_file u64);

    // From the /device/ path, if it exists.

    method!("device/device_blocked", device_blocked parse_file u8);

    method!("device/device_busy", device_busy parse_file u8);

    method!("device/model", device_model read_file String);

    method!("device/rev", device_rev read_file String);

    method!("device/state", device_state read_file String);

    method!("device/vendor", device_vendor read_file String);
}
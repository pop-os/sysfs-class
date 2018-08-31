use std::io::Result;
use std::path::{Path, PathBuf};

use SysClass;

/// A block device in /sys/class/block
#[derive(Clone)]
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
    pub fn partition(&self) -> Result<u8> {
        self.parse_file("partition")
    }

    pub fn removable(&self) -> Result<u8> {
        self.parse_file("removable")
    }

    pub fn size(&self) -> Result<u64> {
        self.parse_file("size")
    }
}

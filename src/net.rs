use std::io::Result;
use std::path::{Path, PathBuf};
use SysClass;

#[derive(Clone)]
pub struct Net {
    path: PathBuf
}

impl SysClass for Net {
    fn class() -> &'static str {
        "net"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Net {
    pub fn statistics(&self) -> NetStatistics {
        NetStatistics { parent: self }
    }

    method!(addr_assign_type parse_file u8);
    method!(addr_len parse_file u16);
    method!(address trim_file String);
    method!(broadcast trim_file String);
    method!(carrier parse_file u16);
    method!(carrier_changes parse_file u16);
    method!(carrier_down_count parse_file u16);
    method!(carrier_up_count parse_file u16);
    method!(dev_id trim_file String);
    method!(dev_port parse_file u16);
    method!(dormant parse_file u8);
    method!(duplex trim_file String);
    method!(mtu parse_file u32);
    method!(operstate trim_file String);
    method!(speed parse_file u32);
    method!(tx_queue_len parse_file u32);
}

pub struct NetStatistics<'a> {
    parent: &'a Net,
}

impl<'a> NetStatistics<'a> {
    const DIR: &'static str = "statistics";

    pub fn rx_bytes(&self) -> Result<u64> {
        self.parent.parse_file(&[Self::DIR, "/rx_bytes"].concat())
    }

    pub fn rx_packets(&self) -> Result<u64> {
        self.parent.parse_file(&[Self::DIR, "/rx_packets"].concat())
    }

    pub fn tx_bytes(&self) -> Result<u64> {
        self.parent.parse_file(&[Self::DIR, "/tx_bytes"].concat())
    }

    pub fn tx_packets(&self) -> Result<u64> {
        self.parent.parse_file(&[Self::DIR, "/tx_packets"].concat())
    }
}

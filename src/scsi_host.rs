use crate::SysClass;
use std::io::{self, Result};
use std::path::{Path, PathBuf};

/// Fetch and modify SCSI host parameters.
#[derive(Clone)]
pub struct ScsiHost {
    path: PathBuf,
}

impl SysClass for ScsiHost {
    fn class() -> &'static str {
        "scsi_host"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl ScsiHost {
    method!(active_mod trim_file String);

    method!(can_queue parse_file i32);

    method!(host_busy parse_file u8);

    method!(link_power_management_policy trim_file String);

    /// Sets the power management profile for this SCSI host.
    ///
    /// Multiple profiles are given, and each profile is tried until one succeeds.
    pub fn set_link_power_management_policy<'b>(
        &self,
        profiles: &[&'b str],
    ) -> io::Result<&'b str> {
        debug_assert!(
            !profiles.is_empty(),
            "at least one profile must be specified"
        );

        let mut last_result = Ok(());
        let mut last_prof = "";

        for prof in profiles {
            last_result = self.write_file("link_power_management_policy", prof);
            last_prof = prof;
            if last_result.is_ok() {
                break;
            }
        }

        last_result.map(|_| last_prof)
    }

    method!(proc_name trim_file String);

    method!(sg_tablesize parse_file i32);

    method!(state trim_file String);

    method!(supported_mode parse_file u8);

    method!(use_blk_mq parse_file u8);
}

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub trait SysClass: Sized {
    // Return the class of the sys object, the name of a folder in /sys/class
    fn class() -> &'static str;

    // Create a sys object from an absolute path without checking path for validity
    unsafe fn from_path_unchecked(path: PathBuf) -> Self;

    // Return the path of the sys object
    fn path(&self) -> &Path;

    // Return the path to the sys objects, the full path of a folder in /sys/class
    fn dir() -> PathBuf {
        Path::new("/sys/class").join(Self::class())
    }

    // Create a sys object from a path, checking it for validity
    fn from_path(path: &Path) -> Result<Self> {
        let canon = path.canonicalize()?;

        {
            let parent = canon.parent().ok_or(Error::new(
                ErrorKind::InvalidInput,
                format!("{}: does not have parent", path.display())
            ))?;

            let dir = Self::dir();
            if parent != dir {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{}: is not a child of {}", path.display(), dir.display())
                ));
            }
        }

        fs::read_dir(&canon)?;

        Ok(unsafe { Self::from_path_unchecked(canon) })
    }

    // Retrieve all of the object instances of a sys class
    fn all() -> Result<Vec<Self>> {
        let mut ret = Vec::new();

        for entry_res in fs::read_dir(Self::dir())? {
            let entry = entry_res?;
            ret.push(Self::from_path(&entry.path())?);
        }

        Ok(ret)
    }

    // Create a sys object by name, checking it for validity
    fn new(name: &str) -> Result<Self> {
        Self::from_path(&Self::dir().join(name))
    }

    // Return the name of the sys object
    fn name(&self) -> &str {
        self.path()
            .file_name().unwrap() // A valid path does not end in .., so safe
            .to_str().unwrap() // A valid path must be valid UTF-8, so safe
    }
}

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

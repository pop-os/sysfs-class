use std::fmt::Display;
use std::fs;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub trait SysClass: Sized {
    /// Return the class of the sys object, the name of a folder in /sys/class
    fn class() -> &'static str;

    /// Create a sys object from an absolute path without checking path for validity
    unsafe fn from_path_unchecked(path: PathBuf) -> Self;

    /// Return the path of the sys object
    fn path(&self) -> &Path;

    /// Return the path to the sys objects, the full path of a folder in /sys/class
    fn dir() -> PathBuf {
        Path::new("/sys/class").join(Self::class())
    }

    /// Create a sys object from a path, checking it for validity
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

    /// Retrieve all of the object instances of a sys class
    fn all() -> Result<Vec<Self>> {
        let mut ret = Vec::new();

        for entry_res in fs::read_dir(Self::dir())? {
            let entry = entry_res?;
            ret.push(Self::from_path(&entry.path())?);
        }

        Ok(ret)
    }

    /// Create a sys object by id, checking it for validity
    fn new(id: &str) -> Result<Self> {
        Self::from_path(&Self::dir().join(id))
    }

    /// Return the id of the sys object
    fn id(&self) -> &str {
        self.path()
            .file_name().unwrap() // A valid path does not end in .., so safe
            .to_str().unwrap() // A valid path must be valid UTF-8, so safe
    }

    /// Read a file underneath the sys object
    fn read_file<P: AsRef<Path>>(&self, name: P) -> Result<String> {
        let mut data = String::new();

        {
            let path = self.path().join(name.as_ref());
            let mut file = fs::OpenOptions::new().read(true).open(path)?;
            file.read_to_string(&mut data)?;
        }

        Ok(data)
    }

    /// Parse a number from a file underneath the sys object
    fn parse_file<F: FromStr, P: AsRef<Path>>(&self, name: P) -> Result<F> where F::Err: Display {
        self.read_file(name)?.trim().parse().map_err(|err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("{}", err)
            )
        })
    }

    /// Read a file underneath the sys object and trim whitespace
    fn trim_file<P: AsRef<Path>>(&self, name: P) -> Result<String> {
        let data = self.read_file(name)?;
        Ok(data.trim().to_string())
    }

    /// Write a file underneath the sys object
    fn write_file<P: AsRef<Path>, S: AsRef<[u8]>>(&self, name: P, data: S) -> Result<()> {
        {
            let path = self.path().join(name.as_ref());
            let mut file = fs::OpenOptions::new().write(true).open(path)?;
            file.write_all(data.as_ref())?
        }

        Ok(())
    }
}

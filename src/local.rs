use std::env;

use camino::{Utf8Path, Utf8PathBuf};

use crate::Directories;

/// Local directories are meant mostly for debug purposes while developing an application. By
/// default it provides all available directories in under a `.local` folder in the current working
/// directory.
///
/// An alternative base directory can be provided with the [`LocalDirs::new_at`] function.
///
/// The folders are defined as follows, with `<base>` being either `.local` or a user defined
/// directory:
///
/// | Type   | Location        |
/// | ------ | --------------- |
/// | Cache  | `<base>`/cache  |
/// | Config | `<base>`/config |
/// | Data   | `<base>`/data   |
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalDirs {
    cache_dir: Utf8PathBuf,
    config_dir: Utf8PathBuf,
    data_dir: Utf8PathBuf,
}

impl LocalDirs {
    /// Create a default instance, using the `$PWD/.local` directory as a base.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::env;
    /// use unidirs::{Directories, LocalDirs};
    ///
    /// let dirs = LocalDirs::new().unwrap();
    ///
    /// assert_eq!(env::current_dir().unwrap().join(".local/data"), dirs.data_dir());
    /// ```
    #[must_use]
    pub fn new() -> Option<Self> {
        let base = env::current_dir().ok()?;
        let base = Utf8PathBuf::from_path_buf(base).ok()?.join(".local");

        Some(Self::new_at(base))
    }

    /// Create an instance at the given base directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempfile::TempDir;
    /// use unidirs::{Directories, LocalDirs, Utf8Path};
    ///
    /// let temp = TempDir::new().unwrap();
    /// let dirs = LocalDirs::new_at(Utf8Path::from_path(temp.path()).unwrap());
    ///
    /// assert_eq!(temp.path().join("data"), dirs.data_dir());
    /// ```
    pub fn new_at(base: impl AsRef<Utf8Path>) -> Self {
        fn inner(base: &Utf8Path) -> LocalDirs {
            LocalDirs {
                cache_dir: base.join("cache"),
                config_dir: base.join("config"),
                data_dir: base.join("data"),
            }
        }

        inner(base.as_ref())
    }
}

impl Directories for LocalDirs {
    fn cache_dir(&self) -> &Utf8Path {
        &self.cache_dir
    }

    fn config_dir(&self) -> &Utf8Path {
        &self.config_dir
    }

    fn data_dir(&self) -> &Utf8Path {
        &self.data_dir
    }
}

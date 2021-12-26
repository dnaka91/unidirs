use camino::{Utf8Path, Utf8PathBuf};

use crate::Directories;

///
/// ## Linux
///
/// | Type   | Location                                                 |
/// | ------ | -------------------------------------------------------- |
/// | Cache  | `$XDG_CACHE_HOME`/`<app>` or `$HOME`/.cache/`<app>`      |
/// | Config | `$XDG_CONFIG_HOME`/`<app>` or `$HOME`/.config/`<app>`    |
/// | Data   | `$XDG_DATA_HOME`/`<app>` or `$HOME`/.local/share/`<app>` |
///
/// ## Mac OS
///
/// | Type   | Location                                                     |
/// | ------ | ------------------------------------------------------------ |
/// | Cache  | `$HOME`/Library/Caches/`<qual>`.`<org>`.`<app>`              |
/// | Config | `$HOME`/Library/Application Support/`<qual>`.`<org>`.`<app>` |
/// | Data   | `$HOME`/Library/Application Support/`<qual>`.`<org>`.`<app>` |
///
/// ## Windows
///
/// | Type   | Location                                    |
/// | ------ | ------------------------------------------- |
/// | Cache  | `%LOCALAPPDATA%`\\`<org>`\\`<app>`\cache    |
/// | Config | `%ROAMINGAPPDATA%`\\`<org>`\\`<app>`\config |
/// | Data   | `%ROAMINGAPPDATA%`\\`<org>`\\`<app>`\data   |
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserDirs {
    cache_dir: Utf8PathBuf,
    config_dir: Utf8PathBuf,
    data_dir: Utf8PathBuf,
}

impl UserDirs {
    /// Create a new instance with the given qualifier, organization and application. These values
    /// are used to create a custom directory structure depending on the operating system standards.
    pub fn new(
        qualifier: impl AsRef<str>,
        organization: impl AsRef<str>,
        application: impl AsRef<str>,
    ) -> Option<Self> {
        fn inner(qualifier: &str, organization: &str, application: &str) -> Option<UserDirs> {
            directories::ProjectDirs::from(qualifier, organization, application)
                .and_then(|dirs| UserDirs::from_project_dirs(&dirs).ok())
        }

        inner(
            qualifier.as_ref(),
            organization.as_ref(),
            application.as_ref(),
        )
    }

    fn from_project_dirs(value: &directories::ProjectDirs) -> Result<Self, camino::FromPathError> {
        Ok(Self {
            cache_dir: <&Utf8Path>::try_from(value.cache_dir())?.to_owned(),
            config_dir: <&Utf8Path>::try_from(value.config_dir())?.to_owned(),
            data_dir: <&Utf8Path>::try_from(value.data_dir())?.to_owned(),
        })
    }
}

impl Directories for UserDirs {
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

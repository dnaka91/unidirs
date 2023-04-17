use camino::{Utf8Path, Utf8PathBuf};

use crate::Directories;

/// Service directories are used for applications that run as a service (or often called daemon),
/// usually run by a dedicated user account and controlled by the system rather than the user.
///
/// The directories are therefore rather global and close to the system instead of being located in
/// the user's home directory.
///
/// ## Unix (Linux, Mac OS and others)
///
/// On Unix systems the service folders are mostly standardized. Depending on how a service is run
/// on Mac OS, the [`UserDirs`](crate::UserDirs) might be correct as well, but for system-run
/// services the correct folders are the same as on other Unix systems.
///
/// | Type   | Location           |
/// | ------ | ------------------ |
/// | Cache  | /var/cache/`<app>` |
/// | Config | /etc/`<app>`       |
/// | Data   | /var/lib/`<app>`   |
///
/// ## Windows
///
/// On Windows there are three standard service accounts: `LocalService`, `NetworkService` and
/// `LocalSystem`. These present different capabilities and a network service provides a middle
/// ground with minimal capabilities plus networking access.
///
/// The API might be extended to pick the type of service account in the future.
///
/// | Type   | Location                                                                           |
/// | ------ | ---------------------------------------------------------------------------------- |
/// | Cache  | C:\Windows\ServiceProfiles\NetworkService\AppData\\`<org>`\\`<app>`\Local\cache    |
/// | Config | C:\Windows\ServiceProfiles\NetworkService\AppData\\`<org>`\\`<app>`\Roaming\config |
/// | Data   | C:\Windows\ServiceProfiles\NetworkService\AppData\\`<org>`\\`<app>`\Roaming\data   |
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceDirs {
    cache_dir: Utf8PathBuf,
    config_dir: Utf8PathBuf,
    data_dir: Utf8PathBuf,
}

impl ServiceDirs {
    /// Create a new instance with the given organization and application name. The organization
    /// name is only used on Windows systems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unidirs::{Directories, ServiceDirs};
    ///
    /// let dirs = ServiceDirs::new("example", "app");
    ///
    /// println!("data_dir = {}", dirs.data_dir());
    /// // On Unix:    /var/lib/app
    /// // On Windows: C:\Windows\ServiceProfiles\NetworkService\AppData\example\app\data
    /// ```
    #[allow(unused_variables)]
    #[must_use]
    pub fn new(organization: impl AsRef<str>, application: impl AsRef<str>) -> Self {
        fn inner(organization: &str, application: &str) -> ServiceDirs {
            #[cfg(unix)]
            {
                ServiceDirs {
                    cache_dir: Utf8PathBuf::from(format!("/var/cache/{application}")),
                    config_dir: Utf8PathBuf::from(format!("/etc/{application}")),
                    data_dir: Utf8PathBuf::from(format!("/var/lib/{application}")),
                }
            }

            #[cfg(windows)]
            {
                let app_data = "C:\\Windows\\ServiceProfiles\\NetworkService\\AppData";
                let project_dir = format!("{}/{}", organization, application);

                ServiceDirs {
                    cache_dir: Utf8PathBuf::from(format!(
                        "{}\\Local\\{}\\cache",
                        app_data, project_dir
                    )),
                    config_dir: Utf8PathBuf::from(format!(
                        "{}\\Roaming\\{}\\config",
                        app_data, project_dir
                    )),
                    data_dir: Utf8PathBuf::from(format!(
                        "{}\\Roaming\\{}\\data",
                        app_data, project_dir
                    )),
                }
            }

            #[cfg(not(any(unix, windows)))]
            {
                compile_error!("OS not supported")
            }
        }

        inner(organization.as_ref(), application.as_ref())
    }
}

impl Directories for ServiceDirs {
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

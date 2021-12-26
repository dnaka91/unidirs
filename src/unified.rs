use camino::Utf8Path;

use crate::{
    local::LocalDirs, service::ServiceDirs, simple::SimpleBuilder, user::UserDirs, Directories,
};

/// Unified directories provide a common interface over all different ways of constructing directory
/// providers. It provides constructors for each variant.
///
/// The most significant function is [`UnifiedDirs::simple`], giving an automatic selector for the
/// right directory provider based on the way an application is run.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnifiedDirs {
    /// Local directories for development.
    Local(LocalDirs),
    /// Directories for applications run as service.
    Service(ServiceDirs),
    /// User specific application folders.
    User(UserDirs),
}

impl UnifiedDirs {
    /// Shorthand to create unified dirs with [`LocalDirs`] as backend.
    pub fn local() -> Option<Self> {
        LocalDirs::new().map(Self::Local)
    }

    /// Shorthand to create [`LocalDirs`] backed unified dirs at a specific location.
    pub fn local_at(base: impl AsRef<Utf8Path>) -> Self {
        Self::Local(LocalDirs::new_at(base))
    }

    /// Shorthand to create unified dirs with [`ServiceDirs`] as backend.
    #[must_use]
    pub fn service(organization: impl AsRef<str>, application: impl AsRef<str>) -> Self {
        Self::Service(ServiceDirs::new(organization, application))
    }

    /// Shorthand to create unified dirs with [`UserDirs`] as backend.
    pub fn user(
        qualifier: impl AsRef<str>,
        organization: impl AsRef<str>,
        application: impl AsRef<str>,
    ) -> Option<Self> {
        UserDirs::new(qualifier, organization, application).map(Self::User)
    }

    /// Create a builder for unified directories that uses various detection techniques to select
    /// the right backend. See the [`SimpleBuilder`] implementation for more details about the
    /// used techniques.
    #[must_use]
    pub fn simple<Q, O, A>(qualifier: Q, organization: O, application: A) -> SimpleBuilder<Q, O, A>
    where
        Q: AsRef<str>,
        O: AsRef<str>,
        A: AsRef<str>,
    {
        SimpleBuilder::new(qualifier, organization, application)
    }
}

impl Directories for UnifiedDirs {
    fn cache_dir(&self) -> &Utf8Path {
        match self {
            Self::Local(dirs) => dirs.cache_dir(),
            Self::Service(dirs) => dirs.cache_dir(),
            Self::User(dirs) => dirs.cache_dir(),
        }
    }

    fn config_dir(&self) -> &Utf8Path {
        match self {
            Self::Local(dirs) => dirs.config_dir(),
            Self::Service(dirs) => dirs.config_dir(),
            Self::User(dirs) => dirs.config_dir(),
        }
    }

    fn data_dir(&self) -> &Utf8Path {
        match self {
            Self::Local(dirs) => dirs.data_dir(),
            Self::Service(dirs) => dirs.data_dir(),
            Self::User(dirs) => dirs.data_dir(),
        }
    }
}

use std::env;

use crate::unified::UnifiedDirs;

/// The simple builder is constructed through the [`UnifiedDirs::simple`] method and allows to
/// further configure ways of detecting whether the application is run as a service or by the user.
///
/// [`with`](Self::with) and all the `with_*` functions are called and evaluated in order and
/// immediately (**not** delayed until the call to [`build`](Self::build)). If service mode is
/// detected by any technique, further functions won't be evaluated anymore.
pub struct SimpleBuilder<Q, O, A> {
    service: bool,
    qualifier: Q,
    organization: O,
    application: A,
}

impl<Q, O, A> SimpleBuilder<Q, O, A>
where
    Q: AsRef<str>,
    O: AsRef<str>,
    A: AsRef<str>,
{
    pub(crate) fn new(qualifier: Q, organization: O, application: A) -> Self {
        Self {
            service: false,
            qualifier,
            organization,
            application,
        }
    }

    /// Use certain environment variable names to detect to be in service mode. The value of each
    /// variable doesn't matter, just whether the variable is present.
    ///
    /// Currently the name `SERVCIE` and `DAEMON` indicate the service mode.
    #[must_use]
    pub fn with_env(self) -> Self {
        self.with(|_| env::vars_os().any(|(name, _)| name == "SERVICE" || name == "DAEMON"))
    }

    /// Use certain program arguments to detect to be in service mode.
    ///
    /// Currently the arguments `--service` and `--daemon` indicate the service mode.
    #[must_use]
    pub fn with_args(self) -> Self {
        self.with(|_| env::args_os().any(|name| name == "--service" || name == "--daemon"))
    }

    /// Compare the executing user's account name against the application name to detect the service
    /// mode.
    ///
    /// It is common to create a separate
    #[must_use]
    pub fn with_username(self) -> Self {
        self.with(|builder| whoami::username_os() == builder.application.as_ref())
    }

    /// Define a custom detection logic for the service mode. A positive value means service mode, a
    /// negative value means user mode.
    ///
    /// The provided closure is only called if previous techniques didn't detect a service mode yet.
    ///
    /// # Example
    ///
    /// Consider the application was called with a `SERVICE` environment variable present and the
    /// following builder was used. The closure will not be called as the application was already
    /// detected to be in service mode due to the environment variable.
    ///
    /// ```rust
    /// use std::env;
    /// use unidirs::UnifiedDirs;
    ///
    /// env::set_var("SERVICE", "true");
    ///
    /// let mut called = false;
    /// let dirs = UnifiedDirs::simple("com", "example", "app")
    ///     .with_env()
    ///     .with(|_| {
    ///         called = true;
    ///         true
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(!called);
    /// ```
    ///
    /// As the `with_*` function are called in order, the same setup but in reverse order will call
    /// the lambda as it is evaluated first:
    ///
    /// ```rust
    /// use std::env;
    /// use unidirs::UnifiedDirs;
    ///
    /// env::set_var("SERVICE", "true");
    ///
    /// let mut called = false;
    /// let dirs = UnifiedDirs::simple("com", "example", "app")
    ///     .with(|_| {
    ///         called = true;
    ///         true
    ///     })
    ///     .with_env() // called after `with` now
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(called);
    /// ```
    pub fn with(self, f: impl FnOnce(&Self) -> bool) -> Self {
        Self {
            service: self.service || f(&self),
            qualifier: self.qualifier,
            organization: self.organization,
            application: self.application,
        }
    }

    /// Construct the [`UnifiedDirs`] instance with the backend decided by previously configured
    /// techniques.
    ///
    /// - If the application was built in debug mode (or with `debug_assertions` enabled), it will
    ///   always pick [`LocalDirs`](crate::LocalDirs).
    /// - If any of the configured techniques detected that the application is run in service mode,
    ///   the backend will be [`ServiceDirs`](crate::ServiceDirs).
    /// - Otherwise, it'll be [`UserDirs`](crate::UserDirs).
    #[must_use]
    pub fn build(self) -> Option<UnifiedDirs> {
        fn inner(
            service: bool,
            qualifier: &str,
            organization: &str,
            application: &str,
        ) -> Option<UnifiedDirs> {
            if cfg!(debug_assertions) {
                UnifiedDirs::local()
            } else if service {
                Some(UnifiedDirs::service(organization, application))
            } else {
                UnifiedDirs::user(qualifier, organization, application)
            }
        }

        inner(
            self.service,
            self.qualifier.as_ref(),
            self.organization.as_ref(),
            self.application.as_ref(),
        )
    }

    /// Configure and execute the builder with all detection techniques enabled.
    ///
    /// This is a convenience shorthand for manually calling [`with_env`](Self::with_env),
    /// [`with_args`](Self::with_args) and [`with_username`](Self::with_username) followed by
    /// [`build`](Self::build).
    ///
    /// # Example
    ///
    /// ```rust
    /// use unidirs::UnifiedDirs;
    ///
    /// UnifiedDirs::simple("com", "example", "app").default();
    ///
    /// // The `default()` call is a shorthand for:
    ///
    /// UnifiedDirs::simple("com", "example", "app")
    ///     .with_env()
    ///     .with_args()
    ///     .with_username()
    ///     .build();
    /// ```
    pub fn default(self) -> Option<UnifiedDirs> {
        self.with_env().with_args().with_username().build()
    }
}

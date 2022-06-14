//! Unified directories for different use cases of an application, providing standard directories
//! for local development, when run as service or when run by a user.
//!
//! This crate provides 4 main structures of interest:
//!
//! - [`LocalDirs`](crate::LocalDirs) to use a local directory as basis.
//! - [`ServiceDirs`](crate::ServiceDirs) for use when running as a service.
//! - [`UserDirs`](crate::UserDirs) for use when run by a local user directly.
//! - [`UnifiedDirs`](crate::UnifiedDirs) as a combination of the above three to provide a common
//!   interface.
//!
//! The simplest, but most opinionated, way of using this crate is the [`UnifiedDirs::simple`]
//! function. It will use the local dirs unconditionally in debug mode and uses several heuristics
//! to decide to use service or user dirs.
//!
//! Passing the boolean flag can be done in any way possible. It is very common to pass it from
//! command line arguments or use a environment variable to detect the service mode.
//!
//! ## Using the `simple` builder
//!
//! The call to [`UnifiedDirs::simple`] returns a [`SimpleBuilder`] that can be further configured
//! to use all or only specific heuristics to detect a service or user mode.
//!
//! ```rust
//! use unidirs::{Directories, UnifiedDirs};
//!
//! let dirs = UnifiedDirs::simple("com", "example", "app")
//!     .with_env()
//!     .with_args()
//!     .with_username()
//!     .build()
//!     .unwrap();
//!
//! println!("cache dir: {}", dirs.cache_dir());
//! println!("config dir: {}", dirs.config_dir());
//! println!("data dir: {}", dirs.data_dir());
//!```
//!
//! ## Using `clap` to pass a flag
//!
//! This example uses the popular [`clap`](https://lib.rs/crates/clap) crate to parse and pass the
//! service flag to [`UnifiedDirs`].
//!
//! ```rust
//! use clap::Parser;
//! use unidirs::{Directories, UnifiedDirs};
//!
//! #[derive(Parser)]
//! struct Opt {
//!     #[clap(long, action, alias = "daemon")]
//!     service: bool,
//! }
//!
//! let opt = Opt::parse();
//! let dirs = UnifiedDirs::simple("com", "example", "app")
//!     .with(|_| opt.service)
//!     .build()
//!     .unwrap();
//!
//! println!("cache dir: {}", dirs.cache_dir());
//! println!("config dir: {}", dirs.config_dir());
//! println!("data dir: {}", dirs.data_dir());
//! ```
//!
//! ## Using environment variables to detect service mode
//!
//! In this sample we use the environment variable `RUN_AS_SERVICE` to detect the service mode. The
//! variable can be anything, but pre-fixing it with the applications name is recommended to avoid
//! name clashes (for example `MYAPP_SERVICE`).
//!
//! ```rust
//! use std::env;
//!
//! use unidirs::{Directories, UnifiedDirs};
//!
//! let service = env::var_os("RUN_AS_SERVICE").is_some();
//! let dirs = UnifiedDirs::simple("com", "example", "app")
//!     .with(|_| service)
//!     .build()
//!     .unwrap();
//!
//! println!("cache dir: {}", dirs.cache_dir());
//! println!("config dir: {}", dirs.config_dir());
//! println!("data dir: {}", dirs.data_dir());
//! ```

#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    rust_2018_idioms,
    clippy::all,
    clippy::cargo,
    clippy::pedantic,
    clippy::expect_used,
    clippy::unwrap_used
)]
#![allow(clippy::module_name_repetitions)]

pub use camino::{self, Utf8Path, Utf8PathBuf};

pub use crate::{
    local::LocalDirs, service::ServiceDirs, simple::SimpleBuilder, unified::UnifiedDirs,
    user::UserDirs,
};

mod local;
mod service;
mod simple;
mod unified;
mod user;

/// Common directories that are provided by all `*Dirs` structures. This can be used as an
/// alternative to [`UnifiedDirs`] to abstract over the underlying provider implementation.
///
/// Note that on some platforms the different directories can end up being the same.
pub trait Directories {
    /// The cache directory is a location where an application can save any temporary data. The
    /// contents can potentially be deleted by the system at any time. Therefore, the application
    /// must be able to work without these files or be able to re-create them.
    fn cache_dir(&self) -> &Utf8Path;

    /// The config directory is where an application's settings are stored. These are usually
    /// created by the user and loaded once at startup of the application.
    fn config_dir(&self) -> &Utf8Path;

    /// The data directory hold an application's state data, like a database. The folder is
    /// expected to persist during the normal runtime of the OS.
    fn data_dir(&self) -> &Utf8Path;
}

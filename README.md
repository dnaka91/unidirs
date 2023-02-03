# UniDirs

[![Build Status][build-img]][build-url]
[![Repository][crates-img]][crates-url]
[![Documentation][doc-img]][doc-url]

[build-img]: https://img.shields.io/github/actions/workflow/status/dnaka91/unidirs/ci.yml?branch=main&style=for-the-badge
[build-url]: https://github.com/dnaka91/unidirs/actions?query=workflow%3ACI
[crates-img]: https://img.shields.io/crates/v/unidirs?style=for-the-badge
[crates-url]: https://crates.io/crates/unidirs
[doc-img]: https://img.shields.io/badge/docs.rs-unidirs-4d76ae?style=for-the-badge
[doc-url]: https://docs.rs/unidirs

Unified directories for different use cases of an application, providing standard directories for
local development, when run as service or when run by a user.

## Usage

Add `unidirs` to your project with `cargo add unidirs` (needs [cargo-edit]) or add it manually to
your `Cargo.toml`:

```toml
[dependencies]
unidirs = "0.1.0"
```

Please consult the [documentation][doc-url] for further usage instructions.

## License

This project is licensed under [MIT License](LICENSE) (or <http://opensource.org/licenses/MIT>).

# approxim

![Test](https://img.shields.io/github/actions/workflow/status/jonaspleyer/approxim/test.yml?style=flat-square&label=Test)
[![Crate](https://img.shields.io/crates/v/approxim.svg?style=flat-square)](https://crates.io/crates/approxim)
[![License Apache](https://img.shields.io/badge/License-Apache%202.0-brightgreen.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
[![Docs](https://img.shields.io/docsrs/approxim?style=flat-square)](https://docs.rs/approxim)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.36+-lightgray.svg?style=flat-square)](https://github.com/jonaspleyer/approxim#rust-version-requirements)
![Codecov](https://img.shields.io/codecov/c/github/jonaspleyer/approxim?style=flat-square)

This is a fork of the popular [approx](https://github.com/brendanzab/approx) crate.
Approximate floating point equality comparisons and assertions for the Rust Programming Language.

## Replace `approx`
To act as a drop-in replacement for [approx](https://github.com/brendanzab/approx), one can use the
renaming option provided by cargo.

```toml
# Cargo.toml
[dependencies]
approx = { package = "approxim", version = "CURRENT_VERSION" }
```
Note that no further changes are required. Derive macros should also work with this workaround.

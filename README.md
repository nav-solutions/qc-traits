GNSS Quality Control Traits
===========================

[![Rust](https://github.com/nav-solutions/qc-traits/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/rinex/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/gnss-qc-traits/badge.svg)](https://docs.rs/gnss-qc-traits/)
[![crates.io](https://img.shields.io/crates/d/gnss-qc-traits.svg)](https://crates.io/crates/gnss-qc-traits)

[![MRSV](https://img.shields.io/badge/MSRV-1.82.0-orange?style=for-the-badge)](https://github.com/rust-lang/rust/releases/tag/1.82.0)
[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/nav-solutions/qc-traits/blob/main/LICENSE)

The Quality Control traits library (`gnss-qc-traits`) is a small library
that offers the basic operations to form a geodesy processing pipeline, as used by
our [Quality Control library](https://github.com/nav-solutions/gnss-qc).

## Existing Modules

- html: HTML report rendition
- merge: describes how we stack data into an already existing context
- processing: available on crate feature only,
describes a filter designer and processing ops

## Licensing

This library is part of the [NAV-SLS framework](https://github.com/nav-solutions) which
is delivered under the [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.

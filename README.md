GNSS Generic QC Traits
======================

[![Rust](https://github.com/nav-solutions/qc-traits/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/rinex/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/gnss-qc-traits/badge.svg)](https://docs.rs/gnss-qc-traits/)
[![crates.io](https://img.shields.io/crates/d/gnss-qc-traits.svg)](https://crates.io/crates/gnss-qc-traits)
[![discord server](https://img.shields.io/discord/1342922474110586910?logo=discord)](https://discord.gg/EqhEBXBmJh)

[![MRSV](https://img.shields.io/badge/MSRV-1.82.0-orange?style=for-the-badge)](https://github.com/rust-lang/rust/releases/tag/1.82.0)
[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/nav-solutions/qc-traits/blob/main/LICENSE)

This is our core QC (Qualicy Check) library, used by all our libraries and enabling
QC like operations and geodesic operations.

[This is our main QC library](https://github.com/nav-solutions/gnss-qc), it relies on most our parsers
and this core library.

## Features

- `html`: HTML rendering
- `processing`: advanced processing ops

## Existing Modules

- html: HTML report rendition
- merge: describes how we stack data into an already existing context
- processing: available on crate feature only,
describes a filter designer and processing ops

## Licensing

This library is part of the [NAV-SLS framework](https://github.com/nav-solutions) which
is licensed under [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.

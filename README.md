GNSS Quality Control Traits
===========================

The Quality Control traits library (`gnss-qc-traits`) is a small library
that offers the basic operations to form a geodesy processing pipeline, as used by
our [Quality Control library](https://github.com/rtk-rs/gnss-qc).

## Existing Modules

- html: HTML report rendition
- merge: describes how we stack data into an already existing context
- processing: available on crate feature only,
describes a filter designer and processing ops

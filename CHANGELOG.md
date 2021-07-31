# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `minimum` field in `Settings`, which controls the minimum number of
  available variants a product should have for a `restock` webhook to be
  sent out.

## [0.1.1] - 2021-07-31

### Fixed

- Typos in `stores.rs` which would lead to invalid image URLs being used
  when users used certain [image aliases](README.md#aliases).
- The way "different level" settings values are
  [combined](README.md#settings), as the previous method would "carry
  over" an event's values to the successive ones if they didn't
  explicitly set them to something else.

## [0.1.0] - 2021-07-30

Initial release.

[Unreleased]: https://github.com/subreme/shopify-monitor/compare/0.1.1...HEAD
[0.1.1]: https://github.com/subreme/shopify-monitor/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/subreme/shopify-monitor/releases/tag/0.1.0

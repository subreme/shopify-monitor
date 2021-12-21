# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2021-12-21

### Added

- `minimum` field in `Settings`, which controls the minimum number of
  available variants a product should have for a `restock` webhook to be
  sent out.
- The ability to detect invalid webhook URLs and stop using them.
- The process of automatically stopping the monitoring of a site if none
  of its webhooks are working.
- The logic to terminate the program if no stores are being monitored.
- A self-updating [version number](src/main.rs#L43-L50) in the program's
  start screen.

### Changed

- The monitor's logic to have a "background process" handle tasks that
  would interrupt the monitor.
- The number of fields in `products.json` that are deserialized,
  reducing latency.

### Fixed

- Bug where the Status Codes to the Discord API Responses were
  interpreted incorrectly.
- Process that "spammed" the terminal, sending repeated warnings about
  websites being unreachable.

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

[Unreleased]: https://github.com/subreme/shopify-monitor/compare/0.1.2...HEAD
[0.1.2]: https://github.com/subreme/shopify-monitor/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/subreme/shopify-monitor/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/subreme/shopify-monitor/releases/tag/0.1.0

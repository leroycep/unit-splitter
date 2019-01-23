# Changelog

All notable changes to the [Unit Splitter][unit-splitter] project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[unit-splitter]: https://utils.geemili.xyz/unit-splitter

## [Unreleased]
### Fixed
- [core] Entering a large integer will no longer cause the application to panic
- [core] The split units are now displayed in the order of input requests
- [web] Fix layout growing larger than necessary for large outputs

## [0.2.1] - 2019-01-17
### Changed
- Switched to the `draco` library for webpage
- Updated to Rust 2018 edition
- Updated to Rust 1.32

### Fixed
- [core] Fixed bug where requesting the exact number of units in a range caused
  the application to panic

## [0.2.0] - 2018-10-13
### Added
- [web] Requests text input
- [web] Indication of software version
- [web] Link to changelog

### Changed
- [web] Moved to a new design for the page that is more responsive

### Removed
- [web] Test/Request button based UI

## [0.1.0] - 2018-06-06
### Added
- [web] Text area to take unit string
- [web] Ability to add and remove requests
- [web] Ability to change request amounts


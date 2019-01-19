# Changelog

All notable changes to the [Unit Splitter][unit-splitter] project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- [core] Entering a large integer will no longer cause the application to panic

## 0.2.1 - 2019-01-17
### Changed
- Switched to the `draco` library for webpage
- Updated to Rust 2018 edition
- Updated to Rust 1.32

### Fixed
- Fixed bug where requesting the exact number of units in a range caused the
  application to panic

## 0.2.0 - 2018-10-13
### Added
- Requests text input
- Indication of software version
- Link to changelog

### Changed
- Moved to a new design for the page that is more responsive

### Removed
- Test/Request button based UI

## 0.1.0 - 2018-06-06
### Added
- Text area to take unit string
- Ability to add and remove requests
- Ability to change request amounts


[unit-splitter]: https://utils.geemili.xyz/unit-splitter

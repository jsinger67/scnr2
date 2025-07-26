# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

Be aware that this project is still v0.y.z which means that anything can change anytime:

> "4. Major version zero (0.y.z) is for initial development. Anything MAY change at any time. The
> public API SHOULD NOT be considered stable."
>
> (Semantic Versioning Specification)

## Indicating incompatible changes on major version zero

We defined for this project that while being on major version zero we mark incompatible changes with
new minor version numbers. Please note that this is no version handling covered by `Semver`.

# 0.3.0 - 2025-07-26



### Summary

* Major improvements focused on testing and reliability, highlighted by the addition of a comprehensive test suite.
* Core scanner functionality received performance optimizations and architectural enhancements, while maintaining backward compatibility.
* Documentation and examples were refined for greater clarity and accuracy.


### Extensive Test Suite Addition (Most significant change)

* Added a comprehensive test infrastructure in `match_test.rs`.
  - Tests are now generated using macros for conciseness and maintainability.
* Removed PowerShell automation scripts previously used for test commissioning, as they are no longer needed.


### Core Scanner Implementation Improvements

* Performance: Added `#[inline]` to the `token_type()` method.
* Refactored mode management: `ScannerImpl` and `FindMatches` now use `Cell` for current mode management instead of `RefCell`.
* Improved scanner internals in `find_matches.rs` and `scanner_impl.rs`.


### Documentation and Example Fixes
* README improvements:
  - Corrected token identifiers in examples.
  - Improved regex patterns for comment text handling.
  - Added documentation for lookahead conditions.
  - Fixed function call token regex.
  - Improved string content handling in scanner examples.
* Completed code documentation for use with `cargo doc`.


### Code Generation Updates
* Improved character classes in `scnr2_generate`.
* Updated expected generated code templates.
* Enhanced code generation logic.


### Development Infrastructure
* Added new test workspace configuration.
* Added `CHANGELOG.md`.

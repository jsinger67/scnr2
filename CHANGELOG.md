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

# 0.3.3 - 2025-09-19

* Enhance error message for pattern parsing to include input context
* Increased number of tests in the match_test module, especially checking error cases.
  This increases the test coverage significantly.

# 0.3.2 - 2025-08-14

### Summary

* Comprehensive test suite enhancements focused on robustness, edge-case coverage, and performance validation.
* Added extensive fuzz testing, error handling validation, and pathological case benchmarking.
* Improved test organization and documentation for better maintainability and coverage visibility.

### Added

* **Fuzz Testing Suite** (`tests/match_fuzz.rs`)
  - Edge-case testing for empty input scenarios
  - Long repeated character handling (10,000+ character sequences)
  - Invalid UTF-8 sequence processing validation
  - Pathological string token testing with complex escape sequences
  - Mixed token stream validation for rapid type switching

* **Error Handling Test Suite** (`tests/match_errors.rs`)
  - Invalid UTF-8 byte sequence handling validation
  - Unterminated string and escape sequence error recovery
  - Extremely long token processing (1M+ characters)
  - Deeply nested expression handling (10,000+ nesting levels)
  - Null byte input processing validation
  - Unicode edge case testing (emoji, CJK characters, mathematical symbols)
  - Memory stress testing for large input scenarios
  - Boundary condition scanning from various positions

* **Pathological Case Benchmarks** (`tests/benches/bench.rs`)
  - Performance benchmarking for extremely long strings (100K+ characters)
  - Invalid UTF-8 sequence processing throughput measurement
  - Mixed pathological token stream performance validation
  - Memory allocation stress testing under extreme conditions

* **Test Coverage Documentation** (`tests/TEST_COVERAGE.md`)
  - Comprehensive documentation of test scope and methodology
  - Edge case coverage matrix and validation criteria
  - Error handling assertion documentation
  - Performance benchmark specification and expected outcomes
  - Future improvement roadmap for test expansion

### Changed

* **Test Configuration** (`tests/Cargo.toml`)
  - Updated test registration to include new fuzz and error test modules
  - Enhanced benchmark configuration for pathological case coverage
  - Improved test organization for better module separation

* **Benchmark Infrastructure**
  - Extended criterion benchmark groups to include pathological scenarios
  - Added throughput measurement for stress testing scenarios
  - Enhanced memory usage validation during extreme input processing

### Improved

* **Error Assertion Strength**
  - All tests now include comprehensive token type validation
  - Enhanced span calculation accuracy verification
  - Graceful invalid input handling validation without panics
  - Memory safety verification under stress conditions

* **Test Organization**
  - Clear separation of concerns between unit, integration, fuzz, and performance tests
  - Improved test naming conventions for better discoverability
  - Enhanced documentation linking between test files and coverage reports

### Security

* **Input Validation Hardening**
  - Comprehensive validation of scanner behavior with malformed UTF-8 input
  - Null byte injection handling verification
  - Memory exhaustion prevention testing with extremely large inputs
  - Buffer overflow prevention validation with pathological token sequences

### Testing Infrastructure

* **Coverage Metrics Enhancement**
  - Normal token scanning validation ✅
  - Edge case input handling verification ✅
  - Error condition recovery testing ✅
  - Performance under stress validation ✅
  - Memory safety verification ✅
  - Unicode support validation ✅
  - Boundary condition testing ✅

This release significantly strengthens the reliability and robustness of the scanner through comprehensive test coverage expansion, ensuring stable operation under both normal and extreme conditions.

# 0.3.1 - 2025-08-01

* Fix issue [#735](https://github.com/jsinger67/parol/issues/735) of parol.

  After a newline character the column number is not 0 anymore. The newline logically belongs to the
  current line. Switching to the next line is then done with the next character.

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

<!-- markdownlint-disable first-line-h1 -->

[![Rust](https://github.com/jsinger67/scnr2/actions/workflows/rust.yml/badge.svg)](https://github.com/jsinger67/scnr2/actions/workflows/rust.yml)
[![Docs.rs](https://docs.rs/scnr2/badge.svg)](https://docs.rs/scnr2)
[![Crates.io](https://img.shields.io/crates/v/scnr2.svg)](https://crates.io/crates/scnr2)

<!-- markdownlint-enable first-line-h1 -->

# Attention
***This project is still in its early phases and not ready for use***

# About `scnr2`

This crate provides a scanner/lexer with sufficient regex support and minimal compile time.
The scanners support multiple scanner modes out of the box.

It is intended as the successor of [scnr](https://github.com/jsinger67/scnr).

In contrast to scnr this crate uses code generation during compile time by leveraging rust macros.

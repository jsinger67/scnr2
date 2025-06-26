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

The successor to scnr places greater emphasis on simplicity and speed. It relies on compile-time
code generation using Rust macros. The macro syntax used offers the possibility of defining
transitions between scanner modes in various ways. Specifically, there are three types of
transitions: set, push, and pop. Furthermore, as with scnr, you can define a positive or negative
lookahead for each terminal.

Additionally, scnr2 offers more support for regex features such as case insensitivity.

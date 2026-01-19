# scnr2

[![Rust](https://github.com/jsinger67/scnr2/actions/workflows/rust.yml/badge.svg)](https://github.com/jsinger67/scnr2/actions/workflows/rust.yml)
[![Docs.rs](https://docs.rs/scnr2/badge.svg)](https://docs.rs/scnr2)
[![Crates.io](https://img.shields.io/crates/v/scnr2.svg)](https://crates.io/crates/scnr2)

## Purpose & Overview

**scnr2** is a high-performance Rust crate for building custom scanners and lexers with advanced regular expression support, multi-mode state management, and compile-time code generation. Designed for simplicity, speed, and flexibility, scnr2 empowers developers to create robust tokenizers for complex parsing tasks with minimal runtime overhead.

## Key Advantages

- **Blazing Fast:** All scanner logic is generated at compile time using Rust macros, resulting in zero-cost abstractions and exceptional runtime performance.
- **Ergonomic Macro Syntax:** Define scanners, modes, tokens, and transitions with concise, readable macros—no boilerplate required.
- **Multi-Mode State Machines:** Effortlessly model complex lexing scenarios with built-in support for multiple scanner modes and seamless transitions (`set`, `push`, `pop`).
- **Context-Sensitive Tokenization:** Specify positive/negative lookahead conditions for tokens, enabling powerful context-aware parsing.
- **Full Unicode & Regex Support:** Broad compatibility with Unicode and advanced regex features, including case insensitivity.
- **Extensible & Contributor-Friendly:** Modular design and clear API make it easy to extend, customize, and contribute new features.
- **Python Support:** Official Python bindings are available via the [scnr2](https://pypi.org/project/scnr2/) package on PyPI (`pip install scnr2`). See `scnr2-python` for details.

## Unique Value Propositions

- **Compile-Time Safety:** Catch errors early and eliminate runtime surprises.
- **Minimal Dependencies:** Lightweight footprint for easy integration into any Rust project.
- **Production-Ready:** Proven reliability, actively maintained, and well-documented.

## Quickstart Example

```rust
use scnr2::scanner;

scanner! {
    MyScanner {
        mode INITIAL {
            token r"\d+" => 1; // Numbers
            token r"[a-zA-Z_][a-zA-Z0-9_]*" => 2; // Identifiers
        }
    }
}

fn main() {
    use my_scanner::MyScanner;
    let scanner = MyScanner::new();
    let input = "abc 123";
    for m in scanner.find_matches(input, 0) {
        println!("{}: '{}'", m.token_type, &input[m.span]);
    }
}
```

## Advanced Example: Multi-Mode Scanner

This example demonstrates a scanner with multiple modes and transitions, handling strings both inside and outside comments.

```rust
use scnr2::scanner;

scanner! {
    StringsInCommentsScanner {
        mode INITIAL {
            token r"\r\n|\r|\n" => 1; // "Newline"
            token r"[\s--\r\n]+" => 2; // "Whitespace other than newline"
            token r#"""# => 5; // "StringDelimiter"
            token r"/\*" => 6; // "CommentStart"
            token r"[a-zA-Z_][a-zA-Z0-9_]*" => 9; // "Identifier"
            on 5 push STRING;
            on 6 enter COMMENT;
        }
        mode STRING {
            token r#"""# => 5; // "StringDelimiter"
            token r#"([^"\\]|\\.)*"# => 10; // "StringContent"
            on 5 pop;
        }
        mode COMMENT {
            token r#"""# => 5; // "StringDelimiter"
            token r"\*/" => 7; // "CommentEnd"
            token r#"([^*"]|\*[^\/])*"# => 8; // "CommentText"
            on 5 push STRING;
            on 7 enter INITIAL;
        }
    }
}

const INPUT: &str = r#"Id
"Text with escaped End\""
/* Comment "String in Comment" and "String again" */"#;

fn main() {
    use strings_in_comments_scanner::StringsInCommentsScanner;
    let scanner = StringsInCommentsScanner::new();
    let tokens = scanner
        .find_matches_with_position(INPUT, 0)
        .collect::<Vec<_>>();

    println!("Tokens found: {}", tokens.len());
    for token in &tokens {
        println!(
            "{}: '{}'",
            token,
            INPUT[token.span.clone()].escape_default()
        );
    }
}
```

Sample output:
```
Tokens found: 17
[0..2] tok 9 at 1:1-1:3: 'Id'
[2..3] tok 1 at 2:0-2:1: '\n'
[3..4] tok 5 at 2:1-2:2: '\"'
[4..27] tok 10 at 2:2-2:25: 'Text with escaped End\\\"'
[27..28] tok 5 at 2:25-2:26: '\"'
[28..29] tok 1 at 3:0-3:1: '\n'
[29..31] tok 6 at 3:1-3:3: '/*'
[31..40] tok 8 at 3:3-3:12: ' Comment '
[40..41] tok 5 at 3:12-3:13: '\"'
[41..58] tok 10 at 3:13-3:30: 'String in Comment'
[58..59] tok 5 at 3:30-3:31: '\"'
[59..64] tok 8 at 3:31-3:36: ' and '
[64..65] tok 5 at 3:36-3:37: '\"'
[65..77] tok 10 at 3:37-3:49: 'String again'
[77..78] tok 5 at 3:49-3:50: '\"'
[78..79] tok 8 at 3:50-3:51: ' '
[79..81] tok 7 at 3:51-3:53: '*/'
```

## Advanced Features

- **Mode Switching:** Build nested, stateful scanners with `push`, `enter`, and `pop` transitions.
- **Lookahead:** Use `followed by` and `not followed by` for context-sensitive tokens.
- **Unicode:** Full Unicode support for internationalization.
- **Performance:** Compile-time generation ensures optimal speed.

## Getting Started

1. **Import the macro:** `use scnr2::scanner;`
2. **Define scanner modes and tokens:** Use macro syntax to specify modes, tokens, and transitions.
3. **Instantiate your scanner:** `let scanner = MyScanner::new();`
4. **Scan text:** `scanner.find_matches(input, 0)` yields an iterator of matches.
5. **Get position info:** `scanner.find_matches_with_position(input, 0)` provides line/column data.

## FAQ

**How do I skip whitespaces?**  
Define a token for whitespaces and ignore it, or simply omit a whitespace token—unmatched text is skipped.

**How do I use multiple scanner modes?**  
Define multiple `mode` blocks and use `push`, `enter`, or `pop` transitions.

**How do I detect unmatched input?**  
Add a catch-all token at the end of your mode's token list (e.g., `r"."`) and handle it as an error.

## Contributing

We welcome contributions! Whether you want to add features, improve documentation, or report issues, your input helps make scnr2 better for everyone.

For more examples and API details, see the [docs.rs documentation](https://docs.rs/scnr2).

Check out the [scnr2-python](file:///d:/Source/scnr2/scnr2-python/README.md) directory for details on using scnr2 from Python.

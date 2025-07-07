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

## How to use it

```rust
use scnr2::scanner;

scanner! {
    StringsInCommentsScanner {
        mode INITIAL {
            token r"\r\n|\r|\n" => 1; // "Newline"
            token r"[\s--\r\n]+" => 2; // "Whitespace"
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
            token r"([^\u{2a}\u{22}]|\u{2a}[^\u{2f}])*" => 8; // "CommentText"
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
This yields the following output:
```txt
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

The scanner definition above shows a rather complex scenario in which a string is accepted in both
the INITIAL and COMMENT states, i.e., strings are accepted both outside and inside comments.
This is achieved by switching between different scanner modes on certain terminal types.

### Define lookahead conditions on tokens

To specify lookahead conditions on tokens you can use the following macro syntax.

```rust
scanner! {
    TestScanner {
        mode INITIAL {
            token r"r"[a-zA-Z_][a-zA-Z0-9_]*"" followed by r"\(" => 1; // Function call
            token r">:" not followed by ":" => 2; // Operator x
        }
    }
}
```

Note the `followed by` and `not followed by` constraints.


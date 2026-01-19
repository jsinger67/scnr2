# scnr2 Python Bindings

Python bindings for the `scnr2` high-performance scanner generator crate, powered by PyO3.

## Installation

This package is currently available as a Rust workspace member. To build and install it in your local environment, use `maturin`:

```bash
cd scnr2-python
uv run maturin develop
```

## Basic Usage

The `scnr2` module allows you to define a scanner using a declarative syntax similar to the Rust `scanner!` macro.

```python
import scnr2

# Define your scanner
definition = """
MyScanner {
    mode INITIAL {
        token r"\d+" => 1;           # Match digits (Token Type 1)
        token r"[a-zA-Z_]\w*" => 2;  # Match identifiers (Token Type 2)
        token r"\s+" => 3;           # Match whitespace (Token Type 3)
    }
}
"""

scanner = scnr2.Scanner(definition)

# Scans the input and returns a list of matches
input_text = "123 abc_456"
matches = scanner.find_matches(input_text)

for m in matches:
    print(f"Token {m.token_type}: '{m.text}' at [{m.start}:{m.end}]")

# With position information (line, column)
matches_pos = scanner.find_matches_with_position(input_text)
for m in matches_pos:
    print(f"Token {m.token_type} at {m.start_line}:{m.start_column}")
```

## Advanced Usage

`scnr2` supports advanced features like **Lookahead** (positive and negative) to handle complex lexical structures.

### Lookahead Example

```python
import scnr2

definition = """
AdvancedScanner {
    mode INITIAL {
        # Match 'a' only if followed by 'b'
        token r"a" followed by r"b" => 1;
        
        # Match 'b' only if NOT followed by 'c'
        token r"b" not followed by r"c" => 2;
        
        token r"a" => 3;
        token r"b" => 4;
        token r"c" => 5;
    }
}
"""

scanner = scnr2.Scanner(definition)

# Scans for patterns with lookahead constraints
text = "abc"
matches = scanner.find_matches(text)

# Token 1 will match 'a' in 'ab'
# Token 4 will match 'b' (because it IS followed by 'c', so Token 2 fails)
# Token 5 will match 'c'
```

## Memory Management Note

The current version of the Python bindings uses `Box::leak` for runtime-generated DFA tables and matching logic to satisfy internal Rust lifetime requirements. While highly efficient for long-lived scanner instances, creating an extremely large number of unique `Scanner` objects in a single long-running process may result in memory growth. It is recommended to reuse `Scanner` instances whenever possible.

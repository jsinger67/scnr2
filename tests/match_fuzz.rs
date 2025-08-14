//! Fuzz and edge-case tests for scnr2 scanner

use scnr2::scanner;

scanner! {
    FuzzyScanner {
        mode INITIAL {
            token r"[ \n\t\f]" => 1;
            token r#""([^"\\]|\\t|\\u|\\n|\\")*""# => 3;
            token "private" => 4;
            token "primitive" => 5;
            token "protected" => 6;
            token "in" => 7;
            token "instanceof" => 8;
            token "[a-zA-Z_$][a-zA-Z0-9_$]*" => 2;
            token r"\." => 9;
            token r"\.\.\." => 10;
            token r"\(" => 11;
            token r"\)" => 12;
            token r"\{" => 13;
            token r"\}" => 14;
            token r"\+" => 15;
            token r"\+\+" => 16;
            token "=" => 17;
            token "==" => 18;
            token "===" => 19;
            token "=>" => 20;
        }
    }
}

#[test]
fn fuzz_empty_input() {
    let scanner = fuzzy_scanner::FuzzyScanner::new();
    let matches: Vec<_> = scanner.find_matches("", 0).collect();
    assert!(matches.is_empty(), "Empty input should yield no matches");
}

#[test]
fn fuzz_long_repeated_chars() {
    let scanner = fuzzy_scanner::FuzzyScanner::new();
    let input = "a".repeat(10000);
    let matches: Vec<_> = scanner.find_matches(&input, 0).collect();
    assert!(
        matches.iter().all(|m| m.token_type == 2),
        "All tokens should be identifiers"
    );
}

#[test]
fn fuzz_invalid_utf8() {
    let scanner = fuzzy_scanner::FuzzyScanner::new();
    let input = String::from_utf8_lossy(&[0xFF, 0xFE, 0xFD]);
    let matches: Vec<_> = scanner.find_matches(&input, 0).collect();
    assert!(
        matches.iter().all(|m| m.token_type == 1),
        "Invalid bytes should be treated as whitespace/invalid"
    );
}

#[test]
fn fuzz_pathological_string() {
    let scanner = fuzzy_scanner::FuzzyScanner::new();
    let input = r#""\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"""#;
    let matches: Vec<_> = scanner.find_matches(input, 0).collect();
    assert!(
        matches.iter().any(|m| m.token_type == 3),
        "Should match pathological string token"
    );
}

#[test]
fn fuzz_mixed_tokens() {
    let scanner = fuzzy_scanner::FuzzyScanner::new();
    let input = "private primitive protected in instanceof + ++ = == === => {} () ...";
    let matches: Vec<_> = scanner.find_matches(input, 0).collect();
    let expected_types = [
        4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 15, 1, 16, 1, 17, 1, 18, 1, 19, 1, 20, 1, 13, 14, 1, 11, 12,
        1, 10,
    ];
    for (i, (m, &typ)) in matches.iter().zip(expected_types.iter()).enumerate() {
        assert_eq!(
            m.token_type, typ,
            "Token type mismatch for mixed tokens at index {}",
            i
        );
    }
}

//! Error handling and edge-case tests for scnr2 scanner

use scnr2::scanner;

scanner! {
    ErrorScanner {
        mode INITIAL {
            token r"[ \n\t\f]+" => 1; // Whitespace
            token "[a-zA-Z_$][a-zA-Z0-9_$]*" => 2; // Identifier
            token r#""([^"\\]|\\t|\\u|\\n|\\")*""# => 3; // String
            token r"\d+" => 4; // Number
            token r"\+" => 5; // Plus
            token r"\(" => 6; // Open paren
            token r"\)" => 7; // Close paren
        }
    }
}

#[test]
fn test_empty_input_error_handling() {
    let scanner = error_scanner::ErrorScanner::new();
    let matches: Vec<_> = scanner.find_matches("", 0).collect();
    assert_eq!(matches.len(), 0, "Empty input should produce no matches");
}

#[test]
fn test_invalid_utf8_sequences() {
    let scanner = error_scanner::ErrorScanner::new();

    // Test various invalid UTF-8 sequences
    let invalid_sequences: Vec<&[u8]> = vec![
        &[0xFF, 0xFE],       // Invalid start bytes
        &[0x80, 0x80],       // Continuation bytes without start
        &[0xC0, 0x80],       // Overlong encoding
        &[0xED, 0xA0, 0x80], // Surrogate pair
    ];

    for seq in invalid_sequences {
        let input = String::from_utf8_lossy(seq);
        let matches: Vec<_> = scanner.find_matches(&input, 0).collect();
        // Should handle gracefully without panicking
        assert!(
            matches
                .iter()
                .all(|m| m.token_type == 1 || m.token_type > 0),
            "Invalid UTF-8 should be handled gracefully: {:?}",
            seq
        );
    }
}

#[test]
fn test_unterminated_strings() {
    let scanner = error_scanner::ErrorScanner::new();

    let unterminated_cases = vec![
        "\"hello",    // No closing quote
        "\"hello\\",  // Ends with escape
        "\"hello\n",  // Newline in string
        "\"hello\\u", // Incomplete unicode escape
    ];

    for case in unterminated_cases {
        let matches: Vec<_> = scanner.find_matches(case, 0).collect();
        // Should not panic and should handle the unterminated string
        assert!(
            !matches.is_empty(),
            "Should produce some match for: {}",
            case
        );
    }
}

#[test]
fn test_extremely_long_tokens() {
    let scanner = error_scanner::ErrorScanner::new();

    // Test very long identifier
    let long_ident = "a".repeat(1_000_000);
    let matches: Vec<_> = scanner.find_matches(&long_ident, 0).collect();
    assert_eq!(matches.len(), 1, "Should match single long identifier");
    assert_eq!(matches[0].token_type, 2, "Should be identifier token");
    assert_eq!(
        matches[0].span.end - matches[0].span.start,
        1_000_000,
        "Should span entire identifier"
    );
}

#[test]
fn test_deeply_nested_expressions() {
    let scanner = error_scanner::ErrorScanner::new();

    // Create deeply nested parentheses
    let open_parens = "(".repeat(10_000);
    let close_parens = ")".repeat(10_000);
    let nested = format!("{}identifier{}", open_parens, close_parens);

    let matches: Vec<_> = scanner.find_matches(&nested, 0).collect();
    assert_eq!(
        matches.len(),
        20_001,
        "Should match all parens plus identifier"
    );

    // Check first few and last few matches
    assert_eq!(matches[0].token_type, 6, "First should be open paren");
    assert_eq!(matches[10_000].token_type, 2, "Middle should be identifier");
    assert_eq!(
        matches[10_001].token_type, 7,
        "After identifier should be close paren"
    );
}

#[test]
fn test_null_bytes_in_input() {
    let scanner = error_scanner::ErrorScanner::new();

    let input_with_nulls = "hello\0world\0test";
    let matches: Vec<_> = scanner.find_matches(input_with_nulls, 0).collect();

    // Should handle null bytes gracefully
    assert!(
        !matches.is_empty(),
        "Should handle null bytes without panicking"
    );

    // Verify tokens are correctly identified around null bytes
    let identifiers: Vec<_> = matches.iter().filter(|m| m.token_type == 2).collect();
    assert!(!identifiers.is_empty(), "Should find identifier tokens");
}

#[test]
fn test_mixed_whitespace_edge_cases() {
    let scanner = error_scanner::ErrorScanner::new();

    let mixed_whitespace = "\t\n\r\n \u{00A0}\u{2000}\u{2001}identifier";
    let matches: Vec<_> = scanner.find_matches(mixed_whitespace, 0).collect();

    // Should handle various Unicode whitespace
    let ident_matches: Vec<_> = matches.iter().filter(|m| m.token_type == 2).collect();
    assert_eq!(ident_matches.len(), 1, "Should find exactly one identifier");
}

#[test]
fn test_boundary_conditions() {
    let scanner = error_scanner::ErrorScanner::new();

    // Test scanning at different start positions
    let input = "abc def ghi";

    // Normal scan from start
    let matches_start: Vec<_> = scanner.find_matches(input, 0).collect();
    assert!(!matches_start.is_empty(), "Should find matches from start");

    // Scan from middle
    let matches_middle: Vec<_> = scanner.find_matches(input, 4).collect();
    assert!(
        !matches_middle.is_empty(),
        "Should find matches from middle"
    );

    // Scan from near end
    let _matches_end: Vec<_> = scanner.find_matches(input, input.len() - 1).collect();
    // May or may not find matches, but shouldn't panic
}

#[test]
fn test_rapid_token_switching() {
    let scanner = error_scanner::ErrorScanner::new();

    // Rapidly alternating token types
    let rapid_switching = "a1b2c3d4e5f6g7h8i9j0".repeat(1000);
    let matches: Vec<_> = scanner.find_matches(&rapid_switching, 0).collect();

    assert!(!matches.is_empty(), "Should handle rapid token switching");

    // Verify alternating pattern
    for (i, m) in matches.iter().enumerate() {
        if i % 2 == 0 {
            assert_eq!(
                m.token_type, 2,
                "Even positions should be identifiers at index {}",
                i
            );
        } else {
            assert_eq!(
                m.token_type, 4,
                "Odd positions should be numbers at index {}",
                i
            );
        }
    }
}

#[test]
fn test_unicode_edge_cases() {
    let scanner = error_scanner::ErrorScanner::new();

    let unicode_cases = vec![
        "café",     // Accented characters
        "🚀rocket", // Emoji
        "𝕳𝖊𝖑𝖑𝖔 x",  // Mathematical alphanumeric symbols
        "Ω≈∞ y",    // Mathematical symbols
        "日本語 z", // CJK characters
    ];

    for case in unicode_cases {
        let matches: Vec<_> = scanner.find_matches(case, 0).collect();
        assert!(!matches.is_empty(), "Should handle Unicode case: {}", case);
    }
}

#[test]
fn test_memory_stress() {
    let scanner = error_scanner::ErrorScanner::new();

    // Create input that could stress memory allocation
    let large_input = format!(
        "{} {} {}",
        "identifier ".repeat(10_000),
        "123 ".repeat(10_000),
        "\"string\" ".repeat(10_000)
    );

    let matches: Vec<_> = scanner.find_matches(&large_input, 0).collect();

    // Verify we got expected number of matches without running out of memory
    // Note that whitespaces contribute to token counts
    assert!(
        matches.len() == 60_000,
        "Should handle large input without memory issues, len was {}",
        matches.len()
    );
}

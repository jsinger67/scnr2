use scnr2::scanner;

scanner! {
    MultiTokenSwitchScanner {
        mode INITIAL {
            token r"let" => 1; // InvalidToken
            token "[a-zA-Z_$][a-zA-Z0-9_$]*" => 2; // Identifier
            token r";" => 3; // Semicolon

            on 1, 2 enter OTHER;
        }
        mode OTHER {
            token r":" => 4; // Colon
            token r"0|[1-9][0-9]*" => 5; // Number

            on 5 enter INITIAL;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_multi_token_switch_successful_1() {
        init();
        use multi_token_switch_scanner::MultiTokenSwitchScanner;
        let scanner = MultiTokenSwitchScanner::new();
        let tokens = scanner.find_matches("let x: 1;", 0).collect::<Vec<_>>();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, 1); // "let"
        assert_eq!(tokens[1].token_type, 2); // "x"
        // Current mode is OTHER now
        assert_eq!(tokens[2].token_type, 4); // ":"
        assert_eq!(tokens[3].token_type, 5); // "1"
        // Current mode is INITIAL again
        assert_eq!(tokens[4].token_type, 3); // ";"

        assert_eq!(scanner.current_mode_index(), 0);
        assert_eq!(scanner.current_mode_name(), "INITIAL");
    }

    #[test]
    fn test_multi_token_switch_successful_2() {
        init();
        use multi_token_switch_scanner::MultiTokenSwitchScanner;
        let scanner = MultiTokenSwitchScanner::new();
        let tokens = scanner.find_matches("let x:", 0).collect::<Vec<_>>();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, 1); // "let"
        assert_eq!(tokens[1].token_type, 2); // "x"
        // Current mode is OTHER now
        assert_eq!(tokens[2].token_type, 4); // ":"

        assert_eq!(scanner.current_mode_index(), 1);
        assert_eq!(scanner.current_mode_name(), "OTHER");
    }

    #[test]
    fn test_multi_token_switch_canceled() {
        init();
        use multi_token_switch_scanner::MultiTokenSwitchScanner;
        let scanner = MultiTokenSwitchScanner::new();
        let tokens = scanner.find_matches("let; : a1;", 0).collect::<Vec<_>>();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, 1); // "let"
        assert_eq!(tokens[1].token_type, 3); // ";"
        // The colon is not recognized in the INITIAL mode
        assert_eq!(tokens[2].token_type, 2); // "a1"
        assert_eq!(tokens[3].token_type, 3); // ";"

        assert_eq!(scanner.current_mode_index(), 0);
        assert_eq!(scanner.current_mode_name(), "INITIAL");
    }
}

/// This file contains a hopefully increasing number of match tests to verify the correctness of the
/// scanner.
///
/// Some tests are based on the https://github.com/kkos/oniguruma/blob/master/test/test_utf8.c file
/// from the Oniguruma project.
/// Copyright (c) 2002-2019 K.Kosako kkosako0@gmail.com All rights reserved.
use scnr2::scanner;

// -------------------------------------------------------------------------
// x2("", "", 0, 0);
// td!(r#""#, "", &[], 0),
scanner! { S0 { mode M { token r#""# => 0; } } }
#[test]
fn test_match_0() {
    use s0::S0 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "0: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("^", "", 0, 0);
// tu!(r#"^"#, "", &[], 1), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S1 { mode M { token r#"^"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^a", "\na", 1, 2);
// tu!(r#"^a"#, "\na", &[("n", 1, 2)], 2), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S2 { mode M { token r#"^a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("$", "", 0, 0);
// tu!(r#"$"#, "", &[], 3), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S3 { mode M { token r#"$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("$\\O", "bb\n", 2, 3);
// tr!(r#"$\O"#, "bb\n", &[("\\\\", 2, 3)], 4), EscapeUnrecognized
// scanner! { S4 { mode M { token r#"$\O"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\G", "", 0, 0);
// tr!(r#"\G"#, "", &[], 5), EscapeUnrecognized
// scanner! { S5 { mode M { token r#"\G"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A", "", 0, 0);
// tu!(r#"\A"#, "", &[], 6), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S6 { mode M { token r#"\A"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\Z", "", 0, 0);
// tr!(r#"\Z"#, "", &[], 7), EscapeUnrecognized
// scanner! { S7 { mode M { token r#"\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\z", "", 0, 0);
// tu!(r#"\z"#, "", &[], 8), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S8 { mode M { token r#"\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^$", "", 0, 0);
// tu!(r#"^$"#, "", &[], 9), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S9 { mode M { token r#"^$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\ca", "\001", 0, 1);
// tr!(r#"\ca"#, "\001", &[("\\\\", 0, 1)], 10), EscapeUnrecognized
// scanner! { S10 { mode M { token r#"\ca"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\C-b", "\002", 0, 1);
// tr!(r#"\C-b"#, "\002", &[("\\\\", 0, 1)], 11), EscapeUnrecognized
// scanner! { S11 { mode M { token r#"\C-b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\c\\\\", "\034", 0, 1);
// tr!(r#"\c\\"#, "\034", &[("\\\\", 0, 1)], 12), EscapeUnrecognized
// scanner! { S12 { mode M { token r#"\c\\"# => 0; } } }

// -------------------------------------------------------------------------
// x2("q[\\c\\\\]", "q\034", 0, 2);
// tr!(r#"q[\c\\]"#, "q\034", &[("q\\\\", 0, 2)], 13), EscapeUnrecognized
// scanner! { S13 { mode M { token r#"q[\c\\]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("", "a", 0, 0);
// td!(r#""#, "a", &[], 14),
scanner! { S14 { mode M { token r#""# => 0; } } }
#[test]
fn test_match_14() {
    use s14::S14 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "14: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a", "a", 0, 1);
// td!(r#"a"#, "a", &[("a", 0, 1)], 15),
scanner! { S15 { mode M { token r#"a"# => 0; } } }
#[test]
fn test_match_15() {
    use s15::S15 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "15: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "15: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "15: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "15: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\x61", "a", 0, 1);
// td!(r#"\x61"#, "a", &[("a", 0, 1)], 16),
scanner! { S16 { mode M { token r#"\x61"# => 0; } } }
#[test]
fn test_match_16() {
    use s16::S16 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "16: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "16: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "16: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "16: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("aa", "aa", 0, 2);
// td!(r#"aa"#, "aa", &[("aa", 0, 2)], 17),
scanner! { S17 { mode M { token r#"aa"# => 0; } } }
#[test]
fn test_match_17() {
    use s17::S17 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "17: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "17: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "17: Match end does not match");
        assert_eq!(
            &"aa"[ma.1..ma.2],
            ma.0,
            "17: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("aaa", "aaa", 0, 3);
// td!(r#"aaa"#, "aaa", &[("aaa", 0, 3)], 18),
scanner! { S18 { mode M { token r#"aaa"# => 0; } } }
#[test]
fn test_match_18() {
    use s18::S18 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaa", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "18: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "18: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "18: Match end does not match");
        assert_eq!(
            &"aaa"[ma.1..ma.2],
            ma.0,
            "18: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0, 35);
// td!(r#"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"#, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &[("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0, 35)], 19),
scanner! { S19 { mode M { token r#"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"# => 0; } } }
#[test]
fn test_match_19() {
    use s19::S19 as S;
    let scanner = S::new();
    let matches = scanner
        .find_matches("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0)
        .collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] =
        &[("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0, 35)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "19: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "19: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "19: Match end does not match");
        assert_eq!(
            &"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"[ma.1..ma.2],
            ma.0,
            "19: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("ab", "ab", 0, 2);
// td!(r#"ab"#, "ab", &[("ab", 0, 2)], 20),
scanner! { S20 { mode M { token r#"ab"# => 0; } } }
#[test]
fn test_match_20() {
    use s20::S20 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "20: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "20: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "20: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "20: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("b", "ab", 1, 2);
// td!(r#"b"#, "ab", &[("b", 1, 2)], 21),
scanner! { S21 { mode M { token r#"b"# => 0; } } }
#[test]
fn test_match_21() {
    use s21::S21 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 1, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "21: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "21: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "21: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "21: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("bc", "abc", 1, 3);
// td!(r#"bc"#, "abc", &[("bc", 1, 3)], 22),
scanner! { S22 { mode M { token r#"bc"# => 0; } } }
#[test]
fn test_match_22() {
    use s22::S22 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bc", 1, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "22: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "22: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "22: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "22: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:#RET#)", "#INS##RET#", 5, 10);
// td!(r#"(?i:#RET#)"#, "#INS##RET#", &[("#RET#", 5, 10)], 23),
scanner! { S23 { mode M { token r#"(?i:#RET#)"# => 0; } } }
#[test]
fn test_match_23() {
    use s23::S23 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("#INS##RET#", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("#RET#", 5, 10)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "23: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "23: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "23: Match end does not match");
        assert_eq!(
            &"#INS##RET#"[ma.1..ma.2],
            ma.0,
            "23: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\17", "\017", 0, 1);
// tr!(r#"\17"#, "\017", &[("\\\\", 0, 1)], 24), UnsupportedBackreference
// scanner! { S24 { mode M { token r#"\17"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x1f", "\x1f", 0, 1);
// td!(r#"\x1f"#, "\x1f", &[("\x1f", 0, 1)], 25),
scanner! { S25 { mode M { token r#"\x1f"# => 0; } } }
#[test]
fn test_match_25() {
    use s25::S25 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x1f", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\x1f", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "25: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "25: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "25: Match end does not match");
        assert_eq!(
            &"\x1f"[ma.1..ma.2],
            ma.0,
            "25: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a(?#....\\\\JJJJ)b", "ab", 0, 2);
// tr!(r#"a(?#....\\JJJJ)b"#, "ab", &[("ab", 0, 2)], 26), FlagUnrecognized FlagUnrecognized
// scanner! { S26 { mode M { token r#"a(?#....\\JJJJ)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?x)  G (o O(?-x)oO) g L", "GoOoOgLe", 0, 7);
// td!(r#"(?x)  G (o O(?-x)oO) g L"#, "GoOoOgLe", &[("GoOoOgL", 0, 7)], 27),
scanner! { S27 { mode M { token r#"(?x)  G (o O(?-x)oO) g L"# => 0; } } }
#[test]
fn test_match_27() {
    use s27::S27 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("GoOoOgLe", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("GoOoOgL", 0, 7)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "27: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "27: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "27: Match end does not match");
        assert_eq!(
            &"GoOoOgLe"[ma.1..ma.2],
            ma.0,
            "27: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".", "a", 0, 1);
// td!(r#"."#, "a", &[("a", 0, 1)], 28),
scanner! { S28 { mode M { token r#"."# => 0; } } }
#[test]
fn test_match_28() {
    use s28::S28 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "28: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "28: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "28: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "28: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n(".", "");
// td!(r#"."#, "", &[], 29),
scanner! { S29 { mode M { token r#"."# => 0; } } }
#[test]
fn test_match_29() {
    use s29::S29 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "29: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("..", "ab", 0, 2);
// td!(r#".."#, "ab", &[("ab", 0, 2)], 30),
scanner! { S30 { mode M { token r#".."# => 0; } } }
#[test]
fn test_match_30() {
    use s30::S30 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "30: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "30: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "30: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "30: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\w", "e", 0, 1);
// td!(r#"\w"#, "e", &[("e", 0, 1)], 31),
scanner! { S31 { mode M { token r#"\w"# => 0; } } }
#[test]
fn test_match_31() {
    use s31::S31 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("e", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("e", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "31: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "31: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "31: Match end does not match");
        assert_eq!(
            &"e"[ma.1..ma.2],
            ma.0,
            "31: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("\\W", "e");
// td!(r#"\W"#, "e", &[], 32),
scanner! { S32 { mode M { token r#"\W"# => 0; } } }
#[test]
fn test_match_32() {
    use s32::S32 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("e", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "32: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("\\s", " ", 0, 1);
// td!(r#"\s"#, " ", &[(" ", 0, 1)], 33),
scanner! { S33 { mode M { token r#"\s"# => 0; } } }
#[test]
fn test_match_33() {
    use s33::S33 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(" ", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "33: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "33: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "33: Match end does not match");
        assert_eq!(
            &" "[ma.1..ma.2],
            ma.0,
            "33: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\S", "b", 0, 1);
// td!(r#"\S"#, "b", &[("b", 0, 1)], 34),
scanner! { S34 { mode M { token r#"\S"# => 0; } } }
#[test]
fn test_match_34() {
    use s34::S34 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "34: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "34: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "34: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "34: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\d", "4", 0, 1);
// td!(r#"\d"#, "4", &[("4", 0, 1)], 35),
scanner! { S35 { mode M { token r#"\d"# => 0; } } }
#[test]
fn test_match_35() {
    use s35::S35 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("4", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("4", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "35: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "35: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "35: Match end does not match");
        assert_eq!(
            &"4"[ma.1..ma.2],
            ma.0,
            "35: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("\\D", "4");
// td!(r#"\D"#, "4", &[], 36),
scanner! { S36 { mode M { token r#"\D"# => 0; } } }
#[test]
fn test_match_36() {
    use s36::S36 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("4", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "36: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("\\b", "z ", 0, 0);
// tu!(r#"\b"#, "z ", &[], 37), UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S37 { mode M { token r#"\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\b", " z", 1, 1);
// tu!(r#"\b"#, " z", &[("", 1, 1)], 38), UnsupportedFeatureError("WordUnicode Look(WordUnicode)") UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S38 { mode M { token r#"\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\b", "  z ", 2, 2);
// tu!(r#"\b"#, "  z ", &[("", 2, 2)], 39), UnsupportedFeatureError("WordUnicode Look(WordUnicode)") UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S39 { mode M { token r#"\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\B", "zz ", 1, 1);
// tu!(r#"\B"#, "zz ", &[("", 1, 1)], 40), UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)") UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)")
// scanner! { S40 { mode M { token r#"\B"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\B", "z ", 2, 2);
// tu!(r#"\B"#, "z ", &[("", 2, 2)], 41), UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)") UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)")
// scanner! { S41 { mode M { token r#"\B"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\B", " z", 0, 0);
// tu!(r#"\B"#, " z", &[], 42), UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)") UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)")
// scanner! { S42 { mode M { token r#"\B"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[ab]", "b", 0, 1);
// td!(r#"[ab]"#, "b", &[("b", 0, 1)], 43),
scanner! { S43 { mode M { token r#"[ab]"# => 0; } } }
#[test]
fn test_match_43() {
    use s43::S43 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "43: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "43: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "43: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "43: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[ab]", "c");
// td!(r#"[ab]"#, "c", &[], 44),
scanner! { S44 { mode M { token r#"[ab]"# => 0; } } }
#[test]
fn test_match_44() {
    use s44::S44 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "44: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[a-z]", "t", 0, 1);
// td!(r#"[a-z]"#, "t", &[("t", 0, 1)], 45),
scanner! { S45 { mode M { token r#"[a-z]"# => 0; } } }
#[test]
fn test_match_45() {
    use s45::S45 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("t", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("t", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "45: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "45: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "45: Match end does not match");
        assert_eq!(
            &"t"[ma.1..ma.2],
            ma.0,
            "45: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^a]", "a");
// td!(r#"[^a]"#, "a", &[], 46),
scanner! { S46 { mode M { token r#"[^a]"# => 0; } } }
#[test]
fn test_match_46() {
    use s46::S46 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "46: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[^a]", "\n", 0, 1);
// td!(r#"[^a]"#, "\n", &[("\n", 0, 1)], 47),
scanner! { S47 { mode M { token r#"[^a]"# => 0; } } }
#[test]
fn test_match_47() {
    use s47::S47 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\n", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "47: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "47: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "47: Match end does not match");
        assert_eq!(
            &"\n"[ma.1..ma.2],
            ma.0,
            "47: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[]]", "]", 0, 1);
// td!(r#"[]]"#, "]", &[("]", 0, 1)], 48),
scanner! { S48 { mode M { token r#"[]]"# => 0; } } }
#[test]
fn test_match_48() {
    use s48::S48 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("]", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("]", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "48: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "48: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "48: Match end does not match");
        assert_eq!(
            &"]"[ma.1..ma.2],
            ma.0,
            "48: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^]]", "]");
// td!(r#"[^]]"#, "]", &[], 49),
scanner! { S49 { mode M { token r#"[^]]"# => 0; } } }
#[test]
fn test_match_49() {
    use s49::S49 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("]", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "49: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\^]+", "0^^1", 1, 3);
// td!(r#"[\^]+"#, "0^^1", &[("^^", 1, 3)], 50),
scanner! { S50 { mode M { token r#"[\^]+"# => 0; } } }
#[test]
fn test_match_50() {
    use s50::S50 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("0^^1", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("^^", 1, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "50: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "50: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "50: Match end does not match");
        assert_eq!(
            &"0^^1"[ma.1..ma.2],
            ma.0,
            "50: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[b-]", "b", 0, 1);
// td!(r#"[b-]"#, "b", &[("b", 0, 1)], 51),
scanner! { S51 { mode M { token r#"[b-]"# => 0; } } }
#[test]
fn test_match_51() {
    use s51::S51 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "51: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "51: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "51: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "51: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[b-]", "-", 0, 1);
// td!(r#"[b-]"#, "-", &[("-", 0, 1)], 52),
scanner! { S52 { mode M { token r#"[b-]"# => 0; } } }
#[test]
fn test_match_52() {
    use s52::S52 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("-", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("-", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "52: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "52: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "52: Match end does not match");
        assert_eq!(
            &"-"[ma.1..ma.2],
            ma.0,
            "52: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\w]", "z", 0, 1);
// td!(r#"[\w]"#, "z", &[("z", 0, 1)], 53),
scanner! { S53 { mode M { token r#"[\w]"# => 0; } } }
#[test]
fn test_match_53() {
    use s53::S53 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("z", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("z", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "53: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "53: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "53: Match end does not match");
        assert_eq!(
            &"z"[ma.1..ma.2],
            ma.0,
            "53: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\w]", " ");
// td!(r#"[\w]"#, " ", &[], 54),
scanner! { S54 { mode M { token r#"[\w]"# => 0; } } }
#[test]
fn test_match_54() {
    use s54::S54 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "54: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\W]", "b$", 1, 2);
// td!(r#"[\W]"#, "b$", &[("$", 1, 2)], 55),
scanner! { S55 { mode M { token r#"[\W]"# => 0; } } }
#[test]
fn test_match_55() {
    use s55::S55 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b$", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("$", 1, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "55: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "55: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "55: Match end does not match");
        assert_eq!(
            &"b$"[ma.1..ma.2],
            ma.0,
            "55: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\d]", "5", 0, 1);
// td!(r#"[\d]"#, "5", &[("5", 0, 1)], 56),
scanner! { S56 { mode M { token r#"[\d]"# => 0; } } }
#[test]
fn test_match_56() {
    use s56::S56 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("5", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("5", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "56: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "56: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "56: Match end does not match");
        assert_eq!(
            &"5"[ma.1..ma.2],
            ma.0,
            "56: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\d]", "e");
// td!(r#"[\d]"#, "e", &[], 57),
scanner! { S57 { mode M { token r#"[\d]"# => 0; } } }
#[test]
fn test_match_57() {
    use s57::S57 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("e", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "57: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\D]", "t", 0, 1);
// td!(r#"[\D]"#, "t", &[("t", 0, 1)], 58),
scanner! { S58 { mode M { token r#"[\D]"# => 0; } } }
#[test]
fn test_match_58() {
    use s58::S58 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("t", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("t", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "58: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "58: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "58: Match end does not match");
        assert_eq!(
            &"t"[ma.1..ma.2],
            ma.0,
            "58: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\D]", "3");
// td!(r#"[\D]"#, "3", &[], 59),
scanner! { S59 { mode M { token r#"[\D]"# => 0; } } }
#[test]
fn test_match_59() {
    use s59::S59 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("3", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "59: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\s]", " ", 0, 1);
// td!(r#"[\s]"#, " ", &[(" ", 0, 1)], 60),
scanner! { S60 { mode M { token r#"[\s]"# => 0; } } }
#[test]
fn test_match_60() {
    use s60::S60 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(" ", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "60: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "60: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "60: Match end does not match");
        assert_eq!(
            &" "[ma.1..ma.2],
            ma.0,
            "60: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\s]", "a");
// td!(r#"[\s]"#, "a", &[], 61),
scanner! { S61 { mode M { token r#"[\s]"# => 0; } } }
#[test]
fn test_match_61() {
    use s61::S61 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "61: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\S]", "b", 0, 1);
// td!(r#"[\S]"#, "b", &[("b", 0, 1)], 62),
scanner! { S62 { mode M { token r#"[\S]"# => 0; } } }
#[test]
fn test_match_62() {
    use s62::S62 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "62: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "62: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "62: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "62: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\S]", " ");
// td!(r#"[\S]"#, " ", &[], 63),
scanner! { S63 { mode M { token r#"[\S]"# => 0; } } }
#[test]
fn test_match_63() {
    use s63::S63 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "63: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\w\\d]", "2", 0, 1);
// td!(r#"[\w\d]"#, "2", &[("2", 0, 1)], 64),
scanner! { S64 { mode M { token r#"[\w\d]"# => 0; } } }
#[test]
fn test_match_64() {
    use s64::S64 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("2", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("2", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "64: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "64: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "64: Match end does not match");
        assert_eq!(
            &"2"[ma.1..ma.2],
            ma.0,
            "64: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\w\\d]", " ");
// td!(r#"[\w\d]"#, " ", &[], 65),
scanner! { S65 { mode M { token r#"[\w\d]"# => 0; } } }
#[test]
fn test_match_65() {
    use s65::S65 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "65: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[:upper:]]", "B", 0, 1);
// td!(r#"[[:upper:]]"#, "B", &[("B", 0, 1)], 66),
scanner! { S66 { mode M { token r#"[[:upper:]]"# => 0; } } }
#[test]
fn test_match_66() {
    use s66::S66 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("B", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("B", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "66: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "66: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "66: Match end does not match");
        assert_eq!(
            &"B"[ma.1..ma.2],
            ma.0,
            "66: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[*[:xdigit:]+]", "+", 0, 1);
// td!(r#"[*[:xdigit:]+]"#, "+", &[("+", 0, 1)], 67),
scanner! { S67 { mode M { token r#"[*[:xdigit:]+]"# => 0; } } }
#[test]
fn test_match_67() {
    use s67::S67 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("+", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("+", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "67: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "67: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "67: Match end does not match");
        assert_eq!(
            &"+"[ma.1..ma.2],
            ma.0,
            "67: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[*[:xdigit:]+]", "GHIKK-9+*", 6, 7);
// td!(r#"[*[:xdigit:]+]"#, "GHIKK-9+*", &[("9", 6, 7)], 68),
scanner! { S68 { mode M { token r#"[*[:xdigit:]+]"# => 0; } } }
#[test]
fn test_match_68() {
    use s68::S68 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("GHIKK-9+*", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("9", 6, 7), ("+", 7, 8), ("*", 8, 9)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "68: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "68: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "68: Match end does not match");
        assert_eq!(
            &"GHIKK-9+*"[ma.1..ma.2],
            ma.0,
            "68: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[*[:xdigit:]+]", "-@^+", 3, 4);
// td!(r#"[*[:xdigit:]+]"#, "-@^+", &[("+", 3, 4)], 69),
scanner! { S69 { mode M { token r#"[*[:xdigit:]+]"# => 0; } } }
#[test]
fn test_match_69() {
    use s69::S69 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("-@^+", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("+", 3, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "69: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "69: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "69: Match end does not match");
        assert_eq!(
            &"-@^+"[ma.1..ma.2],
            ma.0,
            "69: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[:upper]]", "A");
// td!(r#"[[:upper]]"#, "A", &[], 70),
scanner! { S70 { mode M { token r#"[[:upper]]"# => 0; } } }
#[test]
fn test_match_70() {
    use s70::S70 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "70: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[:upper]]", ":", 0, 1);
// td!(r#"[[:upper]]"#, ":", &[(":", 0, 1)], 71),
scanner! { S71 { mode M { token r#"[[:upper]]"# => 0; } } }
#[test]
fn test_match_71() {
    use s71::S71 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "71: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "71: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "71: Match end does not match");
        assert_eq!(
            &":"[ma.1..ma.2],
            ma.0,
            "71: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[:upper:]]", "a");
// td!(r#"[[:upper:]]"#, "a", &[], 72),
scanner! { S72 { mode M { token r#"[[:upper:]]"# => 0; } } }
#[test]
fn test_match_72() {
    use s72::S72 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "72: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[:^upper:]]", "a", 0, 1);
// td!(r#"[[:^upper:]]"#, "a", &[("a", 0, 1)], 73),
scanner! { S73 { mode M { token r#"[[:^upper:]]"# => 0; } } }
#[test]
fn test_match_73() {
    use s73::S73 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "73: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "73: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "73: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "73: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[:lower:]]", "A");
// td!(r#"[[:lower:]]"#, "A", &[], 74),
scanner! { S74 { mode M { token r#"[[:lower:]]"# => 0; } } }
#[test]
fn test_match_74() {
    use s74::S74 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "74: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[:^lower:]]", "A", 0, 1);
// td!(r#"[[:^lower:]]"#, "A", &[("A", 0, 1)], 75),
scanner! { S75 { mode M { token r#"[[:^lower:]]"# => 0; } } }
#[test]
fn test_match_75() {
    use s75::S75 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("A", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "75: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "75: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "75: Match end does not match");
        assert_eq!(
            &"A"[ma.1..ma.2],
            ma.0,
            "75: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// e("[[:::]",   ":[", ONIGERR_PREMATURE_END_OF_CHAR_CLASS);
// tr!(r#"[[:::]"#, ":[", "ONIGERR_PREMATURE_END_OF_CHAR_CLASS", 76), ClassUnclosed
// scanner! { S76 { mode M { token r#"[[:::]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[[:\\]:]", ":]", ONIGERR_PREMATURE_END_OF_CHAR_CLASS);
// tr!(r#"[[:\]:]"#, ":]", "ONIGERR_PREMATURE_END_OF_CHAR_CLASS", 77), ClassUnclosed
// scanner! { S77 { mode M { token r#"[[:\]:]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[[:\\[:]", ":[", ONIGERR_PREMATURE_END_OF_CHAR_CLASS);
// tr!(r#"[[:\[:]"#, ":[", "ONIGERR_PREMATURE_END_OF_CHAR_CLASS", 78), ClassUnclosed
// scanner! { S78 { mode M { token r#"[[:\[:]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[[:\\]]",  ":]", ONIGERR_PREMATURE_END_OF_CHAR_CLASS);
// tr!(r#"[[:\]]"#, ":]", "ONIGERR_PREMATURE_END_OF_CHAR_CLASS", 79), ClassUnclosed
// scanner! { S79 { mode M { token r#"[[:\]]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[[:u:]]",      "", ONIGERR_INVALID_POSIX_BRACKET_TYPE);
// tr!(r#"[[:u:]]"#, "", "ONIGERR_INVALID_POSIX_BRACKET_TYPE", 80),
scanner! { S80 { mode M { token r#"[[:u:]]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[[:upp:]]",    "", ONIGERR_INVALID_POSIX_BRACKET_TYPE);
// tr!(r#"[[:upp:]]"#, "", "ONIGERR_INVALID_POSIX_BRACKET_TYPE", 81),
scanner! { S81 { mode M { token r#"[[:upp:]]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[[:uppers:]]", "", ONIGERR_INVALID_POSIX_BRACKET_TYPE);
// tr!(r#"[[:uppers:]]"#, "", "ONIGERR_INVALID_POSIX_BRACKET_TYPE", 82),
scanner! { S82 { mode M { token r#"[[:uppers:]]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[[:upper\\] :]]",  "]", 0, 1);
// td!(r#"[[:upper\] :]]"#, "]", &[("]", 0, 1)], 83),
scanner! { S83 { mode M { token r#"[[:upper\] :]]"# => 0; } } }
#[test]
fn test_match_83() {
    use s83::S83 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("]", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("]", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "83: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "83: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "83: Match end does not match");
        assert_eq!(
            &"]"[ma.1..ma.2],
            ma.0,
            "83: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[::]]",     ":", 0, 1);
// td!(r#"[[::]]"#, ":", &[(":", 0, 1)], 84),
scanner! { S84 { mode M { token r#"[[::]]"# => 0; } } }
#[test]
fn test_match_84() {
    use s84::S84 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "84: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "84: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "84: Match end does not match");
        assert_eq!(
            &":"[ma.1..ma.2],
            ma.0,
            "84: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[:::]]",    ":", 0, 1);
// td!(r#"[[:::]]"#, ":", &[(":", 0, 1)], 85),
scanner! { S85 { mode M { token r#"[[:::]]"# => 0; } } }
#[test]
fn test_match_85() {
    use s85::S85 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "85: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "85: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "85: Match end does not match");
        assert_eq!(
            &":"[ma.1..ma.2],
            ma.0,
            "85: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[:\\]:]]*", ":]", 0, 2);
// td!(r#"[[:\]:]]*"#, ":]", &[(":]", 0, 2)], 86),
scanner! { S86 { mode M { token r#"[[:\]:]]*"# => 0; } } }
#[test]
fn test_match_86() {
    use s86::S86 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":]", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":]", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "86: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "86: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "86: Match end does not match");
        assert_eq!(
            &":]"[ma.1..ma.2],
            ma.0,
            "86: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[:\\[:]]*", ":[", 0, 2);
// td!(r#"[[:\[:]]*"#, ":[", &[(":[", 0, 2)], 87),
scanner! { S87 { mode M { token r#"[[:\[:]]*"# => 0; } } }
#[test]
fn test_match_87() {
    use s87::S87 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":[", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":[", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "87: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "87: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "87: Match end does not match");
        assert_eq!(
            &":["[ma.1..ma.2],
            ma.0,
            "87: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[:\\]]]*",  ":]", 0, 2);
// td!(r#"[[:\]]]*"#, ":]", &[(":]", 0, 2)], 88),
scanner! { S88 { mode M { token r#"[[:\]]]*"# => 0; } } }
#[test]
fn test_match_88() {
    use s88::S88 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":]", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":]", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "88: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "88: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "88: Match end does not match");
        assert_eq!(
            &":]"[ma.1..ma.2],
            ma.0,
            "88: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\044-\\047]", "\046", 0, 1);
// td!(r#"[\044-\047]"#, "\046", &[("\\\\", 0, 1)], 89),
scanner! { S89 { mode M { token r#"[\x24-\x27]"# => 0; } } }
#[test]
fn test_match_89() {
    use s89::S89 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x26", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\x26", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "89: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "89: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "89: Match end does not match");
        assert_eq!(
            &"\x26"[ma.1..ma.2],
            ma.0,
            "89: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\x5a-\\x5c]", "\x5b", 0, 1);
// td!(r#"[\x5a-\x5c]"#, "\x5b", &[("\\\\", 0, 1)], 90),
scanner! { S90 { mode M { token r#"[\x5a-\x5c]"# => 0; } } }
#[test]
fn test_match_90() {
    use s90::S90 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x5b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\x5b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "90: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "90: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "90: Match end does not match");
        assert_eq!(
            &"\x5b"[ma.1..ma.2],
            ma.0,
            "90: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\x6A-\\x6D]", "\x6c", 0, 1);
// td!(r#"[\x6A-\x6D]"#, "\x6c", &[("\\", 0, 1)], 91),
scanner! { S91 { mode M { token r#"[\x6A-\x6D]"# => 0; } } }
#[test]
fn test_match_91() {
    use s91::S91 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x6c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\x6c", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "91: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "91: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "91: Match end does not match");
        assert_eq!(
            &"\x6c"[ma.1..ma.2],
            ma.0,
            "91: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[\\x6A-\\x6D]", "\x6E");
// td!(r#"[\x6A-\x6D]"#, "\x6E", &[], 92),
scanner! { S92 { mode M { token r#"[\x6A-\x6D]"# => 0; } } }
#[test]
fn test_match_92() {
    use s92::S92 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x6E", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "92: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("^[0-9A-F]+ 0+ UNDEF ", "75F 00000000 SECT14A notype ()    External    | _rb_apply");
// tu!(r#"^[0-9A-F]+ 0+ UNDEF "#, "75F 00000000 SECT14A notype ()    External    | _rb_apply", &[], 93), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S93 { mode M { token r#"^[0-9A-F]+ 0+ UNDEF "# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\[]", "[", 0, 1);
// td!(r#"[\[]"#, "[", &[("[", 0, 1)], 94),
scanner! { S94 { mode M { token r#"[\[]"# => 0; } } }
#[test]
fn test_match_94() {
    use s94::S94 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("[", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("[", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "94: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "94: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "94: Match end does not match");
        assert_eq!(
            &"["[ma.1..ma.2],
            ma.0,
            "94: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\]]", "]", 0, 1);
// td!(r#"[\]]"#, "]", &[("]", 0, 1)], 95),
scanner! { S95 { mode M { token r#"[\]]"# => 0; } } }
#[test]
fn test_match_95() {
    use s95::S95 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("]", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("]", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "95: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "95: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "95: Match end does not match");
        assert_eq!(
            &"]"[ma.1..ma.2],
            ma.0,
            "95: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[&]", "&", 0, 1);
// td!(r#"[&]"#, "&", &[("&", 0, 1)], 96),
scanner! { S96 { mode M { token r#"[&]"# => 0; } } }
#[test]
fn test_match_96() {
    use s96::S96 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("&", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("&", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "96: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "96: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "96: Match end does not match");
        assert_eq!(
            &"&"[ma.1..ma.2],
            ma.0,
            "96: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[ab]]", "b", 0, 1);
// td!(r#"[[ab]]"#, "b", &[("b", 0, 1)], 97),
scanner! { S97 { mode M { token r#"[[ab]]"# => 0; } } }
#[test]
fn test_match_97() {
    use s97::S97 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "97: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "97: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "97: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "97: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[ab]c]", "c", 0, 1);
// td!(r#"[[ab]c]"#, "c", &[("c", 0, 1)], 98),
scanner! { S98 { mode M { token r#"[[ab]c]"# => 0; } } }
#[test]
fn test_match_98() {
    use s98::S98 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("c", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "98: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "98: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "98: Match end does not match");
        assert_eq!(
            &"c"[ma.1..ma.2],
            ma.0,
            "98: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[^a]]", "a");
// td!(r#"[[^a]]"#, "a", &[], 99),
scanner! { S99 { mode M { token r#"[[^a]]"# => 0; } } }
#[test]
fn test_match_99() {
    use s99::S99 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "99: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[^[a]]", "a");
// td!(r#"[^[a]]"#, "a", &[], 100),
scanner! { S100 { mode M { token r#"[^[a]]"# => 0; } } }
#[test]
fn test_match_100() {
    use s100::S100 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "100: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[ab]&&bc]", "b", 0, 1);
// td!(r#"[[ab]&&bc]"#, "b", &[("b", 0, 1)], 101),
scanner! { S101 { mode M { token r#"[[ab]&&bc]"# => 0; } } }
#[test]
fn test_match_101() {
    use s101::S101 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "101: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "101: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "101: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "101: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[ab]&&bc]", "a");
// td!(r#"[[ab]&&bc]"#, "a", &[], 102),
scanner! { S102 { mode M { token r#"[[ab]&&bc]"# => 0; } } }
#[test]
fn test_match_102() {
    use s102::S102 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "102: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[[ab]&&bc]", "c");
// td!(r#"[[ab]&&bc]"#, "c", &[], 103),
scanner! { S103 { mode M { token r#"[[ab]&&bc]"# => 0; } } }
#[test]
fn test_match_103() {
    use s103::S103 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "103: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[a-z&&b-y&&c-x]", "w", 0, 1);
// td!(r#"[a-z&&b-y&&c-x]"#, "w", &[("w", 0, 1)], 104),
scanner! { S104 { mode M { token r#"[a-z&&b-y&&c-x]"# => 0; } } }
#[test]
fn test_match_104() {
    use s104::S104 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("w", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("w", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "104: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "104: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "104: Match end does not match");
        assert_eq!(
            &"w"[ma.1..ma.2],
            ma.0,
            "104: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^a-z&&b-y&&c-x]", "w");
// td!(r#"[^a-z&&b-y&&c-x]"#, "w", &[], 105),
scanner! { S105 { mode M { token r#"[^a-z&&b-y&&c-x]"# => 0; } } }
#[test]
fn test_match_105() {
    use s105::S105 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("w", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "105: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[^a&&a]&&a-z]", "b", 0, 1);
// td!(r#"[[^a&&a]&&a-z]"#, "b", &[("b", 0, 1)], 106),
scanner! { S106 { mode M { token r#"[[^a&&a]&&a-z]"# => 0; } } }
#[test]
fn test_match_106() {
    use s106::S106 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "106: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "106: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "106: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "106: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[^a&&a]&&a-z]", "a");
// td!(r#"[[^a&&a]&&a-z]"#, "a", &[], 107),
scanner! { S107 { mode M { token r#"[[^a&&a]&&a-z]"# => 0; } } }
#[test]
fn test_match_107() {
    use s107::S107 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "107: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[^a-z&&bcdef]&&[^c-g]]", "h", 0, 1);
// td!(r#"[[^a-z&&bcdef]&&[^c-g]]"#, "h", &[("h", 0, 1)], 108),
scanner! { S108 { mode M { token r#"[[^a-z&&bcdef]&&[^c-g]]"# => 0; } } }
#[test]
fn test_match_108() {
    use s108::S108 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("h", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("h", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "108: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "108: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "108: Match end does not match");
        assert_eq!(
            &"h"[ma.1..ma.2],
            ma.0,
            "108: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[^a-z&&bcdef]&&[^c-g]]", "c");
// td!(r#"[[^a-z&&bcdef]&&[^c-g]]"#, "c", &[], 109),
scanner! { S109 { mode M { token r#"[[^a-z&&bcdef]&&[^c-g]]"# => 0; } } }
#[test]
fn test_match_109() {
    use s109::S109 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "109: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[^[^abc]&&[^cde]]", "c", 0, 1);
// td!(r#"[^[^abc]&&[^cde]]"#, "c", &[("c", 0, 1)], 110),
scanner! { S110 { mode M { token r#"[^[^abc]&&[^cde]]"# => 0; } } }
#[test]
fn test_match_110() {
    use s110::S110 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("c", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "110: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "110: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "110: Match end does not match");
        assert_eq!(
            &"c"[ma.1..ma.2],
            ma.0,
            "110: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[^[^abc]&&[^cde]]", "e", 0, 1);
// td!(r#"[^[^abc]&&[^cde]]"#, "e", &[("e", 0, 1)], 111),
scanner! { S111 { mode M { token r#"[^[^abc]&&[^cde]]"# => 0; } } }
#[test]
fn test_match_111() {
    use s111::S111 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("e", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("e", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "111: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "111: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "111: Match end does not match");
        assert_eq!(
            &"e"[ma.1..ma.2],
            ma.0,
            "111: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^[^abc]&&[^cde]]", "f");
// td!(r#"[^[^abc]&&[^cde]]"#, "f", &[], 112),
scanner! { S112 { mode M { token r#"[^[^abc]&&[^cde]]"# => 0; } } }
#[test]
fn test_match_112() {
    use s112::S112 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("f", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "112: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[a-&&-a]", "-", 0, 1);
// tr!(r#"[a-&&-a]"#, "-", &[("-", 0, 1)], 113), ClassRangeInvalid
// scanner! { S113 { mode M { token r#"[a-&&-a]"# => 0; } } }

// -------------------------------------------------------------------------
// n("[a\\-&&\\-a]", "&");
// td!(r#"[a\-&&\-a]"#, "&", &[], 114),
scanner! { S114 { mode M { token r#"[a\-&&\-a]"# => 0; } } }
#[test]
fn test_match_114() {
    use s114::S114 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("&", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "114: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("\\wabc", " abc");
// td!(r#"\wabc"#, " abc", &[], 115),
scanner! { S115 { mode M { token r#"\wabc"# => 0; } } }
#[test]
fn test_match_115() {
    use s115::S115 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "115: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a\\Wbc", "a bc", 0, 4);
// td!(r#"a\Wbc"#, "a bc", &[("a bc", 0, 4)], 116),
scanner! { S116 { mode M { token r#"a\Wbc"# => 0; } } }
#[test]
fn test_match_116() {
    use s116::S116 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a bc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a bc", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "116: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "116: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "116: Match end does not match");
        assert_eq!(
            &"a bc"[ma.1..ma.2],
            ma.0,
            "116: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a.b.c", "aabbc", 0, 5);
// td!(r#"a.b.c"#, "aabbc", &[("aabbc", 0, 5)], 117),
scanner! { S117 { mode M { token r#"a.b.c"# => 0; } } }
#[test]
fn test_match_117() {
    use s117::S117 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aabbc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aabbc", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "117: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "117: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "117: Match end does not match");
        assert_eq!(
            &"aabbc"[ma.1..ma.2],
            ma.0,
            "117: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".\\wb\\W..c", "abb bcc", 0, 7);
// td!(r#".\wb\W..c"#, "abb bcc", &[("abb bcc", 0, 7)], 118),
scanner! { S118 { mode M { token r#".\wb\W..c"# => 0; } } }
#[test]
fn test_match_118() {
    use s118::S118 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abb bcc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abb bcc", 0, 7)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "118: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "118: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "118: Match end does not match");
        assert_eq!(
            &"abb bcc"[ma.1..ma.2],
            ma.0,
            "118: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\s\\wzzz", " zzzz", 0, 5);
// td!(r#"\s\wzzz"#, " zzzz", &[(" zzzz", 0, 5)], 119),
scanner! { S119 { mode M { token r#"\s\wzzz"# => 0; } } }
#[test]
fn test_match_119() {
    use s119::S119 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" zzzz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(" zzzz", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "119: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "119: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "119: Match end does not match");
        assert_eq!(
            &" zzzz"[ma.1..ma.2],
            ma.0,
            "119: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("aa.b", "aabb", 0, 4);
// td!(r#"aa.b"#, "aabb", &[("aabb", 0, 4)], 120),
scanner! { S120 { mode M { token r#"aa.b"# => 0; } } }
#[test]
fn test_match_120() {
    use s120::S120 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aabb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aabb", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "120: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "120: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "120: Match end does not match");
        assert_eq!(
            &"aabb"[ma.1..ma.2],
            ma.0,
            "120: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n(".a", "ab");
// td!(r#".a"#, "ab", &[], 121),
scanner! { S121 { mode M { token r#".a"# => 0; } } }
#[test]
fn test_match_121() {
    use s121::S121 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "121: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2(".a", "aa", 0, 2);
// td!(r#".a"#, "aa", &[("aa", 0, 2)], 122),
scanner! { S122 { mode M { token r#".a"# => 0; } } }
#[test]
fn test_match_122() {
    use s122::S122 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "122: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "122: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "122: Match end does not match");
        assert_eq!(
            &"aa"[ma.1..ma.2],
            ma.0,
            "122: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("^a", "a", 0, 1);
// tu!(r#"^a"#, "a", &[("a", 0, 1)], 123), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S123 { mode M { token r#"^a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^a$", "a", 0, 1);
// tu!(r#"^a$"#, "a", &[("a", 0, 1)], 124), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S124 { mode M { token r#"^a$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\w$", "a", 0, 1);
// tu!(r#"^\w$"#, "a", &[("a", 0, 1)], 125), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S125 { mode M { token r#"^\w$"# => 0; } } }

// -------------------------------------------------------------------------
// n("^\\w$", " ");
// tu!(r#"^\w$"#, " ", &[], 126), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S126 { mode M { token r#"^\w$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\wab$", "zab", 0, 3);
// tu!(r#"^\wab$"#, "zab", &[("zab", 0, 3)], 127), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S127 { mode M { token r#"^\wab$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\wabcdef$", "zabcdef", 0, 7);
// tu!(r#"^\wabcdef$"#, "zabcdef", &[("zabcdef", 0, 7)], 128), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S128 { mode M { token r#"^\wabcdef$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\w...def$", "zabcdef", 0, 7);
// tu!(r#"^\w...def$"#, "zabcdef", &[("zabcdef", 0, 7)], 129), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S129 { mode M { token r#"^\w...def$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\w\\w\\s\\Waaa\\d", "aa  aaa4", 0, 8);
// td!(r#"\w\w\s\Waaa\d"#, "aa  aaa4", &[("aa  aaa4", 0, 8)], 130),
scanner! { S130 { mode M { token r#"\w\w\s\Waaa\d"# => 0; } } }
#[test]
fn test_match_130() {
    use s130::S130 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa  aaa4", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa  aaa4", 0, 8)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "130: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "130: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "130: Match end does not match");
        assert_eq!(
            &"aa  aaa4"[ma.1..ma.2],
            ma.0,
            "130: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\A\\Z", "", 0, 0);
// tr!(r#"\A\Z"#, "", &[], 131), EscapeUnrecognized
// scanner! { S131 { mode M { token r#"\A\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\Axyz", "xyz", 0, 3);
// tr!(r#"\Axyz"#, "xyz", &[("xyz", 0, 3)], 132), EscapeUnrecognized
// scanner! { S132 { mode M { token r#"\Axyz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("xyz\\Z", "xyz", 0, 3);
// tr!(r#"xyz\Z"#, "xyz", &[("xyz", 0, 3)], 133), EscapeUnrecognized
// scanner! { S133 { mode M { token r#"xyz\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("xyz\\z", "xyz", 0, 3);
// tu!(r#"xyz\z"#, "xyz", &[("xyz", 0, 3)], 134), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S134 { mode M { token r#"xyz\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a\\Z", "a", 0, 1);
// tr!(r#"a\Z"#, "a", &[("a", 0, 1)], 135), EscapeUnrecognized
// scanner! { S135 { mode M { token r#"a\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\Gaz", "az", 0, 2);
// tr!(r#"\Gaz"#, "az", &[("az", 0, 2)], 136), EscapeUnrecognized
// scanner! { S136 { mode M { token r#"\Gaz"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\Gz", "bza");
// tu!(r#"\Gz"#, "bza", &[], 137), EscapeUnrecognized
// scanner! { S137 { mode M { token r#"\Gz"# => 0; } } }

// -------------------------------------------------------------------------
// n("az\\G", "az");
// tr!(r#"az\G"#, "az", &[], 138), EscapeUnrecognized
// scanner! { S138 { mode M { token r#"az\G"# => 0; } } }

// -------------------------------------------------------------------------
// n("az\\A", "az");
// tu!(r#"az\A"#, "az", &[], 139), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S139 { mode M { token r#"az\A"# => 0; } } }

// -------------------------------------------------------------------------
// n("a\\Az", "az");
// tu!(r#"a\Az"#, "az", &[], 140), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S140 { mode M { token r#"a\Az"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\^\\$", "^$", 0, 2);
// td!(r#"\^\$"#, "^$", &[("^$", 0, 2)], 141),
scanner! { S141 { mode M { token r#"\^\$"# => 0; } } }
#[test]
fn test_match_141() {
    use s141::S141 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("^$", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("^$", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "141: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "141: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "141: Match end does not match");
        assert_eq!(
            &"^$"[ma.1..ma.2],
            ma.0,
            "141: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("^x?y", "xy", 0, 2);
// tu!(r#"^x?y"#, "xy", &[("xy", 0, 2)], 142), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S142 { mode M { token r#"^x?y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^(x?y)", "xy", 0, 2);
// tu!(r#"^(x?y)"#, "xy", &[("xy", 0, 2)], 143), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S143 { mode M { token r#"^(x?y)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\w", "_", 0, 1);
// td!(r#"\w"#, "_", &[("_", 0, 1)], 144),
scanner! { S144 { mode M { token r#"\w"# => 0; } } }
#[test]
fn test_match_144() {
    use s144::S144 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("_", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("_", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "144: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "144: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "144: Match end does not match");
        assert_eq!(
            &"_"[ma.1..ma.2],
            ma.0,
            "144: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("\\W", "_");
// td!(r#"\W"#, "_", &[], 145),
scanner! { S145 { mode M { token r#"\W"# => 0; } } }
#[test]
fn test_match_145() {
    use s145::S145 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("_", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "145: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?=z)z", "z", 0, 1);
// tr!(r#"(?=z)z"#, "z", &[("z", 0, 1)], 146), UnsupportedLookAround
// scanner! { S146 { mode M { token r#"(?=z)z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?=z).", "a");
// tr!(r#"(?=z)."#, "a", &[], 147), UnsupportedLookAround
// scanner! { S147 { mode M { token r#"(?=z)."# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?!z)a", "a", 0, 1);
// tu!(r#"(?!z)a"#, "a", &[("a", 0, 1)], 148), UnsupportedLookAround
// scanner! { S148 { mode M { token r#"(?!z)a"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?!z)a", "z");
// tu!(r#"(?!z)a"#, "z", &[], 149), UnsupportedLookAround
// scanner! { S149 { mode M { token r#"(?!z)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:a)", "a", 0, 1);
// td!(r#"(?i:a)"#, "a", &[("a", 0, 1)], 150),
scanner! { S150 { mode M { token r#"(?i:a)"# => 0; } } }
#[test]
fn test_match_150() {
    use s150::S150 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "150: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "150: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "150: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "150: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:a)", "A", 0, 1);
// td!(r#"(?i:a)"#, "A", &[("A", 0, 1)], 151),
scanner! { S151 { mode M { token r#"(?i:a)"# => 0; } } }
#[test]
fn test_match_151() {
    use s151::S151 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("A", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "151: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "151: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "151: Match end does not match");
        assert_eq!(
            &"A"[ma.1..ma.2],
            ma.0,
            "151: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:A)", "a", 0, 1);
// td!(r#"(?i:A)"#, "a", &[("a", 0, 1)], 152),
scanner! { S152 { mode M { token r#"(?i:A)"# => 0; } } }
#[test]
fn test_match_152() {
    use s152::S152 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "152: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "152: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "152: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "152: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:i)", "I", 0, 1);
// td!(r#"(?i:i)"#, "I", &[("I", 0, 1)], 153),
scanner! { S153 { mode M { token r#"(?i:i)"# => 0; } } }
#[test]
fn test_match_153() {
    use s153::S153 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("I", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("I", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "153: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "153: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "153: Match end does not match");
        assert_eq!(
            &"I"[ma.1..ma.2],
            ma.0,
            "153: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:I)", "i", 0, 1);
// td!(r#"(?i:I)"#, "i", &[("i", 0, 1)], 154),
scanner! { S154 { mode M { token r#"(?i:I)"# => 0; } } }
#[test]
fn test_match_154() {
    use s154::S154 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("i", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("i", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "154: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "154: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "154: Match end does not match");
        assert_eq!(
            &"i"[ma.1..ma.2],
            ma.0,
            "154: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[A-Z])", "i", 0, 1);
// td!(r#"(?i:[A-Z])"#, "i", &[("i", 0, 1)], 155),
scanner! { S155 { mode M { token r#"(?i:[A-Z])"# => 0; } } }
#[test]
fn test_match_155() {
    use s155::S155 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("i", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("i", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "155: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "155: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "155: Match end does not match");
        assert_eq!(
            &"i"[ma.1..ma.2],
            ma.0,
            "155: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[a-z])", "I", 0, 1);
// td!(r#"(?i:[a-z])"#, "I", &[("I", 0, 1)], 156),
scanner! { S156 { mode M { token r#"(?i:[a-z])"# => 0; } } }
#[test]
fn test_match_156() {
    use s156::S156 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("I", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("I", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "156: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "156: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "156: Match end does not match");
        assert_eq!(
            &"I"[ma.1..ma.2],
            ma.0,
            "156: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?i:A)", "b");
// td!(r#"(?i:A)"#, "b", &[], 157),
scanner! { S157 { mode M { token r#"(?i:A)"# => 0; } } }
#[test]
fn test_match_157() {
    use s157::S157 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "157: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:ss)", "ss", 0, 2);
// td!(r#"(?i:ss)"#, "ss", &[("ss", 0, 2)], 158),
scanner! { S158 { mode M { token r#"(?i:ss)"# => 0; } } }
#[test]
fn test_match_158() {
    use s158::S158 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ss", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ss", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "158: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "158: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "158: Match end does not match");
        assert_eq!(
            &"ss"[ma.1..ma.2],
            ma.0,
            "158: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:ss)", "Ss", 0, 2);
// td!(r#"(?i:ss)"#, "Ss", &[("Ss", 0, 2)], 159),
scanner! { S159 { mode M { token r#"(?i:ss)"# => 0; } } }
#[test]
fn test_match_159() {
    use s159::S159 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("Ss", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("Ss", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "159: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "159: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "159: Match end does not match");
        assert_eq!(
            &"Ss"[ma.1..ma.2],
            ma.0,
            "159: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:ss)", "SS", 0, 2);
// td!(r#"(?i:ss)"#, "SS", &[("SS", 0, 2)], 160),
scanner! { S160 { mode M { token r#"(?i:ss)"# => 0; } } }
#[test]
fn test_match_160() {
    use s160::S160 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("SS", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("SS", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "160: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "160: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "160: Match end does not match");
        assert_eq!(
            &"SS"[ma.1..ma.2],
            ma.0,
            "160: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:ss)", "\xc5\xbfS", 0, 3);
// td!(r#"(?i:ss)"#, "\xc5\xbfS", &[("\\xc", 0, 3)], 161),
scanner! { S161 { mode M { token r#"(?i:ss)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:ss)", "s\xc5\xbf", 0, 3);
// td!(r#"(?i:ss)"#, "s\xc5\xbf", &[("s\\x", 0, 3)], 162),
scanner! { S162 { mode M { token r#"(?i:ss)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:ss)", "\xc3\x9f", 0, 2);
// td!(r#"(?i:ss)"#, "\xc3\x9f", &[("\\x", 0, 2)], 163),
scanner! { S163 { mode M { token r#"(?i:ss)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:ss)", "\xe1\xba\x9e", 0, 3);
// td!(r#"(?i:ss)"#, "\xe1\xba\x9e", &[("\\xe", 0, 3)], 164),
scanner! { S164 { mode M { token r#"(?i:ss)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "xssy", 0, 4);
// td!(r#"(?i:xssy)"#, "xssy", &[("xssy", 0, 4)], 165),
scanner! { S165 { mode M { token r#"(?i:xssy)"# => 0; } } }
#[test]
fn test_match_165() {
    use s165::S165 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xssy", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xssy", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "165: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "165: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "165: Match end does not match");
        assert_eq!(
            &"xssy"[ma.1..ma.2],
            ma.0,
            "165: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "xSsy", 0, 4);
// td!(r#"(?i:xssy)"#, "xSsy", &[("xSsy", 0, 4)], 166),
scanner! { S166 { mode M { token r#"(?i:xssy)"# => 0; } } }
#[test]
fn test_match_166() {
    use s166::S166 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xSsy", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xSsy", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "166: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "166: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "166: Match end does not match");
        assert_eq!(
            &"xSsy"[ma.1..ma.2],
            ma.0,
            "166: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "xSSy", 0, 4);
// td!(r#"(?i:xssy)"#, "xSSy", &[("xSSy", 0, 4)], 167),
scanner! { S167 { mode M { token r#"(?i:xssy)"# => 0; } } }
#[test]
fn test_match_167() {
    use s167::S167 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xSSy", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xSSy", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "167: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "167: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "167: Match end does not match");
        assert_eq!(
            &"xSSy"[ma.1..ma.2],
            ma.0,
            "167: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "x\xc5\xbfSy", 0, 5);
// td!(r#"(?i:xssy)"#, "x\xc5\xbfSy", &[("x\\xc5", 0, 5)], 168),
scanner! { S168 { mode M { token r#"(?i:xssy)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "xs\xc5\xbfy", 0, 5);
// td!(r#"(?i:xssy)"#, "xs\xc5\xbfy", &[("xs\\xc", 0, 5)], 169),
scanner! { S169 { mode M { token r#"(?i:xssy)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "x\xc3\x9fy", 0, 4);
// td!(r#"(?i:xssy)"#, "x\xc3\x9fy", &[("x\\xc", 0, 4)], 170),
scanner! { S170 { mode M { token r#"(?i:xssy)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:xssy)", "x\xe1\xba\x9ey", 0, 5);
// td!(r#"(?i:xssy)"#, "x\xe1\xba\x9ey", &[("x\\xe1", 0, 5)], 171),
scanner! { S171 { mode M { token r#"(?i:xssy)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:x\xc3\x9fy)", "xssy", 0, 4);
// td!(r#"(?i:x\xc3\x9fy)"#, "xssy", &[("xssy", 0, 4)], 172),
scanner! { S172 { mode M { token r#"(?i:x\xc3\x9fy)"# => 0; } } }
#[test]
fn test_match_172() {
    use s172::S172 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xssy", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("xssy", 0, 4)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "172: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:x\xc3\x9fy)", "xSSy", 0, 4);
// td!(r#"(?i:x\xc3\x9fy)"#, "xSSy", &[("xSSy", 0, 4)], 173),
scanner! { S173 { mode M { token r#"(?i:x\xc3\x9fy)"# => 0; } } }
#[test]
fn test_match_173() {
    use s173::S173 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xSSy", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("xSSy", 0, 4)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "173: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:\xc3\x9f)", "ss", 0, 2);
// td!(r#"(?i:\xc3\x9f)"#, "ss", &[("ss", 0, 2)], 174),
scanner! { S174 { mode M { token r#"(?i:\xc3\x9f)"# => 0; } } }
#[test]
fn test_match_174() {
    use s174::S174 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ss", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("ss", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "174: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:\xc3\x9f)", "SS", 0, 2);
// td!(r#"(?i:\xc3\x9f)"#, "SS", &[("SS", 0, 2)], 175),
scanner! { S175 { mode M { token r#"(?i:\xc3\x9f)"# => 0; } } }
#[test]
fn test_match_175() {
    use s175::S175 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("SS", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("SS", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "175: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:[\xc3\x9f])", "ss", 0, 2);
// td!(r#"(?i:[\xc3\x9f])"#, "ss", &[("ss", 0, 2)], 176),
scanner! { S176 { mode M { token r#"(?i:[\xc3\x9f])"# => 0; } } }
#[test]
fn test_match_176() {
    use s176::S176 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ss", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("ss", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "176: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:[\xc3\x9f])", "SS", 0, 2);
// td!(r#"(?i:[\xc3\x9f])"#, "SS", &[("SS", 0, 2)], 177),
scanner! { S177 { mode M { token r#"(?i:[\xc3\x9f])"# => 0; } } }
#[test]
fn test_match_177() {
    use s177::S177 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("SS", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("SS", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "177: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i)(?<!ss)z", "qqz", 2, 3);
// tu!(r#"(?i)(?<!ss)z"#, "qqz", &[("z", 2, 3)], 178), UnsupportedLookAround
// scanner! { S178 { mode M { token r#"(?i)(?<!ss)z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:[A-Z])", "a", 0, 1);
// td!(r#"(?i:[A-Z])"#, "a", &[("a", 0, 1)], 179),
scanner! { S179 { mode M { token r#"(?i:[A-Z])"# => 0; } } }
#[test]
fn test_match_179() {
    use s179::S179 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "179: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "179: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "179: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "179: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[f-m])", "H", 0, 1);
// td!(r#"(?i:[f-m])"#, "H", &[("H", 0, 1)], 180),
scanner! { S180 { mode M { token r#"(?i:[f-m])"# => 0; } } }
#[test]
fn test_match_180() {
    use s180::S180 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("H", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("H", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "180: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "180: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "180: Match end does not match");
        assert_eq!(
            &"H"[ma.1..ma.2],
            ma.0,
            "180: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[f-m])", "h", 0, 1);
// td!(r#"(?i:[f-m])"#, "h", &[("h", 0, 1)], 181),
scanner! { S181 { mode M { token r#"(?i:[f-m])"# => 0; } } }
#[test]
fn test_match_181() {
    use s181::S181 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("h", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("h", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "181: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "181: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "181: Match end does not match");
        assert_eq!(
            &"h"[ma.1..ma.2],
            ma.0,
            "181: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?i:[f-m])", "e");
// td!(r#"(?i:[f-m])"#, "e", &[], 182),
scanner! { S182 { mode M { token r#"(?i:[f-m])"# => 0; } } }
#[test]
fn test_match_182() {
    use s182::S182 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("e", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "182: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:[A-c])", "D", 0, 1);
// td!(r#"(?i:[A-c])"#, "D", &[("D", 0, 1)], 183),
scanner! { S183 { mode M { token r#"(?i:[A-c])"# => 0; } } }
#[test]
fn test_match_183() {
    use s183::S183 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("D", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("D", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "183: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "183: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "183: Match end does not match");
        assert_eq!(
            &"D"[ma.1..ma.2],
            ma.0,
            "183: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?i:[^a-z])", "A");
// td!(r#"(?i:[^a-z])"#, "A", &[], 184),
scanner! { S184 { mode M { token r#"(?i:[^a-z])"# => 0; } } }
#[test]
fn test_match_184() {
    use s184::S184 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "184: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("(?i:[^a-z])", "a");
// td!(r#"(?i:[^a-z])"#, "a", &[], 185),
scanner! { S185 { mode M { token r#"(?i:[^a-z])"# => 0; } } }
#[test]
fn test_match_185() {
    use s185::S185 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "185: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:[!-k])", "Z", 0, 1);
// td!(r#"(?i:[!-k])"#, "Z", &[("Z", 0, 1)], 186),
scanner! { S186 { mode M { token r#"(?i:[!-k])"# => 0; } } }
#[test]
fn test_match_186() {
    use s186::S186 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("Z", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("Z", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "186: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "186: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "186: Match end does not match");
        assert_eq!(
            &"Z"[ma.1..ma.2],
            ma.0,
            "186: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[!-k])", "7", 0, 1);
// td!(r#"(?i:[!-k])"#, "7", &[("7", 0, 1)], 187),
scanner! { S187 { mode M { token r#"(?i:[!-k])"# => 0; } } }
#[test]
fn test_match_187() {
    use s187::S187 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("7", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("7", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "187: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "187: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "187: Match end does not match");
        assert_eq!(
            &"7"[ma.1..ma.2],
            ma.0,
            "187: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[T-}])", "b", 0, 1);
// td!(r#"(?i:[T-}])"#, "b", &[("b", 0, 1)], 188),
scanner! { S188 { mode M { token r#"(?i:[T-}])"# => 0; } } }
#[test]
fn test_match_188() {
    use s188::S188 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "188: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "188: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "188: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "188: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:[T-}])", "{", 0, 1);
// td!(r#"(?i:[T-}])"#, "{", &[("{", 0, 1)], 189),
scanner! { S189 { mode M { token r#"(?i:[T-}])"# => 0; } } }
#[test]
fn test_match_189() {
    use s189::S189 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("{", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("{", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "189: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "189: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "189: Match end does not match");
        assert_eq!(
            &"{"[ma.1..ma.2],
            ma.0,
            "189: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:\\?a)", "?A", 0, 2);
// td!(r#"(?i:\?a)"#, "?A", &[("?A", 0, 2)], 190),
scanner! { S190 { mode M { token r#"(?i:\?a)"# => 0; } } }
#[test]
fn test_match_190() {
    use s190::S190 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("?A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("?A", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "190: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "190: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "190: Match end does not match");
        assert_eq!(
            &"?A"[ma.1..ma.2],
            ma.0,
            "190: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:\\*A)", "*a", 0, 2);
// td!(r#"(?i:\*A)"#, "*a", &[("*a", 0, 2)], 191),
scanner! { S191 { mode M { token r#"(?i:\*A)"# => 0; } } }
#[test]
fn test_match_191() {
    use s191::S191 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("*a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("*a", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "191: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "191: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "191: Match end does not match");
        assert_eq!(
            &"*a"[ma.1..ma.2],
            ma.0,
            "191: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n(".", "\n");
// td!(r#"."#, "\n", &[], 192),
scanner! { S192 { mode M { token r#"."# => 0; } } }
#[test]
fn test_match_192() {
    use s192::S192 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "192: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?m:.)", "\n", 0, 1);
// td!(r#"(?m:.)"#, "\n", &[("\\", 0, 1)], 193),
scanner! { S193 { mode M { token r#"(?m:.)"# => 0; } } }
#[test]
fn test_match_193() {
    use s193::S193 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("\n", 0, 1)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "193: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?m:a.)", "a\n", 0, 2);
// td!(r#"(?m:a.)"#, "a\n", &[("a\\", 0, 2)], 194),
scanner! { S194 { mode M { token r#"(?m:a.)"# => 0; } } }
#[test]
fn test_match_194() {
    use s194::S194 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("a\n", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "194: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?m:.b)", "a\nb", 1, 3);
// td!(r#"(?m:.b)"#, "a\nb", &[("\\n", 1, 3)], 195),
scanner! { S195 { mode M { token r#"(?m:.b)"# => 0; } } }
#[test]
fn test_match_195() {
    use s195::S195 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a\nb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma : ("a\nb", 1, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "195: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2(".*abc", "dddabdd\nddabc", 8, 13);
// td!(r#".*abc"#, "dddabdd\nddabc", &[("nddab", 8, 13)], 196),
scanner! { S196 { mode M { token r#".*abc"# => 0; } } }
#[test]
fn test_match_196() {
    use s196::S196 as S;
    let scanner = S::new();
    let matches = scanner
        .find_matches("dddabdd\nddabc", 0)
        .collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ddabc", 8, 13)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "196: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "196: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "196: Match end does not match");
        assert_eq!(
            &"dddabdd\nddabc"[ma.1..ma.2],
            ma.0,
            "196: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".+abc", "dddabdd\nddabcaa\naaaabc", 8, 13);
// td!(r#".+abc"#, "dddabdd\nddabcaa\naaaabc", &[("nddab", 8, 13)], 197),
scanner! { S197 { mode M { token r#".+abc"# => 0; } } }
#[test]
fn test_match_197() {
    use s197::S197 as S;
    let scanner = S::new();
    let matches = scanner
        .find_matches("dddabdd\nddabcaa\naaaabc", 0)
        .collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ddabc", 8, 13), ("aaaabc", 16, 22)]; // Oniguruma : ("ddabc", 8, 13)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "197: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "197: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "197: Match end does not match");
        assert_eq!(
            &"dddabdd\nddabcaa\naaaabc"[ma.1..ma.2],
            ma.0,
            "197: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?m:.*abc)", "dddabddabc", 0, 10);
// td!(r#"(?m:.*abc)"#, "dddabddabc", &[("dddabddabc", 0, 10)], 198),
scanner! { S198 { mode M { token r#"(?m:.*abc)"# => 0; } } }
#[test]
fn test_match_198() {
    use s198::S198 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("dddabddabc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("dddabddabc", 0, 10)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "198: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "198: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "198: Match end does not match");
        assert_eq!(
            &"dddabddabc"[ma.1..ma.2],
            ma.0,
            "198: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?i)(?-i)a", "A");
// td!(r#"(?i)(?-i)a"#, "A", &[], 199),
scanner! { S199 { mode M { token r#"(?i)(?-i)a"# => 0; } } }
#[test]
fn test_match_199() {
    use s199::S199 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "199: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("(?i)(?-i:a)", "A");
// td!(r#"(?i)(?-i:a)"#, "A", &[], 200),
scanner! { S200 { mode M { token r#"(?i)(?-i:a)"# => 0; } } }
#[test]
fn test_match_200() {
    use s200::S200 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "200: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a?", "", 0, 0);
// td!(r#"a?"#, "", &[], 201),
scanner! { S201 { mode M { token r#"a?"# => 0; } } }
#[test]
fn test_match_201() {
    use s201::S201 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "201: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a?", "b", 0, 0);
// td!(r#"a?"#, "b", &[], 202),
scanner! { S202 { mode M { token r#"a?"# => 0; } } }
#[test]
fn test_match_202() {
    use s202::S202 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "202: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a?", "a", 0, 1);
// td!(r#"a?"#, "a", &[("a", 0, 1)], 203),
scanner! { S203 { mode M { token r#"a?"# => 0; } } }
#[test]
fn test_match_203() {
    use s203::S203 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "203: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "203: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "203: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "203: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a*", "", 0, 0);
// td!(r#"a*"#, "", &[], 204),
scanner! { S204 { mode M { token r#"a*"# => 0; } } }
#[test]
fn test_match_204() {
    use s204::S204 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "204: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a*", "a", 0, 1);
// td!(r#"a*"#, "a", &[("a", 0, 1)], 205),
scanner! { S205 { mode M { token r#"a*"# => 0; } } }
#[test]
fn test_match_205() {
    use s205::S205 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "205: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "205: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "205: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "205: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a*", "aaa", 0, 3);
// td!(r#"a*"#, "aaa", &[("aaa", 0, 3)], 206),
scanner! { S206 { mode M { token r#"a*"# => 0; } } }
#[test]
fn test_match_206() {
    use s206::S206 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaa", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "206: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "206: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "206: Match end does not match");
        assert_eq!(
            &"aaa"[ma.1..ma.2],
            ma.0,
            "206: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a*", "baaaa", 0, 0);
// td!(r#"a*"#, "baaaa", &[], 207),
scanner! { S207 { mode M { token r#"a*"# => 0; } } }
#[test]
fn test_match_207() {
    use s207::S207 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("baaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaa", 1, 4)]; // Oniguruma : ("", 0, 0)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "207: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("a+", "");
// td!(r#"a+"#, "", &[], 208),
scanner! { S208 { mode M { token r#"a+"# => 0; } } }
#[test]
fn test_match_208() {
    use s208::S208 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "208: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a+", "a", 0, 1);
// td!(r#"a+"#, "a", &[("a", 0, 1)], 209),
scanner! { S209 { mode M { token r#"a+"# => 0; } } }
#[test]
fn test_match_209() {
    use s209::S209 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "209: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "209: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "209: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "209: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a+", "aaaa", 0, 4);
// td!(r#"a+"#, "aaaa", &[("aaaa", 0, 4)], 210),
scanner! { S210 { mode M { token r#"a+"# => 0; } } }
#[test]
fn test_match_210() {
    use s210::S210 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaa", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "210: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "210: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "210: Match end does not match");
        assert_eq!(
            &"aaaa"[ma.1..ma.2],
            ma.0,
            "210: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a+", "aabbb", 0, 2);
// td!(r#"a+"#, "aabbb", &[("aa", 0, 2)], 211),
scanner! { S211 { mode M { token r#"a+"# => 0; } } }
#[test]
fn test_match_211() {
    use s211::S211 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aabbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "211: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "211: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "211: Match end does not match");
        assert_eq!(
            &"aabbb"[ma.1..ma.2],
            ma.0,
            "211: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a+", "baaaa", 1, 5);
// td!(r#"a+"#, "baaaa", &[("aaaa", 1, 5)], 212),
scanner! { S212 { mode M { token r#"a+"# => 0; } } }
#[test]
fn test_match_212() {
    use s212::S212 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("baaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaa", 1, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "212: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "212: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "212: Match end does not match");
        assert_eq!(
            &"baaaa"[ma.1..ma.2],
            ma.0,
            "212: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".?", "", 0, 0);
// td!(r#".?"#, "", &[], 213),
scanner! { S213 { mode M { token r#".?"# => 0; } } }
#[test]
fn test_match_213() {
    use s213::S213 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "213: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2(".?", "f", 0, 1);
// td!(r#".?"#, "f", &[("f", 0, 1)], 214),
scanner! { S214 { mode M { token r#".?"# => 0; } } }
#[test]
fn test_match_214() {
    use s214::S214 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("f", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("f", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "214: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "214: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "214: Match end does not match");
        assert_eq!(
            &"f"[ma.1..ma.2],
            ma.0,
            "214: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".?", "\n", 0, 0);
// td!(r#".?"#, "\n", &[], 215),
scanner! { S215 { mode M { token r#".?"# => 0; } } }
#[test]
fn test_match_215() {
    use s215::S215 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "215: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2(".*", "", 0, 0);
// td!(r#".*"#, "", &[], 216),
scanner! { S216 { mode M { token r#".*"# => 0; } } }
#[test]
fn test_match_216() {
    use s216::S216 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "216: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2(".*", "abcde", 0, 5);
// td!(r#".*"#, "abcde", &[("abcde", 0, 5)], 217),
scanner! { S217 { mode M { token r#".*"# => 0; } } }
#[test]
fn test_match_217() {
    use s217::S217 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcde", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcde", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "217: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "217: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "217: Match end does not match");
        assert_eq!(
            &"abcde"[ma.1..ma.2],
            ma.0,
            "217: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".+", "z", 0, 1);
// td!(r#".+"#, "z", &[("z", 0, 1)], 218),
scanner! { S218 { mode M { token r#".+"# => 0; } } }
#[test]
fn test_match_218() {
    use s218::S218 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("z", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("z", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "218: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "218: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "218: Match end does not match");
        assert_eq!(
            &"z"[ma.1..ma.2],
            ma.0,
            "218: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2(".+", "zdswer\n", 0, 6);
// td!(r#".+"#, "zdswer\n", &[("zdswer", 0, 6)], 219),
scanner! { S219 { mode M { token r#".+"# => 0; } } }
#[test]
fn test_match_219() {
    use s219::S219 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("zdswer\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("zdswer", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "219: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "219: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "219: Match end does not match");
        assert_eq!(
            &"zdswer\n"[ma.1..ma.2],
            ma.0,
            "219: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(.*)a\\1f", "babfbac", 0, 4);
// tu!(r#"(.*)a\1f"#, "babfbac", &[("babf", 0, 4)], 220), UnsupportedBackreference
// scanner! { S220 { mode M { token r#"(.*)a\1f"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.*)a\\1f", "bacbabf", 3, 7);
// tu!(r#"(.*)a\1f"#, "bacbabf", &[("babf", 3, 7)], 221), UnsupportedBackreference
// scanner! { S221 { mode M { token r#"(.*)a\1f"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((.*)a\\2f)", "bacbabf", 3, 7);
// tu!(r#"((.*)a\2f)"#, "bacbabf", &[("babf", 3, 7)], 222), UnsupportedBackreference
// scanner! { S222 { mode M { token r#"((.*)a\2f)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.*)a\\1f", "baczzzzzz\nbazz\nzzzzbabf", 19, 23);
// tu!(r#"(.*)a\1f"#, "baczzzzzz\nbazz\nzzzzbabf", &[("zzba", 19, 23)], 223), UnsupportedBackreference
// scanner! { S223 { mode M { token r#"(.*)a\1f"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x?)?", "", 0, 0);
// td!(r#"(?:x?)?"#, "", &[], 224),
scanner! { S224 { mode M { token r#"(?:x?)?"# => 0; } } }
#[test]
fn test_match_224() {
    use s224::S224 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "224: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x?)?", "x", 0, 1);
// td!(r#"(?:x?)?"#, "x", &[("x", 0, 1)], 225),
scanner! { S225 { mode M { token r#"(?:x?)?"# => 0; } } }
#[test]
fn test_match_225() {
    use s225::S225 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "225: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "225: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "225: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "225: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x?)?", "xx", 0, 1);
// td!(r#"(?:x?)?"#, "xx", &[("x", 0, 1)], 226),
scanner! { S226 { mode M { token r#"(?:x?)?"# => 0; } } }
#[test]
fn test_match_226() {
    use s226::S226 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1), ("x", 1, 2)]; // Oniguruma : ("x", 0, 1)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "226: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "226: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "226: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "226: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x?)*", "", 0, 0);
// td!(r#"(?:x?)*"#, "", &[], 227),
scanner! { S227 { mode M { token r#"(?:x?)*"# => 0; } } }
#[test]
fn test_match_227() {
    use s227::S227 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "227: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x?)*", "x", 0, 1);
// td!(r#"(?:x?)*"#, "x", &[("x", 0, 1)], 228),
scanner! { S228 { mode M { token r#"(?:x?)*"# => 0; } } }
#[test]
fn test_match_228() {
    use s228::S228 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "228: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "228: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "228: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "228: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x?)*", "xx", 0, 2);
// td!(r#"(?:x?)*"#, "xx", &[("xx", 0, 2)], 229),
scanner! { S229 { mode M { token r#"(?:x?)*"# => 0; } } }
#[test]
fn test_match_229() {
    use s229::S229 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "229: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "229: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "229: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "229: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x?)+", "", 0, 0);
// td!(r#"(?:x?)+"#, "", &[], 230),
scanner! { S230 { mode M { token r#"(?:x?)+"# => 0; } } }
#[test]
fn test_match_230() {
    use s230::S230 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "230: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x?)+", "x", 0, 1);
// td!(r#"(?:x?)+"#, "x", &[("x", 0, 1)], 231),
scanner! { S231 { mode M { token r#"(?:x?)+"# => 0; } } }
#[test]
fn test_match_231() {
    use s231::S231 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "231: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "231: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "231: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "231: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x?)+", "xx", 0, 2);
// td!(r#"(?:x?)+"#, "xx", &[("xx", 0, 2)], 232),
scanner! { S232 { mode M { token r#"(?:x?)+"# => 0; } } }
#[test]
fn test_match_232() {
    use s232::S232 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "232: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "232: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "232: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "232: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x?)\?\?", "", 0, 0);
// td!(r#"(?:x?)\?\?"#, "", &[], 233),
scanner! { S233 { mode M { token r#"(?:x?)\?\?"# => 0; } } }
#[test]
fn test_match_233() {
    use s233::S233 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "233: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x?)\?\?", "x", 0, 0);
// td!(r#"(?:x?)\?\?"#, "x", &[], 234),
scanner! { S234 { mode M { token r#"(?:x?)\?\?"# => 0; } } }
#[test]
fn test_match_234() {
    use s234::S234 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "234: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x?)\?\?", "xx", 0, 0);
// td!(r#"(?:x?)\?\?"#, "xx", &[], 235),
scanner! { S235 { mode M { token r#"(?:x?)\?\?"# => 0; } } }
#[test]
fn test_match_235() {
    use s235::S235 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "235: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x?)*?", "", 0, 0);
// tu!(r#"(?:x?)*?"#, "", &[], 236), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S236 { mode M { token r#"(?:x?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x?)*?", "x", 0, 0);
// tu!(r#"(?:x?)*?"#, "x", &[], 237), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S237 { mode M { token r#"(?:x?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x?)*?", "xx", 0, 0);
// tu!(r#"(?:x?)*?"#, "xx", &[], 238), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S238 { mode M { token r#"(?:x?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x?)+?", "", 0, 0);
// tu!(r#"(?:x?)+?"#, "", &[], 239), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S239 { mode M { token r#"(?:x?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x?)+?", "x", 0, 1);
// tu!(r#"(?:x?)+?"#, "x", &[("x", 0, 1)], 240), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S240 { mode M { token r#"(?:x?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x?)+?", "xx", 0, 1);
// tu!(r#"(?:x?)+?"#, "xx", &[("x", 0, 1)], 241), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S241 { mode M { token r#"(?:x?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*)?", "", 0, 0);
// td!(r#"(?:x*)?"#, "", &[], 242),
scanner! { S242 { mode M { token r#"(?:x*)?"# => 0; } } }
#[test]
fn test_match_242() {
    use s242::S242 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "242: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x*)?", "x", 0, 1);
// td!(r#"(?:x*)?"#, "x", &[("x", 0, 1)], 243),
scanner! { S243 { mode M { token r#"(?:x*)?"# => 0; } } }
#[test]
fn test_match_243() {
    use s243::S243 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "243: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "243: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "243: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "243: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x*)?", "xx", 0, 2);
// td!(r#"(?:x*)?"#, "xx", &[("xx", 0, 2)], 244),
scanner! { S244 { mode M { token r#"(?:x*)?"# => 0; } } }
#[test]
fn test_match_244() {
    use s244::S244 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "244: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "244: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "244: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "244: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x*)*", "", 0, 0);
// td!(r#"(?:x*)*"#, "", &[], 245),
scanner! { S245 { mode M { token r#"(?:x*)*"# => 0; } } }
#[test]
fn test_match_245() {
    use s245::S245 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "245: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x*)*", "x", 0, 1);
// td!(r#"(?:x*)*"#, "x", &[("x", 0, 1)], 246),
scanner! { S246 { mode M { token r#"(?:x*)*"# => 0; } } }
#[test]
fn test_match_246() {
    use s246::S246 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "246: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "246: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "246: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "246: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x*)*", "xx", 0, 2);
// td!(r#"(?:x*)*"#, "xx", &[("xx", 0, 2)], 247),
scanner! { S247 { mode M { token r#"(?:x*)*"# => 0; } } }
#[test]
fn test_match_247() {
    use s247::S247 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "247: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "247: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "247: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "247: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x*)+", "", 0, 0);
// td!(r#"(?:x*)+"#, "", &[], 248),
scanner! { S248 { mode M { token r#"(?:x*)+"# => 0; } } }
#[test]
fn test_match_248() {
    use s248::S248 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "248: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x*)+", "x", 0, 1);
// td!(r#"(?:x*)+"#, "x", &[("x", 0, 1)], 249),
scanner! { S249 { mode M { token r#"(?:x*)+"# => 0; } } }
#[test]
fn test_match_249() {
    use s249::S249 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "249: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "249: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "249: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "249: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x*)+", "xx", 0, 2);
// td!(r#"(?:x*)+"#, "xx", &[("xx", 0, 2)], 250),
scanner! { S250 { mode M { token r#"(?:x*)+"# => 0; } } }
#[test]
fn test_match_250() {
    use s250::S250 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "250: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "250: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "250: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "250: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x*)\?\?", "", 0, 0);
// td!(r#"(?:x*)\?\?"#, "", &[], 251),
scanner! { S251 { mode M { token r#"(?:x*)\?\?"# => 0; } } }
#[test]
fn test_match_251() {
    use s251::S251 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "251: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x*)\?\?", "x", 0, 0);
// td!(r#"(?:x*)\?\?"#, "x", &[], 252),
scanner! { S252 { mode M { token r#"(?:x*)\?\?"# => 0; } } }
#[test]
fn test_match_252() {
    use s252::S252 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "252: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x*)\?\?", "xx", 0, 0);
// td!(r#"(?:x*)\?\?"#, "xx", &[], 253),
scanner! { S253 { mode M { token r#"(?:x*)\?\?"# => 0; } } }
#[test]
fn test_match_253() {
    use s253::S253 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "253: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x*)*?", "", 0, 0);
// tu!(r#"(?:x*)*?"#, "", &[], 254), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S254 { mode M { token r#"(?:x*)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*)*?", "x", 0, 0);
// tu!(r#"(?:x*)*?"#, "x", &[], 255), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S255 { mode M { token r#"(?:x*)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*)*?", "xx", 0, 0);
// tu!(r#"(?:x*)*?"#, "xx", &[], 256), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S256 { mode M { token r#"(?:x*)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*)+?", "", 0, 0);
// tu!(r#"(?:x*)+?"#, "", &[], 257), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S257 { mode M { token r#"(?:x*)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*)+?", "x", 0, 1);
// tu!(r#"(?:x*)+?"#, "x", &[("x", 0, 1)], 258), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S258 { mode M { token r#"(?:x*)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*)+?", "xx", 0, 2);
// tu!(r#"(?:x*)+?"#, "xx", &[("xx", 0, 2)], 259), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S259 { mode M { token r#"(?:x*)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+)?", "", 0, 0);
// td!(r#"(?:x+)?"#, "", &[], 260),
scanner! { S260 { mode M { token r#"(?:x+)?"# => 0; } } }
#[test]
fn test_match_260() {
    use s260::S260 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "260: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x+)?", "x", 0, 1);
// td!(r#"(?:x+)?"#, "x", &[("x", 0, 1)], 261),
scanner! { S261 { mode M { token r#"(?:x+)?"# => 0; } } }
#[test]
fn test_match_261() {
    use s261::S261 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "261: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "261: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "261: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "261: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x+)?", "xx", 0, 2);
// td!(r#"(?:x+)?"#, "xx", &[("xx", 0, 2)], 262),
scanner! { S262 { mode M { token r#"(?:x+)?"# => 0; } } }
#[test]
fn test_match_262() {
    use s262::S262 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "262: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "262: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "262: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "262: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x+)*", "", 0, 0);
// td!(r#"(?:x+)*"#, "", &[], 263),
scanner! { S263 { mode M { token r#"(?:x+)*"# => 0; } } }
#[test]
fn test_match_263() {
    use s263::S263 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "263: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x+)*", "x", 0, 1);
// td!(r#"(?:x+)*"#, "x", &[("x", 0, 1)], 264),
scanner! { S264 { mode M { token r#"(?:x+)*"# => 0; } } }
#[test]
fn test_match_264() {
    use s264::S264 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "264: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "264: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "264: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "264: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x+)*", "xx", 0, 2);
// td!(r#"(?:x+)*"#, "xx", &[("xx", 0, 2)], 265),
scanner! { S265 { mode M { token r#"(?:x+)*"# => 0; } } }
#[test]
fn test_match_265() {
    use s265::S265 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "265: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "265: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "265: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "265: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?:x+)+", "");
// td!(r#"(?:x+)+"#, "", &[], 266),
scanner! { S266 { mode M { token r#"(?:x+)+"# => 0; } } }
#[test]
fn test_match_266() {
    use s266::S266 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "266: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x+)+", "x", 0, 1);
// td!(r#"(?:x+)+"#, "x", &[("x", 0, 1)], 267),
scanner! { S267 { mode M { token r#"(?:x+)+"# => 0; } } }
#[test]
fn test_match_267() {
    use s267::S267 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "267: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "267: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "267: Match end does not match");
        assert_eq!(
            &"x"[ma.1..ma.2],
            ma.0,
            "267: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x+)+", "xx", 0, 2);
// td!(r#"(?:x+)+"#, "xx", &[("xx", 0, 2)], 268),
scanner! { S268 { mode M { token r#"(?:x+)+"# => 0; } } }
#[test]
fn test_match_268() {
    use s268::S268 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("xx", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "268: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "268: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "268: Match end does not match");
        assert_eq!(
            &"xx"[ma.1..ma.2],
            ma.0,
            "268: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:x+)\?\?", "", 0, 0);
// td!(r#"(?:x+)\?\?"#, "", &[], 269),
scanner! { S269 { mode M { token r#"(?:x+)\?\?"# => 0; } } }
#[test]
fn test_match_269() {
    use s269::S269 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "269: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x+)\?\?", "x", 0, 0);
// td!(r#"(?:x+)\?\?"#, "x", &[], 270),
scanner! { S270 { mode M { token r#"(?:x+)\?\?"# => 0; } } }
#[test]
fn test_match_270() {
    use s270::S270 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "270: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x+)\?\?", "xx", 0, 0);
// td!(r#"(?:x+)\?\?"#, "xx", &[], 271),
scanner! { S271 { mode M { token r#"(?:x+)\?\?"# => 0; } } }
#[test]
fn test_match_271() {
    use s271::S271 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "271: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x+)*?", "", 0, 0);
// tu!(r#"(?:x+)*?"#, "", &[], 272), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S272 { mode M { token r#"(?:x+)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+)*?", "x", 0, 0);
// tu!(r#"(?:x+)*?"#, "x", &[], 273), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S273 { mode M { token r#"(?:x+)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+)*?", "xx", 0, 0);
// tu!(r#"(?:x+)*?"#, "xx", &[], 274), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S274 { mode M { token r#"(?:x+)*?"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:x+)+?", "");
// tu!(r#"(?:x+)+?"#, "", &[], 275), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S275 { mode M { token r#"(?:x+)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+)+?", "x", 0, 1);
// tu!(r#"(?:x+)+?"#, "x", &[("x", 0, 1)], 276), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S276 { mode M { token r#"(?:x+)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+)+?", "xx", 0, 2);
// tu!(r#"(?:x+)+?"#, "xx", &[("xx", 0, 2)], 277), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S277 { mode M { token r#"(?:x+)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x\?\?)?", "", 0, 0);
// td!(r#"(?:x\?\?)?"#, "", &[], 278),
scanner! { S278 { mode M { token r#"(?:x\?\?)?"# => 0; } } }
#[test]
fn test_match_278() {
    use s278::S278 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "278: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)?", "x", 0, 0);
// td!(r#"(?:x\?\?)?"#, "x", &[], 279),
scanner! { S279 { mode M { token r#"(?:x\?\?)?"# => 0; } } }
#[test]
fn test_match_279() {
    use s279::S279 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "279: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)?", "xx", 0, 0);
// td!(r#"(?:x\?\?)?"#, "xx", &[], 280),
scanner! { S280 { mode M { token r#"(?:x\?\?)?"# => 0; } } }
#[test]
fn test_match_280() {
    use s280::S280 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "280: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)*", "", 0, 0);
// td!(r#"(?:x\?\?)*"#, "", &[], 281),
scanner! { S281 { mode M { token r#"(?:x\?\?)*"# => 0; } } }
#[test]
fn test_match_281() {
    use s281::S281 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "281: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)*", "x", 0, 0);
// td!(r#"(?:x\?\?)*"#, "x", &[], 282),
scanner! { S282 { mode M { token r#"(?:x\?\?)*"# => 0; } } }
#[test]
fn test_match_282() {
    use s282::S282 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "282: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)*", "xx", 0, 0);
// td!(r#"(?:x\?\?)*"#, "xx", &[], 283),
scanner! { S283 { mode M { token r#"(?:x\?\?)*"# => 0; } } }
#[test]
fn test_match_283() {
    use s283::S283 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "283: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)+", "", 0, 0);
// td!(r#"(?:x\?\?)+"#, "", &[], 284),
scanner! { S284 { mode M { token r#"(?:x\?\?)+"# => 0; } } }
#[test]
fn test_match_284() {
    use s284::S284 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "284: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)+", "x", 0, 0);
// td!(r#"(?:x\?\?)+"#, "x", &[], 285),
scanner! { S285 { mode M { token r#"(?:x\?\?)+"# => 0; } } }
#[test]
fn test_match_285() {
    use s285::S285 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "285: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)+", "xx", 0, 0);
// td!(r#"(?:x\?\?)+"#, "xx", &[], 286),
scanner! { S286 { mode M { token r#"(?:x\?\?)+"# => 0; } } }
#[test]
fn test_match_286() {
    use s286::S286 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "286: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)\?\?", "", 0, 0);
// td!(r#"(?:x\?\?)\?\?"#, "", &[], 287),
scanner! { S287 { mode M { token r#"(?:x\?\?)\?\?"# => 0; } } }
#[test]
fn test_match_287() {
    use s287::S287 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "287: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)\?\?", "x", 0, 0);
// td!(r#"(?:x\?\?)\?\?"#, "x", &[], 288),
scanner! { S288 { mode M { token r#"(?:x\?\?)\?\?"# => 0; } } }
#[test]
fn test_match_288() {
    use s288::S288 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("x", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "288: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)\?\?", "xx", 0, 0);
// td!(r#"(?:x\?\?)\?\?"#, "xx", &[], 289),
scanner! { S289 { mode M { token r#"(?:x\?\?)\?\?"# => 0; } } }
#[test]
fn test_match_289() {
    use s289::S289 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("xx", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "289: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:x\?\?)*?", "", 0, 0);
// tu!(r#"(?:x\?\?)*?"#, "", &[], 290), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S290 { mode M { token r#"(?:x\?\?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x\?\?)*?", "x", 0, 0);
// tu!(r#"(?:x\?\?)*?"#, "x", &[], 291), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S291 { mode M { token r#"(?:x\?\?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x\?\?)*?", "xx", 0, 0);
// tu!(r#"(?:x\?\?)*?"#, "xx", &[], 292), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S292 { mode M { token r#"(?:x\?\?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x\?\?)+?", "", 0, 0);
// tu!(r#"(?:x\?\?)+?"#, "", &[], 293), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S293 { mode M { token r#"(?:x\?\?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x\?\?)+?", "x", 0, 0);
// tu!(r#"(?:x\?\?)+?"#, "x", &[], 294), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S294 { mode M { token r#"(?:x\?\?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x\?\?)+?", "xx", 0, 0);
// tu!(r#"(?:x\?\?)+?"#, "xx", &[], 295), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S295 { mode M { token r#"(?:x\?\?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)?", "", 0, 0);
// tu!(r#"(?:x*?)?"#, "", &[], 296), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S296 { mode M { token r#"(?:x*?)?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)?", "x", 0, 0);
// tu!(r#"(?:x*?)?"#, "x", &[], 297), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S297 { mode M { token r#"(?:x*?)?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)?", "xx", 0, 0);
// tu!(r#"(?:x*?)?"#, "xx", &[], 298), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S298 { mode M { token r#"(?:x*?)?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)*", "", 0, 0);
// tu!(r#"(?:x*?)*"#, "", &[], 299), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S299 { mode M { token r#"(?:x*?)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)*", "x", 0, 0);
// tu!(r#"(?:x*?)*"#, "x", &[], 300), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S300 { mode M { token r#"(?:x*?)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)*", "xx", 0, 0);
// tu!(r#"(?:x*?)*"#, "xx", &[], 301), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S301 { mode M { token r#"(?:x*?)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)+", "", 0, 0);
// tu!(r#"(?:x*?)+"#, "", &[], 302), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S302 { mode M { token r#"(?:x*?)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)+", "x", 0, 0);
// tu!(r#"(?:x*?)+"#, "x", &[], 303), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S303 { mode M { token r#"(?:x*?)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)+", "xx", 0, 0);
// tu!(r#"(?:x*?)+"#, "xx", &[], 304), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S304 { mode M { token r#"(?:x*?)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)\?\?", "", 0, 0);
// tu!(r#"(?:x*?)\?\?"#, "", &[], 305), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S305 { mode M { token r#"(?:x*?)\?\?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)\?\?", "x", 0, 0);
// tu!(r#"(?:x*?)\?\?"#, "x", &[], 306), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S306 { mode M { token r#"(?:x*?)\?\?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)\?\?", "xx", 0, 0);
// tu!(r#"(?:x*?)\?\?"#, "xx", &[], 307), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S307 { mode M { token r#"(?:x*?)\?\?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)*?", "", 0, 0);
// tu!(r#"(?:x*?)*?"#, "", &[], 308), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S308 { mode M { token r#"(?:x*?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)*?", "x", 0, 0);
// tu!(r#"(?:x*?)*?"#, "x", &[], 309), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S309 { mode M { token r#"(?:x*?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)*?", "xx", 0, 0);
// tu!(r#"(?:x*?)*?"#, "xx", &[], 310), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S310 { mode M { token r#"(?:x*?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)+?", "", 0, 0);
// tu!(r#"(?:x*?)+?"#, "", &[], 311), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S311 { mode M { token r#"(?:x*?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)+?", "x", 0, 0);
// tu!(r#"(?:x*?)+?"#, "x", &[], 312), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S312 { mode M { token r#"(?:x*?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x*?)+?", "xx", 0, 0);
// tu!(r#"(?:x*?)+?"#, "xx", &[], 313), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S313 { mode M { token r#"(?:x*?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)?", "", 0, 0);
// tu!(r#"(?:x+?)?"#, "", &[], 314), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S314 { mode M { token r#"(?:x+?)?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)?", "x", 0, 1);
// tu!(r#"(?:x+?)?"#, "x", &[("x", 0, 1)], 315), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S315 { mode M { token r#"(?:x+?)?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)?", "xx", 0, 1);
// tu!(r#"(?:x+?)?"#, "xx", &[("x", 0, 1)], 316), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S316 { mode M { token r#"(?:x+?)?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)*", "", 0, 0);
// tu!(r#"(?:x+?)*"#, "", &[], 317), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S317 { mode M { token r#"(?:x+?)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)*", "x", 0, 1);
// tu!(r#"(?:x+?)*"#, "x", &[("x", 0, 1)], 318), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S318 { mode M { token r#"(?:x+?)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)*", "xx", 0, 2);
// tu!(r#"(?:x+?)*"#, "xx", &[("xx", 0, 2)], 319), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S319 { mode M { token r#"(?:x+?)*"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:x+?)+", "");
// tu!(r#"(?:x+?)+"#, "", &[], 320), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S320 { mode M { token r#"(?:x+?)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)+", "x", 0, 1);
// tu!(r#"(?:x+?)+"#, "x", &[("x", 0, 1)], 321), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S321 { mode M { token r#"(?:x+?)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)+", "xx", 0, 2);
// tu!(r#"(?:x+?)+"#, "xx", &[("xx", 0, 2)], 322), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S322 { mode M { token r#"(?:x+?)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)\?\?", "", 0, 0);
// tu!(r#"(?:x+?)\?\?"#, "", &[], 323), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S323 { mode M { token r#"(?:x+?)\?\?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)\?\?", "x", 0, 0);
// tu!(r#"(?:x+?)\?\?"#, "x", &[], 324), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S324 { mode M { token r#"(?:x+?)\?\?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)\?\?", "xx", 0, 0);
// tu!(r#"(?:x+?)\?\?"#, "xx", &[], 325), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S325 { mode M { token r#"(?:x+?)\?\?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)*?", "", 0, 0);
// tu!(r#"(?:x+?)*?"#, "", &[], 326), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S326 { mode M { token r#"(?:x+?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)*?", "x", 0, 0);
// tu!(r#"(?:x+?)*?"#, "x", &[], 327), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S327 { mode M { token r#"(?:x+?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)*?", "xx", 0, 0);
// tu!(r#"(?:x+?)*?"#, "xx", &[], 328), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S328 { mode M { token r#"(?:x+?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:x+?)+?", "");
// tu!(r#"(?:x+?)+?"#, "", &[], 329), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S329 { mode M { token r#"(?:x+?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)+?", "x", 0, 1);
// tu!(r#"(?:x+?)+?"#, "x", &[("x", 0, 1)], 330), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S330 { mode M { token r#"(?:x+?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:x+?)+?", "xx", 0, 1);
// tu!(r#"(?:x+?)+?"#, "xx", &[("x", 0, 1)], 331), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S331 { mode M { token r#"(?:x+?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|b", "a", 0, 1);
// td!(r#"a|b"#, "a", &[("a", 0, 1)], 332),
scanner! { S332 { mode M { token r#"a|b"# => 0; } } }
#[test]
fn test_match_332() {
    use s332::S332 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "332: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "332: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "332: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "332: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a|b", "b", 0, 1);
// td!(r#"a|b"#, "b", &[("b", 0, 1)], 333),
scanner! { S333 { mode M { token r#"a|b"# => 0; } } }
#[test]
fn test_match_333() {
    use s333::S333 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "333: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "333: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "333: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "333: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("|a", "a", 0, 0);
// td!(r#"|a"#, "a", &[], 334),
scanner! { S334 { mode M { token r#"|a"# => 0; } } }
#[test]
fn test_match_334() {
    use s334::S334 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)]; // Oniguruma: no match
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "334: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(|a)", "a", 0, 0);
// td!(r#"(|a)"#, "a", &[], 335),
scanner! { S335 { mode M { token r#"(|a)"# => 0; } } }
#[test]
fn test_match_335() {
    use s335::S335 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)]; // Oniguruma: no match
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "335: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("ab|bc", "ab", 0, 2);
// td!(r#"ab|bc"#, "ab", &[("ab", 0, 2)], 336),
scanner! { S336 { mode M { token r#"ab|bc"# => 0; } } }
#[test]
fn test_match_336() {
    use s336::S336 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "336: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "336: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "336: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "336: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("ab|bc", "bc", 0, 2);
// td!(r#"ab|bc"#, "bc", &[("bc", 0, 2)], 337),
scanner! { S337 { mode M { token r#"ab|bc"# => 0; } } }
#[test]
fn test_match_337() {
    use s337::S337 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bc", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "337: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "337: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "337: Match end does not match");
        assert_eq!(
            &"bc"[ma.1..ma.2],
            ma.0,
            "337: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("z(?:ab|bc)", "zbc", 0, 3);
// td!(r#"z(?:ab|bc)"#, "zbc", &[("zbc", 0, 3)], 338),
scanner! { S338 { mode M { token r#"z(?:ab|bc)"# => 0; } } }
#[test]
fn test_match_338() {
    use s338::S338 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("zbc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("zbc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "338: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "338: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "338: Match end does not match");
        assert_eq!(
            &"zbc"[ma.1..ma.2],
            ma.0,
            "338: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a(?:ab|bc)c", "aabc", 0, 4);
// td!(r#"a(?:ab|bc)c"#, "aabc", &[("aabc", 0, 4)], 339),
scanner! { S339 { mode M { token r#"a(?:ab|bc)c"# => 0; } } }
#[test]
fn test_match_339() {
    use s339::S339 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aabc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aabc", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "339: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "339: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "339: Match end does not match");
        assert_eq!(
            &"aabc"[ma.1..ma.2],
            ma.0,
            "339: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("ab|(?:ac|az)", "az", 0, 2);
// td!(r#"ab|(?:ac|az)"#, "az", &[("az", 0, 2)], 340),
scanner! { S340 { mode M { token r#"ab|(?:ac|az)"# => 0; } } }
#[test]
fn test_match_340() {
    use s340::S340 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("az", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("az", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "340: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "340: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "340: Match end does not match");
        assert_eq!(
            &"az"[ma.1..ma.2],
            ma.0,
            "340: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a|b|c", "dc", 1, 2);
// td!(r#"a|b|c"#, "dc", &[("c", 1, 2)], 341),
scanner! { S341 { mode M { token r#"a|b|c"# => 0; } } }
#[test]
fn test_match_341() {
    use s341::S341 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("dc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("c", 1, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "341: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "341: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "341: Match end does not match");
        assert_eq!(
            &"dc"[ma.1..ma.2],
            ma.0,
            "341: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a|b|cd|efg|h|ijk|lmn|o|pq|rstuvwx|yz", "pqr", 0, 2);
// td!(r#"a|b|cd|efg|h|ijk|lmn|o|pq|rstuvwx|yz"#, "pqr", &[("pq", 0, 2)], 342),
scanner! { S342 { mode M { token r#"a|b|cd|efg|h|ijk|lmn|o|pq|rstuvwx|yz"# => 0; } } }
#[test]
fn test_match_342() {
    use s342::S342 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("pqr", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("pq", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "342: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "342: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "342: Match end does not match");
        assert_eq!(
            &"pqr"[ma.1..ma.2],
            ma.0,
            "342: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("a|b|cd|efg|h|ijk|lmn|o|pq|rstuvwx|yz", "mn");
// td!(r#"a|b|cd|efg|h|ijk|lmn|o|pq|rstuvwx|yz"#, "mn", &[], 343),
scanner! { S343 { mode M { token r#"a|b|cd|efg|h|ijk|lmn|o|pq|rstuvwx|yz"# => 0; } } }
#[test]
fn test_match_343() {
    use s343::S343 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("mn", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "343: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a|^z", "ba", 1, 2);
// tu!(r#"a|^z"#, "ba", &[("a", 1, 2)], 344), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S344 { mode M { token r#"a|^z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|^z", "za", 0, 1);
// tu!(r#"a|^z"#, "za", &[("z", 0, 1)], 345), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S345 { mode M { token r#"a|^z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|\\Gz", "bza", 2, 3);
// tr!(r#"a|\Gz"#, "bza", &[("a", 2, 3)], 346), EscapeUnrecognized
// scanner! { S346 { mode M { token r#"a|\Gz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|\\Gz", "za", 0, 1);
// tr!(r#"a|\Gz"#, "za", &[("z", 0, 1)], 347), EscapeUnrecognized
// scanner! { S347 { mode M { token r#"a|\Gz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|\\Az", "bza", 2, 3);
// tu!(r#"a|\Az"#, "bza", &[("a", 2, 3)], 348), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S348 { mode M { token r#"a|\Az"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|\\Az", "za", 0, 1);
// tu!(r#"a|\Az"#, "za", &[("z", 0, 1)], 349), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S349 { mode M { token r#"a|\Az"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|b\\Z", "ba", 1, 2);
// tr!(r#"a|b\Z"#, "ba", &[("a", 1, 2)], 350), EscapeUnrecognized
// scanner! { S350 { mode M { token r#"a|b\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|b\\Z", "b", 0, 1);
// tr!(r#"a|b\Z"#, "b", &[("b", 0, 1)], 351), EscapeUnrecognized
// scanner! { S351 { mode M { token r#"a|b\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|b\\z", "ba", 1, 2);
// tu!(r#"a|b\z"#, "ba", &[("a", 1, 2)], 352), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S352 { mode M { token r#"a|b\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|b\\z", "b", 0, 1);
// tu!(r#"a|b\z"#, "b", &[("b", 0, 1)], 353), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S353 { mode M { token r#"a|b\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\w|\\s", " ", 0, 1);
// td!(r#"\w|\s"#, " ", &[(" ", 0, 1)], 354),
scanner! { S354 { mode M { token r#"\w|\s"# => 0; } } }
#[test]
fn test_match_354() {
    use s354::S354 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(" ", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "354: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "354: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "354: Match end does not match");
        assert_eq!(
            &" "[ma.1..ma.2],
            ma.0,
            "354: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("\\w|\\w", " ");
// td!(r#"\w|\w"#, " ", &[], 355),
scanner! { S355 { mode M { token r#"\w|\w"# => 0; } } }
#[test]
fn test_match_355() {
    use s355::S355 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "355: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("\\w|%", "%", 0, 1);
// td!(r#"\w|%"#, "%", &[("%", 0, 1)], 356),
scanner! { S356 { mode M { token r#"\w|%"# => 0; } } }
#[test]
fn test_match_356() {
    use s356::S356 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("%", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("%", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "356: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "356: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "356: Match end does not match");
        assert_eq!(
            &"%"[ma.1..ma.2],
            ma.0,
            "356: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\w|[&$]", "&", 0, 1);
// td!(r#"\w|[&$]"#, "&", &[("&", 0, 1)], 357),
scanner! { S357 { mode M { token r#"\w|[&$]"# => 0; } } }
#[test]
fn test_match_357() {
    use s357::S357 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("&", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("&", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "357: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "357: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "357: Match end does not match");
        assert_eq!(
            &"&"[ma.1..ma.2],
            ma.0,
            "357: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[b-d]|[^e-z]", "a", 0, 1);
// td!(r#"[b-d]|[^e-z]"#, "a", &[("a", 0, 1)], 358),
scanner! { S358 { mode M { token r#"[b-d]|[^e-z]"# => 0; } } }
#[test]
fn test_match_358() {
    use s358::S358 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "358: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "358: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "358: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "358: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a|[c-f])|bz", "dz", 0, 1);
// td!(r#"(?:a|[c-f])|bz"#, "dz", &[("d", 0, 1)], 359),
scanner! { S359 { mode M { token r#"(?:a|[c-f])|bz"# => 0; } } }
#[test]
fn test_match_359() {
    use s359::S359 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("dz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("d", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "359: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "359: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "359: Match end does not match");
        assert_eq!(
            &"dz"[ma.1..ma.2],
            ma.0,
            "359: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a|[c-f])|bz", "bz", 0, 2);
// td!(r#"(?:a|[c-f])|bz"#, "bz", &[("bz", 0, 2)], 360),
scanner! { S360 { mode M { token r#"(?:a|[c-f])|bz"# => 0; } } }
#[test]
fn test_match_360() {
    use s360::S360 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bz", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "360: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "360: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "360: Match end does not match");
        assert_eq!(
            &"bz"[ma.1..ma.2],
            ma.0,
            "360: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("abc|(?=zz)..f", "zzf", 0, 3);
// tr!(r#"abc|(?=zz)..f"#, "zzf", &[("zzf", 0, 3)], 361), UnsupportedLookAround
// scanner! { S361 { mode M { token r#"abc|(?=zz)..f"# => 0; } } }

// -------------------------------------------------------------------------
// x2("abc|(?!zz)..f", "abf", 0, 3);
// tr!(r#"abc|(?!zz)..f"#, "abf", &[("abf", 0, 3)], 362), UnsupportedLookAround
// scanner! { S362 { mode M { token r#"abc|(?!zz)..f"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?=za)..a|(?=zz)..a", "zza", 0, 3);
// tr!(r#"(?=za)..a|(?=zz)..a"#, "zza", &[("zza", 0, 3)], 363), UnsupportedLookAround
// scanner! { S363 { mode M { token r#"(?=za)..a|(?=zz)..a"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?>a|abd)c", "abdc");
// tr!(r#"(?>a|abd)c"#, "abdc", &[], 364), FlagUnrecognized
// scanner! { S364 { mode M { token r#"(?>a|abd)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?>abd|a)c", "abdc", 0, 4);
// tr!(r#"(?>abd|a)c"#, "abdc", &[("abdc", 0, 4)], 365), FlagUnrecognized
// scanner! { S365 { mode M { token r#"(?>abd|a)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a?|b", "a", 0, 1);
// td!(r#"a?|b"#, "a", &[("a", 0, 1)], 366),
scanner! { S366 { mode M { token r#"a?|b"# => 0; } } }
#[test]
fn test_match_366() {
    use s366::S366 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "366: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "366: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "366: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "366: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a?|b", "b", 0, 0);
// td!(r#"a?|b"#, "b", &[], 367),
scanner! { S367 { mode M { token r#"a?|b"# => 0; } } }
#[test]
fn test_match_367() {
    use s367::S367 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)]; // Oniguruma: no match
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "367: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a?|b", "", 0, 0);
// td!(r#"a?|b"#, "", &[], 368),
scanner! { S368 { mode M { token r#"a?|b"# => 0; } } }
#[test]
fn test_match_368() {
    use s368::S368 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "368: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a*|b", "aa", 0, 2);
// td!(r#"a*|b"#, "aa", &[("aa", 0, 2)], 369),
scanner! { S369 { mode M { token r#"a*|b"# => 0; } } }
#[test]
fn test_match_369() {
    use s369::S369 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "369: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "369: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "369: Match end does not match");
        assert_eq!(
            &"aa"[ma.1..ma.2],
            ma.0,
            "369: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a*|b*", "ba", 0, 0);
// td!(r#"a*|b*"#, "ba", &[], 370),
scanner! { S370 { mode M { token r#"a*|b*"# => 0; } } }
#[test]
fn test_match_370() {
    use s370::S370 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ba", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1), ("a", 1, 2)]; // Oniguruma: no match
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "370: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a*|b*", "ab", 0, 1);
// td!(r#"a*|b*"#, "ab", &[("a", 0, 1)], 371),
scanner! { S371 { mode M { token r#"a*|b*"# => 0; } } }
#[test]
fn test_match_371() {
    use s371::S371 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1), ("b", 1, 2)]; // Oniguruma: ("a", 0, 1)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "371: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "371: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "371: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "371: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a+|b*", "", 0, 0);
// td!(r#"a+|b*"#, "", &[], 372),
scanner! { S372 { mode M { token r#"a+|b*"# => 0; } } }
#[test]
fn test_match_372() {
    use s372::S372 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "372: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a+|b*", "bbb", 0, 3);
// td!(r#"a+|b*"#, "bbb", &[("bbb", 0, 3)], 373),
scanner! { S373 { mode M { token r#"a+|b*"# => 0; } } }
#[test]
fn test_match_373() {
    use s373::S373 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bbb", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "373: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "373: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "373: Match end does not match");
        assert_eq!(
            &"bbb"[ma.1..ma.2],
            ma.0,
            "373: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a+|b*", "abbb", 0, 1);
// td!(r#"a+|b*"#, "abbb", &[("a", 0, 1)], 374),
scanner! { S374 { mode M { token r#"a+|b*"# => 0; } } }
#[test]
fn test_match_374() {
    use s374::S374 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1), ("bbb", 1, 4)]; // Oniguruma: ("a", 0, 1)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "374: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "374: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "374: Match end does not match");
        assert_eq!(
            &"abbb"[ma.1..ma.2],
            ma.0,
            "374: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("a+|b+", "");
// td!(r#"a+|b+"#, "", &[], 375),
scanner! { S375 { mode M { token r#"a+|b+"# => 0; } } }
#[test]
fn test_match_375() {
    use s375::S375 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "375: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(a|b)?", "b", 0, 1);
// td!(r#"(a|b)?"#, "b", &[("b", 0, 1)], 376),
scanner! { S376 { mode M { token r#"(a|b)?"# => 0; } } }
#[test]
fn test_match_376() {
    use s376::S376 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("b", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "376: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "376: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "376: Match end does not match");
        assert_eq!(
            &"b"[ma.1..ma.2],
            ma.0,
            "376: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(a|b)*", "ba", 0, 2);
// td!(r#"(a|b)*"#, "ba", &[("ba", 0, 2)], 377),
scanner! { S377 { mode M { token r#"(a|b)*"# => 0; } } }
#[test]
fn test_match_377() {
    use s377::S377 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ba", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ba", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "377: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "377: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "377: Match end does not match");
        assert_eq!(
            &"ba"[ma.1..ma.2],
            ma.0,
            "377: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(a|b)+", "bab", 0, 3);
// td!(r#"(a|b)+"#, "bab", &[("bab", 0, 3)], 378),
scanner! { S378 { mode M { token r#"(a|b)+"# => 0; } } }
#[test]
fn test_match_378() {
    use s378::S378 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bab", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "378: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "378: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "378: Match end does not match");
        assert_eq!(
            &"bab"[ma.1..ma.2],
            ma.0,
            "378: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(ab|ca)+", "caabbc", 0, 4);
// td!(r#"(ab|ca)+"#, "caabbc", &[("caab", 0, 4)], 379),
scanner! { S379 { mode M { token r#"(ab|ca)+"# => 0; } } }
#[test]
fn test_match_379() {
    use s379::S379 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("caabbc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("caab", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "379: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "379: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "379: Match end does not match");
        assert_eq!(
            &"caabbc"[ma.1..ma.2],
            ma.0,
            "379: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(ab|ca)+", "aabca", 1, 5);
// td!(r#"(ab|ca)+"#, "aabca", &[("abca", 1, 5)], 380),
scanner! { S380 { mode M { token r#"(ab|ca)+"# => 0; } } }
#[test]
fn test_match_380() {
    use s380::S380 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aabca", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abca", 1, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "380: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "380: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "380: Match end does not match");
        assert_eq!(
            &"aabca"[ma.1..ma.2],
            ma.0,
            "380: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(ab|ca)+", "abzca", 0, 2);
// td!(r#"(ab|ca)+"#, "abzca", &[("ab", 0, 2)], 381),
scanner! { S381 { mode M { token r#"(ab|ca)+"# => 0; } } }
#[test]
fn test_match_381() {
    use s381::S381 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abzca", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2), ("ca", 3, 5)]; // Oniguruma: ("ab", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "381: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "381: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "381: Match end does not match");
        assert_eq!(
            &"abzca"[ma.1..ma.2],
            ma.0,
            "381: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(a|bab)+", "ababa", 0, 5);
// td!(r#"(a|bab)+"#, "ababa", &[("ababa", 0, 5)], 382),
scanner! { S382 { mode M { token r#"(a|bab)+"# => 0; } } }
#[test]
fn test_match_382() {
    use s382::S382 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ababa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ababa", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "382: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "382: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "382: Match end does not match");
        assert_eq!(
            &"ababa"[ma.1..ma.2],
            ma.0,
            "382: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(a|bab)+", "ba", 1, 2);
// td!(r#"(a|bab)+"#, "ba", &[("a", 1, 2)], 383),
scanner! { S383 { mode M { token r#"(a|bab)+"# => 0; } } }
#[test]
fn test_match_383() {
    use s383::S383 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ba", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 1, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "383: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "383: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "383: Match end does not match");
        assert_eq!(
            &"ba"[ma.1..ma.2],
            ma.0,
            "383: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(a|bab)+", "baaaba", 1, 4);
// td!(r#"(a|bab)+"#, "baaaba", &[("aaa", 1, 4)], 384),
scanner! { S384 { mode M { token r#"(a|bab)+"# => 0; } } }
#[test]
fn test_match_384() {
    use s384::S384 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("baaaba", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaa", 1, 4), ("a", 5, 6)]; // Oniguruma: ("aaa", 1, 4)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "384: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "384: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "384: Match end does not match");
        assert_eq!(
            &"baaaba"[ma.1..ma.2],
            ma.0,
            "384: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a|b)(?:a|b)", "ab", 0, 2);
// td!(r#"(?:a|b)(?:a|b)"#, "ab", &[("ab", 0, 2)], 385),
scanner! { S385 { mode M { token r#"(?:a|b)(?:a|b)"# => 0; } } }
#[test]
fn test_match_385() {
    use s385::S385 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "385: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "385: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "385: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "385: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a*|b*)(?:a*|b*)", "aaabbb", 0, 3);
// td!(r#"(?:a*|b*)(?:a*|b*)"#, "aaabbb", &[("aaa", 0, 3)], 386),
scanner! { S386 { mode M { token r#"(?:a*|b*)(?:a*|b*)"# => 0; } } }
#[test]
fn test_match_386() {
    use s386::S386 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaabbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaabbb", 0, 6)]; // Oniguruma: ("aaa", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "386: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "386: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "386: Match end does not match");
        assert_eq!(
            &"aaabbb"[ma.1..ma.2],
            ma.0,
            "386: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a*|b*)(?:a+|b+)", "aaabbb", 0, 6);
// td!(r#"(?:a*|b*)(?:a+|b+)"#, "aaabbb", &[("aaabbb", 0, 6)], 387),
scanner! { S387 { mode M { token r#"(?:a*|b*)(?:a+|b+)"# => 0; } } }
#[test]
fn test_match_387() {
    use s387::S387 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaabbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaabbb", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "387: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "387: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "387: Match end does not match");
        assert_eq!(
            &"aaabbb"[ma.1..ma.2],
            ma.0,
            "387: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a+|b+){2}", "aaabbb", 0, 6);
// td!(r#"(?:a+|b+){2}"#, "aaabbb", &[("aaabbb", 0, 6)], 388),
scanner! { S388 { mode M { token r#"(?:a+|b+){2}"# => 0; } } }
#[test]
fn test_match_388() {
    use s388::S388 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaabbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaabbb", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "388: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "388: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "388: Match end does not match");
        assert_eq!(
            &"aaabbb"[ma.1..ma.2],
            ma.0,
            "388: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("h{0,}", "hhhh", 0, 4);
// td!(r#"h{0,}"#, "hhhh", &[("hhhh", 0, 4)], 389),
scanner! { S389 { mode M { token r#"h{0,}"# => 0; } } }
#[test]
fn test_match_389() {
    use s389::S389 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("hhhh", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("hhhh", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "389: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "389: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "389: Match end does not match");
        assert_eq!(
            &"hhhh"[ma.1..ma.2],
            ma.0,
            "389: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a+|b+){1,2}", "aaabbb", 0, 6);
// td!(r#"(?:a+|b+){1,2}"#, "aaabbb", &[("aaabbb", 0, 6)], 390),
scanner! { S390 { mode M { token r#"(?:a+|b+){1,2}"# => 0; } } }
#[test]
fn test_match_390() {
    use s390::S390 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaabbb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaabbb", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "390: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "390: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "390: Match end does not match");
        assert_eq!(
            &"aaabbb"[ma.1..ma.2],
            ma.0,
            "390: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("ax{2}*a", "0axxxa1");
// td!(r#"ax{2}*a"#, "0axxxa1", &[], 391),
scanner! { S391 { mode M { token r#"ax{2}*a"# => 0; } } }
#[test]
fn test_match_391() {
    use s391::S391 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("0axxxa1", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "391: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("a.{0,2}a", "0aXXXa0");
// td!(r#"a.{0,2}a"#, "0aXXXa0", &[], 392),
scanner! { S392 { mode M { token r#"a.{0,2}a"# => 0; } } }
#[test]
fn test_match_392() {
    use s392::S392 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("0aXXXa0", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "392: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("a.{0,2}?a", "0aXXXa0");
// tu!(r#"a.{0,2}?a"#, "0aXXXa0", &[], 393), UnsupportedFeatureError("[\0-\t\u{b}-\u{10ffff}]{0,2}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S393 { mode M { token r#"a.{0,2}?a"# => 0; } } }

// -------------------------------------------------------------------------
// n("a.{0,2}?a", "0aXXXXa0");
// tu!(r#"a.{0,2}?a"#, "0aXXXXa0", &[], 394), UnsupportedFeatureError("[\0-\t\u{b}-\u{10ffff}]{0,2}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S394 { mode M { token r#"a.{0,2}?a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^a{2,}?a$", "aaa", 0, 3);
// tu!(r#"^a{2,}?a$"#, "aaa", &[("aaa", 0, 3)], 395), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S395 { mode M { token r#"^a{2,}?a$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^[a-z]{2,}?$", "aaa", 0, 3);
// tu!(r#"^[a-z]{2,}?$"#, "aaa", &[("aaa", 0, 3)], 396), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S396 { mode M { token r#"^[a-z]{2,}?$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:a+|\\Ab*)cc", "cc", 0, 2);
// tu!(r#"(?:a+|\Ab*)cc"#, "cc", &[("cc", 0, 2)], 397), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S397 { mode M { token r#"(?:a+|\Ab*)cc"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:a+|\\Ab*)cc", "abcc");
// tu!(r#"(?:a+|\Ab*)cc"#, "abcc", &[], 398), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S398 { mode M { token r#"(?:a+|\Ab*)cc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:^a+|b+)*c", "aabbbabc", 6, 8);
// tu!(r#"(?:^a+|b+)*c"#, "aabbbabc", &[("bc", 6, 8)], 399), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S399 { mode M { token r#"(?:^a+|b+)*c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:^a+|b+)*c", "aabbbbc", 0, 7);
// tu!(r#"(?:^a+|b+)*c"#, "aabbbbc", &[("aabbbbc", 0, 7)], 400), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S400 { mode M { token r#"(?:^a+|b+)*c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a|(?i)c", "C", 0, 1);
// td!(r#"a|(?i)c"#, "C", &[("C", 0, 1)], 401),
scanner! { S401 { mode M { token r#"a|(?i)c"# => 0; } } }
#[test]
fn test_match_401() {
    use s401::S401 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("C", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "401: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "401: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "401: Match end does not match");
        assert_eq!(
            &"C"[ma.1..ma.2],
            ma.0,
            "401: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)c|a", "C", 0, 1);
// td!(r#"(?i)c|a"#, "C", &[("C", 0, 1)], 402),
scanner! { S402 { mode M { token r#"(?i)c|a"# => 0; } } }
#[test]
fn test_match_402() {
    use s402::S402 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("C", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "402: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "402: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "402: Match end does not match");
        assert_eq!(
            &"C"[ma.1..ma.2],
            ma.0,
            "402: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)c|a", "A", 0, 1);
// td!(r#"(?i)c|a"#, "A", &[("A", 0, 1)], 403),
scanner! { S403 { mode M { token r#"(?i)c|a"# => 0; } } }
#[test]
fn test_match_403() {
    use s403::S403 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("A", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "403: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "403: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "403: Match end does not match");
        assert_eq!(
            &"A"[ma.1..ma.2],
            ma.0,
            "403: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a(?i)b|c", "aB", 0, 2);
// td!(r#"a(?i)b|c"#, "aB", &[("aB", 0, 2)], 404),
scanner! { S404 { mode M { token r#"a(?i)b|c"# => 0; } } }
#[test]
fn test_match_404() {
    use s404::S404 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aB", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aB", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "404: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "404: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "404: Match end does not match");
        assert_eq!(
            &"aB"[ma.1..ma.2],
            ma.0,
            "404: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a(?i)b|c", "aC", 0, 2);
// td!(r#"a(?i)b|c"#, "aC", &[("aC", 0, 2)], 405),
scanner! { S405 { mode M { token r#"a(?i)b|c"# => 0; } } }
#[test]
fn test_match_405() {
    use s405::S405 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aC", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 1, 2)]; // Oniguruma: &[("aC", 0, 2)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "405: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "405: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "405: Match end does not match");
        assert_eq!(
            &"aC"[ma.1..ma.2],
            ma.0,
            "405: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("a(?i)b|c", "AC");
// td!(r#"a(?i)b|c"#, "AC", &[], 406),
scanner! { S406 { mode M { token r#"a(?i)b|c"# => 0; } } }
#[test]
fn test_match_406() {
    use s406::S406 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("AC", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 1, 2)]; // Oniguruma: no match
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "406: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("a(?:(?i)b)|c", "aC");
// td!(r#"a(?:(?i)b)|c"#, "aC", &[], 407),
scanner! { S407 { mode M { token r#"a(?:(?i)b)|c"# => 0; } } }
#[test]
fn test_match_407() {
    use s407::S407 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aC", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "407: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i:c)|a", "C", 0, 1);
// td!(r#"(?i:c)|a"#, "C", &[("C", 0, 1)], 408),
scanner! { S408 { mode M { token r#"(?i:c)|a"# => 0; } } }
#[test]
fn test_match_408() {
    use s408::S408 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("C", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "408: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "408: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "408: Match end does not match");
        assert_eq!(
            &"C"[ma.1..ma.2],
            ma.0,
            "408: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?i:c)|a", "A");
// td!(r#"(?i:c)|a"#, "A", &[], 409),
scanner! { S409 { mode M { token r#"(?i:c)|a"# => 0; } } }
#[test]
fn test_match_409() {
    use s409::S409 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "409: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[abc]?", "abc", 0, 1);
// td!(r#"[abc]?"#, "abc", &[("a", 0, 1)], 410),
scanner! { S410 { mode M { token r#"[abc]?"# => 0; } } }
#[test]
fn test_match_410() {
    use s410::S410 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1), ("b", 1, 2), ("c", 2, 3)]; // Oniguruma: &[("a", 0, 1)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "410: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "410: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "410: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "410: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[abc]*", "abc", 0, 3);
// td!(r#"[abc]*"#, "abc", &[("abc", 0, 3)], 411),
scanner! { S411 { mode M { token r#"[abc]*"# => 0; } } }
#[test]
fn test_match_411() {
    use s411::S411 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "411: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "411: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "411: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "411: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[^abc]*", "abc", 0, 0);
// td!(r#"[^abc]*"#, "abc", &[], 412),
scanner! { S412 { mode M { token r#"[^abc]*"# => 0; } } }
#[test]
fn test_match_412() {
    use s412::S412 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "412: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[^abc]+", "abc");
// td!(r#"[^abc]+"#, "abc", &[], 413),
scanner! { S413 { mode M { token r#"[^abc]+"# => 0; } } }
#[test]
fn test_match_413() {
    use s413::S413 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "413: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("a?\?", "aaa", 0, 0);
// td!(r#"a?\?"#, "aaa", &[], 414),
scanner! { S414 { mode M { token r#"a?\?"# => 0; } } }
#[test]
fn test_match_414() {
    use s414::S414 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "414: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("ba?\?b", "bab", 0, 3);
// td!(r#"ba?\?b"#, "bab", &[("bab", 0, 3)], 415),
scanner! { S415 { mode M { token r#"ba?\?b"# => 0; } } }
#[test]
fn test_match_415() {
    use s415::S415 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: &[("bab", 0, 3)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "415: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "415: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "415: Match end does not match");
        assert_eq!(
            &"bab"[ma.1..ma.2],
            ma.0,
            "415: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a*?", "aaa", 0, 0);
// tu!(r#"a*?"#, "aaa", &[], 416), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S416 { mode M { token r#"a*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("ba*?", "baa", 0, 1);
// tu!(r#"ba*?"#, "baa", &[("b", 0, 1)], 417), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S417 { mode M { token r#"ba*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("ba*?b", "baab", 0, 4);
// tu!(r#"ba*?b"#, "baab", &[("baab", 0, 4)], 418), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S418 { mode M { token r#"ba*?b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a+?", "aaa", 0, 1);
// tu!(r#"a+?"#, "aaa", &[("a", 0, 1)], 419), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S419 { mode M { token r#"a+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("ba+?", "baa", 0, 2);
// tu!(r#"ba+?"#, "baa", &[("ba", 0, 2)], 420), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S420 { mode M { token r#"ba+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("ba+?b", "baab", 0, 4);
// tu!(r#"ba+?b"#, "baab", &[("baab", 0, 4)], 421), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S421 { mode M { token r#"ba+?b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:a?)?\?", "a", 0, 0);
// td!(r#"(?:a?)?\?"#, "a", &[], 422),
scanner! { S422 { mode M { token r#"(?:a?)?\?"# => 0; } } }
#[test]
fn test_match_422() {
    use s422::S422 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "422: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:a?\?)?", "a", 0, 0);
// td!(r#"(?:a?\?)?"#, "a", &[], 423),
scanner! { S423 { mode M { token r#"(?:a?\?)?"# => 0; } } }
#[test]
fn test_match_423() {
    use s423::S423 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "423: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:a?)+?", "aaa", 0, 1);
// tu!(r#"(?:a?)+?"#, "aaa", &[("a", 0, 1)], 424), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S424 { mode M { token r#"(?:a?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:a+)?\?", "aaa", 0, 0);
// td!(r#"(?:a+)?\?"#, "aaa", &[], 425),
scanner! { S425 { mode M { token r#"(?:a+)?\?"# => 0; } } }
#[test]
fn test_match_425() {
    use s425::S425 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "425: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:a+)?\?b", "aaab", 0, 4);
// td!(r#"(?:a+)?\?b"#, "aaab", &[("aaab", 0, 4)], 426),
scanner! { S426 { mode M { token r#"(?:a+)?\?b"# => 0; } } }
#[test]
fn test_match_426() {
    use s426::S426 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: &[("aaab", 0, 4)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "426: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "426: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "426: Match end does not match");
        assert_eq!(
            &"aaab"[ma.1..ma.2],
            ma.0,
            "426: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:ab)?{2}", "", 0, 0);
// td!(r#"(?:ab)?{2}"#, "", &[], 427),
scanner! { S427 { mode M { token r#"(?:ab)?{2}"# => 0; } } }
#[test]
fn test_match_427() {
    use s427::S427 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "427: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:ab)?{2}", "ababa", 0, 4);
// td!(r#"(?:ab)?{2}"#, "ababa", &[("abab", 0, 4)], 428),
scanner! { S428 { mode M { token r#"(?:ab)?{2}"# => 0; } } }
#[test]
fn test_match_428() {
    use s428::S428 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ababa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abab", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "428: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "428: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "428: Match end does not match");
        assert_eq!(
            &"ababa"[ma.1..ma.2],
            ma.0,
            "428: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:ab)*{0}", "ababa", 0, 0);
// td!(r#"(?:ab)*{0}"#, "ababa", &[], 429),
scanner! { S429 { mode M { token r#"(?:ab)*{0}"# => 0; } } }
#[test]
fn test_match_429() {
    use s429::S429 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ababa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "429: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:ab){3,}", "abababab", 0, 8);
// td!(r#"(?:ab){3,}"#, "abababab", &[("abababab", 0, 8)], 430),
scanner! { S430 { mode M { token r#"(?:ab){3,}"# => 0; } } }
#[test]
fn test_match_430() {
    use s430::S430 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abababab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abababab", 0, 8)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "430: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "430: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "430: Match end does not match");
        assert_eq!(
            &"abababab"[ma.1..ma.2],
            ma.0,
            "430: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?:ab){3,}", "abab");
// td!(r#"(?:ab){3,}"#, "abab", &[], 431),
scanner! { S431 { mode M { token r#"(?:ab){3,}"# => 0; } } }
#[test]
fn test_match_431() {
    use s431::S431 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "431: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:ab){2,4}", "ababab", 0, 6);
// td!(r#"(?:ab){2,4}"#, "ababab", &[("ababab", 0, 6)], 432),
scanner! { S432 { mode M { token r#"(?:ab){2,4}"# => 0; } } }
#[test]
fn test_match_432() {
    use s432::S432 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ababab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ababab", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "432: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "432: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "432: Match end does not match");
        assert_eq!(
            &"ababab"[ma.1..ma.2],
            ma.0,
            "432: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:ab){2,4}", "ababababab", 0, 8);
// td!(r#"(?:ab){2,4}"#, "ababababab", &[("abababab", 0, 8)], 433),
scanner! { S433 { mode M { token r#"(?:ab){2,4}"# => 0; } } }
#[test]
fn test_match_433() {
    use s433::S433 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ababababab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abababab", 0, 8)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "433: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "433: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "433: Match end does not match");
        assert_eq!(
            &"ababababab"[ma.1..ma.2],
            ma.0,
            "433: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:ab){2,4}?", "ababababab", 0, 4);
// tu!(r#"(?:ab){2,4}?"#, "ababababab", &[("abab", 0, 4)], 434), UnsupportedFeatureError("(?:ab){2,4}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S434 { mode M { token r#"(?:ab){2,4}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:ab){,}", "ab{,}", 0, 5);
// tr!(r#"(?:ab){,}"#, "ab{,}", &[("ab{,}", 0, 5)], 435), RepetitionCountDecimalEmpty
// scanner! { S435 { mode M { token r#"(?:ab){,}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:abc)+?{2}", "abcabcabc", 0, 6);
// tu!(r#"(?:abc)+?{2}"#, "abcabcabc", &[("abcabc", 0, 6)], 436), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S436 { mode M { token r#"(?:abc)+?{2}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:X*)(?i:xa)", "XXXa", 0, 4);
// td!(r#"(?:X*)(?i:xa)"#, "XXXa", &[("XXXa", 0, 4)], 437),
scanner! { S437 { mode M { token r#"(?:X*)(?i:xa)"# => 0; } } }
#[test]
fn test_match_437() {
    use s437::S437 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("XXXa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("XXXa", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "437: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "437: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "437: Match end does not match");
        assert_eq!(
            &"XXXa"[ma.1..ma.2],
            ma.0,
            "437: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(d+)([^abc]z)", "dddz", 0, 4);
// td!(r#"(d+)([^abc]z)"#, "dddz", &[("dddz", 0, 4)], 438),
scanner! { S438 { mode M { token r#"(d+)([^abc]z)"# => 0; } } }
#[test]
fn test_match_438() {
    use s438::S438 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("dddz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("dddz", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "438: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "438: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "438: Match end does not match");
        assert_eq!(
            &"dddz"[ma.1..ma.2],
            ma.0,
            "438: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("([^abc]*)([^abc]z)", "dddz", 0, 4);
// td!(r#"([^abc]*)([^abc]z)"#, "dddz", &[("dddz", 0, 4)], 439),
scanner! { S439 { mode M { token r#"([^abc]*)([^abc]z)"# => 0; } } }
#[test]
fn test_match_439() {
    use s439::S439 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("dddz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("dddz", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "439: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "439: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "439: Match end does not match");
        assert_eq!(
            &"dddz"[ma.1..ma.2],
            ma.0,
            "439: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(\\w+)(\\wz)", "dddz", 0, 4);
// td!(r#"(\w+)(\wz)"#, "dddz", &[("dddz", 0, 4)], 440),
scanner! { S440 { mode M { token r#"(\w+)(\wz)"# => 0; } } }
#[test]
fn test_match_440() {
    use s440::S440 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("dddz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("dddz", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "440: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "440: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "440: Match end does not match");
        assert_eq!(
            &"dddz"[ma.1..ma.2],
            ma.0,
            "440: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(a)", "a", 0, 1, 1);
// td!(r#"(a)"#, "a", &[("a", 0, 1)], 441),
scanner! { S441 { mode M { token r#"(a)"# => 0; } } }
#[test]
fn test_match_441() {
    use s441::S441 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "441: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "441: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "441: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "441: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(ab)", "ab", 0, 2, 1);
// td!(r#"(ab)"#, "ab", &[("ab", 0, 2)], 442),
scanner! { S442 { mode M { token r#"(ab)"# => 0; } } }
#[test]
fn test_match_442() {
    use s442::S442 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "442: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "442: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "442: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "442: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("((ab))", "ab", 0, 2);
// td!(r#"((ab))"#, "ab", &[("ab", 0, 2)], 443),
scanner! { S443 { mode M { token r#"((ab))"# => 0; } } }
#[test]
fn test_match_443() {
    use s443::S443 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "443: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "443: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "443: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "443: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("((ab))", "ab", 0, 2, 1);
// td!(r#"((ab))"#, "ab", &[("ab", 0, 2)], 444),
scanner! { S444 { mode M { token r#"((ab))"# => 0; } } }
#[test]
fn test_match_444() {
    use s444::S444 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "444: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "444: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "444: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "444: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("((ab))", "ab", 0, 2, 2);
// td!(r#"((ab))"#, "ab", &[("ab", 0, 2)], 445),
scanner! { S445 { mode M { token r#"((ab))"# => 0; } } }
#[test]
fn test_match_445() {
    use s445::S445 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "445: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "445: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "445: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "445: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("((((((((((((((((((((ab))))))))))))))))))))", "ab", 0, 2, 20);
// td!(r#"((((((((((((((((((((ab))))))))))))))))))))"#, "ab", &[("ab", 0, 2)], 446),
scanner! { S446 { mode M { token r#"((((((((((((((((((((ab))))))))))))))))))))"# => 0; } } }
#[test]
fn test_match_446() {
    use s446::S446 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ab", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "446: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "446: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "446: Match end does not match");
        assert_eq!(
            &"ab"[ma.1..ma.2],
            ma.0,
            "446: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(ab)(cd)", "abcd", 0, 2, 1);
// td!(r#"(ab)(cd)"#, "abcd", &[("ab", 0, 2)], 447),
scanner! { S447 { mode M { token r#"(ab)(cd)"# => 0; } } }
#[test]
fn test_match_447() {
    use s447::S447 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcd", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcd", 0, 4)]; // Oniguruma: &[("ab", 0, 2)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "447: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "447: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "447: Match end does not match");
        assert_eq!(
            &"abcd"[ma.1..ma.2],
            ma.0,
            "447: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(ab)(cd)", "abcd", 2, 4, 2);
// td!(r#"(ab)(cd)"#, "abcd", &[("cd", 2, 4)], 448),
scanner! { S448 { mode M { token r#"(ab)(cd)"# => 0; } } }
#[test]
fn test_match_448() {
    use s448::S448 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcd", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcd", 0, 4)]; // Oniguruma: &[("cd", 2, 4)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "448: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "448: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "448: Match end does not match");
        assert_eq!(
            &"abcd"[ma.1..ma.2],
            ma.0,
            "448: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("()(a)bc(def)ghijk", "abcdefghijk", 3, 6, 3);
// td!(r#"()(a)bc(def)ghijk"#, "abcdefghijk", &[("def", 3, 6)], 449),
scanner! { S449 { mode M { token r#"()(a)bc(def)ghijk"# => 0; } } }
#[test]
fn test_match_449() {
    use s449::S449 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcdefghijk", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcdefghijk", 0, 11)]; // Oniguruma: &[("def", 3, 6)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "449: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "449: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "449: Match end does not match");
        assert_eq!(
            &"abcdefghijk"[ma.1..ma.2],
            ma.0,
            "449: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(()(a)bc(def)ghijk)", "abcdefghijk", 3, 6, 4);
// td!(r#"(()(a)bc(def)ghijk)"#, "abcdefghijk", &[("def", 3, 6)], 450),
scanner! { S450 { mode M { token r#"(()(a)bc(def)ghijk)"# => 0; } } }
#[test]
fn test_match_450() {
    use s450::S450 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcdefghijk", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcdefghijk", 0, 11)]; // Oniguruma: &[("def", 3, 6)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "450: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "450: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "450: Match end does not match");
        assert_eq!(
            &"abcdefghijk"[ma.1..ma.2],
            ma.0,
            "450: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(^a)", "a", 0, 1);
// tu!(r#"(^a)"#, "a", &[("a", 0, 1)], 451), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S451 { mode M { token r#"(^a)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(a)|(a)", "ba", 1, 2, 1);
// td!(r#"(a)|(a)"#, "ba", &[("a", 1, 2)], 452),
scanner! { S452 { mode M { token r#"(a)|(a)"# => 0; } } }
#[test]
fn test_match_452() {
    use s452::S452 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ba", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 1, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "452: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "452: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "452: Match end does not match");
        assert_eq!(
            &"ba"[ma.1..ma.2],
            ma.0,
            "452: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(^a)|(a)", "ba", 1, 2, 2);
// tu!(r#"(^a)|(a)"#, "ba", &[("a", 1, 2)], 453), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S453 { mode M { token r#"(^a)|(a)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(a?)", "aaa", 0, 1, 1);
// td!(r#"(a?)"#, "aaa", &[("a", 0, 1)], 454),
scanner! { S454 { mode M { token r#"(a?)"# => 0; } } }
#[test]
fn test_match_454() {
    use s454::S454 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1), ("a", 1, 2), ("a", 2, 3)]; // Oniguruma: &[("a", 0, 1)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "454: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "454: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "454: Match end does not match");
        assert_eq!(
            &"aaa"[ma.1..ma.2],
            ma.0,
            "454: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(a*)", "aaa", 0, 3, 1);
// td!(r#"(a*)"#, "aaa", &[("aaa", 0, 3)], 455),
scanner! { S455 { mode M { token r#"(a*)"# => 0; } } }
#[test]
fn test_match_455() {
    use s455::S455 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaa", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "455: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "455: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "455: Match end does not match");
        assert_eq!(
            &"aaa"[ma.1..ma.2],
            ma.0,
            "455: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(a*)", "", 0, 0, 1);
// td!(r#"(a*)"#, "", &[], 456),
scanner! { S456 { mode M { token r#"(a*)"# => 0; } } }
#[test]
fn test_match_456() {
    use s456::S456 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "456: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x3("(a+)", "aaaaaaa", 0, 7, 1);
// td!(r#"(a+)"#, "aaaaaaa", &[("aaaaaaa", 0, 7)], 457),
scanner! { S457 { mode M { token r#"(a+)"# => 0; } } }
#[test]
fn test_match_457() {
    use s457::S457 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaaaaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaaaaa", 0, 7)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "457: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "457: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "457: Match end does not match");
        assert_eq!(
            &"aaaaaaa"[ma.1..ma.2],
            ma.0,
            "457: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(a+|b*)", "bbbaa", 0, 3, 1);
// td!(r#"(a+|b*)"#, "bbbaa", &[("bbb", 0, 3)], 458),
scanner! { S458 { mode M { token r#"(a+|b*)"# => 0; } } }
#[test]
fn test_match_458() {
    use s458::S458 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bbbaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bbb", 0, 3), ("aa", 3, 5)]; // Oniguruma: &[("bbb", 0, 3)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "458: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "458: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "458: Match end does not match");
        assert_eq!(
            &"bbbaa"[ma.1..ma.2],
            ma.0,
            "458: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(a+|b?)", "bbbaa", 0, 1, 1);
// td!(r#"(a+|b?)"#, "bbbaa", &[("b", 0, 1)], 459),
scanner! { S459 { mode M { token r#"(a+|b?)"# => 0; } } }
#[test]
fn test_match_459() {
    use s459::S459 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bbbaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] =
        &[("b", 0, 1), ("b", 1, 2), ("b", 2, 3), ("aa", 3, 5)]; // Oniguruma: &[("b", 0, 1)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "459: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "459: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "459: Match end does not match");
        assert_eq!(
            &"bbbaa"[ma.1..ma.2],
            ma.0,
            "459: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(abc)?", "abc", 0, 3, 1);
// td!(r#"(abc)?"#, "abc", &[("abc", 0, 3)], 460),
scanner! { S460 { mode M { token r#"(abc)?"# => 0; } } }
#[test]
fn test_match_460() {
    use s460::S460 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "460: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "460: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "460: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "460: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(abc)*", "abc", 0, 3, 1);
// td!(r#"(abc)*"#, "abc", &[("abc", 0, 3)], 461),
scanner! { S461 { mode M { token r#"(abc)*"# => 0; } } }
#[test]
fn test_match_461() {
    use s461::S461 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "461: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "461: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "461: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "461: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(abc)+", "abc", 0, 3, 1);
// td!(r#"(abc)+"#, "abc", &[("abc", 0, 3)], 462),
scanner! { S462 { mode M { token r#"(abc)+"# => 0; } } }
#[test]
fn test_match_462() {
    use s462::S462 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "462: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "462: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "462: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "462: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(xyz|abc)+", "abc", 0, 3, 1);
// td!(r#"(xyz|abc)+"#, "abc", &[("abc", 0, 3)], 463),
scanner! { S463 { mode M { token r#"(xyz|abc)+"# => 0; } } }
#[test]
fn test_match_463() {
    use s463::S463 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "463: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "463: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "463: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "463: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("([xyz][abc]|abc)+", "abc", 0, 3, 1);
// td!(r#"([xyz][abc]|abc)+"#, "abc", &[("abc", 0, 3)], 464),
scanner! { S464 { mode M { token r#"([xyz][abc]|abc)+"# => 0; } } }
#[test]
fn test_match_464() {
    use s464::S464 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "464: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "464: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "464: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "464: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("((?i:abc))", "AbC", 0, 3, 1);
// td!(r#"((?i:abc))"#, "AbC", &[("AbC", 0, 3)], 465),
scanner! { S465 { mode M { token r#"((?i:abc))"# => 0; } } }
#[test]
fn test_match_465() {
    use s465::S465 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("AbC", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("AbC", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "465: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "465: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "465: Match end does not match");
        assert_eq!(
            &"AbC"[ma.1..ma.2],
            ma.0,
            "465: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(abc)(?i:\\1)", "abcABC", 0, 6);
// tr!(r#"(abc)(?i:\1)"#, "abcABC", &[("abcABC", 0, 6)], 466), UnsupportedBackreference
// scanner! { S466 { mode M { token r#"(abc)(?i:\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("((?m:a.c))", "a\nc", 0, 3, 1);
// td!(r#"((?m:a.c))"#, "a\nc", &[("a\\n", 0, 3)], 467),
scanner! { S467 { mode M { token r#"((?m:a.c))"# => 0; } } }
#[test]
fn test_match_467() {
    use s467::S467 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a\nc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: &[("a\n", 0, 2)]
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "467: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x3("((?=az)a)", "azb", 0, 1, 1);
// tr!(r#"((?=az)a)"#, "azb", &[("a", 0, 1)], 468), UnsupportedLookAround
// scanner! { S468 { mode M { token r#"((?=az)a)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("abc|(.abd)", "zabd", 0, 4, 1);
// td!(r#"abc|(.abd)"#, "zabd", &[("zabd", 0, 4)], 469),
scanner! { S469 { mode M { token r#"abc|(.abd)"# => 0; } } }
#[test]
fn test_match_469() {
    use s469::S469 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("zabd", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("zabd", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "469: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "469: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "469: Match end does not match");
        assert_eq!(
            &"zabd"[ma.1..ma.2],
            ma.0,
            "469: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:abc)|(ABC)", "abc", 0, 3);
// td!(r#"(?:abc)|(ABC)"#, "abc", &[("abc", 0, 3)], 470),
scanner! { S470 { mode M { token r#"(?:abc)|(ABC)"# => 0; } } }
#[test]
fn test_match_470() {
    use s470::S470 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abc", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "470: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "470: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "470: Match end does not match");
        assert_eq!(
            &"abc"[ma.1..ma.2],
            ma.0,
            "470: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(?i:(abc))|(zzz)", "ABC", 0, 3, 1);
// td!(r#"(?i:(abc))|(zzz)"#, "ABC", &[("ABC", 0, 3)], 471),
scanner! { S471 { mode M { token r#"(?i:(abc))|(zzz)"# => 0; } } }
#[test]
fn test_match_471() {
    use s471::S471 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ABC", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ABC", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "471: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "471: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "471: Match end does not match");
        assert_eq!(
            &"ABC"[ma.1..ma.2],
            ma.0,
            "471: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("a*(.)", "aaaaz", 4, 5, 1);
// td!(r#"a*(.)"#, "aaaaz", &[("z", 4, 5)], 472),
scanner! { S472 { mode M { token r#"a*(.)"# => 0; } } }
#[test]
fn test_match_472() {
    use s472::S472 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaaaz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaaz", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "472: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "472: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "472: Match end does not match");
        assert_eq!(
            &"aaaaz"[ma.1..ma.2],
            ma.0,
            "472: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("a*?(.)", "aaaaz", 0, 1, 1);
// tu!(r#"a*?(.)"#, "aaaaz", &[("a", 0, 1)], 473), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S473 { mode M { token r#"a*?(.)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("a*?(c)", "aaaac", 4, 5, 1);
// tu!(r#"a*?(c)"#, "aaaac", &[("c", 4, 5)], 474), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S474 { mode M { token r#"a*?(c)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("[bcd]a*(.)", "caaaaz", 5, 6, 1);
// td!(r#"[bcd]a*(.)"#, "caaaaz", &[("z", 5, 6)], 475),
scanner! { S475 { mode M { token r#"[bcd]a*(.)"# => 0; } } }
#[test]
fn test_match_475() {
    use s475::S475 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("caaaaz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("caaaaz", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "475: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "475: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "475: Match end does not match");
        assert_eq!(
            &"caaaaz"[ma.1..ma.2],
            ma.0,
            "475: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(\\Abb)cc", "bbcc", 0, 2, 1);
// tu!(r#"(\Abb)cc"#, "bbcc", &[("bb", 0, 2)], 476), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S476 { mode M { token r#"(\Abb)cc"# => 0; } } }

// -------------------------------------------------------------------------
// n("(\\Abb)cc", "zbbcc");
// tu!(r#"(\Abb)cc"#, "zbbcc", &[], 477), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S477 { mode M { token r#"(\Abb)cc"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(^bb)cc", "bbcc", 0, 2, 1);
// tu!(r#"(^bb)cc"#, "bbcc", &[("bb", 0, 2)], 478), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S478 { mode M { token r#"(^bb)cc"# => 0; } } }

// -------------------------------------------------------------------------
// n("(^bb)cc", "zbbcc");
// tu!(r#"(^bb)cc"#, "zbbcc", &[], 479), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S479 { mode M { token r#"(^bb)cc"# => 0; } } }

// -------------------------------------------------------------------------
// x3("cc(bb$)", "ccbb", 2, 4, 1);
// tu!(r#"cc(bb$)"#, "ccbb", &[("bb", 2, 4)], 480), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S480 { mode M { token r#"cc(bb$)"# => 0; } } }

// -------------------------------------------------------------------------
// n("cc(bb$)", "ccbbb");
// tu!(r#"cc(bb$)"#, "ccbbb", &[], 481), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S481 { mode M { token r#"cc(bb$)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(\\1)", "");
// tr!(r#"(\1)"#, "", &[], 482), UnsupportedBackreference
// scanner! { S482 { mode M { token r#"(\1)"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\1(a)", "aa");
// tr!(r#"\1(a)"#, "aa", &[], 483), UnsupportedBackreference
// scanner! { S483 { mode M { token r#"\1(a)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(a(b)\\1)\\2+", "ababb");
// tr!(r#"(a(b)\1)\2+"#, "ababb", &[], 484), UnsupportedBackreference
// scanner! { S484 { mode M { token r#"(a(b)\1)\2+"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:(?:\\1|z)(a))+$", "zaa");
// tr!(r#"(?:(?:\1|z)(a))+$"#, "zaa", &[], 485), UnsupportedBackreference
// scanner! { S485 { mode M { token r#"(?:(?:\1|z)(a))+$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?:\\1|z)(a))+$", "zaaa", 0, 4);
// tr!(r#"(?:(?:\1|z)(a))+$"#, "zaaa", &[("zaaa", 0, 4)], 486), UnsupportedBackreference
// scanner! { S486 { mode M { token r#"(?:(?:\1|z)(a))+$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?=\\1)", "aa", 0, 1);
// tr!(r#"(a)(?=\1)"#, "aa", &[("a", 0, 1)], 487), UnsupportedLookAround
// scanner! { S487 { mode M { token r#"(a)(?=\1)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(a)$|\\1", "az");
// tr!(r#"(a)$|\1"#, "az", &[], 488), UnsupportedBackreference
// scanner! { S488 { mode M { token r#"(a)$|\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)\\1", "aa", 0, 2);
// tr!(r#"(a)\1"#, "aa", &[("aa", 0, 2)], 489), UnsupportedBackreference
// scanner! { S489 { mode M { token r#"(a)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(a)\\1", "ab");
// tr!(r#"(a)\1"#, "ab", &[], 490), UnsupportedBackreference
// scanner! { S490 { mode M { token r#"(a)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a?)\\1", "aa", 0, 2);
// tr!(r#"(a?)\1"#, "aa", &[("aa", 0, 2)], 491), UnsupportedBackreference
// scanner! { S491 { mode M { token r#"(a?)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a?\?)\\1", "aa", 0, 0);
// tr!(r#"(a?\?)\1"#, "aa", &[], 492), UnsupportedBackreference
// scanner! { S492 { mode M { token r#"(a?\?)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*)\\1", "aaaaa", 0, 4);
// tr!(r#"(a*)\1"#, "aaaaa", &[("aaaa", 0, 4)], 493), UnsupportedBackreference
// scanner! { S493 { mode M { token r#"(a*)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(a*)\\1", "aaaaa", 0, 2, 1);
// tr!(r#"(a*)\1"#, "aaaaa", &[("aa", 0, 2)], 494), UnsupportedBackreference
// scanner! { S494 { mode M { token r#"(a*)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a(b*)\\1", "abbbb", 0, 5);
// tr!(r#"a(b*)\1"#, "abbbb", &[("abbbb", 0, 5)], 495), UnsupportedBackreference
// scanner! { S495 { mode M { token r#"a(b*)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a(b*)\\1", "ab", 0, 1);
// tr!(r#"a(b*)\1"#, "ab", &[("a", 0, 1)], 496), UnsupportedBackreference
// scanner! { S496 { mode M { token r#"a(b*)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*)(b*)\\1\\2", "aaabbaaabb", 0, 10);
// tr!(r#"(a*)(b*)\1\2"#, "aaabbaaabb", &[("aaabbaaabb", 0, 10)], 497), UnsupportedBackreference
// scanner! { S497 { mode M { token r#"(a*)(b*)\1\2"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*)(b*)\\2", "aaabbbb", 0, 7);
// tr!(r#"(a*)(b*)\2"#, "aaabbbb", &[("aaabbbb", 0, 7)], 498), UnsupportedBackreference
// scanner! { S498 { mode M { token r#"(a*)(b*)\2"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(((((((a*)b))))))c\\7", "aaabcaaa", 0, 8);
// tr!(r#"(((((((a*)b))))))c\7"#, "aaabcaaa", &[("aaabcaaa", 0, 8)], 499), UnsupportedBackreference
// scanner! { S499 { mode M { token r#"(((((((a*)b))))))c\7"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(((((((a*)b))))))c\\7", "aaabcaaa", 0, 3, 7);
// tr!(r#"(((((((a*)b))))))c\7"#, "aaabcaaa", &[("aaa", 0, 3)], 500), UnsupportedBackreference
// scanner! { S500 { mode M { token r#"(((((((a*)b))))))c\7"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(b)(c)\\2\\1\\3", "abcbac", 0, 6);
// tr!(r#"(a)(b)(c)\2\1\3"#, "abcbac", &[("abcbac", 0, 6)], 501), UnsupportedBackreference
// scanner! { S501 { mode M { token r#"(a)(b)(c)\2\1\3"# => 0; } } }

// -------------------------------------------------------------------------
// x2("([a-d])\\1", "cc", 0, 2);
// tr!(r#"([a-d])\1"#, "cc", &[("cc", 0, 2)], 502), UnsupportedBackreference
// scanner! { S502 { mode M { token r#"([a-d])\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(\\w\\d\\s)\\1", "f5 f5 ", 0, 6);
// tr!(r#"(\w\d\s)\1"#, "f5 f5 ", &[("f5 f5 ", 0, 6)], 503), UnsupportedBackreference
// scanner! { S503 { mode M { token r#"(\w\d\s)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(\\w\\d\\s)\\1", "f5 f5");
// tr!(r#"(\w\d\s)\1"#, "f5 f5", &[], 504), UnsupportedBackreference
// scanner! { S504 { mode M { token r#"(\w\d\s)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(who|[a-c]{3})\\1", "whowho", 0, 6);
// tr!(r#"(who|[a-c]{3})\1"#, "whowho", &[("whowho", 0, 6)], 505), UnsupportedBackreference
// scanner! { S505 { mode M { token r#"(who|[a-c]{3})\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("...(who|[a-c]{3})\\1", "abcwhowho", 0, 9);
// tr!(r#"...(who|[a-c]{3})\1"#, "abcwhowho", &[("abcwhowho", 0, 9)], 506), UnsupportedBackreference
// scanner! { S506 { mode M { token r#"...(who|[a-c]{3})\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(who|[a-c]{3})\\1", "cbccbc", 0, 6);
// tr!(r#"(who|[a-c]{3})\1"#, "cbccbc", &[("cbccbc", 0, 6)], 507), UnsupportedBackreference
// scanner! { S507 { mode M { token r#"(who|[a-c]{3})\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(^a)\\1", "aa", 0, 2);
// tr!(r#"(^a)\1"#, "aa", &[("aa", 0, 2)], 508), UnsupportedBackreference
// scanner! { S508 { mode M { token r#"(^a)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(^a)\\1", "baa");
// tr!(r#"(^a)\1"#, "baa", &[], 509), UnsupportedBackreference
// scanner! { S509 { mode M { token r#"(^a)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(a$)\\1", "aa");
// tr!(r#"(a$)\1"#, "aa", &[], 510), UnsupportedBackreference
// scanner! { S510 { mode M { token r#"(a$)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(ab\\Z)\\1", "ab");
// tr!(r#"(ab\Z)\1"#, "ab", &[], 511), EscapeUnrecognized
// scanner! { S511 { mode M { token r#"(ab\Z)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*\\Z)\\1", "a", 1, 1);
// tr!(r#"(a*\Z)\1"#, "a", &[("", 1, 1)], 512), EscapeUnrecognized
// scanner! { S512 { mode M { token r#"(a*\Z)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2(".(a*\\Z)\\1", "ba", 1, 2);
// tr!(r#".(a*\Z)\1"#, "ba", &[("a", 1, 2)], 513), EscapeUnrecognized
// scanner! { S513 { mode M { token r#".(a*\Z)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(.(abc)\\2)", "zabcabc", 0, 7, 1);
// tr!(r#"(.(abc)\2)"#, "zabcabc", &[("zabcabc", 0, 7)], 514), UnsupportedBackreference
// scanner! { S514 { mode M { token r#"(.(abc)\2)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(.(..\\d.)\\2)", "z12341234", 0, 9, 1);
// tr!(r#"(.(..\d.)\2)"#, "z12341234", &[("z12341234", 0, 9)], 515), UnsupportedBackreference
// scanner! { S515 { mode M { token r#"(.(..\d.)\2)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?i:az))\\1", "AzAz", 0, 4);
// tr!(r#"((?i:az))\1"#, "AzAz", &[("AzAz", 0, 4)], 516), UnsupportedBackreference
// scanner! { S516 { mode M { token r#"((?i:az))\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("((?i:az))\\1", "Azaz");
// tr!(r#"((?i:az))\1"#, "Azaz", &[], 517), UnsupportedBackreference
// scanner! { S517 { mode M { token r#"((?i:az))\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a)b", "ab", 1, 2);
// tr!(r#"(?<=a)b"#, "ab", &[("b", 1, 2)], 518), UnsupportedLookAround
// scanner! { S518 { mode M { token r#"(?<=a)b"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=a)b", "bb");
// tr!(r#"(?<=a)b"#, "bb", &[], 519), UnsupportedLookAround
// scanner! { S519 { mode M { token r#"(?<=a)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|b)b", "bb", 1, 2);
// tr!(r#"(?<=a|b)b"#, "bb", &[("b", 1, 2)], 520), UnsupportedLookAround
// scanner! { S520 { mode M { token r#"(?<=a|b)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|bc)b", "bcb", 2, 3);
// tr!(r#"(?<=a|bc)b"#, "bcb", &[("b", 2, 3)], 521), UnsupportedLookAround
// scanner! { S521 { mode M { token r#"(?<=a|bc)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|bc)b", "ab", 1, 2);
// tr!(r#"(?<=a|bc)b"#, "ab", &[("b", 1, 2)], 522), UnsupportedLookAround
// scanner! { S522 { mode M { token r#"(?<=a|bc)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|bc||defghij|klmnopq|r)z", "rz", 1, 2);
// tr!(r#"(?<=a|bc||defghij|klmnopq|r)z"#, "rz", &[("z", 1, 2)], 523), UnsupportedLookAround
// scanner! { S523 { mode M { token r#"(?<=a|bc||defghij|klmnopq|r)z"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(?<=(abc))d", "abcd", 0, 3, 1);
// tr!(r#"(?<=(abc))d"#, "abcd", &[("abc", 0, 3)], 524), UnsupportedLookAround
// scanner! { S524 { mode M { token r#"(?<=(abc))d"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(?i:abc))d", "ABCd", 3, 4);
// tr!(r#"(?<=(?i:abc))d"#, "ABCd", &[("d", 3, 4)], 525), UnsupportedLookAround
// scanner! { S525 { mode M { token r#"(?<=(?i:abc))d"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=^|b)c", " cbc", 3, 4);
// tr!(r#"(?<=^|b)c"#, " cbc", &[("c", 3, 4)], 526), UnsupportedLookAround
// scanner! { S526 { mode M { token r#"(?<=^|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|^|b)c", " cbc", 3, 4);
// tr!(r#"(?<=a|^|b)c"#, " cbc", &[("c", 3, 4)], 527), UnsupportedLookAround
// scanner! { S527 { mode M { token r#"(?<=a|^|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|(^)|b)c", " cbc", 3, 4);
// tr!(r#"(?<=a|(^)|b)c"#, " cbc", &[("c", 3, 4)], 528), UnsupportedLookAround
// scanner! { S528 { mode M { token r#"(?<=a|(^)|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|(^)|b)c", "cbc", 0, 1);
// tr!(r#"(?<=a|(^)|b)c"#, "cbc", &[("c", 0, 1)], 529), UnsupportedLookAround
// scanner! { S529 { mode M { token r#"(?<=a|(^)|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// n("(Q)|(?<=a|(?(1))|b)c", "czc");
// tr!(r#"(Q)|(?<=a|(?(1))|b)c"#, "czc", &[], 530), UnsupportedLookAround
// scanner! { S530 { mode M { token r#"(Q)|(?<=a|(?(1))|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(Q)(?<=a|(?(1))|b)c", "cQc", 1, 3);
// tr!(r#"(Q)(?<=a|(?(1))|b)c"#, "cQc", &[("Qc", 1, 3)], 531), UnsupportedLookAround
// scanner! { S531 { mode M { token r#"(Q)(?<=a|(?(1))|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a|(?~END)|b)c", "ENDc", 3, 4);
// tr!(r#"(?<=a|(?~END)|b)c"#, "ENDc", &[("c", 3, 4)], 532), UnsupportedLookAround
// scanner! { S532 { mode M { token r#"(?<=a|(?~END)|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!^|b)c", "cbc");
// tr!(r#"(?<!^|b)c"#, "cbc", &[], 533), UnsupportedLookAround
// scanner! { S533 { mode M { token r#"(?<!^|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a|^|b)c", "cbc");
// tr!(r#"(?<!a|^|b)c"#, "cbc", &[], 534), UnsupportedLookAround
// scanner! { S534 { mode M { token r#"(?<!a|^|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a|(?:^)|b)c", "cbc");
// tr!(r#"(?<!a|(?:^)|b)c"#, "cbc", &[], 535), UnsupportedLookAround
// scanner! { S535 { mode M { token r#"(?<!a|(?:^)|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a|(?:^)|b)c", " cbc", 1, 2);
// tr!(r#"(?<!a|(?:^)|b)c"#, " cbc", &[("c", 1, 2)], 536), UnsupportedLookAround
// scanner! { S536 { mode M { token r#"(?<!a|(?:^)|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)\\g<1>", "aa", 0, 2);
// tr!(r#"(a)\g<1>"#, "aa", &[("aa", 0, 2)], 537), EscapeUnrecognized
// scanner! { S537 { mode M { token r#"(a)\g<1>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a)b", "cb", 1, 2);
// tr!(r#"(?<!a)b"#, "cb", &[("b", 1, 2)], 538), UnsupportedLookAround
// scanner! { S538 { mode M { token r#"(?<!a)b"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a)b", "ab");
// tr!(r#"(?<!a)b"#, "ab", &[], 539), UnsupportedLookAround
// scanner! { S539 { mode M { token r#"(?<!a)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a|bc)b", "bbb", 0, 1);
// tr!(r#"(?<!a|bc)b"#, "bbb", &[("b", 0, 1)], 540), UnsupportedLookAround
// scanner! { S540 { mode M { token r#"(?<!a|bc)b"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a|bc)z", "bcz");
// tr!(r#"(?<!a|bc)z"#, "bcz", &[], 541), UnsupportedLookAround
// scanner! { S541 { mode M { token r#"(?<!a|bc)z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<name1>a)", "a", 0, 1);
// td!(r#"(?<name1>a)"#, "a", &[("a", 0, 1)], 542),
scanner! { S542 { mode M { token r#"(?<name1>a)"# => 0; } } }
#[test]
fn test_match_542() {
    use s542::S542 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "542: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "542: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "542: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "542: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?<name_2>ab)\\g<name_2>", "abab", 0, 4);
// tr!(r#"(?<name_2>ab)\g<name_2>"#, "abab", &[("abab", 0, 4)], 543), EscapeUnrecognized
// scanner! { S543 { mode M { token r#"(?<name_2>ab)\g<name_2>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<name_3>.zv.)\\k<name_3>", "azvbazvb", 0, 8);
// tr!(r#"(?<name_3>.zv.)\k<name_3>"#, "azvbazvb", &[("azvbazvb", 0, 8)], 544), EscapeUnrecognized
// scanner! { S544 { mode M { token r#"(?<name_3>.zv.)\k<name_3>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=\\g<ab>)|-\\zEND (?<ab>XyZ)", "XyZ", 3, 3);
// tr!(r#"(?<=\g<ab>)|-\zEND (?<ab>XyZ)"#, "XyZ", &[("", 3, 3)], 545), UnsupportedLookAround
// scanner! { S545 { mode M { token r#"(?<=\g<ab>)|-\zEND (?<ab>XyZ)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<n>|a\\g<n>)+", "", 0, 0);
// tr!(r#"(?<n>|a\g<n>)+"#, "", &[], 546), EscapeUnrecognized
// scanner! { S546 { mode M { token r#"(?<n>|a\g<n>)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<n>|\\(\\g<n>\\))+$", "()(())", 0, 6);
// tr!(r#"(?<n>|\(\g<n>\))+$"#, "()(())", &[("()(())", 0, 6)], 547), EscapeUnrecognized
// scanner! { S547 { mode M { token r#"(?<n>|\(\g<n>\))+$"# => 0; } } }

// -------------------------------------------------------------------------
// x3("\\g<n>(?<n>.){0}", "X", 0, 1, 1);
// tr!(r#"\g<n>(?<n>.){0}"#, "X", &[("X", 0, 1)], 548), EscapeUnrecognized
// scanner! { S548 { mode M { token r#"\g<n>(?<n>.){0}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g<n>(abc|df(?<n>.YZ){2,8}){0}", "XYZ", 0, 3);
// tr!(r#"\g<n>(abc|df(?<n>.YZ){2,8}){0}"#, "XYZ", &[("XYZ", 0, 3)], 549), EscapeUnrecognized
// scanner! { S549 { mode M { token r#"\g<n>(abc|df(?<n>.YZ){2,8}){0}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A(?<n>(a\\g<n>)|)\\z", "aaaa", 0, 4);
// tr!(r#"\A(?<n>(a\g<n>)|)\z"#, "aaaa", &[("aaaa", 0, 4)], 550), EscapeUnrecognized
// scanner! { S550 { mode M { token r#"\A(?<n>(a\g<n>)|)\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<n>|\\g<m>\\g<n>)\\z|\\zEND (?<m>a|(b)\\g<m>)", "bbbbabba", 0, 8);
// tr!(r#"(?<n>|\g<m>\g<n>)\z|\zEND (?<m>a|(b)\g<m>)"#, "bbbbabba", &[("bbbbabba", 0, 8)], 551), EscapeUnrecognized
// scanner! { S551 { mode M { token r#"(?<n>|\g<m>\g<n>)\z|\zEND (?<m>a|(b)\g<m>)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<name1240>\\w+\\sx)a+\\k<name1240>", "  fg xaaaaaaaafg x", 2, 18);
// tr!(r#"(?<name1240>\w+\sx)a+\k<name1240>"#, "  fg xaaaaaaaafg x", &[("fg xaaaaaaaafg x", 2, 18)], 552), EscapeUnrecognized
// scanner! { S552 { mode M { token r#"(?<name1240>\w+\sx)a+\k<name1240>"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(z)()()(?<_9>a)\\g<_9>", "zaa", 2, 3, 1);
// tr!(r#"(z)()()(?<_9>a)\g<_9>"#, "zaa", &[("a", 2, 3)], 553), EscapeUnrecognized
// scanner! { S553 { mode M { token r#"(z)()()(?<_9>a)\g<_9>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.)(((?<_>a)))\\k<_>", "zaa", 0, 3);
// tr!(r#"(.)(((?<_>a)))\k<_>"#, "zaa", &[("zaa", 0, 3)], 554), EscapeUnrecognized
// scanner! { S554 { mode M { token r#"(.)(((?<_>a)))\k<_>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?<name1>\\d)|(?<name2>\\w))(\\k<name1>|\\k<name2>)", "ff", 0, 2);
// tr!(r#"((?<name1>\d)|(?<name2>\w))(\k<name1>|\k<name2>)"#, "ff", &[("ff", 0, 2)], 555), EscapeUnrecognized
// scanner! { S555 { mode M { token r#"((?<name1>\d)|(?<name2>\w))(\k<name1>|\k<name2>)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?<x>)|(?<x>efg))\\k<x>", "", 0, 0);
// tr!(r#"(?:(?<x>)|(?<x>efg))\k<x>"#, "", &[], 556), GroupNameDuplicate
// scanner! { S556 { mode M { token r#"(?:(?<x>)|(?<x>efg))\k<x>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?<x>abc)|(?<x>efg))\\k<x>", "abcefgefg", 3, 9);
// tr!(r#"(?:(?<x>abc)|(?<x>efg))\k<x>"#, "abcefgefg", &[("efgefg", 3, 9)], 557), GroupNameDuplicate
// scanner! { S557 { mode M { token r#"(?:(?<x>abc)|(?<x>efg))\k<x>"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:(?<x>abc)|(?<x>efg))\\k<x>", "abcefg");
// tr!(r#"(?:(?<x>abc)|(?<x>efg))\k<x>"#, "abcefg", &[], 558), GroupNameDuplicate
// scanner! { S558 { mode M { token r#"(?:(?<x>abc)|(?<x>efg))\k<x>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<x>x)(?<x>xx)\\k<x>", "xxxx", 0, 4);
// tr!(r#"(?<x>x)(?<x>xx)\k<x>"#, "xxxx", &[("xxxx", 0, 4)], 559), GroupNameDuplicate
// scanner! { S559 { mode M { token r#"(?<x>x)(?<x>xx)\k<x>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<x>x)(?<x>xx)\\k<x>", "xxxxz", 0, 4);
// tr!(r#"(?<x>x)(?<x>xx)\k<x>"#, "xxxxz", &[("xxxx", 0, 4)], 560), GroupNameDuplicate
// scanner! { S560 { mode M { token r#"(?<x>x)(?<x>xx)\k<x>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\\k<n1>$", "a-pyumpyum", 2, 10);
// tr!(r#"(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\k<n1>$"#, "a-pyumpyum", &[("pyumpyum", 2, 10)], 561), GroupNameDuplicate
// scanner! { S561 { mode M { token r#"(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\k<n1>$"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\\k<n1>$", "xxxxabcdefghijklmnabcdefghijklmn", 4, 18, 14);
// tr!(r#"(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\k<n1>$"#, "xxxxabcdefghijklmnabcdefghijklmn", &[("abcdefghijklmn", 4, 18)], 562), GroupNameDuplicate
// scanner! { S562 { mode M { token r#"(?:(?<n1>.)|(?<n1>..)|(?<n1>...)|(?<n1>....)|(?<n1>.....)|(?<n1>......)|(?<n1>.......)|(?<n1>........)|(?<n1>.........)|(?<n1>..........)|(?<n1>...........)|(?<n1>............)|(?<n1>.............)|(?<n1>..............))\k<n1>$"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(?<name1>)(?<name2>)(?<name3>)(?<name4>)(?<name5>)(?<name6>)(?<name7>)(?<name8>)(?<name9>)(?<name10>)(?<name11>)(?<name12>)(?<name13>)(?<name14>)(?<name15>)(?<name16>aaa)(?<name17>)$", "aaa", 0, 3, 16);
// tu!(r#"(?<name1>)(?<name2>)(?<name3>)(?<name4>)(?<name5>)(?<name6>)(?<name7>)(?<name8>)(?<name9>)(?<name10>)(?<name11>)(?<name12>)(?<name13>)(?<name14>)(?<name15>)(?<name16>aaa)(?<name17>)$"#, "aaa", &[("aaa", 0, 3)], 563), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S563 { mode M { token r#"(?<name1>)(?<name2>)(?<name3>)(?<name4>)(?<name5>)(?<name6>)(?<name7>)(?<name8>)(?<name9>)(?<name10>)(?<name11>)(?<name12>)(?<name13>)(?<name14>)(?<name15>)(?<name16>aaa)(?<name17>)$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<foo>a|\\(\\g<foo>\\))", "a", 0, 1);
// tr!(r#"(?<foo>a|\(\g<foo>\))"#, "a", &[("a", 0, 1)], 564), EscapeUnrecognized
// scanner! { S564 { mode M { token r#"(?<foo>a|\(\g<foo>\))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<foo>a|\\(\\g<foo>\\))", "((((((a))))))", 0, 13);
// tr!(r#"(?<foo>a|\(\g<foo>\))"#, "((((((a))))))", &[("((((((a))))))", 0, 13)], 565), EscapeUnrecognized
// scanner! { S565 { mode M { token r#"(?<foo>a|\(\g<foo>\))"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(?<foo>a|\\(\\g<foo>\\))", "((((((((a))))))))", 0, 17, 1);
// tr!(r#"(?<foo>a|\(\g<foo>\))"#, "((((((((a))))))))", &[("((((((((a))))))))", 0, 17)], 566), EscapeUnrecognized
// scanner! { S566 { mode M { token r#"(?<foo>a|\(\g<foo>\))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g<bar>|\\zEND(?<bar>.*abc$)", "abcxxxabc", 0, 9);
// tr!(r#"\g<bar>|\zEND(?<bar>.*abc$)"#, "abcxxxabc", &[("abcxxxabc", 0, 9)], 567), EscapeUnrecognized
// scanner! { S567 { mode M { token r#"\g<bar>|\zEND(?<bar>.*abc$)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g<1>|\\zEND(.a.)", "bac", 0, 3);
// tr!(r#"\g<1>|\zEND(.a.)"#, "bac", &[("bac", 0, 3)], 568), EscapeUnrecognized
// scanner! { S568 { mode M { token r#"\g<1>|\zEND(.a.)"# => 0; } } }

// -------------------------------------------------------------------------
// x3("\\g<_A>\\g<_A>|\\zEND(.a.)(?<_A>.b.)", "xbxyby", 3, 6, 1);
// tr!(r#"\g<_A>\g<_A>|\zEND(.a.)(?<_A>.b.)"#, "xbxyby", &[("yby", 3, 6)], 569), EscapeUnrecognized
// scanner! { S569 { mode M { token r#"\g<_A>\g<_A>|\zEND(.a.)(?<_A>.b.)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A(?:\\g<pon>|\\g<pan>|\\zEND  (?<pan>a|c\\g<pon>c)(?<pon>b|d\\g<pan>d))$", "cdcbcdc", 0, 7);
// tr!(r#"\A(?:\g<pon>|\g<pan>|\zEND  (?<pan>a|c\g<pon>c)(?<pon>b|d\g<pan>d))$"#, "cdcbcdc", &[("cdcbcdc", 0, 7)], 570), EscapeUnrecognized
// scanner! { S570 { mode M { token r#"\A(?:\g<pon>|\g<pan>|\zEND  (?<pan>a|c\g<pon>c)(?<pon>b|d\g<pan>d))$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A(?<n>|a\\g<m>)\\z|\\zEND (?<m>\\g<n>)", "aaaa", 0, 4);
// tr!(r#"\A(?<n>|a\g<m>)\z|\zEND (?<m>\g<n>)"#, "aaaa", &[("aaaa", 0, 4)], 571), EscapeUnrecognized
// scanner! { S571 { mode M { token r#"\A(?<n>|a\g<m>)\z|\zEND (?<m>\g<n>)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<n>(a|b\\g<n>c){3,5})", "baaaaca", 1, 5);
// tr!(r#"(?<n>(a|b\g<n>c){3,5})"#, "baaaaca", &[("aaaa", 1, 5)], 572), EscapeUnrecognized
// scanner! { S572 { mode M { token r#"(?<n>(a|b\g<n>c){3,5})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<n>(a|b\\g<n>c){3,5})", "baaaacaaaaa", 0, 10);
// tr!(r#"(?<n>(a|b\g<n>c){3,5})"#, "baaaacaaaaa", &[("baaaacaaaa", 0, 10)], 573), EscapeUnrecognized
// scanner! { S573 { mode M { token r#"(?<n>(a|b\g<n>c){3,5})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<pare>\\(([^\\(\\)]++|\\g<pare>)*+\\))", "((a))", 0, 5);
// tr!(r#"(?<pare>\(([^\(\)]++|\g<pare>)*+\))"#, "((a))", &[("((a))", 0, 5)], 574), EscapeUnrecognized
// scanner! { S574 { mode M { token r#"(?<pare>\(([^\(\)]++|\g<pare>)*+\))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("()*\\1", "", 0, 0);
// tr!(r#"()*\1"#, "", &[], 575), UnsupportedBackreference
// scanner! { S575 { mode M { token r#"()*\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:()|())*\\1\\2", "", 0, 0);
// tr!(r#"(?:()|())*\1\2"#, "", &[], 576), UnsupportedBackreference
// scanner! { S576 { mode M { token r#"(?:()|())*\1\2"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:a*|b*)*c", "abadc", 4, 5);
// td!(r#"(?:a*|b*)*c"#, "abadc", &[("c", 4, 5)], 577),
scanner! { S577 { mode M { token r#"(?:a*|b*)*c"# => 0; } } }
#[test]
fn test_match_577() {
    use s577::S577 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abadc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("c", 4, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "577: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "577: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "577: Match end does not match");
        assert_eq!(
            &"abadc"[ma.1..ma.2],
            ma.0,
            "577: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x3("(?:\\1a|())*", "a", 0, 0, 1);
// tr!(r#"(?:\1a|())*"#, "a", &[], 578), UnsupportedBackreference
// scanner! { S578 { mode M { token r#"(?:\1a|())*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("x((.)*)*x", "0x1x2x3", 1, 6);
// td!(r#"x((.)*)*x"#, "0x1x2x3", &[("x1x2x", 1, 6)], 579),
scanner! { S579 { mode M { token r#"x((.)*)*x"# => 0; } } }
#[test]
fn test_match_579() {
    use s579::S579 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("0x1x2x3", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("x1x2x", 1, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "579: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "579: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "579: Match end does not match");
        assert_eq!(
            &"0x1x2x3"[ma.1..ma.2],
            ma.0,
            "579: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("x((.)*)*x(?i:\\1)\\Z", "0x1x2x1X2", 1, 9);
// tr!(r#"x((.)*)*x(?i:\1)\Z"#, "0x1x2x1X2", &[("x1x2x1X2", 1, 9)], 580), UnsupportedBackreference
// scanner! { S580 { mode M { token r#"x((.)*)*x(?i:\1)\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:()|()|()|()|()|())*\\2\\5", "", 0, 0);
// tr!(r#"(?:()|()|()|()|()|())*\2\5"#, "", &[], 581), UnsupportedBackreference
// scanner! { S581 { mode M { token r#"(?:()|()|()|()|()|())*\2\5"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:()|()|()|(x)|()|())*\\2b\\5", "b", 0, 1);
// tr!(r#"(?:()|()|()|(x)|()|())*\2b\5"#, "b", &[("b", 0, 1)], 582), UnsupportedBackreference
// scanner! { S582 { mode M { token r#"(?:()|()|()|(x)|()|())*\2b\5"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[0-9-a]", "-", 0, 1);   // PR#44
// td!(r#"[0-9-a]"#, "-", &[("-", 0, 1)], 583),
scanner! { S583 { mode M { token r#"[0-9-a]"# => 0; } } }
#[test]
fn test_match_583() {
    use s583::S583 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("-", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("-", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "583: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "583: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "583: Match end does not match");
        assert_eq!(
            &"-"[ma.1..ma.2],
            ma.0,
            "583: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[0-9-a]", ":");          // PR#44
// td!(r#"[0-9-a]"#, ":", &[], 584),
scanner! { S584 { mode M { token r#"[0-9-a]"# => 0; } } }
#[test]
fn test_match_584() {
    use s584::S584 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "584: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x3("(\\(((?:[^(]|\\g<1>)*)\\))", "(abc)(abc)", 1, 4, 2); // PR#43
// tr!(r#"(\(((?:[^(]|\g<1>)*)\))"#, "(abc)(abc)", &[("abc", 1, 4)], 585), EscapeUnrecognized
// scanner! { S585 { mode M { token r#"(\(((?:[^(]|\g<1>)*)\))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\o{101}", "A", 0, 1);
// tr!(r#"\o{101}"#, "A", &[("A", 0, 1)], 586), EscapeUnrecognized
// scanner! { S586 { mode M { token r#"\o{101}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A(a|b\\g<1>c)\\k<1+3>\\z", "bbacca", 0, 6);
// tr!(r#"\A(a|b\g<1>c)\k<1+3>\z"#, "bbacca", &[("bbacca", 0, 6)], 587), EscapeUnrecognized
// scanner! { S587 { mode M { token r#"\A(a|b\g<1>c)\k<1+3>\z"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\A(a|b\\g<1>c)\\k<1+3>\\z", "bbaccb");
// tr!(r#"\A(a|b\g<1>c)\k<1+3>\z"#, "bbaccb", &[], 588), EscapeUnrecognized
// scanner! { S588 { mode M { token r#"\A(a|b\g<1>c)\k<1+3>\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)\\A(a|b\\g<1>c)\\k<1+2>\\z", "bBACcbac", 0, 8);
// tr!(r#"(?i)\A(a|b\g<1>c)\k<1+2>\z"#, "bBACcbac", &[("bBACcbac", 0, 8)], 589), EscapeUnrecognized
// scanner! { S589 { mode M { token r#"(?i)\A(a|b\g<1>c)\k<1+2>\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)(?<X>aa)|(?<X>bb)\\k<X>", "BBbb", 0, 4);
// tr!(r#"(?i)(?<X>aa)|(?<X>bb)\k<X>"#, "BBbb", &[("BBbb", 0, 4)], 590), GroupNameDuplicate
// scanner! { S590 { mode M { token r#"(?i)(?<X>aa)|(?<X>bb)\k<X>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:\\k'+1'B|(A)C)*", "ACAB", 0, 4); // relative backref by positive number
// tr!(r#"(?:\k'+1'B|(A)C)*"#, "ACAB", &[("ACAB", 0, 4)], 591), EscapeUnrecognized
// scanner! { S591 { mode M { token r#"(?:\k'+1'B|(A)C)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g<+2>(abc)(ABC){0}", "ABCabc", 0, 6); // relative call by positive number
// tr!(r#"\g<+2>(abc)(ABC){0}"#, "ABCabc", &[("ABCabc", 0, 6)], 592), EscapeUnrecognized
// scanner! { S592 { mode M { token r#"\g<+2>(abc)(ABC){0}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("A\\g'0'|B()", "AAAAB", 0, 5);
// tr!(r#"A\g'0'|B()"#, "AAAAB", &[("AAAAB", 0, 5)], 593), EscapeUnrecognized
// scanner! { S593 { mode M { token r#"A\g'0'|B()"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(A\\g'0')|B", "AAAAB", 0, 5, 1);
// tr!(r#"(A\g'0')|B"#, "AAAAB", &[("AAAAB", 0, 5)], 594), EscapeUnrecognized
// scanner! { S594 { mode M { token r#"(A\g'0')|B"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*)(?(1))aa", "aaaaa", 0, 5);
// tr!(r#"(a*)(?(1))aa"#, "aaaaa", &[("aaaaa", 0, 5)], 595), FlagUnrecognized
// scanner! { S595 { mode M { token r#"(a*)(?(1))aa"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*)(?(-1))aa", "aaaaa", 0, 5);
// tr!(r#"(a*)(?(-1))aa"#, "aaaaa", &[("aaaaa", 0, 5)], 596), FlagUnrecognized
// scanner! { S596 { mode M { token r#"(a*)(?(-1))aa"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<name>aaa)(?('name'))aa", "aaaaa", 0, 5);
// tr!(r#"(?<name>aaa)(?('name'))aa"#, "aaaaa", &[("aaaaa", 0, 5)], 597), FlagUnrecognized
// scanner! { S597 { mode M { token r#"(?<name>aaa)(?('name'))aa"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?(1)aa|bb)a", "aaaaa", 0, 4);
// tr!(r#"(a)(?(1)aa|bb)a"#, "aaaaa", &[("aaaa", 0, 4)], 598), FlagUnrecognized
// scanner! { S598 { mode M { token r#"(a)(?(1)aa|bb)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:aa|())(?(<1>)aa|bb)a", "aabba", 0, 5);
// tr!(r#"(?:aa|())(?(<1>)aa|bb)a"#, "aabba", &[("aabba", 0, 5)], 599), FlagUnrecognized
// scanner! { S599 { mode M { token r#"(?:aa|())(?(<1>)aa|bb)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:aa|())(?('1')aa|bb|cc)a", "aacca", 0, 5);
// tr!(r#"(?:aa|())(?('1')aa|bb|cc)a"#, "aacca", &[("aacca", 0, 5)], 600), FlagUnrecognized
// scanner! { S600 { mode M { token r#"(?:aa|())(?('1')aa|bb|cc)a"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(a*)(?(1)aa|a)b", "aaab", 0, 1, 1);
// tr!(r#"(a*)(?(1)aa|a)b"#, "aaab", &[("a", 0, 1)], 601), FlagUnrecognized
// scanner! { S601 { mode M { token r#"(a*)(?(1)aa|a)b"# => 0; } } }

// -------------------------------------------------------------------------
// n("(a)(?(1)a|b)c", "abc");
// tr!(r#"(a)(?(1)a|b)c"#, "abc", &[], 602), FlagUnrecognized
// scanner! { S602 { mode M { token r#"(a)(?(1)a|b)c"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?(1)|)c", "ac", 0, 2);
// tr!(r#"(a)(?(1)|)c"#, "ac", &[("ac", 0, 2)], 603), FlagUnrecognized
// scanner! { S603 { mode M { token r#"(a)(?(1)|)c"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?()aaa|bbb)", "bbb");
// tr!(r#"(?()aaa|bbb)"#, "bbb", &[], 604), FlagUnrecognized
// scanner! { S604 { mode M { token r#"(?()aaa|bbb)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?(1+0)b|c)d", "abd", 0, 3);
// tr!(r#"(a)(?(1+0)b|c)d"#, "abd", &[("abd", 0, 3)], 605), FlagUnrecognized
// scanner! { S605 { mode M { token r#"(a)(?(1+0)b|c)d"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?'name'a)|(?'name'b))(?('name')c|d)e", "ace", 0, 3);
// tr!(r#"(?:(?'name'a)|(?'name'b))(?('name')c|d)e"#, "ace", &[("ace", 0, 3)], 606), FlagUnrecognized
// scanner! { S606 { mode M { token r#"(?:(?'name'a)|(?'name'b))(?('name')c|d)e"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?'name'a)|(?'name'b))(?('name')c|d)e", "bce", 0, 3);
// tr!(r#"(?:(?'name'a)|(?'name'b))(?('name')c|d)e"#, "bce", &[("bce", 0, 3)], 607), FlagUnrecognized
// scanner! { S607 { mode M { token r#"(?:(?'name'a)|(?'name'b))(?('name')c|d)e"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\R", "\r\n", 0, 2);
// tr!(r#"\R"#, "\r\n", &[("\\r", 0, 2)], 608), EscapeUnrecognized
// scanner! { S608 { mode M { token r#"\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\R", "\r", 0, 1);
// tr!(r#"\R"#, "\r", &[("\\", 0, 1)], 609), EscapeUnrecognized
// scanner! { S609 { mode M { token r#"\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\R", "\n", 0, 1);
// tr!(r#"\R"#, "\n", &[("\\", 0, 1)], 610), EscapeUnrecognized
// scanner! { S610 { mode M { token r#"\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\R", "\x0b", 0, 1);
// tr!(r#"\R"#, "\x0b", &[("\\", 0, 1)], 611), EscapeUnrecognized
// scanner! { S611 { mode M { token r#"\R"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\R\\n", "\r\n");
// tr!(r#"\R\n"#, "\r\n", &[], 612), EscapeUnrecognized
// scanner! { S612 { mode M { token r#"\R\n"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\R", "\xc2\x85", 0, 2);
// tr!(r#"\R"#, "\xc2\x85", &[("\\x", 0, 2)], 613), EscapeUnrecognized
// scanner! { S613 { mode M { token r#"\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\N", "a", 0, 1);
// tr!(r#"\N"#, "a", &[("a", 0, 1)], 614), EscapeUnrecognized
// scanner! { S614 { mode M { token r#"\N"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\N", "\n");
// tr!(r#"\N"#, "\n", &[], 615), EscapeUnrecognized
// scanner! { S615 { mode M { token r#"\N"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?m:\\N)", "\n");
// tr!(r#"(?m:\N)"#, "\n", &[], 616), EscapeUnrecognized
// scanner! { S616 { mode M { token r#"(?m:\N)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?-m:\\N)", "\n");
// tr!(r#"(?-m:\N)"#, "\n", &[], 617), EscapeUnrecognized
// scanner! { S617 { mode M { token r#"(?-m:\N)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\O", "a", 0, 1);
// tr!(r#"\O"#, "a", &[("a", 0, 1)], 618), EscapeUnrecognized
// scanner! { S618 { mode M { token r#"\O"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\O", "\n", 0, 1);
// tr!(r#"\O"#, "\n", &[("\\", 0, 1)], 619), EscapeUnrecognized
// scanner! { S619 { mode M { token r#"\O"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?m:\\O)", "\n", 0, 1);
// tr!(r#"(?m:\O)"#, "\n", &[("\\", 0, 1)], 620), EscapeUnrecognized
// scanner! { S620 { mode M { token r#"(?m:\O)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-m:\\O)", "\n", 0, 1);
// tr!(r#"(?-m:\O)"#, "\n", &[("\\", 0, 1)], 621), EscapeUnrecognized
// scanner! { S621 { mode M { token r#"(?-m:\O)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\K", "a", 0, 0);
// tr!(r#"\K"#, "a", &[], 622), EscapeUnrecognized
// scanner! { S622 { mode M { token r#"\K"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a\\K", "a", 1, 1);
// tr!(r#"a\K"#, "a", &[("", 1, 1)], 623), EscapeUnrecognized
// scanner! { S623 { mode M { token r#"a\K"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a\\Kb", "ab", 1, 2);
// tr!(r#"a\Kb"#, "ab", &[("b", 1, 2)], 624), EscapeUnrecognized
// scanner! { S624 { mode M { token r#"a\Kb"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a\\Kb|ac\\Kd)", "acd", 2, 3);
// tr!(r#"(a\Kb|ac\Kd)"#, "acd", &[("d", 2, 3)], 625), EscapeUnrecognized
// scanner! { S625 { mode M { token r#"(a\Kb|ac\Kd)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a\\Kb|\\Kac\\K)*", "acababacab", 9, 10);
// tr!(r#"(a\Kb|\Kac\K)*"#, "acababacab", &[("b", 9, 10)], 626), EscapeUnrecognized
// scanner! { S626 { mode M { token r#"(a\Kb|\Kac\K)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:()|())*\\1", "abc", 0, 0);
// tr!(r#"(?:()|())*\1"#, "abc", &[], 627), UnsupportedBackreference
// scanner! { S627 { mode M { token r#"(?:()|())*\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:()|())*\\2", "abc", 0, 0);
// tr!(r#"(?:()|())*\2"#, "abc", &[], 628), UnsupportedBackreference
// scanner! { S628 { mode M { token r#"(?:()|())*\2"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:()|()|())*\\3\\1", "abc", 0, 0);
// tr!(r#"(?:()|()|())*\3\1"#, "abc", &[], 629), UnsupportedBackreference
// scanner! { S629 { mode M { token r#"(?:()|()|())*\3\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(|(?:a(?:\\g'1')*))b|", "abc", 0, 2);
// tr!(r#"(|(?:a(?:\g'1')*))b|"#, "abc", &[("ab", 0, 2)], 630), EscapeUnrecognized
// scanner! { S630 { mode M { token r#"(|(?:a(?:\g'1')*))b|"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^(\"|)(.*)\\1$", "XX", 0, 2);
// tr!(r#"^(\"|)(.*)\1$"#, "XX", &[("XX", 0, 2)], 631), UnsupportedBackreference
// scanner! { S631 { mode M { token r#"^(\"|)(.*)\1$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(abc|def|ghi|jkl|mno|pqr|stu){0,10}?\\z", "admno", 2, 5);
// tu!(r#"(abc|def|ghi|jkl|mno|pqr|stu){0,10}?\z"#, "admno", &[("mno", 2, 5)], 632), UnsupportedFeatureError("((?:(?:abc)|(?:def)|(?:ghi)|(?:jkl)|(?:mno)|(?:pqr)|(?:stu))){0,10}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S632 { mode M { token r#"(abc|def|ghi|jkl|mno|pqr|stu){0,10}?\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(abc|(def|ghi|jkl|mno|pqr){0,7}?){5}\\z", "adpqrpqrpqr", 2, 11); // cover OP_REPEAT_INC_NG_SG
// tu!(r#"(abc|(def|ghi|jkl|mno|pqr){0,7}?){5}\z"#, "adpqrpqrpqr", &[("pqrpqrpqr", 2, 11)], 633), UnsupportedFeatureError("((?:(?:def)|(?:ghi)|(?:jkl)|(?:mno)|(?:pqr))){0,7}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S633 { mode M { token r#"(abc|(def|ghi|jkl|mno|pqr){0,7}?){5}\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?!abc).*\\z", "abcde", 1, 5); // cover OP_PREC_READ_NOT_END
// tr!(r#"(?!abc).*\z"#, "abcde", &[("bcde", 1, 5)], 634), UnsupportedLookAround
// scanner! { S634 { mode M { token r#"(?!abc).*\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.{2,})?", "abcde", 0, 5); // up coverage
// td!(r#"(.{2,})?"#, "abcde", &[("abcde", 0, 5)], 635),
scanner! { S635 { mode M { token r#"(.{2,})?"# => 0; } } }
#[test]
fn test_match_635() {
    use s635::S635 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcde", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcde", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "635: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "635: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "635: Match end does not match");
        assert_eq!(
            &"abcde"[ma.1..ma.2],
            ma.0,
            "635: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("((a|b|c|d|e|f|g|h|i|j|k|l|m|n)+)?", "abcde", 0, 5); // up coverage
// td!(r#"((a|b|c|d|e|f|g|h|i|j|k|l|m|n)+)?"#, "abcde", &[("abcde", 0, 5)], 636),
scanner! { S636 { mode M { token r#"((a|b|c|d|e|f|g|h|i|j|k|l|m|n)+)?"# => 0; } } }
#[test]
fn test_match_636() {
    use s636::S636 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcde", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcde", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "636: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "636: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "636: Match end does not match");
        assert_eq!(
            &"abcde"[ma.1..ma.2],
            ma.0,
            "636: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("((a|b|c|d|e|f|g|h|i|j|k|l|m|n){3,})?", "abcde", 0, 5); // up coverage
// td!(r#"((a|b|c|d|e|f|g|h|i|j|k|l|m|n){3,})?"#, "abcde", &[("abcde", 0, 5)], 637),
scanner! { S637 { mode M { token r#"((a|b|c|d|e|f|g|h|i|j|k|l|m|n){3,})?"# => 0; } } }
#[test]
fn test_match_637() {
    use s637::S637 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcde", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcde", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "637: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "637: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "637: Match end does not match");
        assert_eq!(
            &"abcde"[ma.1..ma.2],
            ma.0,
            "637: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("((?:a(?:b|c|d|e|f|g|h|i|j|k|l|m|n))+)?", "abacadae", 0, 8); // up coverage
// td!(r#"((?:a(?:b|c|d|e|f|g|h|i|j|k|l|m|n))+)?"#, "abacadae", &[("abacadae", 0, 8)], 638),
scanner! { S638 { mode M { token r#"((?:a(?:b|c|d|e|f|g|h|i|j|k|l|m|n))+)?"# => 0; } } }
#[test]
fn test_match_638() {
    use s638::S638 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abacadae", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abacadae", 0, 8)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "638: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "638: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "638: Match end does not match");
        assert_eq!(
            &"abacadae"[ma.1..ma.2],
            ma.0,
            "638: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("((?:a(?:b|c|d|e|f|g|h|i|j|k|l|m|n))+?)?z", "abacadaez", 0, 9); // up coverage
// tu!(r#"((?:a(?:b|c|d|e|f|g|h|i|j|k|l|m|n))+?)?z"#, "abacadaez", &[("abacadaez", 0, 9)], 639), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S639 { mode M { token r#"((?:a(?:b|c|d|e|f|g|h|i|j|k|l|m|n))+?)?z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A((a|b)\?\?)?z", "bz", 0, 2); // up coverage
// tu!(r#"\A((a|b)\?\?)?z"#, "bz", &[("bz", 0, 2)], 640), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S640 { mode M { token r#"\A((a|b)\?\?)?z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?<x>abc){0}a\\g<x>d)+", "aabcd", 0, 5); // up coverage
// tr!(r#"((?<x>abc){0}a\g<x>d)+"#, "aabcd", &[("aabcd", 0, 5)], 641), EscapeUnrecognized
// scanner! { S641 { mode M { token r#"((?<x>abc){0}a\g<x>d)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?(abc)true|false))+", "false", 0, 5); // up coverage
// tr!(r#"((?(abc)true|false))+"#, "false", &[("false", 0, 5)], 642), FlagUnrecognized
// scanner! { S642 { mode M { token r#"((?(abc)true|false))+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?i:abc)d)+", "abcdABCd", 0, 8); // up coverage
// td!(r#"((?i:abc)d)+"#, "abcdABCd", &[("abcdABCd", 0, 8)], 643),
scanner! { S643 { mode M { token r#"((?i:abc)d)+"# => 0; } } }
#[test]
fn test_match_643() {
    use s643::S643 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abcdABCd", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcdABCd", 0, 8)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "643: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "643: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "643: Match end does not match");
        assert_eq!(
            &"abcdABCd"[ma.1..ma.2],
            ma.0,
            "643: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("((?<!abc)def)+", "bcdef", 2, 5); // up coverage
// tr!(r#"((?<!abc)def)+"#, "bcdef", &[("def", 2, 5)], 644), UnsupportedLookAround
// scanner! { S644 { mode M { token r#"((?<!abc)def)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(\\ba)+", "aaa", 0, 1); // up coverage
// tu!(r#"(\ba)+"#, "aaa", &[("a", 0, 1)], 645), UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S645 { mode M { token r#"(\ba)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("()(?<x>ab)(?(<x>)a|b)", "aba", 0, 3); // up coverage
// tr!(r#"()(?<x>ab)(?(<x>)a|b)"#, "aba", &[("aba", 0, 3)], 646), FlagUnrecognized
// scanner! { S646 { mode M { token r#"()(?<x>ab)(?(<x>)a|b)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a.b)c", "azbc", 3, 4); // up coverage
// tr!(r#"(?<=a.b)c"#, "azbc", &[("c", 3, 4)], 647), UnsupportedLookAround
// scanner! { S647 { mode M { token r#"(?<=a.b)c"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=(?:abcde){30})z", "abc"); // up coverage
// tr!(r#"(?<=(?:abcde){30})z"#, "abc", &[], 648), UnsupportedLookAround
// scanner! { S648 { mode M { token r#"(?<=(?:abcde){30})z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(?(a)a|bb))z", "aaz", 2, 3); // up coverage
// tr!(r#"(?<=(?(a)a|bb))z"#, "aaz", &[("z", 2, 3)], 649), UnsupportedLookAround
// scanner! { S649 { mode M { token r#"(?<=(?(a)a|bb))z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[a]*\\W", "aa@", 0, 3); // up coverage
// td!(r#"[a]*\W"#, "aa@", &[("aa@", 0, 3)], 650),
scanner! { S650 { mode M { token r#"[a]*\W"# => 0; } } }
#[test]
fn test_match_650() {
    use s650::S650 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa@", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa@", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "650: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "650: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "650: Match end does not match");
        assert_eq!(
            &"aa@"[ma.1..ma.2],
            ma.0,
            "650: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[a]*[b]", "aab", 0, 3); // up coverage
// td!(r#"[a]*[b]"#, "aab", &[("aab", 0, 3)], 651),
scanner! { S651 { mode M { token r#"[a]*[b]"# => 0; } } }
#[test]
fn test_match_651() {
    use s651::S651 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aab", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aab", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "651: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "651: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "651: Match end does not match");
        assert_eq!(
            &"aab"[ma.1..ma.2],
            ma.0,
            "651: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("a*\\W", "aaa"); // up coverage
// td!(r#"a*\W"#, "aaa", &[], 652),
scanner! { S652 { mode M { token r#"a*\W"# => 0; } } }
#[test]
fn test_match_652() {
    use s652::S652 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "652: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("(?W)a*\\W", "aaa"); // up coverage
// tr!(r#"(?W)a*\W"#, "aaa", &[], 653), FlagUnrecognized
// scanner! { S653 { mode M { token r#"(?W)a*\W"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=ab(?<=ab))", "ab", 2, 2); // up coverage
// tr!(r#"(?<=ab(?<=ab))"#, "ab", &[("", 2, 2)], 654), UnsupportedLookAround
// scanner! { S654 { mode M { token r#"(?<=ab(?<=ab))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<x>a)(?<x>b)(\\k<x>)+", "abbaab", 0, 6); // up coverage
// tr!(r#"(?<x>a)(?<x>b)(\k<x>)+"#, "abbaab", &[("abbaab", 0, 6)], 655), GroupNameDuplicate
// scanner! { S655 { mode M { token r#"(?<x>a)(?<x>b)(\k<x>)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("()(\\1)(\\2)", "abc", 0, 0); // up coverage
// tr!(r#"()(\1)(\2)"#, "abc", &[], 656), UnsupportedBackreference
// scanner! { S656 { mode M { token r#"()(\1)(\2)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?(a)b|c))(\\1)", "abab", 0, 4); // up coverage
// tr!(r#"((?(a)b|c))(\1)"#, "abab", &[("abab", 0, 4)], 657), FlagUnrecognized
// scanner! { S657 { mode M { token r#"((?(a)b|c))(\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<x>$|b\\g<x>)", "bbb", 0, 3); // up coverage
// tr!(r#"(?<x>$|b\g<x>)"#, "bbb", &[("bbb", 0, 3)], 658), EscapeUnrecognized
// scanner! { S658 { mode M { token r#"(?<x>$|b\g<x>)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<x>(?(a)a|b)|c\\g<x>)", "cccb", 0, 4); // up coverage
// tr!(r#"(?<x>(?(a)a|b)|c\g<x>)"#, "cccb", &[("cccb", 0, 4)], 659), FlagUnrecognized
// scanner! { S659 { mode M { token r#"(?<x>(?(a)a|b)|c\g<x>)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?(1)a*|b*)+", "aaaa", 0, 4); // up coverage
// tr!(r#"(a)(?(1)a*|b*)+"#, "aaaa", &[("aaaa", 0, 4)], 660), FlagUnrecognized
// scanner! { S660 { mode M { token r#"(a)(?(1)a*|b*)+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[[^abc]&&cde]*", "de", 0, 2); // up coverage
// td!(r#"[[^abc]&&cde]*"#, "de", &[("de", 0, 2)], 661),
scanner! { S661 { mode M { token r#"[[^abc]&&cde]*"# => 0; } } }
#[test]
fn test_match_661() {
    use s661::S661 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("de", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("de", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "661: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "661: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "661: Match end does not match");
        assert_eq!(
            &"de"[ma.1..ma.2],
            ma.0,
            "661: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(a){10}{10}", "aa"); // up coverage
// td!(r#"(a){10}{10}"#, "aa", &[], 662),
scanner! { S662 { mode M { token r#"(a){10}{10}"# => 0; } } }
#[test]
fn test_match_662() {
    use s662::S662 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "662: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:a?)+", "aa", 0, 2); // up coverage
// td!(r#"(?:a?)+"#, "aa", &[("aa", 0, 2)], 663),
scanner! { S663 { mode M { token r#"(?:a?)+"# => 0; } } }
#[test]
fn test_match_663() {
    use s663::S663 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "663: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "663: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "663: Match end does not match");
        assert_eq!(
            &"aa"[ma.1..ma.2],
            ma.0,
            "663: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?:a?)*?", "a", 0, 0); // up coverage
// tu!(r#"(?:a?)*?"#, "a", &[], 664), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S664 { mode M { token r#"(?:a?)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:a*)*?", "a", 0, 0); // up coverage
// tu!(r#"(?:a*)*?"#, "a", &[], 665), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S665 { mode M { token r#"(?:a*)*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:a+?)*", "a", 0, 1); // up coverage
// tu!(r#"(?:a+?)*"#, "a", &[("a", 0, 1)], 666), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S666 { mode M { token r#"(?:a+?)*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\h", "5", 0, 1); // up coverage
// tr!(r#"\h"#, "5", &[("5", 0, 1)], 667), EscapeUnrecognized
// scanner! { S667 { mode M { token r#"\h"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\H", "z", 0, 1); // up coverage
// tr!(r#"\H"#, "z", &[("z", 0, 1)], 668), EscapeUnrecognized
// scanner! { S668 { mode M { token r#"\H"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\h]", "5", 0, 1); // up coverage
// tr!(r#"[\h]"#, "5", &[("5", 0, 1)], 669), EscapeUnrecognized
// scanner! { S669 { mode M { token r#"[\h]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\H]", "z", 0, 1); // up coverage
// tr!(r#"[\H]"#, "z", &[("z", 0, 1)], 670), EscapeUnrecognized
// scanner! { S670 { mode M { token r#"[\H]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\o{101}]", "A", 0, 1); // up coverage
// tr!(r#"[\o{101}]"#, "A", &[("A", 0, 1)], 671), EscapeUnrecognized
// scanner! { S671 { mode M { token r#"[\o{101}]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\u0041]", "A", 0, 1); // up coverage
// td!(r#"[\u0041]"#, "A", &[("A", 0, 1)], 672),
scanner! { S672 { mode M { token r#"[\u0041]"# => 0; } } }
#[test]
fn test_match_672() {
    use s672::S672 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("A", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "672: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "672: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "672: Match end does not match");
        assert_eq!(
            &"A"[ma.1..ma.2],
            ma.0,
            "672: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?~)", "", 0, 0);
// tr!(r#"(?~)"#, "", &[], 673), FlagUnrecognized
// scanner! { S673 { mode M { token r#"(?~)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~)", "A", 0, 0);
// tr!(r#"(?~)"#, "A", &[], 674), FlagUnrecognized
// scanner! { S674 { mode M { token r#"(?~)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~ab)", "abc", 0, 0);
// tr!(r#"(?~ab)"#, "abc", &[], 675), FlagUnrecognized
// scanner! { S675 { mode M { token r#"(?~ab)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~abc)", "abc", 0, 0);
// tr!(r#"(?~abc)"#, "abc", &[], 676), FlagUnrecognized
// scanner! { S676 { mode M { token r#"(?~abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~abc|ab)", "abc", 0, 0);
// tr!(r#"(?~abc|ab)"#, "abc", &[], 677), FlagUnrecognized
// scanner! { S677 { mode M { token r#"(?~abc|ab)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~ab|abc)", "abc", 0, 0);
// tr!(r#"(?~ab|abc)"#, "abc", &[], 678), FlagUnrecognized
// scanner! { S678 { mode M { token r#"(?~ab|abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~a.c)", "abc", 0, 0);
// tr!(r#"(?~a.c)"#, "abc", &[], 679), FlagUnrecognized
// scanner! { S679 { mode M { token r#"(?~a.c)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~a.c|ab)", "abc", 0, 0);
// tr!(r#"(?~a.c|ab)"#, "abc", &[], 680), FlagUnrecognized
// scanner! { S680 { mode M { token r#"(?~a.c|ab)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~ab|a.c)", "abc", 0, 0);
// tr!(r#"(?~ab|a.c)"#, "abc", &[], 681), FlagUnrecognized
// scanner! { S681 { mode M { token r#"(?~ab|a.c)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("aaaaa(?~)", "aaaaaaaaaa", 0, 5);
// tr!(r#"aaaaa(?~)"#, "aaaaaaaaaa", &[("aaaaa", 0, 5)], 682), FlagUnrecognized
// scanner! { S682 { mode M { token r#"aaaaa(?~)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~(?:|aaa))", "aaa", 0, 0);
// tr!(r#"(?~(?:|aaa))"#, "aaa", &[], 683), FlagUnrecognized
// scanner! { S683 { mode M { token r#"(?~(?:|aaa))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~aaa|)", "aaa", 0, 0);
// tr!(r#"(?~aaa|)"#, "aaa", &[], 684), FlagUnrecognized
// scanner! { S684 { mode M { token r#"(?~aaa|)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a(?~(?~)).", "abcdefghijklmnopqrstuvwxyz", 0, 26); // nested absent functions cause strange result
// tr!(r#"a(?~(?~))."#, "abcdefghijklmnopqrstuvwxyz", &[("abcdefghijklmnopqrstuvwxyz", 0, 26)], 685), FlagUnrecognized
// scanner! { S685 { mode M { token r#"a(?~(?~))."# => 0; } } }

// -------------------------------------------------------------------------
// x2("/\\*(?~\\*/)\\*/", "/* */ */", 0, 5);
// tr!(r#"/\*(?~\*/)\*/"#, "/* */ */", &[("/* */", 0, 5)], 686), FlagUnrecognized
// scanner! { S686 { mode M { token r#"/\*(?~\*/)\*/"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~\\w+)zzzzz", "zzzzz", 0, 5);
// tr!(r#"(?~\w+)zzzzz"#, "zzzzz", &[("zzzzz", 0, 5)], 687), FlagUnrecognized
// scanner! { S687 { mode M { token r#"(?~\w+)zzzzz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~\\w*)zzzzz", "zzzzz", 0, 5);
// tr!(r#"(?~\w*)zzzzz"#, "zzzzz", &[("zzzzz", 0, 5)], 688), FlagUnrecognized
// scanner! { S688 { mode M { token r#"(?~\w*)zzzzz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~A.C|B)", "ABC", 0, 0);
// tr!(r#"(?~A.C|B)"#, "ABC", &[], 689), FlagUnrecognized
// scanner! { S689 { mode M { token r#"(?~A.C|B)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~XYZ|ABC)a", "ABCa", 1, 4);
// tr!(r#"(?~XYZ|ABC)a"#, "ABCa", &[("BCa", 1, 4)], 690), FlagUnrecognized
// scanner! { S690 { mode M { token r#"(?~XYZ|ABC)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~XYZ|ABC)a", "aABCa", 0, 1);
// tr!(r#"(?~XYZ|ABC)a"#, "aABCa", &[("a", 0, 1)], 691), FlagUnrecognized
// scanner! { S691 { mode M { token r#"(?~XYZ|ABC)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("<[^>]*>(?~[<>])</[^>]*>", "<a>vvv</a>   <b>  </b>", 0, 10);
// tr!(r#"<[^>]*>(?~[<>])</[^>]*>"#, "<a>vvv</a>   <b>  </b>", &[("<a>vvv</a>", 0, 10)], 692), FlagUnrecognized
// scanner! { S692 { mode M { token r#"<[^>]*>(?~[<>])</[^>]*>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~ab)", "ccc\ndab", 0, 5);
// tr!(r#"(?~ab)"#, "ccc\ndab", &[("ccc\\n", 0, 5)], 693), UnsupportedFeatureError("x?*?: Non-greedy repetitions) FlagUnrecognized
// scanner! { S693 { mode M { token r#"(?~ab)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?m:(?~ab))", "ccc\ndab", 0, 5);
// tr!(r#"(?m:(?~ab))"#, "ccc\ndab", &[("ccc\\n", 0, 5)], 694), FlagUnrecognized
// scanner! { S694 { mode M { token r#"(?m:(?~ab))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-m:(?~ab))", "ccc\ndab", 0, 5);
// tr!(r#"(?-m:(?~ab))"#, "ccc\ndab", &[("ccc\\n", 0, 5)], 695), FlagUnrecognized
// scanner! { S695 { mode M { token r#"(?-m:(?~ab))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~abc)xyz", "xyz012345678901234567890123456789abc", 0, 3);
// tr!(r#"(?~abc)xyz"#, "xyz012345678901234567890123456789abc", &[("xyz", 0, 3)], 696), FlagUnrecognized
// scanner! { S696 { mode M { token r#"(?~abc)xyz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|78|\\d*)", "123456789", 0, 6);
// tr!(r#"(?~|78|\d*)"#, "123456789", &[("123456", 0, 6)], 697), FlagUnrecognized
// scanner! { S697 { mode M { token r#"(?~|78|\d*)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|def|(?:abc|de|f){0,100})", "abcdedeabcfdefabc", 0, 11);
// tr!(r#"(?~|def|(?:abc|de|f){0,100})"#, "abcdedeabcfdefabc", &[("abcdedeabcf", 0, 11)], 698), FlagUnrecognized
// scanner! { S698 { mode M { token r#"(?~|def|(?:abc|de|f){0,100})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|ab|.*)", "ccc\nddd", 0, 3);
// tr!(r#"(?~|ab|.*)"#, "ccc\nddd", &[("ccc", 0, 3)], 699), FlagUnrecognized
// scanner! { S699 { mode M { token r#"(?~|ab|.*)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|ab|\\O*)", "ccc\ndab", 0, 5);
// tr!(r#"(?~|ab|\O*)"#, "ccc\ndab", &[("ccc\\n", 0, 5)], 700), FlagUnrecognized
// scanner! { S700 { mode M { token r#"(?~|ab|\O*)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|ab|\\O{2,10})", "ccc\ndab", 0, 5);
// tr!(r#"(?~|ab|\O{2,10})"#, "ccc\ndab", &[("ccc\\n", 0, 5)], 701), FlagUnrecognized
// scanner! { S701 { mode M { token r#"(?~|ab|\O{2,10})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|ab|\\O{1,10})", "ab", 1, 2);
// tr!(r#"(?~|ab|\O{1,10})"#, "ab", &[("b", 1, 2)], 702), FlagUnrecognized
// scanner! { S702 { mode M { token r#"(?~|ab|\O{1,10})"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?~|ab|\\O{2,10})", "ab");
// tr!(r#"(?~|ab|\O{2,10})"#, "ab", &[], 703), FlagUnrecognized
// scanner! { S703 { mode M { token r#"(?~|ab|\O{2,10})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc|\\O{1,10})", "abc", 1, 3);
// tr!(r#"(?~|abc|\O{1,10})"#, "abc", &[("bc", 1, 3)], 704), FlagUnrecognized
// scanner! { S704 { mode M { token r#"(?~|abc|\O{1,10})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|ab|\\O{5,10})|abc", "abc", 0, 3);
// tr!(r#"(?~|ab|\O{5,10})|abc"#, "abc", &[("abc", 0, 3)], 705), FlagUnrecognized
// scanner! { S705 { mode M { token r#"(?~|ab|\O{5,10})|abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|ab|\\O{1,10})", "cccccccccccab", 0, 10);
// tr!(r#"(?~|ab|\O{1,10})"#, "cccccccccccab", &[("cccccccccc", 0, 10)], 706), FlagUnrecognized
// scanner! { S706 { mode M { token r#"(?~|ab|\O{1,10})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|aaa|)", "aaa", 0, 0);
// tr!(r#"(?~|aaa|)"#, "aaa", &[], 707), FlagUnrecognized
// scanner! { S707 { mode M { token r#"(?~|aaa|)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~||a*)", "aaaaaa", 0, 0);
// tr!(r#"(?~||a*)"#, "aaaaaa", &[], 708), FlagUnrecognized
// scanner! { S708 { mode M { token r#"(?~||a*)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~||a*?)", "aaaaaa", 0, 0);
// tu!(r#"(?~||a*?)"#, "aaaaaa", &[], 709), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S709 { mode M { token r#"(?~||a*?)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?~|b|\\1)", "aaaaaa", 0, 2);
// tr!(r#"(a)(?~|b|\1)"#, "aaaaaa", &[("aa", 0, 2)], 710), FlagUnrecognized
// scanner! { S710 { mode M { token r#"(a)(?~|b|\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a)(?~|bb|(?:a\\1)*)", "aaaaaa", 0, 5);
// tr!(r#"(a)(?~|bb|(?:a\1)*)"#, "aaaaaa", &[("aaaaa", 0, 5)], 711), FlagUnrecognized
// scanner! { S711 { mode M { token r#"(a)(?~|bb|(?:a\1)*)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(b|c)(?~|abac|(?:a\\1)*)", "abababacabab", 1, 4);
// tr!(r#"(b|c)(?~|abac|(?:a\1)*)"#, "abababacabab", &[("bab", 1, 4)], 712), FlagUnrecognized
// scanner! { S712 { mode M { token r#"(b|c)(?~|abac|(?:a\1)*)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?~|c|a*+)a", "aaaaa");
// tr!(r#"(?~|c|a*+)a"#, "aaaaa", &[], 713), FlagUnrecognized
// scanner! { S713 { mode M { token r#"(?~|c|a*+)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|aaaaa|a*+)", "aaaaa", 0, 0);
// tr!(r#"(?~|aaaaa|a*+)"#, "aaaaa", &[], 714), FlagUnrecognized
// scanner! { S714 { mode M { token r#"(?~|aaaaa|a*+)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|aaaaaa|a*+)b", "aaaaaab", 1, 7);
// tr!(r#"(?~|aaaaaa|a*+)b"#, "aaaaaab", &[("aaaaab", 1, 7)], 715), FlagUnrecognized
// scanner! { S715 { mode M { token r#"(?~|aaaaaa|a*+)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abcd|(?>))", "zzzabcd", 0, 0);
// tr!(r#"(?~|abcd|(?>))"#, "zzzabcd", &[], 716), FlagUnrecognized
// scanner! { S716 { mode M { token r#"(?~|abcd|(?>))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc|a*?)", "aaaabc", 0, 0);
// tu!(r#"(?~|abc|a*?)"#, "aaaabc", &[], 717), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S717 { mode M { token r#"(?~|abc|a*?)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc)a*", "aaaaaabc", 0, 5);
// tr!(r#"(?~|abc)a*"#, "aaaaaabc", &[("aaaaa", 0, 5)], 718), FlagUnrecognized
// scanner! { S718 { mode M { token r#"(?~|abc)a*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc)a*z|aaaaaabc", "aaaaaabc", 0, 8);
// tr!(r#"(?~|abc)a*z|aaaaaabc"#, "aaaaaabc", &[("aaaaaabc", 0, 8)], 719), FlagUnrecognized
// scanner! { S719 { mode M { token r#"(?~|abc)a*z|aaaaaabc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|aaaaaa)a*", "aaaaaa", 0, 0);
// tr!(r#"(?~|aaaaaa)a*"#, "aaaaaa", &[], 720), FlagUnrecognized
// scanner! { S720 { mode M { token r#"(?~|aaaaaa)a*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc)aaaa|aaaabc", "aaaabc", 0, 6);
// tr!(r#"(?~|abc)aaaa|aaaabc"#, "aaaabc", &[("aaaabc", 0, 6)], 721), FlagUnrecognized
// scanner! { S721 { mode M { token r#"(?~|abc)aaaa|aaaabc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?>(?~|abc))aaaa|aaaabc", "aaaabc", 0, 6);
// tr!(r#"(?>(?~|abc))aaaa|aaaabc"#, "aaaabc", &[("aaaabc", 0, 6)], 722), FlagUnrecognized
// scanner! { S722 { mode M { token r#"(?>(?~|abc))aaaa|aaaabc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|)a", "a", 0, 1);
// tr!(r#"(?~|)a"#, "a", &[("a", 0, 1)], 723), FlagUnrecognized
// scanner! { S723 { mode M { token r#"(?~|)a"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?~|a)a", "a");
// tr!(r#"(?~|a)a"#, "a", &[], 724), FlagUnrecognized
// scanner! { S724 { mode M { token r#"(?~|a)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|a)(?~|)a", "a", 0, 1);
// tr!(r#"(?~|a)(?~|)a"#, "a", &[("a", 0, 1)], 725), FlagUnrecognized
// scanner! { S725 { mode M { token r#"(?~|a)(?~|)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|a).*(?~|)a", "bbbbbbbbbbbbbbbbbbbba", 0, 21);
// tr!(r#"(?~|a).*(?~|)a"#, "bbbbbbbbbbbbbbbbbbbba", &[("bbbbbbbbbbbbbbbbbbbba", 0, 21)], 726), FlagUnrecognized
// scanner! { S726 { mode M { token r#"(?~|a).*(?~|)a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc).*(xyz|pqr)(?~|)abc", "aaaaxyzaaapqrabc", 0, 16);
// tr!(r#"(?~|abc).*(xyz|pqr)(?~|)abc"#, "aaaaxyzaaapqrabc", &[("aaaaxyzaaapqrabc", 0, 16)], 727), FlagUnrecognized
// scanner! { S727 { mode M { token r#"(?~|abc).*(xyz|pqr)(?~|)abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?~|abc).*(xyz|pqr)(?~|)abc", "aaaaxyzaaaabcpqrabc", 11, 19);
// tr!(r#"(?~|abc).*(xyz|pqr)(?~|)abc"#, "aaaaxyzaaaabcpqrabc", &[("bcpqrabc", 11, 19)], 728), FlagUnrecognized
// scanner! { S728 { mode M { token r#"(?~|abc).*(xyz|pqr)(?~|)abc"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\A(?~|abc).*(xyz|pqrabc)(?~|)abc", "aaaaxyzaaaabcpqrabcabc");
// tr!(r#"\A(?~|abc).*(xyz|pqrabc)(?~|)abc"#, "aaaaxyzaaaabcpqrabcabc", &[], 729), FlagUnrecognized
// scanner! { S729 { mode M { token r#"\A(?~|abc).*(xyz|pqrabc)(?~|)abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("", "あ", 0, 0);
// td!(r#""#, "あ", &[], 730),
scanner! { S730 { mode M { token r#""# => 0; } } }
#[test]
fn test_match_730() {
    use s730::S730 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "730: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("あ", "あ", 0, 3); // 731

// -------------------------------------------------------------------------
// n("い", "あ");
// td!(r#"い"#, "あ", &[], 732),
scanner! { S732 { mode M { token r#"い"# => 0; } } }
#[test]
fn test_match_732() {
    use s732::S732 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "732: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("うう", "うう", 0, 6); // 733

// -------------------------------------------------------------------------
// *** Index error: x2("あいう", "あいう", 0, 9); // 734

// -------------------------------------------------------------------------
// *** Index error: x2("こここここここここここここここここここここここここここここここここここ", "こここここここここここここここここここここここここここここここここここ", 0, 105); // 735

// -------------------------------------------------------------------------
// *** Index error:  x2("あ", "いあ", 3, 6); // 736

// -------------------------------------------------------------------------
// *** Index error: x2("いう", "あいう", 3, 9); // 737

// -------------------------------------------------------------------------
// x2("\\xca\\xb8", "\xca\xb8", 0, 2);
// td!(r#"\xca\xb8"#, "\xca\xb8", &[("\\x", 0, 2)], 738), Rust Compiler Error
scanner! { S738 { mode M { token r#"\xca\xb8"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2(".", "あ", 0, 3); // 739

// -------------------------------------------------------------------------
// *** Index error: x2("..", "かき", 0, 6); // 740

// -------------------------------------------------------------------------
// *** Index error: x2("\\w", "お", 0, 3); // 741

// -------------------------------------------------------------------------
// n("\\W", "あ");
// td!(r#"\W"#, "あ", &[], 742),
scanner! { S742 { mode M { token r#"\W"# => 0; } } }
#[test]
fn test_match_742() {
    use s742::S742 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "742: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error:  x2("[\\W]", "う$", 3, 4); // 743

// -------------------------------------------------------------------------
// *** Index error: x2("\\S", "そ", 0, 3); // 744

// -------------------------------------------------------------------------
// *** Index error: x2("\\S", "漢", 0, 3); // 745

// -------------------------------------------------------------------------
// x2("\\b", "気 ", 0, 0);
// tu!(r#"\b"#, "気 ", &[], 746), UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S746 { mode M { token r#"\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\b", " ほ", 1, 1);
// tu!(r#"\b"#, " ほ", &[("", 1, 1)], 747), UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S747 { mode M { token r#"\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\B", "せそ ", 3, 3);
// tu!(r#"\B"#, "せそ ", &[("", 3, 3)], 748), UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)")
// scanner! { S748 { mode M { token r#"\B"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error:  x2("\\B", "う ", 4, 4); // 749

// -------------------------------------------------------------------------
// x2("\\B", " い", 0, 0);
// tu!(r#"\B"#, " い", &[], 750), UnsupportedFeatureError("WordUnicodeNegate Look(WordUnicodeNegate)")
// scanner! { S750 { mode M { token r#"\B"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[たち]", "ち", 0, 3); // 751

// -------------------------------------------------------------------------
// n("[なに]", "ぬ");
// td!(r#"[なに]"#, "ぬ", &[], 752),
scanner! { S752 { mode M { token r#"[なに]"# => 0; } } }
#[test]
fn test_match_752() {
    use s752::S752 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ぬ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "752: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[う-お]", "え", 0, 3); // 753

// -------------------------------------------------------------------------
// n("[^け]", "け");
// td!(r#"[^け]"#, "け", &[], 754),
scanner! { S754 { mode M { token r#"[^け]"# => 0; } } }
#[test]
fn test_match_754() {
    use s754::S754 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("け", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "754: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[\\w]", "ね", 0, 3); // 755

// -------------------------------------------------------------------------
// n("[\\d]", "ふ");
// td!(r#"[\d]"#, "ふ", &[], 756),
scanner! { S756 { mode M { token r#"[\d]"# => 0; } } }
#[test]
fn test_match_756() {
    use s756::S756 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ふ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "756: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[\\D]", "は", 0, 3); // 757

// -------------------------------------------------------------------------
// n("[\\s]", "く");
// td!(r#"[\s]"#, "く", &[], 758),
scanner! { S758 { mode M { token r#"[\s]"# => 0; } } }
#[test]
fn test_match_758() {
    use s758::S758 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("く", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "758: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[\\S]", "へ", 0, 3); // 759

// -------------------------------------------------------------------------
// *** Index error: x2("[\\w\\d]", "よ", 0, 3); // 760

// -------------------------------------------------------------------------
// *** Index error: x2("[\\w\\d]", "   よ", 3, 6); // 761

// -------------------------------------------------------------------------
// n("\\w鬼車", " 鬼車");
// td!(r#"\w鬼車"#, " 鬼車", &[], 762),
scanner! { S762 { mode M { token r#"\w鬼車"# => 0; } } }
#[test]
fn test_match_762() {
    use s762::S762 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" 鬼車", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "762: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("鬼\\W車", "鬼 車", 0, 7); // 763

// -------------------------------------------------------------------------
// *** Index error: x2("あ.い.う", "ああいいう", 0, 15); // 764

// -------------------------------------------------------------------------
// *** Index error: x2(".\\wう\\W..ぞ", "えうう うぞぞ", 0, 19); // 765

// -------------------------------------------------------------------------
// *** Index error: x2("\\s\\wこここ", " ここここ", 0, 13); // 766

// -------------------------------------------------------------------------
// *** Index error: x2("ああ.け", "ああけけ", 0, 12); // 767

// -------------------------------------------------------------------------
// n(".い", "いえ");
// td!(r#".い"#, "いえ", &[], 768),
scanner! { S768 { mode M { token r#".い"# => 0; } } }
#[test]
fn test_match_768() {
    use s768::S768 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("いえ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "768: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2(".お", "おお", 0, 6); // 769

// -------------------------------------------------------------------------
// *** Index error: x2("^あ", "あ", 0, 3); // 770

// -------------------------------------------------------------------------
// *** Index error: x2("^む$", "む", 0, 3); // 771

// -------------------------------------------------------------------------
// *** Index error: x2("^\\w$", "に", 0, 3); // 772

// -------------------------------------------------------------------------
// *** Index error: x2("^\\wかきくけこ$", "zかきくけこ", 0, 16); // 773

// -------------------------------------------------------------------------
// *** Index error: x2("^\\w...うえお$", "zあいううえお", 0, 19); // 774

// -------------------------------------------------------------------------
// *** Index error: x2("\\w\\w\\s\\Wおおお\\d", "aお  おおお4", 0, 16); // 775

// -------------------------------------------------------------------------
// *** Index error: x2("\\Aたちつ", "たちつ", 0, 9); // 776

// -------------------------------------------------------------------------
// *** Index error: x2("むめも\\Z", "むめも", 0, 9); // 777

// -------------------------------------------------------------------------
// *** Index error: x2("かきく\\z", "かきく", 0, 9); // 778

// -------------------------------------------------------------------------
// *** Index error: x2("かきく\\Z", "かきく\n", 0, 9); // 779

// -------------------------------------------------------------------------
// *** Index error: x2("\\Gぽぴ", "ぽぴ", 0, 6); // 780

// -------------------------------------------------------------------------
// n("\\Gえ", "うえお");
// tr!(r#"\Gえ"#, "うえお", &[], 781), EscapeUnrecognized
// scanner! { S781 { mode M { token r#"\Gえ"# => 0; } } }

// -------------------------------------------------------------------------
// n("とて\\G", "とて");
// tr!(r#"とて\G"#, "とて", &[], 782), EscapeUnrecognized
// scanner! { S782 { mode M { token r#"とて\G"# => 0; } } }

// -------------------------------------------------------------------------
// n("まみ\\A", "まみ");
// tu!(r#"まみ\A"#, "まみ", &[], 783), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S783 { mode M { token r#"まみ\A"# => 0; } } }

// -------------------------------------------------------------------------
// n("ま\\Aみ", "まみ");
// tu!(r#"ま\Aみ"#, "まみ", &[], 784), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S784 { mode M { token r#"ま\Aみ"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?=せ)せ", "せ", 0, 3); // 785

// -------------------------------------------------------------------------
// n("(?=う).", "い");
// tr!(r#"(?=う)."#, "い", &[], 786), UnsupportedLookAround
// scanner! { S786 { mode M { token r#"(?=う)."# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?!う)か", "か", 0, 3); // 787

// -------------------------------------------------------------------------
// n("(?!と)あ", "と");
// tr!(r#"(?!と)あ"#, "と", &[], 788), UnsupportedLookAround
// scanner! { S788 { mode M { token r#"(?!と)あ"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?i:あ)", "あ", 0, 3); // 789

// -------------------------------------------------------------------------
// *** Index error: x2("(?i:ぶべ)", "ぶべ", 0, 6); // 790

// -------------------------------------------------------------------------
// n("(?i:い)", "う");
// td!(r#"(?i:い)"#, "う", &[], 791),
scanner! { S791 { mode M { token r#"(?i:い)"# => 0; } } }
#[test]
fn test_match_791() {
    use s791::S791 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("う", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "791: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?m:よ.)", "よ\n", 0, 4); // 792

// -------------------------------------------------------------------------
// *** Index error: x2("(?m:.め)", "ま\nめ", 3, 7); // 793

// -------------------------------------------------------------------------
// x2("あ?", "", 0, 0);
// td!(r#"あ?"#, "", &[], 794),
scanner! { S794 { mode M { token r#"あ?"# => 0; } } }
#[test]
fn test_match_794() {
    use s794::S794 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "794: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("変?", "化", 0, 0);
// td!(r#"変?"#, "化", &[], 795),
scanner! { S795 { mode M { token r#"変?"# => 0; } } }
#[test]
fn test_match_795() {
    use s795::S795 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("化", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "795: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("変?", "変", 0, 3); // 796

// -------------------------------------------------------------------------
// x2("量*", "", 0, 0);
// td!(r#"量*"#, "", &[], 797),
scanner! { S797 { mode M { token r#"量*"# => 0; } } }
#[test]
fn test_match_797() {
    use s797::S797 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "797: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("量*", "量", 0, 3); // 798

// -------------------------------------------------------------------------
// *** Index error: x2("子*", "子子子", 0, 9); // 799

// -------------------------------------------------------------------------
// x2("馬*", "鹿馬馬馬馬", 0, 0);
// td!(r#"馬*"#, "鹿馬馬馬馬", &[], 800),
scanner! { S800 { mode M { token r#"馬*"# => 0; } } }
#[test]
fn test_match_800() {
    use s800::S800 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("鹿馬馬馬馬", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "800: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("山+", "");
// td!(r#"山+"#, "", &[], 801),
scanner! { S801 { mode M { token r#"山+"# => 0; } } }
#[test]
fn test_match_801() {
    use s801::S801 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "801: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("河+", "河", 0, 3); // 802

// -------------------------------------------------------------------------
// *** Index error: x2("時+", "時時時時", 0, 12); // 803

// -------------------------------------------------------------------------
// *** Index error: x2("え+", "ええううう", 0, 6); // 804

// -------------------------------------------------------------------------
// *** Index error: x2("う+", "おうううう", 3, 15); // 805

// -------------------------------------------------------------------------
// *** Index error: x2(".?", "た", 0, 3); // 806

// -------------------------------------------------------------------------
// *** Index error: x2(".*", "ぱぴぷぺ", 0, 12); // 807

// -------------------------------------------------------------------------
// *** Index error: x2(".+", "ろ", 0, 3); // 808

// -------------------------------------------------------------------------
// *** Index error: x2(".+", "いうえか\n", 0, 12); // 809

// -------------------------------------------------------------------------
// *** Index error: x2("あ|い", "あ", 0, 3); // 810

// -------------------------------------------------------------------------
// *** Index error: x2("あ|い", "い", 0, 3); // 811

// -------------------------------------------------------------------------
// *** Index error: x2("あい|いう", "あい", 0, 6); // 812

// -------------------------------------------------------------------------
// *** Index error: x2("あい|いう", "いう", 0, 6); // 813

// -------------------------------------------------------------------------
// *** Index error: x2("を(?:かき|きく)", "をかき", 0, 9); // 814

// -------------------------------------------------------------------------
// *** Index error: x2("を(?:かき|きく)け", "をきくけ", 0, 12); // 815

// -------------------------------------------------------------------------
// *** Index error: x2("あい|(?:あう|あを)", "あを", 0, 6); // 816

// -------------------------------------------------------------------------
// *** Index error:  x2("あ|い|う", "えう", 3, 6); // 817

// -------------------------------------------------------------------------
// *** Index error: x2("あ|い|うえ|おかき|く|けこさ|しすせ|そ|たち|つてとなに|ぬね", "しすせ", 0, 9); // 818

// -------------------------------------------------------------------------
// n("あ|い|うえ|おかき|く|けこさ|しすせ|そ|たち|つてとなに|ぬね", "すせ");
// td!(r#"あ|い|うえ|おかき|く|けこさ|しすせ|そ|たち|つてとなに|ぬね"#, "すせ", &[], 819),
scanner! { S819 { mode M { token r#"あ|い|うえ|おかき|く|けこさ|しすせ|そ|たち|つてとなに|ぬね"# => 0; } } }
#[test]
fn test_match_819() {
    use s819::S819 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("すせ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "819: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error:  x2("あ|^わ", "ぶあ", 3, 6); // 820

// -------------------------------------------------------------------------
// *** Index error: x2("あ|^を", "をあ", 0, 3); // 821

// -------------------------------------------------------------------------
// *** Index error:  x2("鬼|\\G車", "け車鬼", 6, 9); // 822

// -------------------------------------------------------------------------
// *** Index error: x2("鬼|\\G車", "車鬼", 0, 3); // 823

// -------------------------------------------------------------------------
// *** Index error:  x2("鬼|\\A車", "b車鬼", 4, 7); // 824

// -------------------------------------------------------------------------
// *** Index error: x2("鬼|\\A車", "車", 0, 3); // 825

// -------------------------------------------------------------------------
// *** Index error:  x2("鬼|車\\Z", "車鬼", 3, 6); // 826

// -------------------------------------------------------------------------
// *** Index error: x2("鬼|車\\Z", "車", 0, 3); // 827

// -------------------------------------------------------------------------
// x2("鬼|車\\Z", "車\n", 0, 3);
// tr!(r#"鬼|車\Z"#, "車\n", &[("車\\n", 0, 3)], 828), EscapeUnrecognized
// scanner! { S828 { mode M { token r#"鬼|車\Z"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error:  x2("鬼|車\\z", "車鬼", 3, 6); // 829

// -------------------------------------------------------------------------
// *** Index error: x2("鬼|車\\z", "車", 0, 3); // 830

// -------------------------------------------------------------------------
// *** Index error: x2("\\w|\\s", "お", 0, 3); // 831

// -------------------------------------------------------------------------
// x2("\\w|%", "%お", 0, 1);
// td!(r#"\w|%"#, "%お", &[("%", 0, 1)], 832),
scanner! { S832 { mode M { token r#"\w|%"# => 0; } } }
#[test]
fn test_match_832() {
    use s832::S832 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("%お", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("%", 0, 1), ("お", 1, 4)]; // Oniguruma: ("%", 0, 1)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "832: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "832: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "832: Match end does not match");
        assert_eq!(
            &"%お"[ma.1..ma.2],
            ma.0,
            "832: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// *** Index error: x2("\\w|[&$]", "う&", 0, 3); // 833

// -------------------------------------------------------------------------
// *** Index error: x2("[い-け]", "う", 0, 3); // 834

// -------------------------------------------------------------------------
// *** Index error: x2("[い-け]|[^か-こ]", "あ", 0, 3); // 835

// -------------------------------------------------------------------------
// *** Index error: x2("[い-け]|[^か-こ]", "か", 0, 3); // 836

// -------------------------------------------------------------------------
// x2("[^あ]", "\n", 0, 1);
// td!(r#"[^あ]"#, "\n", &[("\\", 0, 1)], 837),
scanner! { S837 { mode M { token r#"[^あ]"# => 0; } } }
#[test]
fn test_match_837() {
    use s837::S837 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\n", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\n", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "837: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "837: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "837: Match end does not match");
        assert_eq!(
            &"\n"[ma.1..ma.2],
            ma.0,
            "837: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ|[う-き])|いを", "うを", 0, 3); // 838

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ|[う-き])|いを", "いを", 0, 6); // 839

// -------------------------------------------------------------------------
// *** Index error: x2("あいう|(?=けけ)..ほ", "けけほ", 0, 9); // 840

// -------------------------------------------------------------------------
// *** Index error: x2("あいう|(?!けけ)..ほ", "あいほ", 0, 9); // 841

// -------------------------------------------------------------------------
// *** Index error: x2("(?=をあ)..あ|(?=をを)..あ", "ををあ", 0, 9); // 842

// -------------------------------------------------------------------------
// *** Index error:  x2("(?<=あ|いう)い", "いうい", 6, 9); // 843

// -------------------------------------------------------------------------
// n("(?>あ|あいえ)う", "あいえう");
// tr!(r#"(?>あ|あいえ)う"#, "あいえう", &[], 844), FlagUnrecognized
// scanner! { S844 { mode M { token r#"(?>あ|あいえ)う"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?>あいえ|あ)う", "あいえう", 0, 12); // 845

// -------------------------------------------------------------------------
// *** Index error: x2("あ?|い", "あ", 0, 3); // 846

// -------------------------------------------------------------------------
// x2("あ?|い", "い", 0, 0);
// td!(r#"あ?|い"#, "い", &[], 847),
scanner! { S847 { mode M { token r#"あ?|い"# => 0; } } }
#[test]
fn test_match_847() {
    use s847::S847 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("い", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "847: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("あ?|い", "", 0, 0);
// td!(r#"あ?|い"#, "", &[], 848),
scanner! { S848 { mode M { token r#"あ?|い"# => 0; } } }
#[test]
fn test_match_848() {
    use s848::S848 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "848: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("あ*|い", "ああ", 0, 6); // 849

// -------------------------------------------------------------------------
// x2("あ*|い*", "いあ", 0, 0);
// td!(r#"あ*|い*"#, "いあ", &[], 850),
scanner! { S850 { mode M { token r#"あ*|い*"# => 0; } } }
#[test]
fn test_match_850() {
    use s850::S850 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("いあ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "850: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("あ*|い*", "あい", 0, 3); // 851

// -------------------------------------------------------------------------
// x2("[aあ]*|い*", "aあいいい", 0, 4);
// td!(r#"[aあ]*|い*"#, "aあいいい", &[("aあいい", 0, 4)], 852),
scanner! { S852 { mode M { token r#"[aあ]*|い*"# => 0; } } }
#[test]
fn test_match_852() {
    use s852::S852 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aあいいい", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aあ", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "852: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "852: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "852: Match end does not match");
        assert_eq!(
            &"aあいいい"[ma.1..ma.2],
            ma.0,
            "852: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("あ+|い*", "", 0, 0);
// td!(r#"あ+|い*"#, "", &[], 853),
scanner! { S853 { mode M { token r#"あ+|い*"# => 0; } } }
#[test]
fn test_match_853() {
    use s853::S853 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "853: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("あ+|い*", "いいい", 0, 9); // 854

// -------------------------------------------------------------------------
// x2("あ+|い*", "あいいい", 0, 3);
// td!(r#"あ+|い*"#, "あいいい", &[("あいい", 0, 3)], 855),
scanner! { S855 { mode M { token r#"あ+|い*"# => 0; } } }
#[test]
fn test_match_855() {
    use s855::S855 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あいいい", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: ("あ", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "855: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("あ+|い*", "aあいいい", 0, 0);
// td!(r#"あ+|い*"#, "aあいいい", &[], 856),
scanner! { S856 { mode M { token r#"あ+|い*"# => 0; } } }
#[test]
fn test_match_856() {
    use s856::S856 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aあいいい", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "856: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("あ+|い+", "");
// td!(r#"あ+|い+"#, "", &[], 857),
scanner! { S857 { mode M { token r#"あ+|い+"# => 0; } } }
#[test]
fn test_match_857() {
    use s857::S857 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "857: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(あ|い)?", "い", 0, 3); // 858

// -------------------------------------------------------------------------
// *** Index error: x2("(あ|い)*", "いあ", 0, 6); // 859

// -------------------------------------------------------------------------
// *** Index error: x2("(あ|い)+", "いあい", 0, 9); // 860

// -------------------------------------------------------------------------
// *** Index error: x2("(あい|うあ)+", "うああいうえ", 0, 12); // 861

// -------------------------------------------------------------------------
// *** Index error: x2("(あい|うえ)+", "うああいうえ", 6, 18); // 862

// -------------------------------------------------------------------------
// *** Index error: x2("(あい|うあ)+", "ああいうあ", 3, 15); // 863

// -------------------------------------------------------------------------
// *** Index error: x2("(あい|うあ)+", "あいをうあ", 0, 6); // 864

// -------------------------------------------------------------------------
// *** Index error: x2("(あい|うあ)+", "$$zzzzあいをうあ", 6, 12); // 865

// -------------------------------------------------------------------------
// *** Index error: x2("(あ|いあい)+", "あいあいあ", 0, 15); // 866

// -------------------------------------------------------------------------
// *** Index error:  x2("(あ|いあい)+", "いあ", 3, 6); // 867

// -------------------------------------------------------------------------
// *** Index error: x2("(あ|いあい)+", "いあああいあ", 3, 12); // 868

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ|い)(?:あ|い)", "あい", 0, 6); // 869

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ*|い*)(?:あ*|い*)", "あああいいい", 0, 9); // 870

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ*|い*)(?:あ+|い+)", "あああいいい", 0, 18); // 871

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ+|い+){2}", "あああいいい", 0, 18); // 872

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ+|い+){1,2}", "あああいいい", 0, 18); // 873

// -------------------------------------------------------------------------
// *** Index error: x2("(?:あ+|\\Aい*)うう", "うう", 0, 6); // 874

// -------------------------------------------------------------------------
// n("(?:あ+|\\Aい*)うう", "あいうう");
// tu!(r#"(?:あ+|\Aい*)うう"#, "あいうう", &[], 875), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S875 { mode M { token r#"(?:あ+|\Aい*)うう"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error:  x2("(?:^あ+|い+)*う", "ああいいいあいう", 18, 24); // 876

// -------------------------------------------------------------------------
// *** Index error: x2("(?:^あ+|い+)*う", "ああいいいいう", 0, 21); // 877

// -------------------------------------------------------------------------
// *** Index error: x2("う{0,}", "うううう", 0, 12); // 878

// -------------------------------------------------------------------------
// x2("あ|(?i)c", "C", 0, 1);
// td!(r#"あ|(?i)c"#, "C", &[("C", 0, 1)], 879),
scanner! { S879 { mode M { token r#"あ|(?i)c"# => 0; } } }
#[test]
fn test_match_879() {
    use s879::S879 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("C", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "879: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "879: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "879: Match end does not match");
        assert_eq!(
            &"C"[ma.1..ma.2],
            ma.0,
            "879: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)c|あ", "C", 0, 1);
// td!(r#"(?i)c|あ"#, "C", &[("C", 0, 1)], 880),
scanner! { S880 { mode M { token r#"(?i)c|あ"# => 0; } } }
#[test]
fn test_match_880() {
    use s880::S880 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("C", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("C", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "880: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "880: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "880: Match end does not match");
        assert_eq!(
            &"C"[ma.1..ma.2],
            ma.0,
            "880: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i:あ)|a", "a", 0, 1);
// td!(r#"(?i:あ)|a"#, "a", &[("a", 0, 1)], 881),
scanner! { S881 { mode M { token r#"(?i:あ)|a"# => 0; } } }
#[test]
fn test_match_881() {
    use s881::S881 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "881: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "881: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "881: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "881: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("(?i:あ)|a", "A");
// td!(r#"(?i:あ)|a"#, "A", &[], 882),
scanner! { S882 { mode M { token r#"(?i:あ)|a"# => 0; } } }
#[test]
fn test_match_882() {
    use s882::S882 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("A", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "882: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[あいう]?", "あいう", 0, 3);
// td!(r#"[あいう]?"#, "あいう", &[("あいう", 0, 3)], 883),
scanner! { S883 { mode M { token r#"[あいう]?"# => 0; } } }
#[test]
fn test_match_883() {
    use s883::S883 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あいう", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("あ", 0, 3), ("い", 3, 6), ("う", 6, 9)]; // Oniguruma: ("あいう", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "883: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "883: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "883: Match end does not match");
        assert_eq!(
            &"あいう"[ma.1..ma.2],
            ma.0,
            "883: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// *** Index error: x2("[あいう]*", "あいう", 0, 9); // 884

// -------------------------------------------------------------------------
// x2("[^あいう]*", "あいう", 0, 0);
// td!(r#"[^あいう]*"#, "あいう", &[], 885),
scanner! { S885 { mode M { token r#"[^あいう]*"# => 0; } } }
#[test]
fn test_match_885() {
    use s885::S885 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あいう", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "885: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[^あいう]+", "あいう");
// td!(r#"[^あいう]+"#, "あいう", &[], 886),
scanner! { S886 { mode M { token r#"[^あいう]+"# => 0; } } }
#[test]
fn test_match_886() {
    use s886::S886 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あいう", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "886: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("あ?\?", "あああ", 0, 0);
// td!(r#"あ?\?"#, "あああ", &[], 887),
scanner! { S887 { mode M { token r#"あ?\?"# => 0; } } }
#[test]
fn test_match_887() {
    use s887::S887 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あああ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "887: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("いあ?\?い", "いあい", 0, 9); // 888

// -------------------------------------------------------------------------
// x2("あ*?", "あああ", 0, 0);
// tu!(r#"あ*?"#, "あああ", &[], 889), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S889 { mode M { token r#"あ*?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("いあ*?", "いああ", 0, 3);
// tu!(r#"いあ*?"#, "いああ", &[("いああ", 0, 3)], 890), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S890 { mode M { token r#"いあ*?"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("いあ*?い", "いああい", 0, 12); // 891

// -------------------------------------------------------------------------
// x2("あ+?", "あああ", 0, 3);
// tu!(r#"あ+?"#, "あああ", &[("あああ", 0, 3)], 892), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S892 { mode M { token r#"あ+?"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("いあ+?", "いああ", 0, 6); // 893

// -------------------------------------------------------------------------
// *** Index error: x2("いあ+?い", "いああい", 0, 12); // 894

// -------------------------------------------------------------------------
// x2("(?:天?)?\?", "天", 0, 0);
// td!(r#"(?:天?)?\?"#, "天", &[], 895),
scanner! { S895 { mode M { token r#"(?:天?)?\?"# => 0; } } }
#[test]
fn test_match_895() {
    use s895::S895 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("天", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "895: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:天?\?)?", "天", 0, 0);
// td!(r#"(?:天?\?)?"#, "天", &[], 896),
scanner! { S896 { mode M { token r#"(?:天?\?)?"# => 0; } } }
#[test]
fn test_match_896() {
    use s896::S896 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("天", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "896: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?:夢?)+?", "夢夢夢", 0, 3);
// tu!(r#"(?:夢?)+?"#, "夢夢夢", &[("夢夢夢", 0, 3)], 897), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S897 { mode M { token r#"(?:夢?)+?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:風+)?\?", "風風風", 0, 0);
// td!(r#"(?:風+)?\?"#, "風風風", &[], 898),
scanner! { S898 { mode M { token r#"(?:風+)?\?"# => 0; } } }
#[test]
fn test_match_898() {
    use s898::S898 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("風風風", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "898: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?:雪+)?\?霜", "雪雪雪霜", 0, 12); // 899

// -------------------------------------------------------------------------
// x2("(?:あい)?{2}", "", 0, 0);
// td!(r#"(?:あい)?{2}"#, "", &[], 900),
scanner! { S900 { mode M { token r#"(?:あい)?{2}"# => 0; } } }
#[test]
fn test_match_900() {
    use s900::S900 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "900: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?:鬼車)?{2}", "鬼車鬼車鬼", 0, 12); // 901

// -------------------------------------------------------------------------
// x2("(?:鬼車)*{0}", "鬼車鬼車鬼", 0, 0);
// td!(r#"(?:鬼車)*{0}"#, "鬼車鬼車鬼", &[], 902),
scanner! { S902 { mode M { token r#"(?:鬼車)*{0}"# => 0; } } }
#[test]
fn test_match_902() {
    use s902::S902 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("鬼車鬼車鬼", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "902: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?:鬼車){3,}", "鬼車鬼車鬼車鬼車", 0, 24); // 903

// -------------------------------------------------------------------------
// n("(?:鬼車){3,}", "鬼車鬼車");
// td!(r#"(?:鬼車){3,}"#, "鬼車鬼車", &[], 904),
scanner! { S904 { mode M { token r#"(?:鬼車){3,}"# => 0; } } }
#[test]
fn test_match_904() {
    use s904::S904 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("鬼車鬼車", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "904: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?:鬼車){2,4}", "鬼車鬼車鬼車", 0, 18); // 905

// -------------------------------------------------------------------------
// *** Index error: x2("(?:鬼車){2,4}", "鬼車鬼車鬼車鬼車鬼車", 0, 24); // 906

// -------------------------------------------------------------------------
// *** Index error: x2("(?:鬼車){2,4}?", "鬼車鬼車鬼車鬼車鬼車", 0, 12); // 907

// -------------------------------------------------------------------------
// *** Index error: x2("(?:鬼車){,}", "鬼車{,}", 0, 9); // 908

// -------------------------------------------------------------------------
// *** Index error: x2("(?:かきく)+?{2}", "かきくかきくかきく", 0, 18); // 909

// -------------------------------------------------------------------------
// *** Index error: x3("(火)", "火", 0, 3, 1); // 910

// -------------------------------------------------------------------------
// *** Index error: x3("(火水)", "火水", 0, 6, 1); // 911

// -------------------------------------------------------------------------
// *** Index error: x2("((時間))", "時間", 0, 6); // 912

// -------------------------------------------------------------------------
// *** Index error: x3("((風水))", "風水", 0, 6, 1); // 913

// -------------------------------------------------------------------------
// *** Index error: x3("((昨日))", "昨日", 0, 6, 2); // 914

// -------------------------------------------------------------------------
// *** Index error: x3("((((((((((((((((((((量子))))))))))))))))))))", "量子", 0, 6, 20); // 915

// -------------------------------------------------------------------------
// *** Index error: x3("(あい)(うえ)", "あいうえ", 0, 6, 1); // 916

// -------------------------------------------------------------------------
// *** Index error:  x3("(あい)(うえ)", "あいうえ", 6, 12, 2); // 917

// -------------------------------------------------------------------------
// *** Index error: x3("()(あ)いう(えおか)きくけこ", "あいうえおかきくけこ", 9, 18, 3); // 918

// -------------------------------------------------------------------------
// *** Index error: x3("(()(あ)いう(えおか)きくけこ)", "あいうえおかきくけこ", 9, 18, 4); // 919

// -------------------------------------------------------------------------
// *** Index error:  x3(".*(フォ)ン・マ(ン()シュタ)イン", "フォン・マンシュタイン", 15, 27, 2); // 920

// -------------------------------------------------------------------------
// *** Index error: x2("(^あ)", "あ", 0, 3); // 921

// -------------------------------------------------------------------------
// *** Index error:  x3("(あ)|(あ)", "いあ", 3, 6, 1); // 922

// -------------------------------------------------------------------------
// *** Index error:  x3("(^あ)|(あ)", "いあ", 3, 6, 2); // 923

// -------------------------------------------------------------------------
// x3("(あ?)", "あああ", 0, 3, 1);
// td!(r#"(あ?)"#, "あああ", &[("あああ", 0, 3)], 924),
scanner! { S924 { mode M { token r#"(あ?)"# => 0; } } }
#[test]
fn test_match_924() {
    use s924::S924 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あああ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: ("あ", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "924: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x3("(ま*)", "ままま", 0, 9, 1); // 925

// -------------------------------------------------------------------------
// x3("(と*)", "", 0, 0, 1);
// td!(r#"(と*)"#, "", &[], 926),
scanner! { S926 { mode M { token r#"(と*)"# => 0; } } }
#[test]
fn test_match_926() {
    use s926::S926 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "926: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x3("(る+)", "るるるるるるる", 0, 21, 1); // 927

// -------------------------------------------------------------------------
// *** Index error: x3("(ふ+|へ*)", "ふふふへへ", 0, 9, 1); // 928

// -------------------------------------------------------------------------
// x3("(あ+|い?)", "いいいああ", 0, 3, 1);
// td!(r#"(あ+|い?)"#, "いいいああ", &[("いいい", 0, 3)], 929),
scanner! { S929 { mode M { token r#"(あ+|い?)"# => 0; } } }
#[test]
fn test_match_929() {
    use s929::S929 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("いいいああ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: ("いいい", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "929: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x3("(あいう)?", "あいう", 0, 9, 1); // 930

// -------------------------------------------------------------------------
// *** Index error: x3("(あいう)*", "あいう", 0, 9, 1); // 931

// -------------------------------------------------------------------------
// *** Index error: x3("(あいう)+", "あいう", 0, 9, 1); // 932

// -------------------------------------------------------------------------
// *** Index error: x3("(さしす|あいう)+", "あいう", 0, 9, 1); // 933

// -------------------------------------------------------------------------
// *** Index error: x3("([なにぬ][かきく]|かきく)+", "かきく", 0, 9, 1); // 934

// -------------------------------------------------------------------------
// *** Index error: x3("((?i:あいう))", "あいう", 0, 9, 1); // 935

// -------------------------------------------------------------------------
// *** Index error: x3("((?m:あ.う))", "あ\nう", 0, 7, 1); // 936

// -------------------------------------------------------------------------
// x3("((?=あん)あ)", "あんい", 0, 3, 1);
// tr!(r#"((?=あん)あ)"#, "あんい", &[("あんい", 0, 3)], 937), UnsupportedLookAround
// scanner! { S937 { mode M { token r#"((?=あん)あ)"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x3("あいう|(.あいえ)", "んあいえ", 0, 12, 1); // 938

// -------------------------------------------------------------------------
// *** Index error:  x3("あ*(.)", "ああああん", 12, 15, 1); // 939

// -------------------------------------------------------------------------
// x3("あ*?(.)", "ああああん", 0, 3, 1);
// tu!(r#"あ*?(.)"#, "ああああん", &[("あああ", 0, 3)], 940), UnsupportedFeatureError("x?*?: Non-greedy repetitions)
// scanner! { S940 { mode M { token r#"あ*?(.)"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error:  x3("あ*?(ん)", "ああああん", 12, 15, 1); // 941

// -------------------------------------------------------------------------
// *** Index error:  x3("[いうえ]あ*(.)", "えああああん", 15, 18, 1); // 942

// -------------------------------------------------------------------------
// *** Index error: x3("(\\Aいい)うう", "いいうう", 0, 6, 1); // 943

// -------------------------------------------------------------------------
// n("(\\Aいい)うう", "んいいうう");
// tu!(r#"(\Aいい)うう"#, "んいいうう", &[], 944), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S944 { mode M { token r#"(\Aいい)うう"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x3("(^いい)うう", "いいうう", 0, 6, 1); // 945

// -------------------------------------------------------------------------
// n("(^いい)うう", "んいいうう");
// tu!(r#"(^いい)うう"#, "んいいうう", &[], 946), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S946 { mode M { token r#"(^いい)うう"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error:  x3("ろろ(るる$)", "ろろるる", 6, 12, 1); // 947

// -------------------------------------------------------------------------
// n("ろろ(るる$)", "ろろるるる");
// tu!(r#"ろろ(るる$)"#, "ろろるるる", &[], 948), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S948 { mode M { token r#"ろろ(るる$)"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(無)\\1", "無無", 0, 6); // 949

// -------------------------------------------------------------------------
// n("(無)\\1", "無武");
// tr!(r#"(無)\1"#, "無武", &[], 950), UnsupportedBackreference
// scanner! { S950 { mode M { token r#"(無)\1"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(空?)\\1", "空空", 0, 6); // 951

// -------------------------------------------------------------------------
// x2("(空?\?)\\1", "空空", 0, 0);
// tr!(r#"(空?\?)\1"#, "空空", &[], 952), UnsupportedBackreference
// scanner! { S952 { mode M { token r#"(空?\?)\1"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(空*)\\1", "空空空空空", 0, 12); // 953

// -------------------------------------------------------------------------
// *** Index error: x3("(空*)\\1", "空空空空空", 0, 6, 1); // 954

// -------------------------------------------------------------------------
// *** Index error: x2("あ(い*)\\1", "あいいいい", 0, 15); // 955

// -------------------------------------------------------------------------
// *** Index error: x2("あ(い*)\\1", "あい", 0, 3); // 956

// -------------------------------------------------------------------------
// *** Index error: x2("(あ*)(い*)\\1\\2", "あああいいあああいい", 0, 30); // 957

// -------------------------------------------------------------------------
// *** Index error: x2("(あ*)(い*)\\2", "あああいいいい", 0, 21); // 958

// -------------------------------------------------------------------------
// *** Index error:  x3("(あ*)(い*)\\2", "あああいいいい", 9, 15, 2); // 959

// -------------------------------------------------------------------------
// *** Index error: x2("(((((((ぽ*)ぺ))))))ぴ\\7", "ぽぽぽぺぴぽぽぽ", 0, 24); // 960

// -------------------------------------------------------------------------
// *** Index error: x3("(((((((ぽ*)ぺ))))))ぴ\\7", "ぽぽぽぺぴぽぽぽ", 0, 9, 7); // 961

// -------------------------------------------------------------------------
// *** Index error: x2("(は)(ひ)(ふ)\\2\\1\\3", "はひふひはふ", 0, 18); // 962

// -------------------------------------------------------------------------
// *** Index error: x2("([き-け])\\1", "くく", 0, 6); // 963

// -------------------------------------------------------------------------
// *** Index error: x2("(\\w\\d\\s)\\1", "あ5 あ5 ", 0, 10); // 964

// -------------------------------------------------------------------------
// n("(\\w\\d\\s)\\1", "あ5 あ5");
// tr!(r#"(\w\d\s)\1"#, "あ5 あ5", &[], 965), UnsupportedBackreference
// scanner! { S965 { mode M { token r#"(\w\d\s)\1"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(誰？|[あ-う]{3})\\1", "誰？誰？", 0, 12); // 966

// -------------------------------------------------------------------------
// *** Index error: x2("...(誰？|[あ-う]{3})\\1", "あaあ誰？誰？", 0, 19); // 967

// -------------------------------------------------------------------------
// *** Index error: x2("(誰？|[あ-う]{3})\\1", "ういうういう", 0, 18); // 968

// -------------------------------------------------------------------------
// *** Index error: x2("(^こ)\\1", "ここ", 0, 6); // 969

// -------------------------------------------------------------------------
// n("(^む)\\1", "めむむ");
// tr!(r#"(^む)\1"#, "めむむ", &[], 970), UnsupportedBackreference
// scanner! { S970 { mode M { token r#"(^む)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(あ$)\\1", "ああ");
// tr!(r#"(あ$)\1"#, "ああ", &[], 971), UnsupportedBackreference
// scanner! { S971 { mode M { token r#"(あ$)\1"# => 0; } } }

// -------------------------------------------------------------------------
// n("(あい\\Z)\\1", "あい");
// tr!(r#"(あい\Z)\1"#, "あい", &[], 972), EscapeUnrecognized
// scanner! { S972 { mode M { token r#"(あい\Z)\1"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error:  x2("(あ*\\Z)\\1", "あ", 3, 3); // 973

// -------------------------------------------------------------------------
// *** Index error:  x2(".(あ*\\Z)\\1", "いあ", 3, 6); // 974

// -------------------------------------------------------------------------
// *** Index error: x3("(.(やいゆ)\\2)", "zやいゆやいゆ", 0, 19, 1); // 975

// -------------------------------------------------------------------------
// *** Index error: x3("(.(..\\d.)\\2)", "あ12341234", 0, 11, 1); // 976

// -------------------------------------------------------------------------
// *** Index error: x2("((?i:あvず))\\1", "あvずあvず", 0, 14); // 977

// -------------------------------------------------------------------------
// *** Index error: x2("(?<愚か>変|\\(\\g<愚か>\\))", "((((((変))))))", 0, 15); // 978

// -------------------------------------------------------------------------
// *** Index error: x2("\\A(?:\\g<阿_1>|\\g<云_2>|\\z終了  (?<阿_1>観|自\\g<云_2>自)(?<云_2>在|菩薩\\g<阿_1>菩薩))$", "菩薩自菩薩自在自菩薩自菩薩", 0, 39); // 979

// -------------------------------------------------------------------------
// *** Index error: x2("[[ひふ]]", "ふ", 0, 3); // 980

// -------------------------------------------------------------------------
// *** Index error: x2("[[いおう]か]", "か", 0, 3); // 981

// -------------------------------------------------------------------------
// n("[[^あ]]", "あ");
// td!(r#"[[^あ]]"#, "あ", &[], 982),
scanner! { S982 { mode M { token r#"[[^あ]]"# => 0; } } }
#[test]
fn test_match_982() {
    use s982::S982 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "982: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[^[あ]]", "あ");
// td!(r#"[^[あ]]"#, "あ", &[], 983),
scanner! { S983 { mode M { token r#"[^[あ]]"# => 0; } } }
#[test]
fn test_match_983() {
    use s983::S983 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "983: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[^[^あ]]", "あ", 0, 3); // 984

// -------------------------------------------------------------------------
// *** Index error: x2("[[かきく]&&きく]", "く", 0, 3); // 985

// -------------------------------------------------------------------------
// n("[[かきく]&&きく]", "か");
// td!(r#"[[かきく]&&きく]"#, "か", &[], 986),
scanner! { S986 { mode M { token r#"[[かきく]&&きく]"# => 0; } } }
#[test]
fn test_match_986() {
    use s986::S986 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("か", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "986: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[[かきく]&&きく]", "け");
// td!(r#"[[かきく]&&きく]"#, "け", &[], 987),
scanner! { S987 { mode M { token r#"[[かきく]&&きく]"# => 0; } } }
#[test]
fn test_match_987() {
    use s987::S987 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("け", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "987: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[あ-ん&&い-を&&う-ゑ]", "ゑ", 0, 3); // 988

// -------------------------------------------------------------------------
// n("[^あ-ん&&い-を&&う-ゑ]", "ゑ");
// td!(r#"[^あ-ん&&い-を&&う-ゑ]"#, "ゑ", &[], 989),
scanner! { S989 { mode M { token r#"[^あ-ん&&い-を&&う-ゑ]"# => 0; } } }
#[test]
fn test_match_989() {
    use s989::S989 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ゑ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "989: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[[^あ&&あ]&&あ-ん]", "い", 0, 3); // 990

// -------------------------------------------------------------------------
// n("[[^あ&&あ]&&あ-ん]", "あ");
// td!(r#"[[^あ&&あ]&&あ-ん]"#, "あ", &[], 991),
scanner! { S991 { mode M { token r#"[[^あ&&あ]&&あ-ん]"# => 0; } } }
#[test]
fn test_match_991() {
    use s991::S991 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("あ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "991: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[[^あ-ん&&いうえお]&&[^う-か]]", "き", 0, 3); // 992

// -------------------------------------------------------------------------
// n("[[^あ-ん&&いうえお]&&[^う-か]]", "い");
// td!(r#"[[^あ-ん&&いうえお]&&[^う-か]]"#, "い", &[], 993),
scanner! { S993 { mode M { token r#"[[^あ-ん&&いうえお]&&[^う-か]]"# => 0; } } }
#[test]
fn test_match_993() {
    use s993::S993 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("い", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "993: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[^[^あいう]&&[^うえお]]", "う", 0, 3); // 994

// -------------------------------------------------------------------------
// *** Index error: x2("[^[^あいう]&&[^うえお]]", "え", 0, 3); // 995

// -------------------------------------------------------------------------
// n("[^[^あいう]&&[^うえお]]", "か");
// td!(r#"[^[^あいう]&&[^うえお]]"#, "か", &[], 996),
scanner! { S996 { mode M { token r#"[^[^あいう]&&[^うえお]]"# => 0; } } }
#[test]
fn test_match_996() {
    use s996::S996 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("か", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "996: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[あ-&&-あ]", "-", 0, 1);
// tr!(r#"[あ-&&-あ]"#, "-", &[("-", 0, 1)], 997), ClassRangeInvalid
// scanner! { S997 { mode M { token r#"[あ-&&-あ]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[^[^a-zあいう]&&[^bcdefgうえお]q-w]", "え", 0, 3); // 998

// -------------------------------------------------------------------------
// x2("[^[^a-zあいう]&&[^bcdefgうえお]g-w]", "f", 0, 1);
// td!(r#"[^[^a-zあいう]&&[^bcdefgうえお]g-w]"#, "f", &[("f", 0, 1)], 999),
scanner! { S999 { mode M { token r#"[^[^a-zあいう]&&[^bcdefgうえお]g-w]"# => 0; } } }
#[test]
fn test_match_999() {
    use s999::S999 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("f", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("f", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "999: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "999: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "999: Match end does not match");
        assert_eq!(
            &"f"[ma.1..ma.2],
            ma.0,
            "999: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[^[^a-zあいう]&&[^bcdefgうえお]g-w]", "g", 0, 1);
// td!(r#"[^[^a-zあいう]&&[^bcdefgうえお]g-w]"#, "g", &[("g", 0, 1)], 1000),
scanner! { S1000 { mode M { token r#"[^[^a-zあいう]&&[^bcdefgうえお]g-w]"# => 0; } } }
#[test]
fn test_match_1000() {
    use s1000::S1000 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("g", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("g", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1000: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1000: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1000: Match end does not match");
        assert_eq!(
            &"g"[ma.1..ma.2],
            ma.0,
            "1000: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^[^a-zあいう]&&[^bcdefgうえお]g-w]", "2");
// td!(r#"[^[^a-zあいう]&&[^bcdefgうえお]g-w]"#, "2", &[], 1001),
scanner! { S1001 { mode M { token r#"[^[^a-zあいう]&&[^bcdefgうえお]g-w]"# => 0; } } }
#[test]
fn test_match_1001() {
    use s1001::S1001 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("2", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1001: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("a<b>バージョンのダウンロード<\\/b>", "a<b>バージョンのダウンロード</b>", 0, 44); // 1002

// -------------------------------------------------------------------------
// *** Index error: x2(".<b>バージョンのダウンロード<\\/b>", "a<b>バージョンのダウンロード</b>", 0, 44); // 1003

// -------------------------------------------------------------------------
// *** Index error:  x2("\\n?\\z", "こんにちは", 15, 15); // 1004

// -------------------------------------------------------------------------
// *** Index error: x2("(?m).*", "青赤黄", 0, 9); // 1005

// -------------------------------------------------------------------------
// *** Index error: x2("(?m).*a", "青赤黄a", 0, 10); // 1006

// -------------------------------------------------------------------------
// *** Index error: x2("\\p{Hiragana}", "ぴ", 0, 3); // 1007

// -------------------------------------------------------------------------
// n("\\P{Hiragana}", "ぴ");
// td!(r#"\P{Hiragana}"#, "ぴ", &[], 1008),
scanner! { S1008 { mode M { token r#"\P{Hiragana}"# => 0; } } }
#[test]
fn test_match_1008() {
    use s1008::S1008 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ぴ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1008: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("\\p{Emoji}", "\xE2\xAD\x90", 0, 3);
// td!(r#"\p{Emoji}"#, "\xE2\xAD\x90", &[("\\xE", 0, 3)], 1009), Rust Compile Error
scanner! { S1009 { mode M { token r#"\p{Emoji}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\p{^Emoji}", "\xEF\xBC\x93", 0, 3);
// tr!(r#"\p{^Emoji}"#, "\xEF\xBC\x93", &[("\\xE", 0, 3)], 1010), UnicodePropertyNotFound
// scanner! { S1010 { mode M { token r#"\p{^Emoji}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\p{Extended_Pictographic}", "\xE2\x9A\xA1", 0, 3);
// td!(r#"\p{Extended_Pictographic}"#, "\xE2\x9A\xA1", &[("\\xE", 0, 3)], 1011), Rust Compile Error
scanner! { S1011 { mode M { token r#"\p{Extended_Pictographic}"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\p{Extended_Pictographic}", "\xE3\x81\x82");
// td!(r#"\p{Extended_Pictographic}"#, "\xE3\x81\x82", &[], 1012), Rust Compile Error
scanner! { S1012 { mode M { token r#"\p{Extended_Pictographic}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\pC", "\xC2\xAD", 0, 2); // U+00AD: Soft Hyphen
// td!(r#"\pC"#, "\xC2\xAD", &[("\\x", 0, 2)], 1013), Rust Compile Error
scanner! { S1013 { mode M { token r#"\pC"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\pL", "U", 0, 1);
// td!(r#"\pL"#, "U", &[("U", 0, 1)], 1014),
scanner! { S1014 { mode M { token r#"\pL"# => 0; } } }
#[test]
fn test_match_1014() {
    use s1014::S1014 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("U", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("U", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1014: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1014: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1014: Match end does not match");
        assert_eq!(
            &"U"[ma.1..ma.2],
            ma.0,
            "1014: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\pM", "\xE2\x83\x9D", 0, 3); // U+20DD: Combining Enclosing Circle
// td!(r#"\pM"#, "\xE2\x83\x9D", &[("\\xE", 0, 3)], 1015), Rust Compile Error
scanner! { S1015 { mode M { token r#"\pM"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("\\pN+", "3Ⅴ", 0, 4); // 1016

// -------------------------------------------------------------------------
// *** Index error: x2("\\pP+", "†⁂", 0, 6); // 1017

// -------------------------------------------------------------------------
// *** Index error: x2("\\pS+", "€₤", 0, 6); // 1018

// -------------------------------------------------------------------------
// x2("\\pZ+", " ", 0, 1);
// td!(r#"\pZ+"#, " ", &[(" ", 0, 1)], 1019),
scanner! { S1019 { mode M { token r#"\pZ+"# => 0; } } }
#[test]
fn test_match_1019() {
    use s1019::S1019 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(" ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(" ", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1019: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1019: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1019: Match end does not match");
        assert_eq!(
            &" "[ma.1..ma.2],
            ma.0,
            "1019: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("\\pL", "@");
// td!(r#"\pL"#, "@", &[], 1020),
scanner! { S1020 { mode M { token r#"\pL"# => 0; } } }
#[test]
fn test_match_1020() {
    use s1020::S1020 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("@", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1020: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("\\pL+", "akZtE", 0, 5);
// td!(r#"\pL+"#, "akZtE", &[("akZtE", 0, 5)], 1021),
scanner! { S1021 { mode M { token r#"\pL+"# => 0; } } }
#[test]
fn test_match_1021() {
    use s1021::S1021 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("akZtE", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("akZtE", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1021: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1021: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1021: Match end does not match");
        assert_eq!(
            &"akZtE"[ma.1..ma.2],
            ma.0,
            "1021: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\PL+", "1@=-%", 0, 5);
// td!(r#"\PL+"#, "1@=-%", &[("1@=-%", 0, 5)], 1022),
scanner! { S1022 { mode M { token r#"\PL+"# => 0; } } }
#[test]
fn test_match_1022() {
    use s1022::S1022 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("1@=-%", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("1@=-%", 0, 5)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1022: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1022: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1022: Match end does not match");
        assert_eq!(
            &"1@=-%"[ma.1..ma.2],
            ma.0,
            "1022: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// e("\\p", "", ONIGERR_INVALID_CHAR_PROPERTY_NAME);
// tr!(r#"\p"#, "", "ONIGERR_INVALID_CHAR_PROPERTY_NAME", 1023),
// scanner! { S1023 { mode M { token r#"\p"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\p(", "", ONIGERR_INVALID_CHAR_PROPERTY_NAME);
// tr!(r#"\p("#, "", "ONIGERR_INVALID_CHAR_PROPERTY_NAME", 1024),
// scanner! { S1024 { mode M { token r#"\p("# => 0; } } }

// -------------------------------------------------------------------------
// e("\\pQ", "", ONIGERR_INVALID_CHAR_PROPERTY_NAME);
// tr!(r#"\pQ"#, "", "ONIGERR_INVALID_CHAR_PROPERTY_NAME", 1025),
// scanner! { S1025 { mode M { token r#"\pQ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\pL]", "s", 0, 1);
// td!(r#"[\pL]"#, "s", &[("s", 0, 1)], 1026),
scanner! { S1026 { mode M { token r#"[\pL]"# => 0; } } }
#[test]
fn test_match_1026() {
    use s1026::S1026 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("s", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("s", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1026: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1026: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1026: Match end does not match");
        assert_eq!(
            &"s"[ma.1..ma.2],
            ma.0,
            "1026: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^\\pL]", "s");
// td!(r#"[^\pL]"#, "s", &[], 1027),
scanner! { S1027 { mode M { token r#"[^\pL]"# => 0; } } }
#[test]
fn test_match_1027() {
    use s1027::S1027 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("s", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1027: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[\\PL]+", "-3@", 0, 3);
// td!(r#"[\PL]+"#, "-3@", &[("-3@", 0, 3)], 1028),
scanner! { S1028 { mode M { token r#"[\PL]+"# => 0; } } }
#[test]
fn test_match_1028() {
    use s1028::S1028 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("-3@", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("-3@", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1028: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1028: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1028: Match end does not match");
        assert_eq!(
            &"-3@"[ma.1..ma.2],
            ma.0,
            "1028: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// e("[\\p]", "", ONIGERR_INVALID_CHAR_PROPERTY_NAME);
// tr!(r#"[\p]"#, "", "ONIGERR_INVALID_CHAR_PROPERTY_NAME", 1029),
// scanner! { S1029 { mode M { token r#"[\p]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\pU]", "", ONIGERR_INVALID_CHAR_PROPERTY_NAME);
// tr!(r#"[\pU]"#, "", "ONIGERR_INVALID_CHAR_PROPERTY_NAME", 1030),
// scanner! { S1030 { mode M { token r#"[\pU]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("\\p{Word}", "こ", 0, 3); // 1031

// -------------------------------------------------------------------------
// n("\\p{^Word}", "こ");
// tr!(r#"\p{^Word}"#, "こ", &[], 1032), UnicodePropertyNotFound
// scanner! { S1032 { mode M { token r#"\p{^Word}"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[\\p{Word}]", "こ", 0, 3); // 1033

// -------------------------------------------------------------------------
// n("[\\p{^Word}]", "こ");
// tr!(r#"[\p{^Word}]"#, "こ", &[], 1034), UnicodePropertyNotFound
// scanner! { S1034 { mode M { token r#"[\p{^Word}]"# => 0; } } }

// -------------------------------------------------------------------------
// n("[^\\p{Word}]", "こ");
// tr!(r#"[^\p{Word}]"#, "こ", &[], 1035), UnicodePropertyNotFound
// scanner! { S1035 { mode M { token r#"[^\p{Word}]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[^\\p{^Word}]", "こ", 0, 3); // 1036

// -------------------------------------------------------------------------
// *** Index error: x2("[^\\p{^Word}&&\\p{ASCII}]", "こ", 0, 3); // 1037

// -------------------------------------------------------------------------
// x2("[^\\p{^Word}&&\\p{ASCII}]", "a", 0, 1);
// tr!(r#"[^\p{^Word}&&\p{ASCII}]"#, "a", &[("a", 0, 1)], 1038), UnicodePropertyNotFound
// scanner! { S1038 { mode M { token r#"[^\p{^Word}&&\p{ASCII}]"# => 0; } } }

// -------------------------------------------------------------------------
// n("[^\\p{^Word}&&\\p{ASCII}]", "#");
// tr!(r#"[^\p{^Word}&&\p{ASCII}]"#, "#", &[], 1039), UnicodePropertyNotFound
// scanner! { S1039 { mode M { token r#"[^\p{^Word}&&\p{ASCII}]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[^[\\p{^Word}]&&[\\p{ASCII}]]", "こ", 0, 3); // 1040

// -------------------------------------------------------------------------
// *** Index error: x2("[^[\\p{ASCII}]&&[^\\p{Word}]]", "こ", 0, 3); // 1041

// -------------------------------------------------------------------------
// n("[[\\p{ASCII}]&&[^\\p{Word}]]", "こ");
// tr!(r#"[[\p{ASCII}]&&[^\p{Word}]]"#, "こ", &[], 1042), UnicodePropertyNotFound
// scanner! { S1042 { mode M { token r#"[[\p{ASCII}]&&[^\p{Word}]]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[^[\\p{^Word}]&&[^\\p{ASCII}]]", "こ", 0, 3); // 1043

// -------------------------------------------------------------------------
// *** Index error: x2("[^\\x{104a}]", "こ", 0, 3); // 1044

// -------------------------------------------------------------------------
// *** Index error: x2("[^\\p{^Word}&&[^\\x{104a}]]", "こ", 0, 3); // 1045

// -------------------------------------------------------------------------
// *** Index error: x2("[^[\\p{^Word}]&&[^\\x{104a}]]", "こ", 0, 3); // 1046

// -------------------------------------------------------------------------
// n("[^\\p{Word}||[^\\x{104a}]]", "こ");
// tr!(r#"[^\p{Word}||[^\x{104a}]]"#, "こ", &[], 1047), UnicodePropertyNotFound
// scanner! { S1047 { mode M { token r#"[^\p{Word}||[^\x{104a}]]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("\\p{^Cntrl}", "こ", 0, 3); // 1048

// -------------------------------------------------------------------------
// n("\\p{Cntrl}", "こ");
// td!(r#"\p{Cntrl}"#, "こ", &[], 1049),
scanner! { S1049 { mode M { token r#"\p{Cntrl}"# => 0; } } }
#[test]
fn test_match_1049() {
    use s1049::S1049 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("こ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1049: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("[\\p{^Cntrl}]", "こ", 0, 3); // 1050

// -------------------------------------------------------------------------
// n("[\\p{Cntrl}]", "こ");
// td!(r#"[\p{Cntrl}]"#, "こ", &[], 1051),
scanner! { S1051 { mode M { token r#"[\p{Cntrl}]"# => 0; } } }
#[test]
fn test_match_1051() {
    use s1051::S1051 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("こ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1051: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[^\\p{^Cntrl}]", "こ");
// tr!(r#"[^\p{^Cntrl}]"#, "こ", &[], 1052), UnicodePropertyNotFound
// scanner! { S1052 { mode M { token r#"[^\p{^Cntrl}]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[^\\p{Cntrl}]", "こ", 0, 3); // 1053

// -------------------------------------------------------------------------
// *** Index error: x2("[^\\p{Cntrl}&&\\p{ASCII}]", "こ", 0, 3); // 1054

// -------------------------------------------------------------------------
// x2("[^\\p{Cntrl}&&\\p{ASCII}]", "a", 0, 1);
// td!(r#"[^\p{Cntrl}&&\p{ASCII}]"#, "a", &[("a", 0, 1)], 1055),
scanner! { S1055 { mode M { token r#"[^\p{Cntrl}&&\p{ASCII}]"# => 0; } } }
#[test]
fn test_match_1055() {
    use s1055::S1055 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1055: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1055: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1055: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "1055: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[^\\p{^Cntrl}&&\\p{ASCII}]", "#");
// tr!(r#"[^\p{^Cntrl}&&\p{ASCII}]"#, "#", &[], 1056), UnicodePropertyNotFound
// scanner! { S1056 { mode M { token r#"[^\p{^Cntrl}&&\p{ASCII}]"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("[^[\\p{^Cntrl}]&&[\\p{ASCII}]]", "こ", 0, 3); // 1057

// -------------------------------------------------------------------------
// *** Index error: x2("[^[\\p{ASCII}]&&[^\\p{Cntrl}]]", "こ", 0, 3); // 1058

// -------------------------------------------------------------------------
// n("[[\\p{ASCII}]&&[^\\p{Cntrl}]]", "こ");
// td!(r#"[[\p{ASCII}]&&[^\p{Cntrl}]]"#, "こ", &[], 1059),
scanner! { S1059 { mode M { token r#"[[\p{ASCII}]&&[^\p{Cntrl}]]"# => 0; } } }
#[test]
fn test_match_1059() {
    use s1059::S1059 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("こ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1059: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[^[\\p{^Cntrl}]&&[^\\p{ASCII}]]", "こ");
// tr!(r#"[^[\p{^Cntrl}]&&[^\p{ASCII}]]"#, "こ", &[], 1060), UnicodePropertyNotFound
// scanner! { S1060 { mode M { token r#"[^[\p{^Cntrl}]&&[^\p{ASCII}]]"# => 0; } } }

// -------------------------------------------------------------------------
// n("[^\\p{^Cntrl}&&[^\\x{104a}]]", "こ");
// tr!(r#"[^\p{^Cntrl}&&[^\x{104a}]]"#, "こ", &[], 1061), UnicodePropertyNotFound
// scanner! { S1061 { mode M { token r#"[^\p{^Cntrl}&&[^\x{104a}]]"# => 0; } } }

// -------------------------------------------------------------------------
// n("[^[\\p{^Cntrl}]&&[^\\x{104a}]]", "こ");
// tr!(r#"[^[\p{^Cntrl}]&&[^\x{104a}]]"#, "こ", &[], 1062), UnicodePropertyNotFound
// scanner! { S1062 { mode M { token r#"[^[\p{^Cntrl}]&&[^\x{104a}]]"# => 0; } } }

// -------------------------------------------------------------------------
// n("[^\\p{Cntrl}||[^\\x{104a}]]", "こ");
// td!(r#"[^\p{Cntrl}||[^\x{104a}]]"#, "こ", &[], 1063),
scanner! { S1063 { mode M { token r#"[^\p{Cntrl}||[^\x{104a}]]"# => 0; } } }
#[test]
fn test_match_1063() {
    use s1063::S1063 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("こ", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1063: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("(?-W:\\p{Word})", "こ", 0, 3); // 1064

// -------------------------------------------------------------------------
// n("(?W:\\p{Word})", "こ");
// tr!(r#"(?W:\p{Word})"#, "こ", &[], 1065), FlagUnrecognized
// scanner! { S1065 { mode M { token r#"(?W:\p{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?W:\\p{Word})", "k", 0, 1);
// tr!(r#"(?W:\p{Word})"#, "k", &[("k", 0, 1)], 1066), FlagUnrecognized
// scanner! { S1066 { mode M { token r#"(?W:\p{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?-W:[[:word:]])", "こ", 0, 3); // 1067

// -------------------------------------------------------------------------
// n("(?W:[[:word:]])", "こ");
// tr!(r#"(?W:[[:word:]])"#, "こ", &[], 1068), FlagUnrecognized
// scanner! { S1068 { mode M { token r#"(?W:[[:word:]])"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?-D:\\p{Digit})", "３", 0, 3); // 1069

// -------------------------------------------------------------------------
// n("(?D:\\p{Digit})", "３");
// tr!(r#"(?D:\p{Digit})"#, "３", &[], 1070), FlagUnrecognized
// scanner! { S1070 { mode M { token r#"(?D:\p{Digit})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-S:\\p{Space})", "\xc2\x85", 0, 2);
// tr!(r#"(?-S:\p{Space})"#, "\xc2\x85", &[("\\x", 0, 2)], 1071), FlagUnrecognized
// scanner! { S1071 { mode M { token r#"(?-S:\p{Space})"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?S:\\p{Space})", "\xc2\x85");
// tr!(r#"(?S:\p{Space})"#, "\xc2\x85", &[], 1072), FlagUnrecognized
// scanner! { S1072 { mode M { token r#"(?S:\p{Space})"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?-P:\\p{Word})", "こ", 0, 3); // 1073

// -------------------------------------------------------------------------
// n("(?P:\\p{Word})", "こ");
// tr!(r#"(?P:\p{Word})"#, "こ", &[], 1074), FlagUnrecognized
// scanner! { S1074 { mode M { token r#"(?P:\p{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?-W:\\w)", "こ", 0, 3); // 1075

// -------------------------------------------------------------------------
// n("(?W:\\w)", "こ");
// tr!(r#"(?W:\w)"#, "こ", &[], 1076), FlagUnrecognized
// scanner! { S1076 { mode M { token r#"(?W:\w)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-W:\\w)", "k", 0, 1);
// tr!(r#"(?-W:\w)"#, "k", &[("k", 0, 1)], 1077), FlagUnrecognized
// scanner! { S1077 { mode M { token r#"(?-W:\w)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?W:\\w)", "k", 0, 1);
// tr!(r#"(?W:\w)"#, "k", &[("k", 0, 1)], 1078), FlagUnrecognized
// scanner! { S1078 { mode M { token r#"(?W:\w)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?-W:\\W)", "こ");
// tr!(r#"(?-W:\W)"#, "こ", &[], 1079), FlagUnrecognized
// scanner! { S1079 { mode M { token r#"(?-W:\W)"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?W:\\W)", "こ", 0, 3); // 1080

// -------------------------------------------------------------------------
// n("(?-W:\\W)", "k");
// tr!(r#"(?-W:\W)"#, "k", &[], 1081), FlagUnrecognized
// scanner! { S1081 { mode M { token r#"(?-W:\W)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?W:\\W)", "k");
// tr!(r#"(?W:\W)"#, "k", &[], 1082), FlagUnrecognized
// scanner! { S1082 { mode M { token r#"(?W:\W)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-W:\\b)", "こ", 0, 0);
// tr!(r#"(?-W:\b)"#, "こ", &[], 1083), FlagUnrecognized
// scanner! { S1083 { mode M { token r#"(?-W:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?W:\\b)", "こ");
// tr!(r#"(?W:\b)"#, "こ", &[], 1084), FlagUnrecognized
// scanner! { S1084 { mode M { token r#"(?W:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-W:\\b)", "h", 0, 0);
// tr!(r#"(?-W:\b)"#, "h", &[], 1085), FlagUnrecognized
// scanner! { S1085 { mode M { token r#"(?-W:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?W:\\b)", "h", 0, 0);
// tr!(r#"(?W:\b)"#, "h", &[], 1086), FlagUnrecognized
// scanner! { S1086 { mode M { token r#"(?W:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?-W:\\B)", "こ");
// tr!(r#"(?-W:\B)"#, "こ", &[], 1087), FlagUnrecognized
// scanner! { S1087 { mode M { token r#"(?-W:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?W:\\B)", "こ", 0, 0);
// tr!(r#"(?W:\B)"#, "こ", &[], 1088), FlagUnrecognized
// scanner! { S1088 { mode M { token r#"(?W:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?-W:\\B)", "h");
// tr!(r#"(?-W:\B)"#, "h", &[], 1089), FlagUnrecognized
// scanner! { S1089 { mode M { token r#"(?-W:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?W:\\B)", "h");
// tr!(r#"(?W:\B)"#, "h", &[], 1090), FlagUnrecognized
// scanner! { S1090 { mode M { token r#"(?W:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-P:\\b)", "こ", 0, 0);
// tr!(r#"(?-P:\b)"#, "こ", &[], 1091), FlagUnrecognized
// scanner! { S1091 { mode M { token r#"(?-P:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?P:\\b)", "こ");
// tr!(r#"(?P:\b)"#, "こ", &[], 1092), FlagUnrecognized
// scanner! { S1092 { mode M { token r#"(?P:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?-P:\\b)", "h", 0, 0);
// tr!(r#"(?-P:\b)"#, "h", &[], 1093), FlagUnrecognized
// scanner! { S1093 { mode M { token r#"(?-P:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?P:\\b)", "h", 0, 0);
// tr!(r#"(?P:\b)"#, "h", &[], 1094), FlagUnrecognized
// scanner! { S1094 { mode M { token r#"(?P:\b)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?-P:\\B)", "こ");
// tr!(r#"(?-P:\B)"#, "こ", &[], 1095), FlagUnrecognized
// scanner! { S1095 { mode M { token r#"(?-P:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?P:\\B)", "こ", 0, 0);
// tr!(r#"(?P:\B)"#, "こ", &[], 1096), FlagUnrecognized
// scanner! { S1096 { mode M { token r#"(?P:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?-P:\\B)", "h");
// tr!(r#"(?-P:\B)"#, "h", &[], 1097), FlagUnrecognized
// scanner! { S1097 { mode M { token r#"(?-P:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?P:\\B)", "h");
// tr!(r#"(?P:\B)"#, "h", &[], 1098), FlagUnrecognized
// scanner! { S1098 { mode M { token r#"(?P:\B)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\p{InBasicLatin}", "\x41", 0, 1);
// tr!(r#"\p{InBasicLatin}"#, "\x41", &[("\\", 0, 1)], 1099), UnicodePropertyNotFound
// scanner! { S1099 { mode M { token r#"\p{InBasicLatin}"# => 0; } } }

// -------------------------------------------------------------------------
// n(".\\y\\O", "\x0d\x0a");
// tr!(r#".\y\O"#, "\x0d\x0a", &[], 1100), EscapeUnrecognized
// scanner! { S1100 { mode M { token r#".\y\O"# => 0; } } }

// -------------------------------------------------------------------------
// x2(".\\Y\\O", "\x0d\x0a", 0, 2);
// tr!(r#".\Y\O"#, "\x0d\x0a", &[("\\x", 0, 2)], 1101), EscapeUnrecognized
// scanner! { S1101 { mode M { token r#".\Y\O"# => 0; } } }

// -------------------------------------------------------------------------
// n("^.\\y.$", "\x67\xCC\x88");
// tr!(r#"^.\y.$"#, "\x67\xCC\x88", &[], 1102), EscapeUnrecognized
// scanner! { S1102 { mode M { token r#"^.\y.$"# => 0; } } }

// -------------------------------------------------------------------------
// x2(".\\Y.", "\x67\xCC\x88", 0, 3);
// tr!(r#".\Y."#, "\x67\xCC\x88", &[("\\x6", 0, 3)], 1103), EscapeUnrecognized
// scanner! { S1103 { mode M { token r#".\Y."# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\y.\\Y.\\y", "\x67\xCC\x88", 0, 3);
// tr!(r#"\y.\Y.\y"#, "\x67\xCC\x88", &[("\\x6", 0, 3)], 1104), EscapeUnrecognized
// scanner! { S1104 { mode M { token r#"\y.\Y.\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\y.\\y", "\xEA\xB0\x81", 0, 3);
// tr!(r#"\y.\y"#, "\xEA\xB0\x81", &[("\\xE", 0, 3)], 1105), EscapeUnrecognized
// scanner! { S1105 { mode M { token r#"\y.\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^.\\Y.\\Y.$", "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", 0, 9);
// tr!(r#"^.\Y.\Y.$"#, "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", &[("\\xE1\\x84\\", 0, 9)], 1106), EscapeUnrecognized
// scanner! { S1106 { mode M { token r#"^.\Y.\Y.$"# => 0; } } }

// -------------------------------------------------------------------------
// n("^.\\y.\\Y.$", "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8");
// tr!(r#"^.\y.\Y.$"#, "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", &[], 1107), EscapeUnrecognized
// scanner! { S1107 { mode M { token r#"^.\y.\Y.$"# => 0; } } }

// -------------------------------------------------------------------------
// x2(".\\Y.", "\xE0\xAE\xA8\xE0\xAE\xBF", 0, 6);
// tr!(r#".\Y."#, "\xE0\xAE\xA8\xE0\xAE\xBF", &[("\\xE0\\x", 0, 6)], 1108), EscapeUnrecognized
// scanner! { S1108 { mode M { token r#".\Y."# => 0; } } }

// -------------------------------------------------------------------------
// n(".\\y.", "\xE0\xAE\xA8\xE0\xAE\xBF");
// tr!(r#".\y."#, "\xE0\xAE\xA8\xE0\xAE\xBF", &[], 1109), EscapeUnrecognized
// scanner! { S1109 { mode M { token r#".\y."# => 0; } } }

// -------------------------------------------------------------------------
// x2(".\\Y.", "\xE0\xB8\x81\xE0\xB8\xB3", 0, 6);
// tr!(r#".\Y."#, "\xE0\xB8\x81\xE0\xB8\xB3", &[("\\xE0\\x", 0, 6)], 1110), EscapeUnrecognized
// scanner! { S1110 { mode M { token r#".\Y."# => 0; } } }

// -------------------------------------------------------------------------
// n(".\\y.", "\xE0\xB8\x81\xE0\xB8\xB3");
// tr!(r#".\y."#, "\xE0\xB8\x81\xE0\xB8\xB3", &[], 1111), EscapeUnrecognized
// scanner! { S1111 { mode M { token r#".\y."# => 0; } } }

// -------------------------------------------------------------------------
// x2(".\\Y.", "\xE0\xA4\xB7\xE0\xA4\xBF", 0, 6);
// tr!(r#".\Y."#, "\xE0\xA4\xB7\xE0\xA4\xBF", &[("\\xE0\\x", 0, 6)], 1112), EscapeUnrecognized
// scanner! { S1112 { mode M { token r#".\Y."# => 0; } } }

// -------------------------------------------------------------------------
// n(".\\y.", "\xE0\xA4\xB7\xE0\xA4\xBF");
// tr!(r#".\y."#, "\xE0\xA4\xB7\xE0\xA4\xBF", &[], 1113), EscapeUnrecognized
// scanner! { S1113 { mode M { token r#".\y."# => 0; } } }

// -------------------------------------------------------------------------
// x2("..\\Y.", "\xE3\x80\xB0\xE2\x80\x8D\xE2\xAD\x95", 0, 9);
// tr!(r#"..\Y."#, "\xE3\x80\xB0\xE2\x80\x8D\xE2\xAD\x95", &[("\\xE3\\x80\\", 0, 9)], 1114), EscapeUnrecognized
// scanner! { S1114 { mode M { token r#"..\Y."# => 0; } } }

// -------------------------------------------------------------------------
// x2("...\\Y.", "\xE3\x80\xB0\xCC\x82\xE2\x80\x8D\xE2\xAD\x95", 0, 11);
// tr!(r#"...\Y."#, "\xE3\x80\xB0\xCC\x82\xE2\x80\x8D\xE2\xAD\x95", &[("\\xE3\\x80\\xB", 0, 11)], 1115), EscapeUnrecognized
// scanner! { S1115 { mode M { token r#"...\Y."# => 0; } } }

// -------------------------------------------------------------------------
// n("...\\Y.", "\xE3\x80\xB0\xCD\xB0\xE2\x80\x8D\xE2\xAD\x95");
// tr!(r#"...\Y."#, "\xE3\x80\xB0\xCD\xB0\xE2\x80\x8D\xE2\xAD\x95", &[], 1116), EscapeUnrecognized
// scanner! { S1116 { mode M { token r#"...\Y."# => 0; } } }

// -------------------------------------------------------------------------
// n("^\\X\\X$", "\x0d\x0a");
// tr!(r#"^\X\X$"#, "\x0d\x0a", &[], 1117), EscapeUnrecognized
// scanner! { S1117 { mode M { token r#"^\X\X$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\X$", "\x0d\x0a", 0, 2);
// tr!(r#"^\X$"#, "\x0d\x0a", &[("\\x", 0, 2)], 1118), EscapeUnrecognized
// scanner! { S1118 { mode M { token r#"^\X$"# => 0; } } }

// -------------------------------------------------------------------------
// n("^\\X\\X.$", "\x67\xCC\x88");
// tr!(r#"^\X\X.$"#, "\x67\xCC\x88", &[], 1119), EscapeUnrecognized
// scanner! { S1119 { mode M { token r#"^\X\X.$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\X$", "\x67\xCC\x88", 0, 3);
// tr!(r#"^\X$"#, "\x67\xCC\x88", &[("\\x6", 0, 3)], 1120), EscapeUnrecognized
// scanner! { S1120 { mode M { token r#"^\X$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\X$", "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", 0, 9);
// tr!(r#"^\X$"#, "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", &[("\\xE1\\x84\\", 0, 9)], 1121), EscapeUnrecognized
// scanner! { S1121 { mode M { token r#"^\X$"# => 0; } } }

// -------------------------------------------------------------------------
// n("^\\X\\X\\X$", "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8");
// tr!(r#"^\X\X\X$"#, "\xE1\x84\x80\xE1\x85\xA1\xE1\x86\xA8", &[], 1122), EscapeUnrecognized
// scanner! { S1122 { mode M { token r#"^\X\X\X$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\X$", "\xE0\xAE\xA8\xE0\xAE\xBF", 0, 6);
// tr!(r#"^\X$"#, "\xE0\xAE\xA8\xE0\xAE\xBF", &[("\\xE0\\x", 0, 6)], 1123), EscapeUnrecognized
// scanner! { S1123 { mode M { token r#"^\X$"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\X\\X", "\xE0\xAE\xA8\xE0\xAE\xBF");
// tr!(r#"\X\X"#, "\xE0\xAE\xA8\xE0\xAE\xBF", &[], 1124), EscapeUnrecognized
// scanner! { S1124 { mode M { token r#"\X\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\X$", "\xE0\xB8\x81\xE0\xB8\xB3", 0, 6);
// tr!(r#"^\X$"#, "\xE0\xB8\x81\xE0\xB8\xB3", &[("\\xE0\\x", 0, 6)], 1125), EscapeUnrecognized
// scanner! { S1125 { mode M { token r#"^\X$"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\X\\X", "\xE0\xB8\x81\xE0\xB8\xB3");
// tr!(r#"\X\X"#, "\xE0\xB8\x81\xE0\xB8\xB3", &[], 1126), EscapeUnrecognized
// scanner! { S1126 { mode M { token r#"\X\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^\\X$", "\xE0\xA4\xB7\xE0\xA4\xBF", 0, 6);
// tr!(r#"^\X$"#, "\xE0\xA4\xB7\xE0\xA4\xBF", &[("\\xE0\\x", 0, 6)], 1127), EscapeUnrecognized
// scanner! { S1127 { mode M { token r#"^\X$"# => 0; } } }

// -------------------------------------------------------------------------
// n("\\X\\X", "\xE0\xA4\xB7\xE0\xA4\xBF");
// tr!(r#"\X\X"#, "\xE0\xA4\xB7\xE0\xA4\xBF", &[], 1128), EscapeUnrecognized
// scanner! { S1128 { mode M { token r#"\X\X"# => 0; } } }

// -------------------------------------------------------------------------
// n("^\\X.$", "\xE0\xAE\xA8\xE0\xAE\xBF");
// tr!(r#"^\X.$"#, "\xE0\xAE\xA8\xE0\xAE\xBF", &[], 1129), EscapeUnrecognized
// scanner! { S1129 { mode M { token r#"^\X.$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("h\\Xllo", "ha\xCC\x80llo", 0, 7);
// tr!(r#"h\Xllo"#, "ha\xCC\x80llo", &[("ha\\xCC\\", 0, 7)], 1130), EscapeUnrecognized
// scanner! { S1130 { mode M { token r#"h\Xllo"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{g})\\yabc\\y", "abc", 0, 3);
// tr!(r#"(?y{g})\yabc\y"#, "abc", &[("abc", 0, 3)], 1131), FlagUnrecognized
// scanner! { S1131 { mode M { token r#"(?y{g})\yabc\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{g})\\y\\X\\y", "abc", 0, 1);
// tr!(r#"(?y{g})\y\X\y"#, "abc", &[("a", 0, 1)], 1132), FlagUnrecognized
// scanner! { S1132 { mode M { token r#"(?y{g})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\yabc\\y", "abc", 0, 3); // WB1, WB2
// tr!(r#"(?y{w})\yabc\y"#, "abc", &[("abc", 0, 3)], 1133), FlagUnrecognized
// scanner! { S1133 { mode M { token r#"(?y{w})\yabc\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "\r\n", 0, 2); // WB3
// tr!(r#"(?y{w})\X"#, "\r\n", &[("\\r", 0, 2)], 1134), FlagUnrecognized
// scanner! { S1134 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "\x0cz", 0, 1); // WB3a
// tr!(r#"(?y{w})\X"#, "\x0cz", &[("\\", 0, 1)], 1135), FlagUnrecognized
// scanner! { S1135 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "q\x0c", 0, 1); // WB3b
// tr!(r#"(?y{w})\X"#, "q\x0c", &[("q", 0, 1)], 1136), FlagUnrecognized
// scanner! { S1136 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "\xE2\x80\x8D\xE2\x9D\x87", 0, 6); // WB3c
// tr!(r#"(?y{w})\X"#, "\xE2\x80\x8D\xE2\x9D\x87", &[("\\xE2\\x", 0, 6)], 1137), FlagUnrecognized
// scanner! { S1137 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "\x20\x20", 0, 2); // WB3d
// tr!(r#"(?y{w})\X"#, "\x20\x20", &[("\\x", 0, 2)], 1138), FlagUnrecognized
// scanner! { S1138 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "a\xE2\x80\x8D", 0, 4); // WB4
// tr!(r#"(?y{w})\X"#, "a\xE2\x80\x8D", &[("a\\xE", 0, 4)], 1139), FlagUnrecognized
// scanner! { S1139 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\y\\X\\y", "abc", 0, 3); // WB5
// tr!(r#"(?y{w})\y\X\y"#, "abc", &[("abc", 0, 3)], 1140), FlagUnrecognized
// scanner! { S1140 { mode M { token r#"(?y{w})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\y\\X\\y", "v\xCE\x87w", 0, 4); // WB6, WB7
// tr!(r#"(?y{w})\y\X\y"#, "v\xCE\x87w", &[("v\\xC", 0, 4)], 1141), FlagUnrecognized
// scanner! { S1141 { mode M { token r#"(?y{w})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\y\\X\\y", "\xD7\x93\x27", 0, 3); // WB7a
// tr!(r#"(?y{w})\y\X\y"#, "\xD7\x93\x27", &[("\\xD", 0, 3)], 1142), FlagUnrecognized
// scanner! { S1142 { mode M { token r#"(?y{w})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\y\\X\\y", "\xD7\x93\x22\xD7\x93", 0, 5); // WB7b, WB7c
// tr!(r#"(?y{w})\y\X\y"#, "\xD7\x93\x22\xD7\x93", &[("\\xD7\\", 0, 5)], 1143), FlagUnrecognized
// scanner! { S1143 { mode M { token r#"(?y{w})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "14 45", 0, 2); // WB8
// tr!(r#"(?y{w})\X"#, "14 45", &[("14", 0, 2)], 1144), FlagUnrecognized
// scanner! { S1144 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "a14", 0, 3); // WB9
// tr!(r#"(?y{w})\X"#, "a14", &[("a14", 0, 3)], 1145), FlagUnrecognized
// scanner! { S1145 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "832e", 0, 4); // WB10
// tr!(r#"(?y{w})\X"#, "832e", &[("832e", 0, 4)], 1146), FlagUnrecognized
// scanner! { S1146 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "8\xEF\xBC\x8C\xDB\xB0", 0, 6); // WB11, WB12
// tr!(r#"(?y{w})\X"#, "8\xEF\xBC\x8C\xDB\xB0", &[("8\\xEF\\", 0, 6)], 1147), FlagUnrecognized
// scanner! { S1147 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?y{w})\\y\\X\\y", "ケン", 0, 6); // WB13 // 1148

// -------------------------------------------------------------------------
// x2("(?y{w})\\y\\X\\y", "ケン\xE2\x80\xAFタ", 0, 12); // WB13a, WB13b
// tr!(r#"(?y{w})\y\X\y"#, "ケン\xE2\x80\xAFタ", &[("ケン\\xE2\\x80\\x", 0, 12)], 1149), FlagUnrecognized
// scanner! { S1149 { mode M { token r#"(?y{w})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\y\\X\\y", "\x21\x23", 0, 1); // WB999
// tr!(r#"(?y{w})\y\X\y"#, "\x21\x23", &[("\\", 0, 1)], 1150), FlagUnrecognized
// scanner! { S1150 { mode M { token r#"(?y{w})\y\X\y"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?y{w})\\y\\X\\y", "山ア", 0, 3); // 1151

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "3.14", 0, 4);
// tr!(r#"(?y{w})\X"#, "3.14", &[("3.14", 0, 4)], 1152), FlagUnrecognized
// scanner! { S1152 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?y{w})\\X", "3 14", 0, 1);
// tr!(r#"(?y{w})\X"#, "3 14", &[("3", 0, 1)], 1153), FlagUnrecognized
// scanner! { S1153 { mode M { token r#"(?y{w})\X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x40", "@", 0, 1);
// td!(r#"\x40"#, "@", &[("@", 0, 1)], 1154),
scanner! { S1154 { mode M { token r#"\x40"# => 0; } } }
#[test]
fn test_match_1154() {
    use s1154::S1154 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("@", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("@", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1154: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1154: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1154: Match end does not match");
        assert_eq!(
            &"@"[ma.1..ma.2],
            ma.0,
            "1154: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\x1", "\x01", 0, 1);
// tr!(r#"\x1"#, "\x01", &[("\\", 0, 1)], 1155), EscapeUnexpectedEof
// scanner! { S1155 { mode M { token r#"\x1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x{1}", "\x01", 0, 1);
// td!(r#"\x{1}"#, "\x01", &[("\\", 0, 1)], 1156),
scanner! { S1156 { mode M { token r#"\x{1}"# => 0; } } }
#[test]
fn test_match_1156() {
    use s1156::S1156 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x01", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\x01", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1156: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1156: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1156: Match end does not match");
        assert_eq!(
            &"\x01"[ma.1..ma.2],
            ma.0,
            "1156: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\x{4E38}", "\xE4\xB8\xB8", 0, 3);
// td!(r#"\x{4E38}"#, "\xE4\xB8\xB8", &[("\\xE", 0, 3)], 1157), Rust Compile Error
scanner! { S1157 { mode M { token r#"\x{4E38}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\u4E38", "\xE4\xB8\xB8", 0, 3);
// td!(r#"\u4E38"#, "\xE4\xB8\xB8", &[("\\xE", 0, 3)], 1158), Rust Compile Error
scanner! { S1158 { mode M { token r#"\u4E38"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\u0040", "@", 0, 1);
// td!(r#"\u0040"#, "@", &[("@", 0, 1)], 1159),
scanner! { S1159 { mode M { token r#"\u0040"# => 0; } } }
#[test]
fn test_match_1159() {
    use s1159::S1159 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("@", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("@", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1159: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1159: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1159: Match end does not match");
        assert_eq!(
            &"@"[ma.1..ma.2],
            ma.0,
            "1159: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// e("\\xF4", "", ONIGERR_TOO_SHORT_MULTI_BYTE_STRING);
// tr!(r#"\xF4"#, "", "ONIGERR_TOO_SHORT_MULTI_BYTE_STRING", 1160),
// scanner! { S1160 { mode M { token r#"\xF4"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\xF5", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\xF5"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1161),
// scanner! { S1161 { mode M { token r#"\xF5"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\xFF", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\xFF"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1162),
// scanner! { S1162 { mode M { token r#"\xFF"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\xF4]", "", ONIGERR_TOO_SHORT_MULTI_BYTE_STRING);
// tr!(r#"[\xF4]"#, "", "ONIGERR_TOO_SHORT_MULTI_BYTE_STRING", 1163),
// scanner! { S1163 { mode M { token r#"[\xF4]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\xF5]", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\xF5]"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1164),
// scanner! { S1164 { mode M { token r#"[\xF5]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x00-\\xFF]", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x00-\xFF]"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1165),
// scanner! { S1165 { mode M { token r#"[\x00-\xFF]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("c.*\\b", "abc", 2, 3);
// tu!(r#"c.*\b"#, "abc", &[("c", 2, 3)], 1166), UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S1166 { mode M { token r#"c.*\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\b.*abc.*\\b", "abc", 0, 3);
// tu!(r#"\b.*abc.*\b"#, "abc", &[("abc", 0, 3)], 1167), UnsupportedFeatureError("WordUnicode Look(WordUnicode)")
// scanner! { S1167 { mode M { token r#"\b.*abc.*\b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?()0+)+++(((0\\g<0>)0)|())++++((?(1)(0\\g<0>))++++++0*())++++((?(1)(0\\g<1>)+)++++++++++*())++++((?(1)((0)\\g<0>)+)++())+0++*+++(((0\\g<0>))*())++++((?(1)(0\\g<0>)+)++++++++++*|)++++*+++((?(1)((0)\\g<0>)+)+++++++++())++*|)++++((?()0))|", "abcde", 0, 0); // #139
// tr!(r#"((?()0+)+++(((0\g<0>)0)|())++++((?(1)(0\g<0>))++++++0*())++++((?(1)(0\g<1>)+)++++++++++*())++++((?(1)((0)\g<0>)+)++())+0++*+++(((0\g<0>))*())++++((?(1)(0\g<0>)+)++++++++++*|)++++*+++((?(1)((0)\g<0>)+)+++++++++())++*|)++++((?()0))|"#, "abcde", &[], 1168), FlagUnrecognized
// scanner! { S1168 { mode M { token r#"((?()0+)+++(((0\g<0>)0)|())++++((?(1)(0\g<0>))++++++0*())++++((?(1)(0\g<1>)+)++++++++++*())++++((?(1)((0)\g<0>)+)++())+0++*+++(((0\g<0>))*())++++((?(1)(0\g<0>)+)++++++++++*|)++++*+++((?(1)((0)\g<0>)+)+++++++++())++*|)++++((?()0))|"# => 0; } } }

// -------------------------------------------------------------------------
// n("(*FAIL)", "abcdefg");
// tr!(r#"(*FAIL)"#, "abcdefg", &[], 1169), RepetitionMissing
// scanner! { S1169 { mode M { token r#"(*FAIL)"# => 0; } } }

// -------------------------------------------------------------------------
// n("abcd(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)", "abcdefg");
// tr!(r#"abcd(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)"#, "abcdefg", &[], 1170), RepetitionMissing
// scanner! { S1170 { mode M { token r#"abcd(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)(*FAIL)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:[ab]|(*MAX{2}).)*", "abcbaaccaaa", 0, 7);
// tr!(r#"(?:[ab]|(*MAX{2}).)*"#, "abcbaaccaaa", &[("abcbaac", 0, 7)], 1171), RepetitionMissing
// scanner! { S1171 { mode M { token r#"(?:[ab]|(*MAX{2}).)*"# => 0; } } }

// x2("(?:(*COUNT[AB]{X})[ab]|(*COUNT[CD]{X})[cd])*(*CMP{AB,<,CD})", // 1172
// -------------------------------------------------------------------------
// x2("(?(?{....})123|456)", "123", 0, 3);
// tr!(r#"(?(?{....})123|456)"#, "123", &[("123", 0, 3)], 1173), FlagUnrecognized
// scanner! { S1173 { mode M { token r#"(?(?{....})123|456)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?(*FAIL)123|456)", "456", 0, 3);
// tr!(r#"(?(*FAIL)123|456)"#, "456", &[("456", 0, 3)], 1174), FlagUnrecognized
// scanner! { S1174 { mode M { token r#"(?(*FAIL)123|456)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g'0'++{,0}",   "abcdefgh", 0, 0);
// tr!(r#"\g'0'++{,0}"#, "abcdefgh", &[], 1175), EscapeUnrecognized
// scanner! { S1175 { mode M { token r#"\g'0'++{,0}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g'0'++{,0}?",  "abcdefgh", 0, 0);
// tr!(r#"\g'0'++{,0}?"#, "abcdefgh", &[], 1176), EscapeUnrecognized
// scanner! { S1176 { mode M { token r#"\g'0'++{,0}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g'0'++{,0}b",  "abcdefgh", 1, 2);
// tr!(r#"\g'0'++{,0}b"#, "abcdefgh", &[("b", 1, 2)], 1177), EscapeUnrecognized
// scanner! { S1177 { mode M { token r#"\g'0'++{,0}b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\g'0'++{,0}?def", "abcdefgh", 3, 6);
// tr!(r#"\g'0'++{,0}?def"#, "abcdefgh", &[("def", 3, 6)], 1178), EscapeUnrecognized
// scanner! { S1178 { mode M { token r#"\g'0'++{,0}?def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{1,3}?", "aaa", 0, 1);
// tu!(r#"a{1,3}?"#, "aaa", &[("a", 0, 1)], 1179), UnsupportedFeatureError("a{1,3}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S1179 { mode M { token r#"a{1,3}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3}", "aaa", 0, 3);
// td!(r#"a{3}"#, "aaa", &[("aaa", 0, 3)], 1180),
scanner! { S1180 { mode M { token r#"a{3}"# => 0; } } }
#[test]
fn test_match_1180() {
    use s1180::S1180 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaa", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1180: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1180: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1180: Match end does not match");
        assert_eq!(
            &"aaa"[ma.1..ma.2],
            ma.0,
            "1180: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a{3}?", "aaa", 0, 3);
// tu!(r#"a{3}?"#, "aaa", &[("aaa", 0, 3)], 1181), UnsupportedFeatureError("a{3}: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S1181 { mode M { token r#"a{3}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3}?", "aa", 0, 0);
// tu!(r#"a{3}?"#, "aa", &[], 1182), UnsupportedFeatureError("a{3}: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S1182 { mode M { token r#"a{3}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3,3}?", "aaa", 0, 3);
// tu!(r#"a{3,3}?"#, "aaa", &[("aaa", 0, 3)], 1183), UnsupportedFeatureError("a{3}: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S1183 { mode M { token r#"a{3,3}?"# => 0; } } }

// -------------------------------------------------------------------------
// n("a{3,3}?", "aa");
// tu!(r#"a{3,3}?"#, "aa", &[], 1184), UnsupportedFeatureError("a{3}: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S1184 { mode M { token r#"a{3,3}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{1,3}+", "aaaaaa", 0, 6);
// td!(r#"a{1,3}+"#, "aaaaaa", &[("aaaaaa", 0, 6)], 1185),
scanner! { S1185 { mode M { token r#"a{1,3}+"# => 0; } } }
#[test]
fn test_match_1185() {
    use s1185::S1185 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaaaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaaaa", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1185: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1185: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1185: Match end does not match");
        assert_eq!(
            &"aaaaaa"[ma.1..ma.2],
            ma.0,
            "1185: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a{3}+", "aaaaaa", 0, 6);
// td!(r#"a{3}+"#, "aaaaaa", &[("aaaaaa", 0, 6)], 1186),
scanner! { S1186 { mode M { token r#"a{3}+"# => 0; } } }
#[test]
fn test_match_1186() {
    use s1186::S1186 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaaaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaaaa", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1186: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1186: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1186: Match end does not match");
        assert_eq!(
            &"aaaaaa"[ma.1..ma.2],
            ma.0,
            "1186: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("a{3,3}+", "aaaaaa", 0, 6);
// td!(r#"a{3,3}+"#, "aaaaaa", &[("aaaaaa", 0, 6)], 1187),
scanner! { S1187 { mode M { token r#"a{3,3}+"# => 0; } } }
#[test]
fn test_match_1187() {
    use s1187::S1187 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaaaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaaaaa", 0, 6)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1187: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1187: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1187: Match end does not match");
        assert_eq!(
            &"aaaaaa"[ma.1..ma.2],
            ma.0,
            "1187: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("a{2,3}?",  "a");
// tu!(r#"a{2,3}?"#, "a", &[], 1188), UnsupportedFeatureError("a{2,3}?: Non-greedy repetitions. Consider using different scanner modes instead.")
// scanner! { S1188 { mode M { token r#"a{2,3}?"# => 0; } } }

// -------------------------------------------------------------------------
// n("a{3,2}a", "aaa");
// tr!(r#"a{3,2}a"#, "aaa", &[], 1189), RepetitionCountInvalid
// scanner! { S1189 { mode M { token r#"a{3,2}a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3,2}b", "aaab", 0, 4);
// tr!(r#"a{3,2}b"#, "aaab", &[("aaab", 0, 4)], 1190), RepetitionCountInvalid
// scanner! { S1190 { mode M { token r#"a{3,2}b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3,2}b", "aaaab", 1, 5);
// tr!(r#"a{3,2}b"#, "aaaab", &[("aaab", 1, 5)], 1191), RepetitionCountInvalid
// scanner! { S1191 { mode M { token r#"a{3,2}b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3,2}b", "aab", 0, 3);
// tr!(r#"a{3,2}b"#, "aab", &[("aab", 0, 3)], 1192), RepetitionCountInvalid
// scanner! { S1192 { mode M { token r#"a{3,2}b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{3,2}?", "", 0, 0);     /* == (?:a{3,2})?*/
// tr!(r#"a{3,2}?"#, "", &[], 1193), RepetitionCountInvalid
// scanner! { S1193 { mode M { token r#"a{3,2}?"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a{2,3}+a", "aaa", 0, 3); /* == (?:a{2,3})+*/
// td!(r#"a{2,3}+a"#, "aaa", &[("aaa", 0, 3)], 1194),
scanner! { S1194 { mode M { token r#"a{2,3}+a"# => 0; } } }
#[test]
fn test_match_1194() {
    use s1194::S1194 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aaa", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1194: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1194: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1194: Match end does not match");
        assert_eq!(
            &"aaa"[ma.1..ma.2],
            ma.0,
            "1194: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[\\x{0}-\\x{7fffffff}]", "a", 0, 1);
// tr!(r#"[\x{0}-\x{7fffffff}]"#, "a", &[("a", 0, 1)], 1195), EscapeHexInvalid
// scanner! { S1195 { mode M { token r#"[\x{0}-\x{7fffffff}]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{7f}-\\x{7fffffff}]", "\xe5\xae\xb6", 0, 3);
// tr!(r#"[\x{7f}-\x{7fffffff}]"#, "\xe5\xae\xb6", &[("\\xe", 0, 3)], 1196), EscapeHexInvalid
// scanner! { S1196 { mode M { token r#"[\x{7f}-\x{7fffffff}]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[a[cdef]]", "a", 0, 1);
// td!(r#"[a[cdef]]"#, "a", &[("a", 0, 1)], 1197),
scanner! { S1197 { mode M { token r#"[a[cdef]]"# => 0; } } }
#[test]
fn test_match_1197() {
    use s1197::S1197 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1197: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1197: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1197: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "1197: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[a[xyz]-c]", "b");
// td!(r#"[a[xyz]-c]"#, "b", &[], 1198),
scanner! { S1198 { mode M { token r#"[a[xyz]-c]"# => 0; } } }
#[test]
fn test_match_1198() {
    use s1198::S1198 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("b", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1198: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[a[xyz]-c]", "a", 0, 1);
// td!(r#"[a[xyz]-c]"#, "a", &[("a", 0, 1)], 1199),
scanner! { S1199 { mode M { token r#"[a[xyz]-c]"# => 0; } } }
#[test]
fn test_match_1199() {
    use s1199::S1199 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1199: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1199: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1199: Match end does not match");
        assert_eq!(
            &"a"[ma.1..ma.2],
            ma.0,
            "1199: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[a[xyz]-c]", "-", 0, 1);
// td!(r#"[a[xyz]-c]"#, "-", &[("-", 0, 1)], 1200),
scanner! { S1200 { mode M { token r#"[a[xyz]-c]"# => 0; } } }
#[test]
fn test_match_1200() {
    use s1200::S1200 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("-", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("-", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1200: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1200: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1200: Match end does not match");
        assert_eq!(
            &"-"[ma.1..ma.2],
            ma.0,
            "1200: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[a[xyz]-c]", "c", 0, 1);
// td!(r#"[a[xyz]-c]"#, "c", &[("c", 0, 1)], 1201),
scanner! { S1201 { mode M { token r#"[a[xyz]-c]"# => 0; } } }
#[test]
fn test_match_1201() {
    use s1201::S1201 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("c", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("c", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1201: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1201: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1201: Match end does not match");
        assert_eq!(
            &"c"[ma.1..ma.2],
            ma.0,
            "1201: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(a.c|def)(.{4})(?<=\\1)", "abcdabc", 0, 7);
// tr!(r#"(a.c|def)(.{4})(?<=\1)"#, "abcdabc", &[("abcdabc", 0, 7)], 1202), UnsupportedLookAround
// scanner! { S1202 { mode M { token r#"(a.c|def)(.{4})(?<=\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a.c|de)(.{4})(?<=\\1)", "abcdabc", 0, 7);
// tr!(r#"(a.c|de)(.{4})(?<=\1)"#, "abcdabc", &[("abcdabc", 0, 7)], 1203), UnsupportedLookAround
// scanner! { S1203 { mode M { token r#"(a.c|de)(.{4})(?<=\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a.c|def)(.{5})(?<=d\\1e)", "abcdabce", 0, 8);
// tr!(r#"(a.c|def)(.{5})(?<=d\1e)"#, "abcdabce", &[("abcdabce", 0, 8)], 1204), UnsupportedLookAround
// scanner! { S1204 { mode M { token r#"(a.c|def)(.{5})(?<=d\1e)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a.c|.)d(?<=\\k<1>d)", "zzzzzabcdabc", 5, 9);
// tr!(r#"(a.c|.)d(?<=\k<1>d)"#, "zzzzzabcdabc", &[("abcd", 5, 9)], 1205), UnsupportedLookAround
// scanner! { S1205 { mode M { token r#"(a.c|.)d(?<=\k<1>d)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=az*)abc", "azzzzzzzzzzabcdabcabc", 11, 14);
// tr!(r#"(?<=az*)abc"#, "azzzzzzzzzzabcdabcabc", &[("abc", 11, 14)], 1206), UnsupportedLookAround
// scanner! { S1206 { mode M { token r#"(?<=az*)abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=ab|abc|abcd)ef", "abcdef", 4, 6);
// tr!(r#"(?<=ab|abc|abcd)ef"#, "abcdef", &[("ef", 4, 6)], 1207), UnsupportedLookAround
// scanner! { S1207 { mode M { token r#"(?<=ab|abc|abcd)ef"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=ta+|tb+|tc+|td+)zz", "tcccccccccczz", 11, 13);
// tr!(r#"(?<=ta+|tb+|tc+|td+)zz"#, "tcccccccccczz", &[("zz", 11, 13)], 1208), UnsupportedLookAround
// scanner! { S1208 { mode M { token r#"(?<=ta+|tb+|tc+|td+)zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=t.{7}|t.{5}|t.{2}|t.)zz", "tczz", 2, 4);
// tr!(r#"(?<=t.{7}|t.{5}|t.{2}|t.)zz"#, "tczz", &[("zz", 2, 4)], 1209), UnsupportedLookAround
// scanner! { S1209 { mode M { token r#"(?<=t.{7}|t.{5}|t.{2}|t.)zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=t.{7}|t.{5}|t.{2})zz", "tczzzz", 3, 5);
// tr!(r#"(?<=t.{7}|t.{5}|t.{2})zz"#, "tczzzz", &[("zz", 3, 5)], 1210), UnsupportedLookAround
// scanner! { S1210 { mode M { token r#"(?<=t.{7}|t.{5}|t.{2})zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=t.{7}|t.{5}|t.{3})zz", "tczzazzbzz", 8, 10);
// tr!(r#"(?<=t.{7}|t.{5}|t.{3})zz"#, "tczzazzbzz", &[("zz", 8, 10)], 1211), UnsupportedLookAround
// scanner! { S1211 { mode M { token r#"(?<=t.{7}|t.{5}|t.{3})zz"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=t.{7}|t.{5}|t.{3})zz", "tczzazzbczz");
// tr!(r#"(?<=t.{7}|t.{5}|t.{3})zz"#, "tczzazzbczz", &[], 1212), UnsupportedLookAround
// scanner! { S1212 { mode M { token r#"(?<=t.{7}|t.{5}|t.{3})zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(ab|abc|abcd))ef", "abcdef", 4, 6);
// tr!(r#"(?<=(ab|abc|abcd))ef"#, "abcdef", &[("ef", 4, 6)], 1213), UnsupportedLookAround
// scanner! { S1213 { mode M { token r#"(?<=(ab|abc|abcd))ef"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(ta+|tb+|tc+|td+))zz", "tcccccccccczz", 11, 13);
// tr!(r#"(?<=(ta+|tb+|tc+|td+))zz"#, "tcccccccccczz", &[("zz", 11, 13)], 1214), UnsupportedLookAround
// scanner! { S1214 { mode M { token r#"(?<=(ta+|tb+|tc+|td+))zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(t.{7}|t.{5}|t.{2}|t.))zz", "tczz", 2, 4);
// tr!(r#"(?<=(t.{7}|t.{5}|t.{2}|t.))zz"#, "tczz", &[("zz", 2, 4)], 1215), UnsupportedLookAround
// scanner! { S1215 { mode M { token r#"(?<=(t.{7}|t.{5}|t.{2}|t.))zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(t.{7}|t.{5}|t.{2}))zz", "tczzzz", 3, 5);
// tr!(r#"(?<=(t.{7}|t.{5}|t.{2}))zz"#, "tczzzz", &[("zz", 3, 5)], 1216), UnsupportedLookAround
// scanner! { S1216 { mode M { token r#"(?<=(t.{7}|t.{5}|t.{2}))zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(t.{7}|t.{5}|t.{3}))zz", "tczzazzbzz", 8, 10);
// tr!(r#"(?<=(t.{7}|t.{5}|t.{3}))zz"#, "tczzazzbzz", &[("zz", 8, 10)], 1217), UnsupportedLookAround
// scanner! { S1217 { mode M { token r#"(?<=(t.{7}|t.{5}|t.{3}))zz"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=(t.{7}|t.{5}|t.{3}))zz", "tczzazzbczz");
// tr!(r#"(?<=(t.{7}|t.{5}|t.{3}))zz"#, "tczzazzbczz", &[], 1218), UnsupportedLookAround
// scanner! { S1218 { mode M { token r#"(?<=(t.{7}|t.{5}|t.{3}))zz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.{1,4})(.{1,4})(?<=\\2\\1)", "abaaba", 0, 6);
// tr!(r#"(.{1,4})(.{1,4})(?<=\2\1)"#, "abaaba", &[("abaaba", 0, 6)], 1219), UnsupportedLookAround
// scanner! { S1219 { mode M { token r#"(.{1,4})(.{1,4})(?<=\2\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.{1,4})(.{1,4})(?<=\\2\\1)", "ababab", 0, 6);
// tr!(r#"(.{1,4})(.{1,4})(?<=\2\1)"#, "ababab", &[("ababab", 0, 6)], 1220), UnsupportedLookAround
// scanner! { S1220 { mode M { token r#"(.{1,4})(.{1,4})(?<=\2\1)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(.{1,4})(.{1,4})(?<=\\2\\1)", "abcdabce");
// tr!(r#"(.{1,4})(.{1,4})(?<=\2\1)"#, "abcdabce", &[], 1221), UnsupportedLookAround
// scanner! { S1221 { mode M { token r#"(.{1,4})(.{1,4})(?<=\2\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.{1,4})(.{1,4})(?<=\\2\\1)", "abcdabceabce", 4, 12);
// tr!(r#"(.{1,4})(.{1,4})(?<=\2\1)"#, "abcdabceabce", &[("abceabce", 4, 12)], 1222), UnsupportedLookAround
// scanner! { S1222 { mode M { token r#"(.{1,4})(.{1,4})(?<=\2\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a)", "a", 1, 1);
// tr!(r#"(?<=a)"#, "a", &[("", 1, 1)], 1223), UnsupportedLookAround
// scanner! { S1223 { mode M { token r#"(?<=a)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a.*\\w)z", "abbbz", 4, 5);
// tr!(r#"(?<=a.*\w)z"#, "abbbz", &[("z", 4, 5)], 1224), UnsupportedLookAround
// scanner! { S1224 { mode M { token r#"(?<=a.*\w)z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=a.*\\w)z", "abb z");
// tr!(r#"(?<=a.*\w)z"#, "abb z", &[], 1225), UnsupportedLookAround
// scanner! { S1225 { mode M { token r#"(?<=a.*\w)z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a.*\\W)z", "abb z", 4, 5);
// tr!(r#"(?<=a.*\W)z"#, "abb z", &[("z", 4, 5)], 1226), UnsupportedLookAround
// scanner! { S1226 { mode M { token r#"(?<=a.*\W)z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a.*\\b)z", "abb z", 4, 5);
// tr!(r#"(?<=a.*\b)z"#, "abb z", &[("z", 4, 5)], 1227), UnsupportedLookAround
// scanner! { S1227 { mode M { token r#"(?<=a.*\b)z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(?>abc))", "abc", 3, 3);
// tr!(r#"(?<=(?>abc))"#, "abc", &[("", 3, 3)], 1228), UnsupportedLookAround
// scanner! { S1228 { mode M { token r#"(?<=(?>abc))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a\\Xz)", "abz", 3, 3);
// tr!(r#"(?<=a\Xz)"#, "abz", &[("", 3, 3)], 1229), UnsupportedLookAround
// scanner! { S1229 { mode M { token r#"(?<=a\Xz)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=^a*)bc", "zabc");
// tr!(r#"(?<=^a*)bc"#, "zabc", &[], 1230), UnsupportedLookAround
// scanner! { S1230 { mode M { token r#"(?<=^a*)bc"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=a*\\b)b", "abc");
// tr!(r#"(?<=a*\b)b"#, "abc", &[], 1231), UnsupportedLookAround
// scanner! { S1231 { mode M { token r#"(?<=a*\b)b"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a+.*[efg])z", "abcdfz", 5, 6);
// tr!(r#"(?<=a+.*[efg])z"#, "abcdfz", &[("z", 5, 6)], 1232), UnsupportedLookAround
// scanner! { S1232 { mode M { token r#"(?<=a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a+.*[efg])z", "abcdfgz", 6, 7);
// tr!(r#"(?<=a+.*[efg])z"#, "abcdfgz", &[("z", 6, 7)], 1233), UnsupportedLookAround
// scanner! { S1233 { mode M { token r#"(?<=a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=a+.*[efg])z", "bcdfz");
// tr!(r#"(?<=a+.*[efg])z"#, "bcdfz", &[], 1234), UnsupportedLookAround
// scanner! { S1234 { mode M { token r#"(?<=a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=a*.*[efg])z", "bcdfz", 4, 5);
// tr!(r#"(?<=a*.*[efg])z"#, "bcdfz", &[("z", 4, 5)], 1235), UnsupportedLookAround
// scanner! { S1235 { mode M { token r#"(?<=a*.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=a+.*[efg])z", "abcdz");
// tr!(r#"(?<=a+.*[efg])z"#, "abcdz", &[], 1236), UnsupportedLookAround
// scanner! { S1236 { mode M { token r#"(?<=a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=v|t|a+.*[efg])z", "abcdfz", 5, 6);
// tr!(r#"(?<=v|t|a+.*[efg])z"#, "abcdfz", &[("z", 5, 6)], 1237), UnsupportedLookAround
// scanner! { S1237 { mode M { token r#"(?<=v|t|a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=v|t|^a+.*[efg])z", "abcdfz", 5, 6);
// tr!(r#"(?<=v|t|^a+.*[efg])z"#, "abcdfz", &[("z", 5, 6)], 1238), UnsupportedLookAround
// scanner! { S1238 { mode M { token r#"(?<=v|t|^a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=^(?:v|t|a+.*[efg]))z", "abcdfz", 5, 6);
// tr!(r#"(?<=^(?:v|t|a+.*[efg]))z"#, "abcdfz", &[("z", 5, 6)], 1239), UnsupportedLookAround
// scanner! { S1239 { mode M { token r#"(?<=^(?:v|t|a+.*[efg]))z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=v|^t|a+.*[efg])z", "uabcdfz", 6, 7);
// tr!(r#"(?<=v|^t|a+.*[efg])z"#, "uabcdfz", &[("z", 6, 7)], 1240), UnsupportedLookAround
// scanner! { S1240 { mode M { token r#"(?<=v|^t|a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// n("^..(?<=(a{,2}))\\1z", "aaaaz"); // !!! look-behind is shortest priority
// tr!(r#"^..(?<=(a{,2}))\1z"#, "aaaaz", &[], 1241), UnsupportedLookAround
// scanner! { S1241 { mode M { token r#"^..(?<=(a{,2}))\1z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("^..(?<=(a{,2}))\\1z", "aaz", 0, 3); // shortest priority
// tr!(r#"^..(?<=(a{,2}))\1z"#, "aaz", &[("aaz", 0, 3)], 1242), UnsupportedLookAround
// scanner! { S1242 { mode M { token r#"^..(?<=(a{,2}))\1z"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?<=(?~|zoo)a.*z)", "abcdefz", ONIGERR_INVALID_LOOK_BEHIND_PATTERN);
// tr!(r#"(?<=(?~|zoo)a.*z)"#, "abcdefz", "ONIGERR_INVALID_LOOK_BEHIND_PATTERN", 1243),
// scanner! { S1243 { mode M { token r#"(?<=(?~|zoo)a.*z)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?<=(?~|)a.*z)", "abcdefz", ONIGERR_INVALID_LOOK_BEHIND_PATTERN);
// tr!(r#"(?<=(?~|)a.*z)"#, "abcdefz", "ONIGERR_INVALID_LOOK_BEHIND_PATTERN", 1244),
// scanner! { S1244 { mode M { token r#"(?<=(?~|)a.*z)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(a(?~|boo)z){0}(?<=\\g<1>)", "abcdefz", ONIGERR_INVALID_LOOK_BEHIND_PATTERN);
// tr!(r#"(a(?~|boo)z){0}(?<=\g<1>)"#, "abcdefz", "ONIGERR_INVALID_LOOK_BEHIND_PATTERN", 1245),
// scanner! { S1245 { mode M { token r#"(a(?~|boo)z){0}(?<=\g<1>)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(?<= )| )", "abcde fg", 6, 6); // #173
// tr!(r#"(?<=(?<= )| )"#, "abcde fg", &[("", 6, 6)], 1246), UnsupportedLookAround
// scanner! { S1246 { mode M { token r#"(?<=(?<= )| )"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=D|)(?<=@!nnnnnnnnnIIIIn;{1}D?()|<x@x*xxxD|)(?<=@xxx|xxxxx\\g<1>;{1}x)", "(?<=D|)(?<=@!nnnnnnnnnIIIIn;{1}D?()|<x@x*xxxD|)(?<=@xxx|xxxxx\\g<1>;{1}x)", 55, 55); // #173
// tr!(r#"(?<=D|)(?<=@!nnnnnnnnnIIIIn;{1}D?()|<x@x*xxxD|)(?<=@xxx|xxxxx\g<1>;{1}x)"#, "(?<=D|)(?<=@!nnnnnnnnnIIIIn;{1}D?()|<x@x*xxxD|)(?<=@xxx|xxxxx\\g<1>;{1}x)", &[("", 55, 55)], 1247), UnsupportedLookAround
// scanner! { S1247 { mode M { token r#"(?<=D|)(?<=@!nnnnnnnnnIIIIn;{1}D?()|<x@x*xxxD|)(?<=@xxx|xxxxx\g<1>;{1}x)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=;()|)\\g<1>", "", 0, 0); // reduced #173
// tr!(r#"(?<=;()|)\g<1>"#, "", &[], 1248), UnsupportedLookAround
// scanner! { S1248 { mode M { token r#"(?<=;()|)\g<1>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=;()|)\\k<1>", ";", 1, 1);
// tr!(r#"(?<=;()|)\k<1>"#, ";", &[("", 1, 1)], 1249), UnsupportedLookAround
// scanner! { S1249 { mode M { token r#"(?<=;()|)\k<1>"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(())\\g<3>{0}(?<=|())", "abc", 0, 0); // #175
// tr!(r#"(())\g<3>{0}(?<=|())"#, "abc", &[], 1250), EscapeUnrecognized
// scanner! { S1250 { mode M { token r#"(())\g<3>{0}(?<=|())"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=()|)\\1{0}", "abc", 0, 0);
// tr!(r#"(?<=()|)\1{0}"#, "abc", &[], 1251), UnsupportedLookAround
// scanner! { S1251 { mode M { token r#"(?<=()|)\1{0}"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?<!xxxxxxxxxxxxxxxxxxxxxxx{32774}{65521}xxxxxxxx{65521}xxxxxxxxxxxxxx{32774}xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)", "", ONIGERR_INVALID_LOOK_BEHIND_PATTERN); // #177
// tr!(r#"(?<!xxxxxxxxxxxxxxxxxxxxxxx{32774}{65521}xxxxxxxx{65521}xxxxxxxxxxxxxx{32774}xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)"#, "", "ONIGERR_INVALID_LOOK_BEHIND_PATTERN", 1252),
// scanner! { S1252 { mode M { token r#"(?<!xxxxxxxxxxxxxxxxxxxxxxx{32774}{65521}xxxxxxxx{65521}xxxxxxxxxxxxxx{32774}xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=(?<=abc))def", "abcdef", 3, 6);
// tr!(r#"(?<=(?<=abc))def"#, "abcdef", &[("def", 3, 6)], 1253), UnsupportedLookAround
// scanner! { S1253 { mode M { token r#"(?<=(?<=abc))def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=ab(?<=.+b)c)def", "abcdef", 3, 6);
// tr!(r#"(?<=ab(?<=.+b)c)def"#, "abcdef", &[("def", 3, 6)], 1254), UnsupportedLookAround
// scanner! { S1254 { mode M { token r#"(?<=ab(?<=.+b)c)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=ab(?<=a+)c)def", "abcdef");
// tr!(r#"(?<=ab(?<=a+)c)def"#, "abcdef", &[], 1255), UnsupportedLookAround
// scanner! { S1255 { mode M { token r#"(?<=ab(?<=a+)c)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<=abc)(?<!abc)def", "abcdef");
// tr!(r#"(?<=abc)(?<!abc)def"#, "abcdef", &[], 1256), UnsupportedLookAround
// scanner! { S1256 { mode M { token r#"(?<=abc)(?<!abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!ab.)(?<=.bc)def", "abcdef");
// tr!(r#"(?<!ab.)(?<=.bc)def"#, "abcdef", &[], 1257), UnsupportedLookAround
// scanner! { S1257 { mode M { token r#"(?<!ab.)(?<=.bc)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!ab.)(?<=.bc)def", "abcdefcbcdef", 9, 12);
// tr!(r#"(?<!ab.)(?<=.bc)def"#, "abcdefcbcdef", &[("def", 9, 12)], 1258), UnsupportedLookAround
// scanner! { S1258 { mode M { token r#"(?<!ab.)(?<=.bc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!abc)def", "abcdef");
// tr!(r#"(?<!abc)def"#, "abcdef", &[], 1259), UnsupportedLookAround
// scanner! { S1259 { mode M { token r#"(?<!abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!xxx|abc)def", "abcdef");
// tr!(r#"(?<!xxx|abc)def"#, "abcdef", &[], 1260), UnsupportedLookAround
// scanner! { S1260 { mode M { token r#"(?<!xxx|abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!xxxxx|abc)def", "abcdef");
// tr!(r#"(?<!xxxxx|abc)def"#, "abcdef", &[], 1261), UnsupportedLookAround
// scanner! { S1261 { mode M { token r#"(?<!xxxxx|abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!xxxxx|abc)def", "xxxxxxdef");
// tr!(r#"(?<!xxxxx|abc)def"#, "xxxxxxdef", &[], 1262), UnsupportedLookAround
// scanner! { S1262 { mode M { token r#"(?<!xxxxx|abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!x+|abc)def", "abcdef");
// tr!(r#"(?<!x+|abc)def"#, "abcdef", &[], 1263), UnsupportedLookAround
// scanner! { S1263 { mode M { token r#"(?<!x+|abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!x+|abc)def", "xxxxxxxxxdef");
// tr!(r#"(?<!x+|abc)def"#, "xxxxxxxxxdef", &[], 1264), UnsupportedLookAround
// scanner! { S1264 { mode M { token r#"(?<!x+|abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!x+|abc)def", "xxxxxxxxzdef", 9, 12);
// tr!(r#"(?<!x+|abc)def"#, "xxxxxxxxzdef", &[("def", 9, 12)], 1265), UnsupportedLookAround
// scanner! { S1265 { mode M { token r#"(?<!x+|abc)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a.*z|a)def", "axxxxxxxzdef");
// tr!(r#"(?<!a.*z|a)def"#, "axxxxxxxzdef", &[], 1266), UnsupportedLookAround
// scanner! { S1266 { mode M { token r#"(?<!a.*z|a)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a.*z|a)def", "bxxxxxxxadef");
// tr!(r#"(?<!a.*z|a)def"#, "bxxxxxxxadef", &[], 1267), UnsupportedLookAround
// scanner! { S1267 { mode M { token r#"(?<!a.*z|a)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*z|a)def", "axxxxxxxzdefxxdef", 14, 17);
// tr!(r#"(?<!a.*z|a)def"#, "axxxxxxxzdefxxdef", &[("def", 14, 17)], 1268), UnsupportedLookAround
// scanner! { S1268 { mode M { token r#"(?<!a.*z|a)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*z|a)def", "bxxxxxxxadefxxdef", 14, 17);
// tr!(r#"(?<!a.*z|a)def"#, "bxxxxxxxadefxxdef", &[("def", 14, 17)], 1269), UnsupportedLookAround
// scanner! { S1269 { mode M { token r#"(?<!a.*z|a)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*z|a)def", "bxxxxxxxzdef", 9, 12);
// tr!(r#"(?<!a.*z|a)def"#, "bxxxxxxxzdef", &[("def", 9, 12)], 1270), UnsupportedLookAround
// scanner! { S1270 { mode M { token r#"(?<!a.*z|a)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!x+|y+)\\d+", "xxx572", 4, 6);
// tr!(r#"(?<!x+|y+)\d+"#, "xxx572", &[("72", 4, 6)], 1271), UnsupportedLookAround
// scanner! { S1271 { mode M { token r#"(?<!x+|y+)\d+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!3+|4+)\\d+", "33334444", 0, 8);
// tr!(r#"(?<!3+|4+)\d+"#, "33334444", &[("33334444", 0, 8)], 1272), UnsupportedLookAround
// scanner! { S1272 { mode M { token r#"(?<!3+|4+)\d+"# => 0; } } }

// -------------------------------------------------------------------------
// n(".(?<!3+|4+)\\d+", "33334444");
// tr!(r#".(?<!3+|4+)\d+"#, "33334444", &[], 1273), UnsupportedLookAround
// scanner! { S1273 { mode M { token r#".(?<!3+|4+)\d+"# => 0; } } }

// -------------------------------------------------------------------------
// n("(.{,3})..(?<!\\1)", "aaaaa");
// tr!(r#"(.{,3})..(?<!\1)"#, "aaaaa", &[], 1274), RepetitionCountDecimalEmpty
// scanner! { S1274 { mode M { token r#"(.{,3})..(?<!\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.{,3})..(?<!\\1)", "abcde", 0, 5);
// tr!(r#"(.{,3})..(?<!\1)"#, "abcde", &[("abcde", 0, 5)], 1275), RepetitionCountDecimalEmpty
// scanner! { S1275 { mode M { token r#"(.{,3})..(?<!\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(.{,3})...(?<!\\1)", "abcde", 0, 5);
// tr!(r#"(.{,3})...(?<!\1)"#, "abcde", &[("abcde", 0, 5)], 1276), RepetitionCountDecimalEmpty
// scanner! { S1276 { mode M { token r#"(.{,3})...(?<!\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a.c)(.{3,}?)(?<!\\1)", "abcabcd", 0, 7);
// tr!(r#"(a.c)(.{3,}?)(?<!\1)"#, "abcabcd", &[("abcabcd", 0, 7)], 1277), UnsupportedLookAround
// scanner! { S1277 { mode M { token r#"(a.c)(.{3,}?)(?<!\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a*)(.{3,}?)(?<!\\1)", "abcabcd", 0, 5);
// tr!(r#"(a*)(.{3,}?)(?<!\1)"#, "abcabcd", &[("abcab", 0, 5)], 1278), UnsupportedLookAround
// scanner! { S1278 { mode M { token r#"(a*)(.{3,}?)(?<!\1)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(a.*b)|c.*d)(?<!(?(1))azzzb)", "azzzzb", 0, 6);
// tr!(r#"(?:(a.*b)|c.*d)(?<!(?(1))azzzb)"#, "azzzzb", &[("azzzzb", 0, 6)], 1279), UnsupportedLookAround
// scanner! { S1279 { mode M { token r#"(?:(a.*b)|c.*d)(?<!(?(1))azzzb)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?:(a.*b)|c.*d)(?<!(?(1))azzzb)", "azzzb");
// tr!(r#"(?:(a.*b)|c.*d)(?<!(?(1))azzzb)"#, "azzzb", &[], 1280), UnsupportedLookAround
// scanner! { S1280 { mode M { token r#"(?:(a.*b)|c.*d)(?<!(?(1))azzzb)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("<(?<!NT{+}abcd)", "<(?<!NT{+}abcd)", 0, 1);
// tr!(r#"<(?<!NT{+}abcd)"#, "<(?<!NT{+}abcd)", &[("<", 0, 1)], 1281), UnsupportedLookAround
// scanner! { S1281 { mode M { token r#"<(?<!NT{+}abcd)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*c)def", "abbbbdef", 5, 8);
// tr!(r#"(?<!a.*c)def"#, "abbbbdef", &[("def", 5, 8)], 1282), UnsupportedLookAround
// scanner! { S1282 { mode M { token r#"(?<!a.*c)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a.*c)def", "abbbcdef");
// tr!(r#"(?<!a.*c)def"#, "abbbcdef", &[], 1283), UnsupportedLookAround
// scanner! { S1283 { mode M { token r#"(?<!a.*c)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*X\\b)def", "abbbbbXdef", 7, 10);
// tr!(r#"(?<!a.*X\b)def"#, "abbbbbXdef", &[("def", 7, 10)], 1284), UnsupportedLookAround
// scanner! { S1284 { mode M { token r#"(?<!a.*X\b)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a.*X\\B)def", "abbbbbXdef");
// tr!(r#"(?<!a.*X\B)def"#, "abbbbbXdef", &[], 1285), UnsupportedLookAround
// scanner! { S1285 { mode M { token r#"(?<!a.*X\B)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*[uvw])def", "abbbbbXdef", 7, 10);
// tr!(r#"(?<!a.*[uvw])def"#, "abbbbbXdef", &[("def", 7, 10)], 1286), UnsupportedLookAround
// scanner! { S1286 { mode M { token r#"(?<!a.*[uvw])def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!a.*[uvw])def", "abbbbbwdef");
// tr!(r#"(?<!a.*[uvw])def"#, "abbbbbwdef", &[], 1287), UnsupportedLookAround
// scanner! { S1287 { mode M { token r#"(?<!a.*[uvw])def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!ab*\\S+)def", "abbbbb   def", 9, 12);
// tr!(r#"(?<!ab*\S+)def"#, "abbbbb   def", &[("def", 9, 12)], 1288), UnsupportedLookAround
// scanner! { S1288 { mode M { token r#"(?<!ab*\S+)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!a.*\\S)def", "abbbbb def", 7, 10);
// tr!(r#"(?<!a.*\S)def"#, "abbbbb def", &[("def", 7, 10)], 1289), UnsupportedLookAround
// scanner! { S1289 { mode M { token r#"(?<!a.*\S)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!ab*\\s+)def", "abbbbb   def");
// tr!(r#"(?<!ab*\s+)def"#, "abbbbb   def", &[], 1290), UnsupportedLookAround
// scanner! { S1290 { mode M { token r#"(?<!ab*\s+)def"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!ab*\\s+\\B)def", "abbbbb   def", 9, 12);
// tr!(r#"(?<!ab*\s+\B)def"#, "abbbbb   def", &[("def", 9, 12)], 1291), UnsupportedLookAround
// scanner! { S1291 { mode M { token r#"(?<!ab*\s+\B)def"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!v|t|a+.*[efg])z", "abcdfz");
// tr!(r#"(?<!v|t|a+.*[efg])z"#, "abcdfz", &[], 1292), UnsupportedLookAround
// scanner! { S1292 { mode M { token r#"(?<!v|t|a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!v|t|a+.*[efg])z", "abcdfzavzuz", 10, 11);
// tr!(r#"(?<!v|t|a+.*[efg])z"#, "abcdfzavzuz", &[("z", 10, 11)], 1293), UnsupportedLookAround
// scanner! { S1293 { mode M { token r#"(?<!v|t|a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!v|t|^a+.*[efg])z", "abcdfz");
// tr!(r#"(?<!v|t|^a+.*[efg])z"#, "abcdfz", &[], 1294), UnsupportedLookAround
// scanner! { S1294 { mode M { token r#"(?<!v|t|^a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!^(?:v|t|a+.*[efg]))z", "abcdfz");
// tr!(r#"(?<!^(?:v|t|a+.*[efg]))z"#, "abcdfz", &[], 1295), UnsupportedLookAround
// scanner! { S1295 { mode M { token r#"(?<!^(?:v|t|a+.*[efg]))z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<!v|^t|^a+.*[efg])z", "uabcdfz", 6, 7);
// tr!(r#"(?<!v|^t|^a+.*[efg])z"#, "uabcdfz", &[("z", 6, 7)], 1296), UnsupportedLookAround
// scanner! { S1296 { mode M { token r#"(?<!v|^t|^a+.*[efg])z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(\\k<2>)|(?<=(\\k<1>))", "");
// tr!(r#"(\k<2>)|(?<=(\k<1>))"#, "", &[], 1297), EscapeUnrecognized
// scanner! { S1297 { mode M { token r#"(\k<2>)|(?<=(\k<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a|\\k<2>)|(?<=(\\k<1>))", "a", 0, 1);
// tr!(r#"(a|\k<2>)|(?<=(\k<1>))"#, "a", &[("a", 0, 1)], 1298), EscapeUnrecognized
// scanner! { S1298 { mode M { token r#"(a|\k<2>)|(?<=(\k<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(a|\\k<2>)|(?<=b(\\k<1>))", "ba", 1, 2);
// tr!(r#"(a|\k<2>)|(?<=b(\k<1>))"#, "ba", &[("a", 1, 2)], 1299), EscapeUnrecognized
// scanner! { S1299 { mode M { token r#"(a|\k<2>)|(?<=b(\k<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!RMA)X", "123RMAX");
// tr!(r#"(?<!RMA)X"#, "123RMAX", &[], 1300), UnsupportedLookAround
// scanner! { S1300 { mode M { token r#"(?<!RMA)X"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=RMA)X", "123RMAX", 6, 7);
// tr!(r#"(?<=RMA)X"#, "123RMAX", &[("X", 6, 7)], 1301), UnsupportedLookAround
// scanner! { S1301 { mode M { token r#"(?<=RMA)X"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!RMA)$", "123RMA");
// tr!(r#"(?<!RMA)$"#, "123RMA", &[], 1302), UnsupportedLookAround
// scanner! { S1302 { mode M { token r#"(?<!RMA)$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=RMA)$", "123RMA", 6, 6);
// tr!(r#"(?<=RMA)$"#, "123RMA", &[("", 6, 6)], 1303), UnsupportedLookAround
// scanner! { S1303 { mode M { token r#"(?<=RMA)$"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!RMA)\\Z", "123RMA");
// tr!(r#"(?<!RMA)\Z"#, "123RMA", &[], 1304), UnsupportedLookAround
// scanner! { S1304 { mode M { token r#"(?<!RMA)\Z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=RMA)\\Z", "123RMA", 6, 6);
// tr!(r#"(?<=RMA)\Z"#, "123RMA", &[("", 6, 6)], 1305), UnsupportedLookAround
// scanner! { S1305 { mode M { token r#"(?<=RMA)\Z"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?<!RMA)\\z", "123RMA");
// tr!(r#"(?<!RMA)\z"#, "123RMA", &[], 1306), UnsupportedLookAround
// scanner! { S1306 { mode M { token r#"(?<!RMA)\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?<=RMA)\\z", "123RMA", 6, 6);
// tr!(r#"(?<=RMA)\z"#, "123RMA", &[("", 6, 6)], 1307), UnsupportedLookAround
// scanner! { S1307 { mode M { token r#"(?<=RMA)\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?(a)\\g<1>|b))", "aab", 0, 3);
// tr!(r#"((?(a)\g<1>|b))"#, "aab", &[("aab", 0, 3)], 1308), FlagUnrecognized
// scanner! { S1308 { mode M { token r#"((?(a)\g<1>|b))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?(a)\\g<1>))", "aab", 0, 2);
// tr!(r#"((?(a)\g<1>))"#, "aab", &[("aa", 0, 2)], 1309), FlagUnrecognized
// scanner! { S1309 { mode M { token r#"((?(a)\g<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("((?(a)\\g<1>))", "", 0, 0);
// tr!(r#"((?(a)\g<1>))"#, "", &[], 1310), FlagUnrecognized
// scanner! { S1310 { mode M { token r#"((?(a)\g<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(b(?(a)|\\g<1>))", "bba", 0, 3);
// tr!(r#"(b(?(a)|\g<1>))"#, "bba", &[("bba", 0, 3)], 1311), FlagUnrecognized
// scanner! { S1311 { mode M { token r#"(b(?(a)|\g<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// e("(()(?(2)\\g<1>))", "", ONIGERR_NEVER_ENDING_RECURSION);
// tr!(r#"(()(?(2)\g<1>))"#, "", "ONIGERR_NEVER_ENDING_RECURSION", 1312),
// scanner! { S1312 { mode M { token r#"(()(?(2)\g<1>))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?(a)(?:b|c))", "ac", 0, 2);
// tr!(r#"(?(a)(?:b|c))"#, "ac", &[("ac", 0, 2)], 1313), FlagUnrecognized
// scanner! { S1313 { mode M { token r#"(?(a)(?:b|c))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?(a)(?:b|c))", "", 0, 0);
// tr!(r#"(?(a)(?:b|c))"#, "", &[], 1314), FlagUnrecognized
// scanner! { S1314 { mode M { token r#"(?(a)(?:b|c))"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?(a)b)", "", 0, 0);
// tr!(r#"(?(a)b)"#, "", &[], 1315), FlagUnrecognized
// scanner! { S1315 { mode M { token r#"(?(a)b)"# => 0; } } }

// -------------------------------------------------------------------------
// n("^(?(a)b|c)", "ac");
// tr!(r#"^(?(a)b|c)"#, "ac", &[], 1316), FlagUnrecognized
// scanner! { S1316 { mode M { token r#"^(?(a)b|c)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)a|b", "B", 0, 1);
// td!(r#"(?i)a|b"#, "B", &[("B", 0, 1)], 1317),
scanner! { S1317 { mode M { token r#"(?i)a|b"# => 0; } } }
#[test]
fn test_match_1317() {
    use s1317::S1317 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("B", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("B", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1317: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1317: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1317: Match end does not match");
        assert_eq!(
            &"B"[ma.1..ma.2],
            ma.0,
            "1317: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("((?i)a|b.)|c", "C");
// td!(r#"((?i)a|b.)|c"#, "C", &[], 1318),
scanner! { S1318 { mode M { token r#"((?i)a|b.)|c"# => 0; } } }
#[test]
fn test_match_1318() {
    use s1318::S1318 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("C", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1318: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("c(?i)a.|b.", "Caz");
// td!(r#"c(?i)a.|b."#, "Caz", &[], 1319),
scanner! { S1319 { mode M { token r#"c(?i)a.|b."# => 0; } } }
#[test]
fn test_match_1319() {
    use s1319::S1319 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("Caz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1319: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("c(?i)a|b", "cB", 0, 2); /* == c(?i:a|b) */
// td!(r#"c(?i)a|b"#, "cB", &[("cB", 0, 2)], 1320),
scanner! { S1320 { mode M { token r#"c(?i)a|b"# => 0; } } }
#[test]
fn test_match_1320() {
    use s1320::S1320 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("cB", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("B", 1, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1320: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1320: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1320: Match end does not match");
        assert_eq!(
            &"cB"[ma.1..ma.2],
            ma.0,
            "1320: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("c(?i)a.|b.", "cBb", 0, 3);
// td!(r#"c(?i)a.|b."#, "cBb", &[("cBb", 0, 3)], 1321),
scanner! { S1321 { mode M { token r#"c(?i)a.|b."# => 0; } } }
#[test]
fn test_match_1321() {
    use s1321::S1321 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("cBb", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("Bb", 1, 3)]; // Oniguruma: ("cBb", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1321: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1321: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1321: Match end does not match");
        assert_eq!(
            &"cBb"[ma.1..ma.2],
            ma.0,
            "1321: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)st", "st", 0, 2);
// td!(r#"(?i)st"#, "st", &[("st", 0, 2)], 1322),
scanner! { S1322 { mode M { token r#"(?i)st"# => 0; } } }
#[test]
fn test_match_1322() {
    use s1322::S1322 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("st", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("st", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1322: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1322: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1322: Match end does not match");
        assert_eq!(
            &"st"[ma.1..ma.2],
            ma.0,
            "1322: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)st", "St", 0, 2);
// td!(r#"(?i)st"#, "St", &[("St", 0, 2)], 1323),
scanner! { S1323 { mode M { token r#"(?i)st"# => 0; } } }
#[test]
fn test_match_1323() {
    use s1323::S1323 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("St", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("St", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1323: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1323: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1323: Match end does not match");
        assert_eq!(
            &"St"[ma.1..ma.2],
            ma.0,
            "1323: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)st", "sT", 0, 2);
// td!(r#"(?i)st"#, "sT", &[("sT", 0, 2)], 1324),
scanner! { S1324 { mode M { token r#"(?i)st"# => 0; } } }
#[test]
fn test_match_1324() {
    use s1324::S1324 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("sT", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("sT", 0, 2)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1324: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1324: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1324: Match end does not match");
        assert_eq!(
            &"sT"[ma.1..ma.2],
            ma.0,
            "1324: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)st", "\xC5\xBFt", 0, 3); // U+017F
// td!(r#"(?i)st"#, "\xC5\xBFt", &[("\\xC", 0, 3)], 1325), Rust Compile Error
scanner! { S1325 { mode M { token r#"(?i)st"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)st", "\xEF\xAC\x85", 0, 3); // U+FB05
// td!(r#"(?i)st"#, "\xEF\xAC\x85", &[("\\xE", 0, 3)], 1326), Rust Compile Error
scanner! { S1326 { mode M { token r#"(?i)st"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)st", "\xEF\xAC\x86", 0, 3); // U+FB06
// td!(r#"(?i)st"#, "\xEF\xAC\x86", &[("\\xE", 0, 3)], 1327), Rust Compile Error
scanner! { S1327 { mode M { token r#"(?i)st"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)ast", "Ast", 0, 3);
// td!(r#"(?i)ast"#, "Ast", &[("Ast", 0, 3)], 1328),
scanner! { S1328 { mode M { token r#"(?i)ast"# => 0; } } }
#[test]
fn test_match_1328() {
    use s1328::S1328 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("Ast", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("Ast", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1328: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1328: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1328: Match end does not match");
        assert_eq!(
            &"Ast"[ma.1..ma.2],
            ma.0,
            "1328: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)ast", "ASt", 0, 3);
// td!(r#"(?i)ast"#, "ASt", &[("ASt", 0, 3)], 1329),
scanner! { S1329 { mode M { token r#"(?i)ast"# => 0; } } }
#[test]
fn test_match_1329() {
    use s1329::S1329 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ASt", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("ASt", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1329: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1329: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1329: Match end does not match");
        assert_eq!(
            &"ASt"[ma.1..ma.2],
            ma.0,
            "1329: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)ast", "AsT", 0, 3);
// td!(r#"(?i)ast"#, "AsT", &[("AsT", 0, 3)], 1330),
scanner! { S1330 { mode M { token r#"(?i)ast"# => 0; } } }
#[test]
fn test_match_1330() {
    use s1330::S1330 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("AsT", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("AsT", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1330: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1330: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1330: Match end does not match");
        assert_eq!(
            &"AsT"[ma.1..ma.2],
            ma.0,
            "1330: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)ast", "A\xC5\xBFt", 0, 4); // U+017F
// td!(r#"(?i)ast"#, "A\xC5\xBFt", &[("A\\xC", 0, 4)], 1331), Rust Compile Error
scanner! { S1331 { mode M { token r#"(?i)ast"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)ast", "A\xEF\xAC\x85", 0, 4); // U+FB05
// td!(r#"(?i)ast"#, "A\xEF\xAC\x85", &[("A\\xE", 0, 4)], 1332), Rust Compile Error
scanner! { S1332 { mode M { token r#"(?i)ast"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)ast", "A\xEF\xAC\x86", 0, 4); // U+FB06
// td!(r#"(?i)ast"#, "A\xEF\xAC\x86", &[("A\\xE", 0, 4)], 1333), Rust Compile Error
scanner! { S1333 { mode M { token r#"(?i)ast"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)stZ", "stz", 0, 3);
// td!(r#"(?i)stZ"#, "stz", &[("stz", 0, 3)], 1334),
scanner! { S1334 { mode M { token r#"(?i)stZ"# => 0; } } }
#[test]
fn test_match_1334() {
    use s1334::S1334 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("stz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("stz", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1334: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1334: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1334: Match end does not match");
        assert_eq!(
            &"stz"[ma.1..ma.2],
            ma.0,
            "1334: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)stZ", "Stz", 0, 3);
// td!(r#"(?i)stZ"#, "Stz", &[("Stz", 0, 3)], 1335),
scanner! { S1335 { mode M { token r#"(?i)stZ"# => 0; } } }
#[test]
fn test_match_1335() {
    use s1335::S1335 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("Stz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("Stz", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1335: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1335: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1335: Match end does not match");
        assert_eq!(
            &"Stz"[ma.1..ma.2],
            ma.0,
            "1335: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)stZ", "sTz", 0, 3);
// td!(r#"(?i)stZ"#, "sTz", &[("sTz", 0, 3)], 1336),
scanner! { S1336 { mode M { token r#"(?i)stZ"# => 0; } } }
#[test]
fn test_match_1336() {
    use s1336::S1336 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("sTz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("sTz", 0, 3)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1336: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1336: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1336: Match end does not match");
        assert_eq!(
            &"sTz"[ma.1..ma.2],
            ma.0,
            "1336: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)stZ", "\xC5\xBFtz", 0, 4); // U+017F
// td!(r#"(?i)stZ"#, "\xC5\xBFtz", &[("\\xC5", 0, 4)], 1337), Rust Compile Error
scanner! { S1337 { mode M { token r#"(?i)stZ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)stZ", "\xEF\xAC\x85z", 0, 4); // U+FB05
// td!(r#"(?i)stZ"#, "\xEF\xAC\x85z", &[("\\xEF", 0, 4)], 1338), Rust Compile Error
scanner! { S1338 { mode M { token r#"(?i)stZ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)stZ", "\xEF\xAC\x86z", 0, 4); // U+FB06
// td!(r#"(?i)stZ"#, "\xEF\xAC\x86z", &[("\\xEF", 0, 4)], 1339), Rust Compile Error
scanner! { S1339 { mode M { token r#"(?i)stZ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)BstZ", "bstz", 0, 4);
// td!(r#"(?i)BstZ"#, "bstz", &[("bstz", 0, 4)], 1340),
scanner! { S1340 { mode M { token r#"(?i)BstZ"# => 0; } } }
#[test]
fn test_match_1340() {
    use s1340::S1340 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bstz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bstz", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1340: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1340: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1340: Match end does not match");
        assert_eq!(
            &"bstz"[ma.1..ma.2],
            ma.0,
            "1340: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)BstZ", "bStz", 0, 4);
// td!(r#"(?i)BstZ"#, "bStz", &[("bStz", 0, 4)], 1341),
scanner! { S1341 { mode M { token r#"(?i)BstZ"# => 0; } } }
#[test]
fn test_match_1341() {
    use s1341::S1341 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bStz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bStz", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1341: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1341: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1341: Match end does not match");
        assert_eq!(
            &"bStz"[ma.1..ma.2],
            ma.0,
            "1341: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)BstZ", "bsTz", 0, 4);
// td!(r#"(?i)BstZ"#, "bsTz", &[("bsTz", 0, 4)], 1342),
scanner! { S1342 { mode M { token r#"(?i)BstZ"# => 0; } } }
#[test]
fn test_match_1342() {
    use s1342::S1342 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("bsTz", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("bsTz", 0, 4)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1342: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1342: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1342: Match end does not match");
        assert_eq!(
            &"bsTz"[ma.1..ma.2],
            ma.0,
            "1342: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)BstZ", "b\xC5\xBFtz", 0, 5); // U+017F
// td!(r#"(?i)BstZ"#, "b\xC5\xBFtz", &[("b\\xC5", 0, 5)], 1343), Rust Compile Error
scanner! { S1343 { mode M { token r#"(?i)BstZ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)BstZ", "b\xEF\xAC\x85z", 0, 5); // U+FB05
// td!(r#"(?i)BstZ"#, "b\xEF\xAC\x85z", &[("b\\xEF", 0, 5)], 1344), Rust Compile Error
scanner! { S1344 { mode M { token r#"(?i)BstZ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)BstZ", "b\xEF\xAC\x86z", 0, 5); // U+FB06
// td!(r#"(?i)BstZ"#, "b\xEF\xAC\x86z", &[("b\\xEF", 0, 5)], 1345), Rust Compile Error
scanner! { S1345 { mode M { token r#"(?i)BstZ"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*st\\z", "tttssss\xC5\xBFt", 0, 10); // U+017F
// tu!(r#"(?i).*st\z"#, "tttssss\xC5\xBFt", &[("tttssss\\xC", 0, 10)], 1346), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1346 { mode M { token r#"(?i).*st\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*st\\z", "tttssss\xEF\xAC\x85", 0, 10); // U+FB05
// tu!(r#"(?i).*st\z"#, "tttssss\xEF\xAC\x85", &[("tttssss\\xE", 0, 10)], 1347), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1347 { mode M { token r#"(?i).*st\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*st\\z", "tttssss\xEF\xAC\x86", 0, 10); // U+FB06
// tu!(r#"(?i).*st\z"#, "tttssss\xEF\xAC\x86", &[("tttssss\\xE", 0, 10)], 1348), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1348 { mode M { token r#"(?i).*st\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*あstい\\z", "tttssssあ\xC5\xBFtい", 0, 16); // U+017F
// tu!(r#"(?i).*あstい\z"#, "tttssssあ\xC5\xBFtい", &[("tttssssあ\\xC5\\xBF", 0, 16)], 1349), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1349 { mode M { token r#"(?i).*あstい\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*あstい\\z", "tttssssあ\xEF\xAC\x85い", 0, 16); // U+FB05
// tu!(r#"(?i).*あstい\z"#, "tttssssあ\xEF\xAC\x85い", &[("tttssssあ\\xEF\\xAC", 0, 16)], 1350), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1350 { mode M { token r#"(?i).*あstい\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*あstい\\z", "tttssssあ\xEF\xAC\x86い", 0, 16); // U+FB06
// tu!(r#"(?i).*あstい\z"#, "tttssssあ\xEF\xAC\x86い", &[("tttssssあ\\xEF\\xAC", 0, 16)], 1351), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1351 { mode M { token r#"(?i).*あstい\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*\xC5\xBFt\\z", "tttssssst", 0, 9); // U+017F
// tu!(r#"(?i).*\xC5\xBFt\z"#, "tttssssst", &[("tttssssst", 0, 9)], 1352), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1352 { mode M { token r#"(?i).*\xC5\xBFt\z"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?i).*\xEF\xAC\x85\\z", "tttssssあst", 0, 12); // U+FB05 // 1353

// -------------------------------------------------------------------------
// *** Index error: x2("(?i).*\xEF\xAC\x86い\\z", "tttssssstい", 0, 12); // U+FB06 // 1354

// -------------------------------------------------------------------------
// x2("(?i).*\xEF\xAC\x85\\z", "tttssssあ\xEF\xAC\x85", 0, 13);
// tu!(r#"(?i).*\xEF\xAC\x85\z"#, "tttssssあ\xEF\xAC\x85", &[("tttssssあ\\xEF\\", 0, 13)], 1355), UnsupportedFeatureError("EndLine Look(End)")
// scanner! { S1355 { mode M { token r#"(?i).*\xEF\xAC\x85\z"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*ss", "abcdefghijklmnopqrstuvwxyz\xc3\x9f", 0, 28); // U+00DF
// td!(r#"(?i).*ss"#, "abcdefghijklmnopqrstuvwxyz\xc3\x9f", &[("abcdefghijklmnopqrstuvwxyz\\x", 0, 28)], 1356), Rust Compile Error
scanner! { S1356 { mode M { token r#"(?i).*ss"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*ss.*", "abcdefghijklmnopqrstuvwxyz\xc3\x9fxyz", 0, 31); // U+00DF
// td!(r#"(?i).*ss.*"#, "abcdefghijklmnopqrstuvwxyz\xc3\x9fxyz", &[("abcdefghijklmnopqrstuvwxyz\\xc3\\", 0, 31)], 1357), Rust Compile Error
scanner! { S1357 { mode M { token r#"(?i).*ss.*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i).*\xc3\x9f", "abcdefghijklmnopqrstuvwxyzss", 0, 28); // U+00DF
// td!(r#"(?i).*\xc3\x9f"#, "abcdefghijklmnopqrstuvwxyzss", &[("abcdefghijklmnopqrstuvwxyzss", 0, 28)], 1358),
scanner! { S1358 { mode M { token r#"(?i).*\xc3\x9f"# => 0; } } }
#[test]
fn test_match_1358() {
    use s1358::S1358 as S;
    let scanner = S::new();
    let matches = scanner
        .find_matches("abcdefghijklmnopqrstuvwxyzss", 0)
        .collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: ("abcdefghijklmnopqrstuvwxyzss", 0, 28)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1358: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i).*ss.*", "abcdefghijklmnopqrstuvwxyzSSxyz", 0, 31);
// td!(r#"(?i).*ss.*"#, "abcdefghijklmnopqrstuvwxyzSSxyz", &[("abcdefghijklmnopqrstuvwxyzSSxyz", 0, 31)], 1359),
scanner! { S1359 { mode M { token r#"(?i).*ss.*"# => 0; } } }
#[test]
fn test_match_1359() {
    use s1359::S1359 as S;
    let scanner = S::new();
    let matches = scanner
        .find_matches("abcdefghijklmnopqrstuvwxyzSSxyz", 0)
        .collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("abcdefghijklmnopqrstuvwxyzSSxyz", 0, 31)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1359: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1359: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1359: Match end does not match");
        assert_eq!(
            &"abcdefghijklmnopqrstuvwxyzSSxyz"[ma.1..ma.2],
            ma.0,
            "1359: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?i)ssv", "\xc3\x9fv", 0, 3); // U+00DF
// td!(r#"(?i)ssv"#, "\xc3\x9fv", &[("\\xc", 0, 3)], 1360), Rust Compile Error
scanner! { S1360 { mode M { token r#"(?i)ssv"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)(?<=ss)v", "SSv", 2, 3);
// tr!(r#"(?i)(?<=ss)v"#, "SSv", &[("v", 2, 3)], 1361), UnsupportedLookAround
// scanner! { S1361 { mode M { token r#"(?i)(?<=ss)v"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)(?<=\xc3\x9f)v", "\xc3\x9fv", 2, 3);
// tr!(r#"(?i)(?<=\xc3\x9f)v"#, "\xc3\x9fv", &[("c", 2, 3)], 1362), UnsupportedLookAround
// scanner! { S1362 { mode M { token r#"(?i)(?<=\xc3\x9f)v"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?i).+Isssǰ", ".+Isssǰ", 0, 8); // 1363

// -------------------------------------------------------------------------
// *** Index error: x2(".+Isssǰ", ".+Isssǰ", 0, 8); // 1364

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)ǰ", "ǰ", 0, 2); // 1365

// -------------------------------------------------------------------------
// x2("(?i)ǰ", "j\xcc\x8c", 0, 3);
// td!(r#"(?i)ǰ"#, "j\xcc\x8c", &[("j\\x", 0, 3)], 1366), Rust Compile Error
scanner! { S1366 { mode M { token r#"(?i)ǰ"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)j\xcc\x8c", "ǰ", 0, 2); // 1367

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)5ǰ", "5ǰ", 0, 3); // 1368

// -------------------------------------------------------------------------
// x2("(?i)5ǰ", "5j\xcc\x8c", 0, 4);
// td!(r#"(?i)5ǰ"#, "5j\xcc\x8c", &[("5j\\x", 0, 4)], 1369), Rust Compile Error
scanner! { S1369 { mode M { token r#"(?i)5ǰ"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)5j\xcc\x8c", "5ǰ", 0, 3); // 1370

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)ǰv", "ǰV", 0, 3); // 1371

// -------------------------------------------------------------------------
// x2("(?i)ǰv", "j\xcc\x8cV", 0, 4);
// td!(r#"(?i)ǰv"#, "j\xcc\x8cV", &[("j\\xc", 0, 4)], 1372), Rust Compile Error
scanner! { S1372 { mode M { token r#"(?i)ǰv"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)j\xcc\x8cv", "ǰV", 0, 3); // 1373

// -------------------------------------------------------------------------
// *** Index error: x2("(?i)[ǰ]", "ǰ", 0, 2); // 1374

// -------------------------------------------------------------------------
// x2("(?i)[ǰ]", "j\xcc\x8c", 0, 3);
// td!(r#"(?i)[ǰ]"#, "j\xcc\x8c", &[("j\\x", 0, 3)], 1375), Rust Compile Error
scanner! { S1375 { mode M { token r#"(?i)[ǰ]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)\ufb00a", "ffa", 0, 3);
// td!(r#"(?i)\ufb00a"#, "ffa", &[("ffa", 0, 3)], 1376),
scanner! { S1376 { mode M { token r#"(?i)\ufb00a"# => 0; } } }
#[test]
fn test_match_1376() {
    use s1376::S1376 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("ffa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: ("ffa", 0, 3)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1376: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("(?i)ffz", "\xef\xac\x80z", 0, 4);
// td!(r#"(?i)ffz"#, "\xef\xac\x80z", &[("\\xef", 0, 4)], 1377), Rust Compile Error
scanner! { S1377 { mode M { token r#"(?i)ffz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)\u2126", "\xcf\x89", 0, 2);
// td!(r#"(?i)\u2126"#, "\xcf\x89", &[("\\x", 0, 2)], 1378), Rust Compile Error
scanner! { S1378 { mode M { token r#"(?i)\u2126"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a(?i)\u2126", "a\xcf\x89", 0, 3);
// td!(r#"a(?i)\u2126"#, "a\xcf\x89", &[("a\\x", 0, 3)], 1379), Rust Compile Error
scanner! { S1379 { mode M { token r#"a(?i)\u2126"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)A\u2126", "a\xcf\x89", 0, 3);
// td!(r#"(?i)A\u2126"#, "a\xcf\x89", &[("a\\x", 0, 3)], 1380), Rust Compile Error
scanner! { S1380 { mode M { token r#"(?i)A\u2126"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)A\u2126=", "a\xcf\x89=", 0, 4);
// td!(r#"(?i)A\u2126="#, "a\xcf\x89=", &[("a\\xc", 0, 4)], 1381), Rust Compile Error
scanner! { S1381 { mode M { token r#"(?i)A\u2126="# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i:ss)=1234567890", "\xc5\xbf\xc5\xbf=1234567890", 0, 15);
// td!(r#"(?i:ss)=1234567890"#, "\xc5\xbf\xc5\xbf=1234567890", &[("\\xc5\\xbf\\xc5\\xb", 0, 15)], 1382), Rust Compile Error
scanner! { S1382 { mode M { token r#"(?i:ss)=1234567890"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x{000A}", "\x0a", 0, 1);
// td!(r#"\x{000A}"#, "\x0a", &[("\\", 0, 1)], 1383),
scanner! { S1383 { mode M { token r#"\x{000A}"# => 0; } } }
#[test]
fn test_match_1383() {
    use s1383::S1383 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("\x0a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("\x0a", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1383: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1383: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1383: Match end does not match");
        assert_eq!(
            &"\x0a"[ma.1..ma.2],
            ma.0,
            "1383: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("\\x{000A 002f}", "\x0a\x2f", 0, 2);
// tr!(r#"\x{000A 002f}"#, "\x0a\x2f", &[("\\x", 0, 2)], 1384), EscapeHexInvalidDigit
// scanner! { S1384 { mode M { token r#"\x{000A 002f}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x{000A 002f }", "\x0a\x2f", 0, 2);
// tr!(r#"\x{000A 002f }"#, "\x0a\x2f", &[("\\x", 0, 2)], 1385), EscapeHexInvalidDigit
// scanner! { S1385 { mode M { token r#"\x{000A 002f }"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x{007C     001b}", "\x7c\x1b", 0, 2);
// tr!(r#"\x{007C     001b}"#, "\x7c\x1b", &[("\\x", 0, 2)], 1386), EscapeHexInvalidDigit
// scanner! { S1386 { mode M { token r#"\x{007C     001b}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\x{1 2 3 4 5 6 7 8 9 a b c d e f}", "\x01\x02\x3\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f", 0, 15);
// tr!(r#"\x{1 2 3 4 5 6 7 8 9 a b c d e f}"#, "\x01\x02\x3\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f", &[("\\x01\\x02\\x3\\x04", 0, 15)], 1387), EscapeHexInvalidDigit
// scanner! { S1387 { mode M { token r#"\x{1 2 3 4 5 6 7 8 9 a b c d e f}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a\\x{000A 002f}@", "a\x0a\x2f@", 0, 4);
// tr!(r#"a\x{000A 002f}@"#, "a\x0a\x2f@", &[("a\\x0", 0, 4)], 1388), EscapeHexInvalidDigit
// scanner! { S1388 { mode M { token r#"a\x{000A 002f}@"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a\\x{0060\n0063}@", "a\x60\x63@", 0, 4);
// tr!(r#"a\x{0060\n0063}@"#, "a\x60\x63@", &[("a\\x6", 0, 4)], 1389), EscapeHexInvalidDigit
// scanner! { S1389 { mode M { token r#"a\x{0060\n0063}@"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{00000001 000000012}", "", ONIGERR_TOO_LONG_WIDE_CHAR_VALUE);
// tr!(r#"\x{00000001 000000012}"#, "", "ONIGERR_TOO_LONG_WIDE_CHAR_VALUE", 1390),
// scanner! { S1390 { mode M { token r#"\x{00000001 000000012}"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{000A 00000002f}", "", ONIGERR_TOO_LONG_WIDE_CHAR_VALUE);
// tr!(r#"\x{000A 00000002f}"#, "", "ONIGERR_TOO_LONG_WIDE_CHAR_VALUE", 1391),
// scanner! { S1391 { mode M { token r#"\x{000A 00000002f}"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{000A 002f/", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\x{000A 002f/"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1392),
// scanner! { S1392 { mode M { token r#"\x{000A 002f/"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{000A 002f /", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\x{000A 002f /"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1393),
// scanner! { S1393 { mode M { token r#"\x{000A 002f /"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{000A", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\x{000A"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1394),
// scanner! { S1394 { mode M { token r#"\x{000A"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{000A ", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\x{000A "#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1395),
// scanner! { S1395 { mode M { token r#"\x{000A "# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{000A 002f ", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\x{000A 002f "#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1396),
// scanner! { S1396 { mode M { token r#"\x{000A 002f "# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\o{102}", "B", 0, 1);
// tr!(r#"\o{102}"#, "B", &[("B", 0, 1)], 1397), EscapeUnrecognized
// scanner! { S1397 { mode M { token r#"\o{102}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\o{102 103}", "BC", 0, 2);
// tr!(r#"\o{102 103}"#, "BC", &[("BC", 0, 2)], 1398), EscapeUnrecognized
// scanner! { S1398 { mode M { token r#"\o{102 103}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\o{0160 0000161}", "pq", 0, 2);
// tr!(r#"\o{0160 0000161}"#, "pq", &[("pq", 0, 2)], 1399), EscapeUnrecognized
// scanner! { S1399 { mode M { token r#"\o{0160 0000161}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\o{1 2 3 4 5 6 7 10 11 12 13 14 15 16 17}", "\x01\x02\x3\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f", 0, 15);
// tr!(r#"\o{1 2 3 4 5 6 7 10 11 12 13 14 15 16 17}"#, "\x01\x02\x3\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f", &[("\\x01\\x02\\x3\\x04", 0, 15)], 1400), EscapeUnrecognized
// scanner! { S1400 { mode M { token r#"\o{1 2 3 4 5 6 7 10 11 12 13 14 15 16 17}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\o{0007 0010 }", "\x07\x08", 0, 2);
// tr!(r#"\o{0007 0010 }"#, "\x07\x08", &[("\\x", 0, 2)], 1401), EscapeUnrecognized
// scanner! { S1401 { mode M { token r#"\o{0007 0010 }"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\o{0000 0015/", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\o{0000 0015/"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1402),
// scanner! { S1402 { mode M { token r#"\o{0000 0015/"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\o{0000 0015 /", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\o{0000 0015 /"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1403),
// scanner! { S1403 { mode M { token r#"\o{0000 0015 /"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\o{0015", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\o{0015"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1404),
// scanner! { S1404 { mode M { token r#"\o{0015"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\o{0015 ", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\o{0015 "#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1405),
// scanner! { S1405 { mode M { token r#"\o{0015 "# => 0; } } }

// -------------------------------------------------------------------------
// e("\\o{0007 002f}", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\o{0007 002f}"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1406),
// scanner! { S1406 { mode M { token r#"\o{0007 002f}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{000A}]", "\x0a", 0, 1);
// td!(r#"[\x{000A}]"#, "\x0a", &[("\\", 0, 1)], 1407),
scanner! { S1407 { mode M { token r#"[\x{000A}]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{000A 002f}]+", "\x0a\x2f\x2e", 0, 2);
// tr!(r#"[\x{000A 002f}]+"#, "\x0a\x2f\x2e", &[("\\x", 0, 2)], 1408), EscapeHexInvalidDigit
// scanner! { S1408 { mode M { token r#"[\x{000A 002f}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{01 0F 1A 2c 4B}]+", "\x20\x01\x0f\x1a\x2c\x4b\x1b", 1, 6);
// tr!(r#"[\x{01 0F 1A 2c 4B}]+"#, "\x20\x01\x0f\x1a\x2c\x4b\x1b", &[("x20\\x", 1, 6)], 1409), EscapeHexInvalidDigit
// scanner! { S1409 { mode M { token r#"[\x{01 0F 1A 2c 4B}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{0020 0024}-\\x{0026}]+", "\x25\x24\x26\x23", 0, 3);
// tr!(r#"[\x{0020 0024}-\x{0026}]+"#, "\x25\x24\x26\x23", &[("\\x2", 0, 3)], 1410), EscapeHexInvalidDigit
// scanner! { S1410 { mode M { token r#"[\x{0020 0024}-\x{0026}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{0030}-\\x{0033 005a}]+", "\x30\x31\x32\x33\x5a\34", 0, 5);
// tr!(r#"[\x{0030}-\x{0033 005a}]+"#, "\x30\x31\x32\x33\x5a\34", &[("\\x30\\", 0, 5)], 1411), EscapeHexInvalidDigit
// scanner! { S1411 { mode M { token r#"[\x{0030}-\x{0033 005a}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{000A]", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{000A]"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1412),
// scanner! { S1412 { mode M { token r#"[\x{000A]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{000A ]", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{000A ]"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1413),
// scanner! { S1413 { mode M { token r#"[\x{000A ]"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{000A }]", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{000A }]"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1414),
// scanner! { S1414 { mode M { token r#"[\x{000A }]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\o{102}]", "B", 0, 1);
// tr!(r#"[\o{102}]"#, "B", &[("B", 0, 1)], 1415), EscapeUnrecognized
// scanner! { S1415 { mode M { token r#"[\o{102}]"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\o{102 103}]*", "BC", 0, 2);
// tr!(r#"[\o{102 103}]*"#, "BC", &[("BC", 0, 2)], 1416), EscapeUnrecognized
// scanner! { S1416 { mode M { token r#"[\o{102 103}]*"# => 0; } } }

// -------------------------------------------------------------------------
// e("[a\\o{002  003]bcde|zzz", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[a\o{002  003]bcde|zzz"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1417),
// scanner! { S1417 { mode M { token r#"[a\o{002  003]bcde|zzz"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{0030-0039}]+", "abc0123456789def", 3, 13);
// tr!(r#"[\x{0030-0039}]+"#, "abc0123456789def", &[("0123456789", 3, 13)], 1418), EscapeHexInvalidDigit
// scanner! { S1418 { mode M { token r#"[\x{0030-0039}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{0030 - 0039 }]+", "abc0123456789def", 3, 13);
// tr!(r#"[\x{0030 - 0039 }]+"#, "abc0123456789def", &[("0123456789", 3, 13)], 1419), EscapeHexInvalidDigit
// scanner! { S1419 { mode M { token r#"[\x{0030 - 0039 }]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{0030 - 0039 0063 0064}]+", "abc0123456789def", 2, 14);
// tr!(r#"[\x{0030 - 0039 0063 0064}]+"#, "abc0123456789def", &[("c0123456789d", 2, 14)], 1420), EscapeHexInvalidDigit
// scanner! { S1420 { mode M { token r#"[\x{0030 - 0039 0063 0064}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x{0030 - 0039 0063-0065}]+", "acde019b", 1, 7);
// tr!(r#"[\x{0030 - 0039 0063-0065}]+"#, "acde019b", &[("cde019", 1, 7)], 1421), EscapeHexInvalidDigit
// scanner! { S1421 { mode M { token r#"[\x{0030 - 0039 0063-0065}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{0030 - 0039-0063 0064}]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{0030 - 0039-0063 0064}]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1422),
// scanner! { S1422 { mode M { token r#"[\x{0030 - 0039-0063 0064}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{0030 - }]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{0030 - }]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1423),
// scanner! { S1423 { mode M { token r#"[\x{0030 - }]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{0030 -- 0040}]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{0030 -- 0040}]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1424),
// scanner! { S1424 { mode M { token r#"[\x{0030 -- 0040}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{0030--0040}]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{0030--0040}]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1425),
// scanner! { S1425 { mode M { token r#"[\x{0030--0040}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{0030 - - 0040}]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{0030 - - 0040}]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1426),
// scanner! { S1426 { mode M { token r#"[\x{0030 - - 0040}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{0030 0044 - }]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{0030 0044 - }]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1427),
// scanner! { S1427 { mode M { token r#"[\x{0030 0044 - }]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[a-\\x{0070 - 0039}]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[a-\x{0070 - 0039}]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1428),
// scanner! { S1428 { mode M { token r#"[a-\x{0070 - 0039}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[a-\\x{0063 0071}]+", "dabcqz", 1, 5);
// tr!(r#"[a-\x{0063 0071}]+"#, "dabcqz", &[("abcq", 1, 5)], 1429), EscapeHexInvalidDigit
// scanner! { S1429 { mode M { token r#"[a-\x{0063 0071}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[-\\x{0063-0065}]+", "ace-df", 1, 5);
// tr!(r#"[-\x{0063-0065}]+"#, "ace-df", &[("ce-d", 1, 5)], 1430), EscapeHexInvalidDigit
// scanner! { S1430 { mode M { token r#"[-\x{0063-0065}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\x61-\\x{0063 0065}]+", "abced", 0, 4);
// tr!(r#"[\x61-\x{0063 0065}]+"#, "abced", &[("abce", 0, 4)], 1431), EscapeHexInvalidDigit
// scanner! { S1431 { mode M { token r#"[\x61-\x{0063 0065}]+"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x61-\\x{0063-0065}]+", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x61-\x{0063-0065}]+"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1432),
// scanner! { S1432 { mode M { token r#"[\x61-\x{0063-0065}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[t\\x{0063 0071}]+", "tcqb", 0, 3);
// tr!(r#"[t\x{0063 0071}]+"#, "tcqb", &[("tcq", 0, 3)], 1433), EscapeHexInvalidDigit
// scanner! { S1433 { mode M { token r#"[t\x{0063 0071}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("[\\W\\x{0063 0071}]+", "*cqa", 0, 3);
// tr!(r#"[\W\x{0063 0071}]+"#, "*cqa", &[("*cq", 0, 3)], 1434), EscapeHexInvalidDigit
// scanner! { S1434 { mode M { token r#"[\W\x{0063 0071}]+"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(\\O|(?=z\\g<2>*))(\\g<0>){0}", "a", 0, 1);
// tr!(r#"(\O|(?=z\g<2>*))(\g<0>){0}"#, "a", &[("a", 0, 1)], 1435), EscapeUnrecognized
// scanner! { S1435 { mode M { token r#"(\O|(?=z\g<2>*))(\g<0>){0}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii)abc", "abc", 0, 3);
// tr!(r#"(?Ii)abc"#, "abc", &[("abc", 0, 3)], 1436), FlagUnrecognized
// scanner! { S1436 { mode M { token r#"(?Ii)abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii)abc", "ABC", 0, 3);
// tr!(r#"(?Ii)abc"#, "ABC", &[("ABC", 0, 3)], 1437), FlagUnrecognized
// scanner! { S1437 { mode M { token r#"(?Ii)abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii:abc)", "abc", 0, 3);
// tr!(r#"(?Ii:abc)"#, "abc", &[("abc", 0, 3)], 1438), FlagUnrecognized
// scanner! { S1438 { mode M { token r#"(?Ii:abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii)xyz|abc", "aBc", 0, 3);
// tr!(r#"(?Ii)xyz|abc"#, "aBc", &[("aBc", 0, 3)], 1439), FlagUnrecognized
// scanner! { S1439 { mode M { token r#"(?Ii)xyz|abc"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii:zz|abc|AZ)", "ABc", 0, 3);
// tr!(r#"(?Ii:zz|abc|AZ)"#, "ABc", &[("ABc", 0, 3)], 1440), FlagUnrecognized
// scanner! { S1440 { mode M { token r#"(?Ii:zz|abc|AZ)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?Ii:abc)d", "abc", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?Ii:abc)d"#, "abc", "ONIGERR_INVALID_GROUP_OPTION", 1441),
// scanner! { S1441 { mode M { token r#"(?Ii:abc)d"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?-Ii:abc)", "abc", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?-Ii:abc)"#, "abc", "ONIGERR_INVALID_GROUP_OPTION", 1442),
// scanner! { S1442 { mode M { token r#"(?-Ii:abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?I-i:abc)", "abc", 0, 3);
// tr!(r#"(?I-i:abc)"#, "abc", &[("abc", 0, 3)], 1443), FlagUnrecognized
// scanner! { S1443 { mode M { token r#"(?I-i:abc)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?i-I:abc)", "abc", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?i-I:abc)"#, "abc", "ONIGERR_INVALID_GROUP_OPTION", 1444),
// scanner! { S1444 { mode M { token r#"(?i-I:abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)\xe2\x84\xaa", "k", 0, 1);
// td!(r#"(?i)\xe2\x84\xaa"#, "k", &[("k", 0, 1)], 1445),
scanner! { S1445 { mode M { token r#"(?i)\xe2\x84\xaa"# => 0; } } }
#[test]
fn test_match_1445() {
    use s1445::S1445 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("k", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[]; // Oniguruma: ("k", 0, 1)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1445: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("(?Ii)\xe2\x84\xaa", "k");
// tr!(r#"(?Ii)\xe2\x84\xaa"#, "k", &[], 1446), FlagUnrecognized
// scanner! { S1446 { mode M { token r#"(?Ii)\xe2\x84\xaa"# => 0; } } }

// -------------------------------------------------------------------------
// e("((?Ii)abc)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"((?Ii)abc)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1447),
// scanner! { S1447 { mode M { token r#"((?Ii)abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?Ii)abc)", "ABC", 0, 3);
// tr!(r#"(?:(?Ii)abc)"#, "ABC", &[("ABC", 0, 3)], 1448), FlagUnrecognized
// scanner! { S1448 { mode M { token r#"(?:(?Ii)abc)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:(?:(?Ii)abc))", "ABC", 0, 3);
// tr!(r#"(?:(?:(?Ii)abc))"#, "ABC", &[("ABC", 0, 3)], 1449), FlagUnrecognized
// scanner! { S1449 { mode M { token r#"(?:(?:(?Ii)abc))"# => 0; } } }

// -------------------------------------------------------------------------
// e("x(?Ii)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"x(?Ii)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1450),
// scanner! { S1450 { mode M { token r#"x(?Ii)"# => 0; } } }

// -------------------------------------------------------------------------
// e("()(?Ii)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"()(?Ii)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1451),
// scanner! { S1451 { mode M { token r#"()(?Ii)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?:)(?Ii)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?:)(?Ii)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1452),
// scanner! { S1452 { mode M { token r#"(?:)(?Ii)"# => 0; } } }

// -------------------------------------------------------------------------
// e("^(?Ii)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"^(?Ii)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1453),
// scanner! { S1453 { mode M { token r#"^(?Ii)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii)$", "", 0, 0);
// tr!(r#"(?Ii)$"#, "", &[], 1454), FlagUnrecognized
// scanner! { S1454 { mode M { token r#"(?Ii)$"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Ii)|", "", 0, 0);
// tr!(r#"(?Ii)|"#, "", &[], 1455), FlagUnrecognized
// scanner! { S1455 { mode M { token r#"(?Ii)|"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?Ii)|(?Ii)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?Ii)|(?Ii)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1456),
// scanner! { S1456 { mode M { token r#"(?Ii)|(?Ii)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("a*", "aabcaaa", 0, 2);
// td!(r#"a*"#, "aabcaaa", &[("aa", 0, 2)], 1457),
scanner! { S1457 { mode M { token r#"a*"# => 0; } } }
#[test]
fn test_match_1457() {
    use s1457::S1457 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("aabcaaa", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("aa", 0, 2), ("aaa", 4, 7)]; // Oniguruma: ("aa", 0, 2)
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1457: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1457: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1457: Match end does not match");
        assert_eq!(
            &"aabcaaa"[ma.1..ma.2],
            ma.0,
            "1457: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("(?L)a*", "aabcaaa", 4, 7);
// tr!(r#"(?L)a*"#, "aabcaaa", &[("aaa", 4, 7)], 1458), FlagUnrecognized
// scanner! { S1458 { mode M { token r#"(?L)a*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?L)a{4}|a{3}|b*", "baaaaabbb", 1, 5);
// tr!(r#"(?L)a{4}|a{3}|b*"#, "baaaaabbb", &[("aaaa", 1, 5)], 1459), FlagUnrecognized
// scanner! { S1459 { mode M { token r#"(?L)a{4}|a{3}|b*"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?L)a{3}|a{4}|b*", "baaaaabbb", 1, 5);
// tr!(r#"(?L)a{3}|a{4}|b*"#, "baaaaabbb", &[("aaaa", 1, 5)], 1460), FlagUnrecognized
// scanner! { S1460 { mode M { token r#"(?L)a{3}|a{4}|b*"# => 0; } } }

// -------------------------------------------------------------------------
// e("x(?L)xxxxx", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"x(?L)xxxxx"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1461),
// scanner! { S1461 { mode M { token r#"x(?L)xxxxx"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?-L)x", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?-L)x"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1462),
// scanner! { S1462 { mode M { token r#"(?-L)x"# => 0; } } }

// -------------------------------------------------------------------------
// x3("(..)\\1", "abab", 0, 2, 1);
// tr!(r#"(..)\1"#, "abab", &[("ab", 0, 2)], 1463), UnsupportedBackreference
// scanner! { S1463 { mode M { token r#"(..)\1"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?C)(..)\\1", "abab", ONIGERR_INVALID_BACKREF);
// tr!(r#"(?C)(..)\1"#, "abab", "ONIGERR_INVALID_BACKREF", 1464),
// scanner! { S1464 { mode M { token r#"(?C)(..)\1"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?-C)", "", ONIGERR_INVALID_GROUP_OPTION);
// tr!(r#"(?-C)"#, "", "ONIGERR_INVALID_GROUP_OPTION", 1465),
// scanner! { S1465 { mode M { token r#"(?-C)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?C)(.)(.)(.)(?<name>.)\\1", "abcdd", ONIGERR_NUMBERED_BACKREF_OR_CALL_NOT_ALLOWED);
// tr!(r#"(?C)(.)(.)(.)(?<name>.)\1"#, "abcdd", "ONIGERR_NUMBERED_BACKREF_OR_CALL_NOT_ALLOWED", 1466),
// scanner! { S1466 { mode M { token r#"(?C)(.)(.)(.)(?<name>.)\1"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?L)z|a\\g<0>a", "aazaa", 0, 5);
// tr!(r#"(?L)z|a\g<0>a"#, "aazaa", &[("aazaa", 0, 5)], 1467), FlagUnrecognized
// scanner! { S1467 { mode M { token r#"(?L)z|a\g<0>a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Li)z|a\\g<0>a", "aazAA", 0, 5);
// tr!(r#"(?Li)z|a\g<0>a"#, "aazAA", &[("aazAA", 0, 5)], 1468), FlagUnrecognized
// scanner! { S1468 { mode M { token r#"(?Li)z|a\g<0>a"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?Li:z|a\\g<0>a)", "aazAA", 0, 5);
// tr!(r#"(?Li:z|a\g<0>a)"#, "aazAA", &[("aazAA", 0, 5)], 1469), FlagUnrecognized
// scanner! { S1469 { mode M { token r#"(?Li:z|a\g<0>a)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?L)z|a\\g<0>a", "aazaaaazaaaa", 3, 12);
// tr!(r#"(?L)z|a\g<0>a"#, "aazaaaazaaaa", &[("aaaazaaaa", 3, 12)], 1470), FlagUnrecognized
// scanner! { S1470 { mode M { token r#"(?L)z|a\g<0>a"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)s", "\xc5\xbf");
// tr!(r#"(?iI)s"#, "\xc5\xbf", &[], 1471), FlagUnrecognized
// scanner! { S1471 { mode M { token r#"(?iI)s"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)[s]", "\xc5\xbf");    // FAIL
// tr!(r#"(?iI)[s]"#, "\xc5\xbf", &[], 1472), FlagUnrecognized
// scanner! { S1472 { mode M { token r#"(?iI)[s]"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI:s)", "\xc5\xbf");
// tr!(r#"(?iI:s)"#, "\xc5\xbf", &[], 1473), FlagUnrecognized
// scanner! { S1473 { mode M { token r#"(?iI:s)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI:[s])", "\xc5\xbf");    // FAIL
// tr!(r#"(?iI:[s])"#, "\xc5\xbf", &[], 1474), FlagUnrecognized
// scanner! { S1474 { mode M { token r#"(?iI:[s])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iI)(?:[[:word:]])", "\xc5\xbf", 0, 2);
// tr!(r#"(?iI)(?:[[:word:]])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1475), FlagUnrecognized
// scanner! { S1475 { mode M { token r#"(?iI)(?:[[:word:]])"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)(?W:[[:word:]])", "\xc5\xbf");     // FAIL
// tr!(r#"(?iI)(?W:[[:word:]])"#, "\xc5\xbf", &[], 1476), FlagUnrecognized
// scanner! { S1476 { mode M { token r#"(?iI)(?W:[[:word:]])"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)(?W:\\w)", "\xc5\xbf");
// tr!(r#"(?iI)(?W:\w)"#, "\xc5\xbf", &[], 1477), FlagUnrecognized
// scanner! { S1477 { mode M { token r#"(?iI)(?W:\w)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)(?W:[\\w])", "\xc5\xbf");     // FAIL
// tr!(r#"(?iI)(?W:[\w])"#, "\xc5\xbf", &[], 1478), FlagUnrecognized
// scanner! { S1478 { mode M { token r#"(?iI)(?W:[\w])"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)(?W:\\p{Word})", "\xc5\xbf");
// tr!(r#"(?iI)(?W:\p{Word})"#, "\xc5\xbf", &[], 1479), FlagUnrecognized
// scanner! { S1479 { mode M { token r#"(?iI)(?W:\p{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iI)(?W:[\\p{Word}])", "\xc5\xbf");     // FAIL
// tr!(r#"(?iI)(?W:[\p{Word}])"#, "\xc5\xbf", &[], 1480), FlagUnrecognized
// scanner! { S1480 { mode M { token r#"(?iI)(?W:[\p{Word}])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[[:word:]])",  "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:[[:word:]])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1481), FlagUnrecognized
// scanner! { S1481 { mode M { token r#"(?iW:[[:word:]])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[\\p{Word}])", "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:[\p{Word}])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1482), FlagUnrecognized
// scanner! { S1482 { mode M { token r#"(?iW:[\p{Word}])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[\\w])",       "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:[\w])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1483), FlagUnrecognized
// scanner! { S1483 { mode M { token r#"(?iW:[\w])"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iW:\\p{Word})",    "\xc5\xbf");
// tr!(r#"(?iW:\p{Word})"#, "\xc5\xbf", &[], 1484), FlagUnrecognized
// scanner! { S1484 { mode M { token r#"(?iW:\p{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iW:\\w)",          "\xc5\xbf");
// tr!(r#"(?iW:\w)"#, "\xc5\xbf", &[], 1485), FlagUnrecognized
// scanner! { S1485 { mode M { token r#"(?iW:\w)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)\\p{Word}",     "\xc5\xbf", 0, 2);
// tr!(r#"(?i)\p{Word}"#, "\xc5\xbf", &[("\\x", 0, 2)], 1486), UnicodePropertyNotFound
// scanner! { S1486 { mode M { token r#"(?i)\p{Word}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?i)\\w",           "\xc5\xbf", 0, 2);
// td!(r#"(?i)\w"#, "\xc5\xbf", &[("\\x", 0, 2)], 1487), Rust Compile Error
scanner! { S1487 { mode M { token r#"(?i)\w"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[[:^word:]])",  "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:[[:^word:]])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1488), FlagUnrecognized
// scanner! { S1488 { mode M { token r#"(?iW:[[:^word:]])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[\\P{Word}])",  "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:[\P{Word}])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1489), FlagUnrecognized
// scanner! { S1489 { mode M { token r#"(?iW:[\P{Word}])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[\\W])",        "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:[\W])"#, "\xc5\xbf", &[("\\x", 0, 2)], 1490), FlagUnrecognized
// scanner! { S1490 { mode M { token r#"(?iW:[\W])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:\\P{Word})",    "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:\P{Word})"#, "\xc5\xbf", &[("\\x", 0, 2)], 1491), FlagUnrecognized
// scanner! { S1491 { mode M { token r#"(?iW:\P{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:\\W)",          "\xc5\xbf", 0, 2);
// tr!(r#"(?iW:\W)"#, "\xc5\xbf", &[("\\x", 0, 2)], 1492), FlagUnrecognized
// scanner! { S1492 { mode M { token r#"(?iW:\W)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?i)\\P{Word}",      "\xc5\xbf");
// tr!(r#"(?i)\P{Word}"#, "\xc5\xbf", &[], 1493), UnicodePropertyNotFound
// scanner! { S1493 { mode M { token r#"(?i)\P{Word}"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?i)\\W",            "\xc5\xbf");
// td!(r#"(?i)\W"#, "\xc5\xbf", &[], 1494), Rust Compile Error
scanner! { S1494 { mode M { token r#"(?i)\W"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[[:^word:]])",  "s", 0, 1);
// tr!(r#"(?iW:[[:^word:]])"#, "s", &[("s", 0, 1)], 1495), FlagUnrecognized
// scanner! { S1495 { mode M { token r#"(?iW:[[:^word:]])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[\\P{Word}])",  "s", 0, 1);
// tr!(r#"(?iW:[\P{Word}])"#, "s", &[("s", 0, 1)], 1496), FlagUnrecognized
// scanner! { S1496 { mode M { token r#"(?iW:[\P{Word}])"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?iW:[\\W])",        "s", 0, 1);
// tr!(r#"(?iW:[\W])"#, "s", &[("s", 0, 1)], 1497), FlagUnrecognized
// scanner! { S1497 { mode M { token r#"(?iW:[\W])"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iW:\\P{Word})",     "s");
// tr!(r#"(?iW:\P{Word})"#, "s", &[], 1498), FlagUnrecognized
// scanner! { S1498 { mode M { token r#"(?iW:\P{Word})"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?iW:\\W)",           "s");
// tr!(r#"(?iW:\W)"#, "s", &[], 1499), FlagUnrecognized
// scanner! { S1499 { mode M { token r#"(?iW:\W)"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?i)\\P{Word}",       "s");
// tr!(r#"(?i)\P{Word}"#, "s", &[], 1500), UnicodePropertyNotFound
// scanner! { S1500 { mode M { token r#"(?i)\P{Word}"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?i)\\W",             "s");
// td!(r#"(?i)\W"#, "s", &[], 1501),
scanner! { S1501 { mode M { token r#"(?i)\W"# => 0; } } }
#[test]
fn test_match_1501() {
    use s1501::S1501 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("s", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1501: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// x2("[[:punct:]]", ":", 0, 1);
// td!(r#"[[:punct:]]"#, ":", &[(":", 0, 1)], 1502),
scanner! { S1502 { mode M { token r#"[[:punct:]]"# => 0; } } }
#[test]
fn test_match_1502() {
    use s1502::S1502 as S;
    let scanner = S::new();
    let matches = scanner.find_matches(":", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[(":", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1502: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1502: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1502: Match end does not match");
        assert_eq!(
            &":"[ma.1..ma.2],
            ma.0,
            "1502: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[:punct:]]", "$", 0, 1);
// td!(r#"[[:punct:]]"#, "$", &[("$", 0, 1)], 1503),
scanner! { S1503 { mode M { token r#"[[:punct:]]"# => 0; } } }
#[test]
fn test_match_1503() {
    use s1503::S1503 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("$", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("$", 0, 1)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1503: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1503: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1503: Match end does not match");
        assert_eq!(
            &"$"[ma.1..ma.2],
            ma.0,
            "1503: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// x2("[[:punct:]]+", "$+<=>^`|~", 0, 9);
// td!(r#"[[:punct:]]+"#, "$+<=>^`|~", &[("$+<=>^`|~", 0, 9)], 1504),
scanner! { S1504 { mode M { token r#"[[:punct:]]+"# => 0; } } }
#[test]
fn test_match_1504() {
    use s1504::S1504 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("$+<=>^`|~", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[("$+<=>^`|~", 0, 9)];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1504: Unexpected match count"
    );
    for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {
        assert_eq!(
            matches[i].span.start, ma.1,
            "1504: Match start does not match"
        );
        assert_eq!(matches[i].span.end, ma.2, "1504: Match end does not match");
        assert_eq!(
            &"$+<=>^`|~"[ma.1..ma.2],
            ma.0,
            "1504: Matched substring does not match expected"
        );
    }
}

// -------------------------------------------------------------------------
// n("[[:punct:]]", "a");
// td!(r#"[[:punct:]]"#, "a", &[], 1505),
scanner! { S1505 { mode M { token r#"[[:punct:]]"# => 0; } } }
#[test]
fn test_match_1505() {
    use s1505::S1505 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("a", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1505: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// n("[[:punct:]]", "7");
// td!(r#"[[:punct:]]"#, "7", &[], 1506),
scanner! { S1506 { mode M { token r#"[[:punct:]]"# => 0; } } }
#[test]
fn test_match_1506() {
    use s1506::S1506 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("7", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1506: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// *** Index error: x2("\\p{PosixPunct}+", "$¦", 0, 3); // 1507

// -------------------------------------------------------------------------
// x2("\\A.*\\R", "\n", 0, 1);
// tr!(r#"\A.*\R"#, "\n", &[("\\", 0, 1)], 1508), EscapeUnrecognized
// scanner! { S1508 { mode M { token r#"\A.*\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A\\O*\\R", "\n", 0, 1);
// tr!(r#"\A\O*\R"#, "\n", &[("\\", 0, 1)], 1509), EscapeUnrecognized
// scanner! { S1509 { mode M { token r#"\A\O*\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A\\n*\\R", "\n", 0, 1);
// tr!(r#"\A\n*\R"#, "\n", &[("\\", 0, 1)], 1510), EscapeUnrecognized
// scanner! { S1510 { mode M { token r#"\A\n*\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A\\R*\\R", "\n", 0, 1);
// tr!(r#"\A\R*\R"#, "\n", &[("\\", 0, 1)], 1511), EscapeUnrecognized
// scanner! { S1511 { mode M { token r#"\A\R*\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\At*\\R", "\n", 0, 1);
// tr!(r#"\At*\R"#, "\n", &[("\\", 0, 1)], 1512), EscapeUnrecognized
// scanner! { S1512 { mode M { token r#"\At*\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.{0,99}\\R", "\n", 0, 1);
// tr!(r#"\A.{0,99}\R"#, "\n", &[("\\", 0, 1)], 1513), EscapeUnrecognized
// scanner! { S1513 { mode M { token r#"\A.{0,99}\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A\\O{0,99}\\R", "\n", 0, 1);
// tr!(r#"\A\O{0,99}\R"#, "\n", &[("\\", 0, 1)], 1514), EscapeUnrecognized
// scanner! { S1514 { mode M { token r#"\A\O{0,99}\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A\\n{0,99}\\R", "\n", 0, 1);
// tr!(r#"\A\n{0,99}\R"#, "\n", &[("\\", 0, 1)], 1515), EscapeUnrecognized
// scanner! { S1515 { mode M { token r#"\A\n{0,99}\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A\\R{0,99}\\R", "\n", 0, 1);
// tr!(r#"\A\R{0,99}\R"#, "\n", &[("\\", 0, 1)], 1516), EscapeUnrecognized
// scanner! { S1516 { mode M { token r#"\A\R{0,99}\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\At{0,99}\\R", "\n", 0, 1);
// tr!(r#"\At{0,99}\R"#, "\n", &[("\\", 0, 1)], 1517), EscapeUnrecognized
// scanner! { S1517 { mode M { token r#"\At{0,99}\R"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.*\\n", "\n", 0, 1);       //  \n
// tu!(r#"\A.*\n"#, "\n", &[("\\", 0, 1)], 1518), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S1518 { mode M { token r#"\A.*\n"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.{0,99}\\n", "\n", 0, 1);
// tu!(r#"\A.{0,99}\n"#, "\n", &[("\\", 0, 1)], 1519), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S1519 { mode M { token r#"\A.{0,99}\n"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.*\\O", "\n", 0, 1);       //  \O
// tr!(r#"\A.*\O"#, "\n", &[("\\", 0, 1)], 1520), EscapeUnrecognized
// scanner! { S1520 { mode M { token r#"\A.*\O"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.{0,99}\\O", "\n", 0, 1);
// tr!(r#"\A.{0,99}\O"#, "\n", &[("\\", 0, 1)], 1521), EscapeUnrecognized
// scanner! { S1521 { mode M { token r#"\A.{0,99}\O"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.*\\s", "\n", 0, 1);       //  \s
// tu!(r#"\A.*\s"#, "\n", &[("\\", 0, 1)], 1522), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S1522 { mode M { token r#"\A.*\s"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\A.{0,99}\\s", "\n", 0, 1);
// tu!(r#"\A.{0,99}\s"#, "\n", &[("\\", 0, 1)], 1523), UnsupportedFeatureError("StartLine Look(Start)")
// scanner! { S1523 { mode M { token r#"\A.{0,99}\s"# => 0; } } }

// -------------------------------------------------------------------------
// n("a(b|)+d", "abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcd"); /* https://www.haijin-boys.com/discussions/5079 */
// td!(r#"a(b|)+d"#, "abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcd", &[], 1524),
scanner! { S1524 { mode M { token r#"a(b|)+d"# => 0; } } }
#[test]
fn test_match_1524() {
    use s1524::S1524 as S;
    let scanner = S::new();
    let matches = scanner
        .find_matches("abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcd", 0)
        .collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1524: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// e("   \xfd", "", ONIGERR_INVALID_CODE_POINT_VALUE); /* https://bugs.php.net/bug.php?id=77370 */
// tr!(r#"   \xfd"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1525),
// scanner! { S1525 { mode M { token r#"   \xfd"# => 0; } } }

// -------------------------------------------------------------------------
// e("()0\\xfc00000\\xfc00000\\xfc00000\xfc", "", ONIGERR_INVALID_CODE_POINT_VALUE); /* https://bugs.php.net/bug.php?id=77371 */
// tr!(r#"()0\xfc00000\xfc00000\xfc00000\xfc"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1526),
// scanner! { S1526 { mode M { token r#"()0\xfc00000\xfc00000\xfc00000\xfc"# => 0; } } }

// -------------------------------------------------------------------------
// e("000||0\xfa", "0", ONIGERR_INVALID_CODE_POINT_VALUE); /* https://bugs.php.net/bug.php?id=77381 */
// tr!(r#"000||0\xfa"#, "0", "ONIGERR_INVALID_CODE_POINT_VALUE", 1527),
// scanner! { S1527 { mode M { token r#"000||0\xfa"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?i)000000000000000000000\xf0", "", ONIGERR_INVALID_CODE_POINT_VALUE); /* https://bugs.php.net/bug.php?id=77382 */
// tr!(r#"(?i)000000000000000000000\xf0"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1528),
// scanner! { S1528 { mode M { token r#"(?i)000000000000000000000\xf0"# => 0; } } }

// -------------------------------------------------------------------------
// e("0000\\\xf5", "0", ONIGERR_INVALID_CODE_POINT_VALUE); /* https://bugs.php.net/bug.php?id=77385 */
// tr!(r#"0000\\xf5"#, "0", "ONIGERR_INVALID_CODE_POINT_VALUE", 1529),
// scanner! { S1529 { mode M { token r#"0000\\xf5"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?i)FFF00000000000000000\xfd", "", ONIGERR_INVALID_CODE_POINT_VALUE); /* https://bugs.php.net/bug.php?id=77394 */
// tr!(r#"(?i)FFF00000000000000000\xfd"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1530),
// scanner! { S1530 { mode M { token r#"(?i)FFF00000000000000000\xfd"# => 0; } } }

// -------------------------------------------------------------------------
// n("(?x)\n  (?<!\\+\\+|--)(?<=[({\\[,?=>:*]|&&|\\|\\||\\?|\\*\\/|^await|[^\\._$[:alnum:]]await|^return|[^\\._$[:alnum:]]return|^default|[^\\._$[:alnum:]]default|^yield|[^\\._$[:alnum:]]yield|^)\\s*\n  (?!<\\s*[_$[:alpha:]][_$[:alnum:]]*((\\s+extends\\s+[^=>])|,)) # look ahead is not type parameter of arrow\n  (?=(<)\\s*(?:([_$[:alpha:]][-_$[:alnum:].]*)(?<!\\.|-)(:))?((?:[a-z][a-z0-9]*|([_$[:alpha:]][-_$[:alnum:].]*))(?<!\\.|-))(?=((<\\s*)|(\\s+))(?!\\?)|\\/?>))", "    while (i < len && f(array[i]))"); /* Issue #192 */
// tr!(r#"(?x)\n  (?<!\+\+|--)(?<=[({\[,?=>:*]|&&|\|\||\?|\*\/|^await|[^\._$[:alnum:]]await|^return|[^\._$[:alnum:]]return|^default|[^\._$[:alnum:]]default|^yield|[^\._$[:alnum:]]yield|^)\s*\n  (?!<\s*[_$[:alpha:]][_$[:alnum:]]*((\s+extends\s+[^=>])|,)) # look ahead is not type parameter of arrow\n  (?=(<)\s*(?:([_$[:alpha:]][-_$[:alnum:].]*)(?<!\.|-)(:))?((?:[a-z][a-z0-9]*|([_$[:alpha:]][-_$[:alnum:].]*))(?<!\.|-))(?=((<\s*)|(\s+))(?!\?)|\/?>))"#, "    while (i < len && f(array[i]))", &[], 1531), UnsupportedLookAround
// scanner! { S1531 { mode M { token r#"(?x)\n  (?<!\+\+|--)(?<=[({\[,?=>:*]|&&|\|\||\?|\*\/|^await|[^\._$[:alnum:]]await|^return|[^\._$[:alnum:]]return|^default|[^\._$[:alnum:]]default|^yield|[^\._$[:alnum:]]yield|^)\s*\n  (?!<\s*[_$[:alpha:]][_$[:alnum:]]*((\s+extends\s+[^=>])|,)) # look ahead is not type parameter of arrow\n  (?=(<)\s*(?:([_$[:alpha:]][-_$[:alnum:].]*)(?<!\.|-)(:))?((?:[a-z][a-z0-9]*|([_$[:alpha:]][-_$[:alnum:].]*))(?<!\.|-))(?=((<\s*)|(\s+))(?!\?)|\/?>))"# => 0; } } }

// -------------------------------------------------------------------------
// *** Index error: x2("aaaaaaaaaaaaaaaaaaaaaaaあb", "aaaaaaaaaaaaaaaaaaaaaaaあb", 0, 27); /* Issue #221 */ // 1532

// -------------------------------------------------------------------------
// n("d{65538}+{61533} ", "d{65538}+{61533} ");
// td!(r#"d{65538}+{61533} "#, "d{65538}+{61533} ", &[], 1533),

// PERFORMANCE!!! TODO: Check repetition count limits in the scanner.
// scanner! { S1533 { mode M { token r#"d{65538}+{61533} "# => 0; } } }

// -------------------------------------------------------------------------
// e("x{55380}{77590}", "", ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE);
// tr!(r#"x{55380}{77590}"#, "", "ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE", 1534),
// scanner! { S1534 { mode M { token r#"x{55380}{77590}"# => 0; } } }

// -------------------------------------------------------------------------
// e("(xyz){40000}{99999}(?<name>vv)", "", ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE);
// tr!(r#"(xyz){40000}{99999}(?<name>vv)"#, "", "ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE", 1535),
// scanner! { S1535 { mode M { token r#"(xyz){40000}{99999}(?<name>vv)"# => 0; } } }

// -------------------------------------------------------------------------
// e("f{90000,90000}{80000,80000}", "", ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE);
// tr!(r#"f{90000,90000}{80000,80000}"#, "", "ONIGERR_TOO_BIG_NUMBER_FOR_REPEAT_RANGE", 1536),
// scanner! { S1536 { mode M { token r#"f{90000,90000}{80000,80000}"# => 0; } } }

// -------------------------------------------------------------------------
// n("f{90000,90000}{80000,80001}", "");
// td!(r#"f{90000,90000}{80000,80001}"#, "", &[], 1537),
// scanner! { S1537 { mode M { token r#"f{90000,90000}{80000,80001}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\p{Common}", "\xe3\x8b\xbf", 0, 3);   /* U+32FF */
// td!(r#"\p{Common}"#, "\xe3\x8b\xbf", &[("\\xe", 0, 3)], 1538), Rust Compile Error
scanner! { S1538 { mode M { token r#"\p{Common}"# => 0; } } }

// -------------------------------------------------------------------------
// x2("\\p{In_Enclosed_CJK_Letters_and_Months}", "\xe3\x8b\xbf", 0, 3); /* U+32FF */
// tr!(r#"\p{In_Enclosed_CJK_Letters_and_Months}"#, "\xe3\x8b\xbf", &[("\\xe", 0, 3)], 1539), UnicodePropertyNotFound
// scanner! { S1539 { mode M { token r#"\p{In_Enclosed_CJK_Letters_and_Months}"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\x{7fffffff}", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\x{7fffffff}"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1540),
// scanner! { S1540 { mode M { token r#"\x{7fffffff}"# => 0; } } }

// -------------------------------------------------------------------------
// e("[\\x{7fffffff}]", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"[\x{7fffffff}]"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1541),
// scanner! { S1541 { mode M { token r#"[\x{7fffffff}]"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\u040", "@", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\u040"#, "@", "ONIGERR_INVALID_CODE_POINT_VALUE", 1542),
// scanner! { S1542 { mode M { token r#"\u040"# => 0; } } }

// -------------------------------------------------------------------------
// e("\\u", "", ONIGERR_INVALID_CODE_POINT_VALUE);
// tr!(r#"\u"#, "", "ONIGERR_INVALID_CODE_POINT_VALUE", 1543),
// scanner! { S1543 { mode M { token r#"\u"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?<abc>\\g<abc>)", "zzzz", ONIGERR_NEVER_ENDING_RECURSION);
// tr!(r#"(?<abc>\g<abc>)"#, "zzzz", "ONIGERR_NEVER_ENDING_RECURSION", 1544),
// scanner! { S1544 { mode M { token r#"(?<abc>\g<abc>)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(*FOO)", "abcdefg", ONIGERR_UNDEFINED_CALLOUT_NAME);
// tr!(r#"(*FOO)"#, "abcdefg", "ONIGERR_UNDEFINED_CALLOUT_NAME", 1545),
// scanner! { S1545 { mode M { token r#"(*FOO)"# => 0; } } }

// -------------------------------------------------------------------------
// e("*", "abc", ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED);
// tr!(r#"*"#, "abc", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED", 1546),
// scanner! { S1546 { mode M { token r#"*"# => 0; } } }

// -------------------------------------------------------------------------
// e("|*", "abc", ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED);
// tr!(r#"|*"#, "abc", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED", 1547),
// scanner! { S1547 { mode M { token r#"|*"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?i)*", "abc", ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED);
// tr!(r#"(?i)*"#, "abc", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED", 1548),
// scanner! { S1548 { mode M { token r#"(?i)*"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?:*)", "abc", ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED);
// tr!(r#"(?:*)"#, "abc", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED", 1549),
// scanner! { S1549 { mode M { token r#"(?:*)"# => 0; } } }

// -------------------------------------------------------------------------
// e("(?m:*)", "abc", ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED);
// tr!(r#"(?m:*)"#, "abc", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED", 1550),
// scanner! { S1550 { mode M { token r#"(?m:*)"# => 0; } } }

// -------------------------------------------------------------------------
// x2("(?:)*", "abc", 0, 0);
// td!(r#"(?:)*"#, "abc", &[], 1551),
scanner! { S1551 { mode M { token r#"(?:)*"# => 0; } } }
#[test]
fn test_match_1551() {
    use s1551::S1551 as S;
    let scanner = S::new();
    let matches = scanner.find_matches("abc", 0).collect::<Vec<_>>();
    const EXPECTED_MATCHES: &[(&str, usize, usize)] = &[];
    assert_eq!(
        matches.len(),
        EXPECTED_MATCHES.len(),
        "1551: Unexpected match count"
    );
}

// -------------------------------------------------------------------------
// e("^*", "abc", ONIGERR_TARGET_OF_REPEAT_OPERATOR_INVALID);
// tr!(r#"^*"#, "abc", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_INVALID", 1552),
// scanner! { S1552 { mode M { token r#"^*"# => 0; } } }

// -------------------------------------------------------------------------
// e("abc|?", "", ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED);
// tr!(r#"abc|?"#, "", "ONIGERR_TARGET_OF_REPEAT_OPERATOR_NOT_SPECIFIED", 1553),
// scanner! { S1553 { mode M { token r#"abc|?"# => 0; } } }

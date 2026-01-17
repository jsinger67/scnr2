import scnr2

def test_basic():
    definition = """
    MyScanner {
        mode INITIAL {
            token r"\\d+" => 1;
            token r"[a-z]+" => 2;
            token r"\\s+" => 3;
        }
    }
    """
    scanner = scnr2.Scanner(definition)
    input_text = "123 abc 456"
    matches = scanner.find_matches(input_text)
    print(f"Basic Input: {repr(input_text)}")
    for m in matches:
        print(f"  Token {m.token_type}: {repr(m.text)} at [{m.start}:{m.end}]")
        # For find_matches, positions should be None
        assert m.start_line is None
    assert len(matches) == 5

def test_lookahead():
    # Use non-overlapping patterns to avoid shadowing in this test
    definition = """
    LookaheadScanner {
        mode INITIAL {
            token r"a" followed by r"b" => 1;
            token r"c" => 2;
        }
    }
    """
    scanner = scnr2.Scanner(definition)
    
    # "ab" -> matches "a" as token 1 (because followed by "b")
    # Note: scnr2 currently includes lookahead in the span
    input1 = "ab"
    matches1 = scanner.find_matches(input1)
    print(f"Lookahead Input 1: {repr(input1)}")
    for m in matches1:
        print(f"  Token {m.token_type}: {repr(m.text)} at [{m.start}:{m.end}]")
    assert len(matches1) > 0
    assert matches1[0].token_type == 1
    
    # "ac" -> no match for token 1
    input2 = "ac"
    matches2 = scanner.find_matches(input2)
    print(f"Lookahead Input 2: {repr(input2)}")
    for m in matches2:
        print(f"  Token {m.token_type}: {repr(m.text)} at [{m.start}:{m.end}]")
    assert len(matches2) == 1
    assert matches2[0].token_type == 2

def test_positions():
    definition = """
    PosScanner {
        mode INITIAL {
            token r"\\d+" => 1;
            token r"[a-z]+" => 2;
            token r"\\n" => 3;
            token r" " => 4;
        }
    }
    """
    scanner = scnr2.Scanner(definition)
    input_text = "123\nabc"
    matches = scanner.find_matches_with_position(input_text)
    print(f"Position Input: {repr(input_text)}")
    for m in matches:
        print(f"  Token {m.token_type}: {repr(m.text)} at [{m.start}:{m.end}] "
              f"({m.start_line}:{m.start_column} - {m.end_line}:{m.end_column})")
    
    # "123" at [0:3] (1:1 - 1:4)
    assert matches[0].text == "123"
    assert matches[0].start_line == 1
    assert matches[0].start_column == 1
    assert matches[0].end_line == 1
    assert matches[0].end_column == 4

    # "\n" at [3:4] (1:4 - 2:1)
    assert matches[1].text == "\n"
    assert matches[1].start_line == 1
    assert matches[1].start_column == 4
    assert matches[1].end_line == 2
    assert matches[1].end_column == 1

    # "abc" at [4:7] (2:1 - 2:4)
    assert matches[2].text == "abc"
    assert matches[2].start_line == 2
    assert matches[2].start_column == 1
    assert matches[2].end_line == 2
    assert matches[2].end_column == 4

if __name__ == "__main__":
    print("Testing Basic...")
    test_basic()
    print("Testing Lookahead...")
    test_lookahead()
    print("Testing Positions...")
    test_positions()
    print("All tests passed!")

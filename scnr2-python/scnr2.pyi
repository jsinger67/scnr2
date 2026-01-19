from typing import List, Optional

class TokenMatch:
    """Represents a single token match."""
    token_type: int
    start: int
    end: int
    text: str
    start_line: Optional[int]
    start_column: Optional[int]
    end_line: Optional[int]
    end_column: Optional[int]

class Scanner:
    """A scanner engine built from a scnr2 definition."""
    def __init__(self, definition: str) -> None:
        """Create a new scanner from a scnr2 definition string."""
        ...

    def find_matches(self, input: str) -> List[TokenMatch]:
        """Finds all matches in the input string."""
        ...

    def find_matches_with_position(self, input: str) -> List[TokenMatch]:
        """Finds all matches in the input string, including line and column information."""
        ...

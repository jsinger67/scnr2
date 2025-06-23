use crate::scanner_data::TransitionToNumericMode;

use super::pattern::Pattern;

/// A scanner mode that can be used to scan specific parts of the input.
/// It has a name and a set of patterns that are valid token types in this mode.
/// The scanner mode can also have transitions to other scanner modes triggered by a token type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannerMode {
    /// The name of the scanner mode.
    pub name: String,
    /// The regular expressions that are valid token types in this mode, bundled with their token
    /// type numbers.
    /// The priorities of the patterns are determined by their order in the vector. Lower indices
    /// have higher priority if multiple patterns match the input and have the same length.
    pub patterns: Vec<Pattern>,

    /// The transitions between the scanner modes triggered by a token type number.
    /// The entries are tuples of the token type numbers and the new scanner mode index and are
    /// sorted by token type number.
    pub transitions: Vec<TransitionToNumericMode>,
}

impl ScannerMode {
    /// Creates a new scanner mode with the given name and patterns.
    /// # Arguments
    /// * `name` - The name of the scanner mode.
    /// * `patterns` - The regular expressions that are valid token types in this mode, bundled with
    ///   their token type numbers.
    /// * `mode_transitions` - The transitions between the scanner modes triggered by a token type
    ///   number. It is a vector of tuples of the token type numbers and the new scanner mode
    ///   index. The entries should be sorted by token type number.
    ///   The scanner mode index is the index of the scanner mode in the scanner mode vector of
    ///   the scanner and is determined by the order of the insertions of scanner modes into the
    ///   scanner.
    /// # Returns
    /// The new scanner mode.
    pub fn new<P, T>(name: &str, patterns: P, mode_transitions: T) -> Self
    where
        P: IntoIterator<Item = Pattern>,
        T: IntoIterator<Item = TransitionToNumericMode>,
    {
        let patterns = patterns.into_iter().collect::<Vec<_>>();
        let transitions = mode_transitions.into_iter().collect::<Vec<_>>();
        debug_assert!(
            transitions
                .windows(2)
                .all(|w| w[0].token_type() < w[1].token_type()),
            "Transitions are not sorted by token type number."
        );
        Self {
            name: name.to_string(),
            patterns,
            transitions,
        }
    }
}

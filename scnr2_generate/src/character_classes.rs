//! Module with data structures and algorithms to handle character classes for SCNR2 generation
use std::{char, ops::RangeInclusive};

use regex_syntax::hir::HirKind;

use crate::ids::{CharClassIDBase, DisjointCharClassID};

/// Represents a character class with its associated characters and properties.
#[derive(Debug, Clone)]
pub struct CharacterClass {
    /// The kind of characters in the class, represented as a regex syntax HIR kind.
    /// It contains the characters that belong to this class.
    pub characters: HirKind,
    /// A list of indices of elementary character ranges that define the characters in this class.
    /// This is the result of calculating disjoint character classes.
    /// Each index corresponds to an elementary interval in the `CharacterClasses` set.
    pub intervals: Vec<DisjointCharClassID>,
}

impl CharacterClass {
    /// Creates a new `CharacterClass` with the given characters and properties.
    pub fn new(characters: HirKind) -> Self {
        Self {
            characters,
            intervals: Vec::new(),
        }
    }

    /// Checks if the character class contains the given interval.
    fn contains_interval(&self, interval: &std::ops::RangeInclusive<char>) -> bool {
        match &self.characters {
            regex_syntax::hir::HirKind::Empty => true, // An empty Hir matches everything.
            regex_syntax::hir::HirKind::Literal(literal) => {
                // Literals here are separated into single characters.
                let bytes = literal.0.clone();
                // We convert the first 4 bytes to a u32.
                // If the literal is smaller than 4 bytes, take will ensure we only take the bytes
                // that exist.
                let lit: u32 = bytes
                    .iter()
                    .take(4)
                    .fold(0, |acc, &b| (acc << 8) | b as u32);
                let c = char::from_u32(lit).unwrap_or('\0');
                *interval == std::ops::RangeInclusive::new(c, c)
            }
            regex_syntax::hir::HirKind::Class(class) => {
                // Check if the class contains any character in the interval.
                match class {
                    regex_syntax::hir::Class::Unicode(class) => {
                        // Create a ClassUnicodeRange from our RangeInclusive<char>
                        let class_unicode_range = regex_syntax::hir::ClassUnicodeRange::new(
                            *interval.start(),
                            *interval.end(),
                        );

                        let class_from_interval =
                            regex_syntax::hir::ClassUnicode::new(vec![class_unicode_range]);
                        let mut intersection = class.clone();
                        intersection.intersect(&class_from_interval);
                        intersection == class_from_interval
                    }
                    regex_syntax::hir::Class::Bytes(class) =>
                    // For byte classes, we assume they are similar.
                    {
                        // Create a ClassBytesRange from our RangeInclusive<char>
                        let class_bytes_range = regex_syntax::hir::ClassBytesRange::new(
                            *interval.start() as u8,
                            *interval.end() as u8,
                        );
                        let class_from_interval =
                            regex_syntax::hir::ClassBytes::new(vec![class_bytes_range]);
                        let mut intersection = class.clone();
                        intersection.intersect(&class_from_interval);
                        intersection == class_from_interval
                    }
                }
            }
            _ => false, // We assume other Hir kinds do not match any character.
        }
    }

    /// Adds a disjoint interval to the character class.
    fn add_disjoint_interval(&mut self, interval_index: DisjointCharClassID) {
        // Add the interval to the class only if it is not already present
        if self.intervals.contains(&interval_index) {
            return; // Interval already exists, no need to add it again
        }
        self.intervals.push(interval_index);
    }
}

/// Represents a set of character classes
/// It is used to calculate disjoint character classes
#[derive(Debug, Default, Clone)]
pub struct CharacterClasses {
    /// The set of character classes
    pub classes: Vec<CharacterClass>,

    /// Used for generating disjoint character classes and code generation.
    pub elementary_intervals: Vec<RangeInclusive<char>>,

    /// Groups of elementary intervals where each group contains intervals
    /// that belong to exactly the same set of character classes.
    pub intervals: Vec<Vec<RangeInclusive<char>>>,
}

impl CharacterClasses {
    /// Creates a new `CharacterClassSet` with an empty set of character classes.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a character class to the set.
    pub(crate) fn add_hir(&mut self, class: HirKind) {
        if self.classes.iter().any(|c| c.characters == class) {
            return; // Class already exists, no need to add it again
        }
        let new_class = CharacterClass::new(class);
        // If the class is a character class, we can add its intervals directly
        self.classes.push(new_class);
    }

    /// Creates disjoint character classes from the NFA states and lookahead patterns.
    /// This function collects all character classes from the NFA states and lookahead patterns,
    /// then generates disjoint intervals for each character class.
    pub fn create_disjoint_character_classes(&mut self) {
        // Step 1: Collect all boundary points
        // The boundaries are collected in a BTreeSet to ensure they are unique and sorted.
        let mut boundaries = std::collections::BTreeSet::new();
        for character_class in self.classes.iter() {
            match &character_class.characters {
                regex_syntax::hir::HirKind::Literal(literal) => {
                    // Literals here are separated into single characters.
                    let bytes = literal.0.clone();
                    // We convert the first 4 bytes to a u32.
                    // If the literal is smaller than 4 bytes, take will ensure we only take the bytes
                    // that exist.
                    let lit: u32 = bytes
                        .iter()
                        .take(4)
                        .fold(0, |acc, &b| (acc << 8) | b as u32);
                    if let Some(c) = char::from_u32(lit) {
                        boundaries.insert(c);
                        // Add the character after the end as a boundary to create half-open
                        // intervals
                        boundaries.insert(char::from_u32(lit + 1).unwrap_or(char::MAX));
                    }
                }
                regex_syntax::hir::HirKind::Class(class) => match class {
                    regex_syntax::hir::Class::Unicode(unicode) => {
                        for range in unicode.ranges() {
                            boundaries.insert(range.start());
                            // Add the character after the end as a boundary to create half-open
                            // intervals
                            if let Some(next_char) = char::from_u32(range.end() as u32 + 1) {
                                boundaries.insert(next_char);
                            } else {
                                // Handle the case where end() is the last Unicode character
                                boundaries.insert(char::MAX);
                            }
                        }
                    }
                    regex_syntax::hir::Class::Bytes(bytes) => {
                        for range in bytes.ranges() {
                            boundaries.insert(range.start() as char);
                            // Add the character after the end as a boundary to create half-open
                            // intervals
                            if let Some(next_char) = char::from_u32(range.end() as u32 + 1) {
                                boundaries.insert(next_char);
                            } else {
                                // Handle the case where end() is the last byte
                                boundaries.insert(char::MAX);
                            }
                        }
                    }
                },
                _ => {
                    unreachable!(
                        "Only Literal and Class are expected in character classes, found: {:?}",
                        character_class.characters
                    );
                }
            }
        }
        let boundaries: Vec<char> = boundaries.into_iter().collect();

        // Step 2: Generate elementary intervals from the boundaries
        self.elementary_intervals = Vec::new();
        for i in 0..boundaries.len().saturating_sub(1) {
            let start = boundaries[i];
            if let Some(end) = char::from_u32(boundaries[i + 1] as u32 - 1) {
                if start <= end {
                    let interval = start..=end;
                    // Only add if any character class matches it
                    if self
                        .classes
                        .iter()
                        .any(|hir| hir.contains_interval(&interval))
                    {
                        self.elementary_intervals.push(interval);
                    }
                }
            } else {
                let interval = start..=start;
                if self
                    .classes
                    .iter()
                    .any(|hir| hir.contains_interval(&interval))
                {
                    self.elementary_intervals.push(interval);
                }
            }
        }

        self.elementary_intervals
            .sort_by(|a, b| a.start().cmp(b.start()));

        // Step 3: Map each elementary interval to its character class membership
        let mut interval_memberships = Vec::new();
        for interval in &self.elementary_intervals {
            let mut membership = Vec::new();
            for (class_idx, class) in self.classes.iter().enumerate() {
                if class.contains_interval(interval) {
                    membership.push(class_idx);
                }
            }
            interval_memberships.push(membership);
        }

        // Step 4: Group intervals with identical membership

        // A vector to hold grouped intervals
        // Each group will contain intervals that share the same membership pattern
        // No single interval will be in more than one group.
        let mut grouped_intervals: Vec<Vec<RangeInclusive<char>>> = Vec::new();
        // A map to track which membership pattern corresponds to which group index
        // This map will help us avoid creating duplicate groups for the same membership pattern
        // The key is a vector of class indices representing the membership pattern
        let mut membership_to_group_idx: std::collections::HashMap<Vec<usize>, usize> =
            std::collections::HashMap::new();

        for (interval, membership) in self
            .elementary_intervals
            .clone()
            .into_iter()
            .zip(interval_memberships)
        {
            let membership_key = membership.clone();

            let group_idx = if let Some(&group_idx) = membership_to_group_idx.get(&membership_key) {
                // This membership pattern already exists
                grouped_intervals[group_idx].push(interval);
                group_idx
            } else {
                // New membership pattern
                let new_idx = grouped_intervals.len();
                membership_to_group_idx.insert(membership_key.clone(), new_idx);
                grouped_intervals.push(vec![interval]);
                new_idx
            };

            // Update class intervals - assign each class the index of its group
            for class_idx in membership {
                let disjoint_id = group_idx as CharClassIDBase;
                self.classes[class_idx].add_disjoint_interval(disjoint_id.into());
            }
        }

        // Update the intervals field with our grouped intervals
        self.intervals = grouped_intervals;
    }

    /// Retrieves the disjoint character classes for a given `HirKind`.
    pub(crate) fn get_disjoint_classes(&self, hir_kind: &HirKind) -> Vec<DisjointCharClassID> {
        // Find the character class that matches the given HirKind
        if let Some(class) = self.classes.iter().find(|c| c.characters == *hir_kind) {
            class.intervals.clone()
        } else {
            Vec::new()
        }
    }

    /// Generates a function that checks if a character belongs to a specific character class.
    pub(crate) fn generate(&self, name: &str) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(name, proc_macro2::Span::call_site());
        // if self.intervals.is_empty() {
        //     panic!(
        //         "No disjoint character classes found. Did you call `create_disjoint_character_classes`?"
        //     );
        // }
        // Generate elementary intervals
        let intervals = self
            .elementary_intervals
            .iter()
            .map(|interval| {
                let start = interval.start();
                let end = interval.end();
                let char_class_id = self
                    .intervals
                    .iter()
                    .enumerate()
                    .find(|(_, group)| group.contains(interval))
                    .map(|(idx, _)| idx)
                    .expect("Interval should belong to a group");
                if start == end {
                    quote::quote! { (#start..=#start, #char_class_id) }
                } else {
                    quote::quote! { (#start..=#end, #char_class_id) }
                }
            })
            .collect::<Vec<_>>();

        quote::quote! {
            #[allow(clippy::manual_is_ascii_check, dead_code)]
            pub(crate) fn #name(c: char) -> Option<usize> {
                use std::cmp::Ordering;

                // Define elementary intervals
                static INTERVALS: &[(std::ops::RangeInclusive<char>, usize)] = &[
                    #(#intervals),*
                ];

                // Binary search to find the interval containing the character
                let interval_idx = match INTERVALS.binary_search_by(|interval| {
                    if c < *interval.0.start() {
                        Ordering::Greater
                    } else if c > *interval.0.end() {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }) {
                    Ok(idx) => idx,
                    Err(_) => return None,
                };

                INTERVALS[interval_idx].1.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // scanner! {
    //     TestScanner {
    //         mode INITIAL {
    //             token r"\r\n|\r|\n" => 1 not followed by r"!";
    //             token r"[\s--\r\n]+" => 2;
    //             token r#","# => 5;
    //             token r"0|[1-9][0-9]*" => 6;
    //         }
    //     }
    // }

    // This function was generated by the macro above and is used to test the generated function.
    // Todo: Test the actual generated function.
    #[allow(clippy::manual_is_ascii_check, dead_code)]
    pub(crate) fn match_function(c: char) -> Option<usize> {
        use std::cmp::Ordering;
        static INTERVALS: &[(std::ops::RangeInclusive<char>, usize)] = &[
            ('\t'..='\t', 0),
            ('\n'..='\n', 1),
            ('\u{b}'..='\u{c}', 0),
            ('\r'..='\r', 2),
            (' '..=' ', 0),
            ('!'..='!', 3),
            (','..=',', 4),
            ('0'..='0', 5),
            ('1'..='9', 6),
            ('\u{85}'..='\u{85}', 0),
            ('\u{a0}'..='\u{a0}', 0),
            ('\u{1680}'..='\u{1680}', 0),
            ('\u{2000}'..='\u{200a}', 0),
            ('\u{2028}'..='\u{2029}', 0),
            ('\u{202f}'..='\u{202f}', 0),
            ('\u{205f}'..='\u{205f}', 0),
            ('\u{3000}'..='\u{3000}', 0),
        ];
        let interval_idx = match INTERVALS.binary_search_by(|interval| {
            if c < *interval.0.start() {
                Ordering::Greater
            } else if c > *interval.0.end() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(idx) => idx,
            Err(_) => return None,
        };
        INTERVALS[interval_idx].1.into()
    }

    #[test]
    fn test_match_function() {
        // Test the generated function with various characters
        assert_eq!(match_function('\t'), Some(0));
        assert_eq!(match_function('\n'), Some(1));
        assert_eq!(match_function(' '), Some(0));
        assert_eq!(match_function(','), Some(4));
        assert_eq!(match_function('0'), Some(5));
        assert_eq!(match_function('1'), Some(6));
        assert_eq!(match_function('9'), Some(6));
        assert_eq!(match_function('!'), Some(3));
        assert_eq!(match_function('a'), None); // Not in any class
    }
}

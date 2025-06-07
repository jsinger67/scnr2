//! Module with data structures and algorithms to handle character classes for SCNR2 generation
use std::ops::RangeInclusive;

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
                interval.contains(&c)
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
        // Check that the interval is not already present
        // This is a debug assertion to ensure that we do not add the same interval twice.
        // It is normally not expected to fail, but it is a good sanity check.
        debug_assert!(!self.intervals.iter().any(|i| {
            // Check if the interval index is already present in the intervals
            *i == interval_index
        }));

        // Add the interval to the class
        self.intervals.push(interval_index);
    }

    pub(crate) fn generate(&self) -> proc_macro2::TokenStream {
        match &self.characters {
            regex_syntax::hir::HirKind::Empty => {
                quote::quote! {
                    |_c| {
                        // An empty Hir matches everything.
                        true
                    }
                }
            }
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
                quote::quote! {
                     &[#c..=#c]
                }
            }
            regex_syntax::hir::HirKind::Class(class) => match class {
                regex_syntax::hir::Class::Unicode(class_unicode) => {
                    let ranges = class_unicode.ranges().iter().fold(
                        proc_macro2::TokenStream::new(),
                        |mut acc, r| {
                            if !acc.is_empty() {
                                acc.extend(quote::quote! {
                                    ,
                                });
                            }
                            let start: char = r.start();
                            let end: char = r.end();
                            if start == end {
                                acc.extend(quote::quote! {
                                    #start..=#start
                                });
                            } else {
                                acc.extend(quote::quote! {
                                    #start..=#end
                                });
                            }
                            acc
                        },
                    );
                    quote::quote! {
                        {
                            &[
                                #ranges
                            ]
                        }
                    }
                }
                regex_syntax::hir::Class::Bytes(class_bytes) => {
                    let ranges = class_bytes.ranges().iter().fold(
                        proc_macro2::TokenStream::new(),
                        |mut acc, r| {
                            if !acc.is_empty() {
                                acc.extend(quote::quote! {
                                    ,
                                });
                            }
                            let start: char = r.start().into();
                            let end: char = r.end().into();
                            if start == end {
                                acc.extend(quote::quote! {
                                    #start..=#start
                                });
                            } else {
                                acc.extend(quote::quote! {
                                    #start..=#end
                                });
                            }
                            acc
                        },
                    );
                    quote::quote! {
                        |c| {
                            &[
                                #ranges
                            ]
                        }
                    }
                }
            },
            _ => {
                panic!("Unsupported Hir kind: {:?}", self.characters)
            }
        }
    }
}

/// Represents a set of character classes
/// It is used to calculate disjoint character classes
#[derive(Debug, Default, Clone)]
pub struct CharacterClasses {
    /// The set of character classes
    pub classes: Vec<CharacterClass>,

    /// The list of elementary character ranges that define the characters in this set as
    /// disjoint intervals. They are set during the calculation of disjoint character classes.
    pub intervals: Vec<RangeInclusive<char>>,
}

impl CharacterClasses {
    /// Creates a new `CharacterClassSet` with an empty set of character classes.
    pub(crate) fn new() -> Self {
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
    pub(crate) fn create_disjoint_character_classes(&mut self) {
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
        // Elementary intervals are ranges between consecutive boundaries.
        for i in 0..boundaries.len() - 1 {
            let start = boundaries[i];
            // Get character before next boundary.
            // If the next boundary is out of range, use the current character.
            if let Some(end) = char::from_u32(boundaries[i + 1] as u32 - 1) {
                if start <= end {
                    // Create a closed interval [start, end] again
                    // Insert the interval into the elementary intervals only if any character class
                    // matches it.
                    if self
                        .classes
                        .iter()
                        .any(|hir| hir.contains_interval(&(start..=end)))
                    {
                        // We use inclusive ranges to represent the intervals.
                        self.intervals.push(start..=end);
                    }
                }
            } else {
                // If the next boundary is not a valid character, we use the current character
                // as the end of the interval.
                // Insert the interval into the elementary intervals only if any character class
                // matches it.
                if self
                    .classes
                    .iter()
                    .any(|hir| hir.contains_interval(&(start..=start)))
                {
                    // We use inclusive ranges to represent the intervals.
                    self.intervals.push(start..=start);
                }
            }
        }

        // Step 3: Add disjoint intervals to each character class
        for class in self.classes.iter_mut() {
            for (idx, interval) in self.intervals.iter_mut().enumerate() {
                // Check if the character class matches the interval
                if class.contains_interval(interval) {
                    class.add_disjoint_interval((idx as CharClassIDBase).into());
                }
            }
        }
    }

    /// Retrieves the disjoint character classes for a given `HirKind`.
    pub(crate) fn get_disjoint_classes(&self, hir_kind: &HirKind) -> &Vec<DisjointCharClassID> {
        // Find the character class that matches the given HirKind
        if let Some(class) = self.classes.iter().find(|c| c.characters == *hir_kind) {
            // Return the indices of the disjoint intervals for this class
            &class.intervals
        } else {
            // If no matching class is found, return an empty vector
            panic!(
                "No disjoint character class found for HirKind: {:?}",
                hir_kind
            );
        }
    }

    pub(crate) fn generate(&self, name: &str) -> proc_macro2::TokenStream {
        let name = syn::Ident::new(name, proc_macro2::Span::call_site()); // Convert name to an Ident
        let mut match_functions = Vec::new();
        for cc in &self.classes {
            match_functions.push(cc.generate());
        }
        quote::quote! {
            #[allow(clippy::manual_is_ascii_check, dead_code)]
            pub(crate) fn #name(char_class: usize, c: char) -> bool {
                use std::cmp::Ordering;

                // Define a table of closures for each char_class
                static CHAR_CLASS_TABLE: &[&[std::ops::RangeInclusive<char>]] = &[
                                #(
                                    #match_functions,
                                )*
                ];

                // Check if char_class is within bounds
                if let Some(ranges) = CHAR_CLASS_TABLE.get(char_class) {
                    ranges.binary_search_by(|range| {
                        if c < *range.start() {
                            Ordering::Greater
                        } else if c > *range.end() {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    }).is_ok()
                } else {
                    false
                }
            }
        }
    }
}

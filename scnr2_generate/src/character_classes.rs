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
        self.elementary_intervals = Vec::new();
        for i in 0..boundaries.len() - 1 {
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

        // Step 4: Group adjacent intervals with identical membership
        let mut grouped_intervals: Vec<Vec<RangeInclusive<char>>> = Vec::new();
        let mut membership_to_group_idx: std::collections::HashMap<Vec<usize>, usize> =
            std::collections::HashMap::new();

        for (interval, membership) in self
            .elementary_intervals
            .clone()
            .into_iter()
            .zip(interval_memberships)
        {
            let membership_key = membership.clone();

            if let Some(&group_idx) = membership_to_group_idx.get(&membership_key) {
                // This membership pattern already exists
                grouped_intervals[group_idx].push(interval);
            } else {
                // New membership pattern
                membership_to_group_idx.insert(membership_key, grouped_intervals.len());
                grouped_intervals.push(vec![interval]);
            }

            // Update class intervals - assign each class the index of its group
            for class_idx in membership {
                let disjoint_id = (grouped_intervals.len() - 1) as CharClassIDBase;
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
        if self.intervals.is_empty() {
            panic!(
                "No disjoint character classes found. Did you call `create_disjoint_character_classes`?"
            );
        }
        // Generate elementary intervals
        let intervals = self
            .elementary_intervals
            .iter()
            .map(|interval| {
                let start = interval.start();
                let end = interval.end();
                if start == end {
                    quote::quote! { #start..=#start }
                } else {
                    quote::quote! { #start..=#end }
                }
            })
            .collect::<Vec<_>>();

        // Generate grouped intervals, generate the index in the elementary_intervals of intervals
        let grouped_intervals = self
            .intervals
            .iter()
            .map(|intervals| {
                let interval_tokens = intervals
                    .iter()
                    .map(|interval| {
                        // Find the index of the interval in elementary_intervals
                        let index = self
                            .elementary_intervals
                            .iter()
                            .position(|e| e == interval)
                            .expect("Interval not found in elementary intervals");
                        quote::quote! { #index }
                    })
                    .collect::<Vec<_>>();

                quote::quote! {
                    &[
                        #(#interval_tokens),*
                    ]
                }
            })
            .collect::<Vec<_>>();

        quote::quote! {
            #[allow(clippy::manual_is_ascii_check, dead_code)]
            pub(crate) fn #name(c: char) -> Option<usize> {
                use std::cmp::Ordering;

                // Define elementary intervals
                static INTERVALS: &[std::ops::RangeInclusive<char>] = &[
                    #(#intervals),*
                ];

                // Define grouped intervals
                static GROUPED_INTERVALS: &[&[usize]] = &[
                    #(#grouped_intervals),*
                ];

                // Binary search to find the interval containing the character
                let interval_idx = match INTERVALS.binary_search_by(|interval| {
                    if c < *interval.start() {
                        Ordering::Greater
                    } else if c > *interval.end() {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }) {
                    Ok(idx) => idx,
                    Err(_) => return None,
                };

                // Binary search to find the group that might contain the interval
                // Since groups are sorted by first entry, we can use binary search to narrow down
                let mut left = 0;
                let mut right = GROUPED_INTERVALS.len() - 1;

                while left <= right {
                    let mid = left + (right - left) / 2;
                    let group = GROUPED_INTERVALS[mid];

                    // Since elements in group are sorted, check first and last
                    if interval_idx < group[0] {
                        // Interval is before this group
                        if mid == 0 { return None; }
                        right = mid - 1;
                    } else if interval_idx > group[group.len() - 1] {
                        // Interval is after this group
                        left = mid + 1;
                    } else {
                        // Interval may be in this group - do a binary search within the group
                        return match group.binary_search(&interval_idx) {
                            Ok(_) => Some(mid),
                            Err(_) => None
                        };
                    }
                }
                None
            }
        }
    }
}

//! This module contains the NFA (Non-deterministic Finite Automaton) representation of the regex
//! syntax.
//! The NFA is created from the high-level intermediate representation (HIR) of the regex syntax.
//! Furthermore, it provides methods to support the conversion of the NFA into a
//! DFA (Deterministic Finite Automaton), like 'epsilon closure' and 'subset construction'.
use std::ops::RangeInclusive;

use crate::{
    Result,
    character_classes::CharacterClasses,
    pattern::{Lookahead, Pattern},
};
use regex_syntax::hir::{Hir, HirKind, Look};

#[derive(Debug)]
struct UnsupportedFeatureError(String);

impl std::fmt::Display for UnsupportedFeatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The regex feature '{}' is not supported.", self.0)
    }
}

impl std::error::Error for UnsupportedFeatureError {}

macro_rules! unsupported {
    ($feature:expr) => {
        Err(Box::new(UnsupportedFeatureError($feature.to_string())))
    };
}

/// Represents a state in the NFA.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NfaState {
    /// The unique identifier for the state.
    pub id: usize,
    /// The set of transitions from this state.
    pub transitions: Vec<NfaTransition>,
    /// The terminal type of the state and the pattern if it is an accepting state.
    pub accept_data: Option<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The Possible types of character classes that can be used in the NFA transitions.
pub enum CharacterClassType {
    /// A character class extracted from the regex syntax.
    /// It can be `Hir::Class(...)` for a character class or `Hir::Literal(...)` for a literal.
    /// Before calculating disjoint character classes, the `HirKind` is used.
    HirKind(HirKind),
    /// An elementary, non-overlapping character range.
    /// After calculating disjoint character classes, this is used.
    Range(RangeInclusive<char>),
}

/// Represents a transition in the NFA.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NfaTransition {
    /// The characters that triggers the transition.
    /// If `None`, it represents an epsilon transition.
    /// If `Some`, it contains a specific character class.
    pub character_class: Option<CharacterClassType>,
    /// The target state of the transition.
    pub target: usize,
}

impl NfaState {
    /// Creates a new NFA state with the given ID.
    pub fn new(id: usize) -> Self {
        NfaState {
            id,
            ..Default::default()
        }
    }

    /// Adds a transition to this state.
    fn add_transition(&mut self, transition: NfaTransition) {
        self.transitions.push(transition);
    }

    /// Apply an offset to every state number.
    fn offset(&mut self, offset: usize) {
        self.id += offset;
        for transition in self.transitions.iter_mut() {
            transition.target += offset;
        }
    }

    fn is_empty(&self) -> bool {
        self.transitions.is_empty()
    }

    /// Returns the epsilon transitions of this state.
    fn epsilon_transitions(&self) -> impl Iterator<Item = &NfaTransition> {
        self.transitions
            .iter()
            .filter(|t| t.character_class.is_none())
    }

    /// Returns the transitions of this state that have a symbol.
    fn transitions(&self) -> impl Iterator<Item = &NfaTransition> {
        self.transitions
            .iter()
            .filter(|t| t.character_class.is_some())
    }
}

impl NfaTransition {
    /// Creates a new NFA transition with the given symbol and target state.
    fn new(character_class: Option<CharacterClassType>, target: usize) -> Self {
        NfaTransition {
            character_class,
            target,
        }
    }
}

/// The Nfa structure represents a Non-deterministic Finite Automaton (NFA).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Nfa {
    /// The pattern that the NFA represents.
    pub pattern: String,
    /// The set of states in the NFA.
    pub states: Vec<NfaState>,
    /// The start state of the NFA. This value changes during the NFA construction.
    pub start_state: usize,
    /// The end state of the NFA.
    /// This value changes during the NFA construction.
    /// It is an accepting state after the NFA is fully constructed.
    pub end_state: usize,
}

impl Nfa {
    /// Creates a new NFA with the given start and end states.
    pub fn new() -> Self {
        let mut me: Self = Default::default();
        me.add_state(NfaState::new(0));
        me
    }

    /// Builds an NFA from a regex pattern.
    /// # Arguments
    /// * `pattern` - A string slice that holds the regex pattern.
    /// * `terminal` - The terminal type of the NFA.
    /// # Returns
    /// An `Nfa` that represents the NFA of the regex pattern.
    pub fn build(pattern: &Pattern) -> Result<Self> {
        let hir = regex_syntax::parse(&pattern.pattern)?;
        match Nfa::try_from(hir) {
            Ok(mut nfa) => {
                // Set the terminal type and pattern for the end state
                if let Some(end_state) = nfa.states.get_mut(nfa.end_state) {
                    end_state.accept_data = Some(pattern.clone());
                }
                nfa.set_pattern(&pattern.pattern);
                Ok(nfa)
            }
            Err(e) => Err(e),
        }
    }

    /// Builds an NFA from a collection of regex patterns.
    /// # Arguments
    /// * `patterns` - A vector of string slices that hold the regex patterns and their terminal types.
    /// # Returns
    /// An `Nfa` that represents the NFA of the regex patterns.
    pub fn build_from_patterns(patterns: &[Pattern]) -> Result<Self> {
        let mut nfa = Nfa::new();
        for pattern in patterns {
            let nfa2 = Nfa::build(pattern)?;
            nfa.alternation(nfa2);
        }
        Ok(nfa)
    }

    /// Sets the pattern for the NFA.
    /// # Arguments
    /// * `pattern` - A string slice that holds the regex pattern.
    fn set_pattern(&mut self, pattern: &str) {
        self.pattern = pattern.to_string();
    }

    /// Adds a state to the NFA.
    fn add_state(&mut self, state: NfaState) {
        self.states.push(state);
    }

    fn new_state(&mut self) -> usize {
        let state = self.states.len();
        self.add_state(NfaState::new(state));
        state
    }

    /// Apply an offset to every state number.
    fn shift_ids(&mut self, offset: usize) -> (usize, usize) {
        for state in self.states.iter_mut() {
            state.offset(offset);
        }
        self.start_state += offset;
        self.end_state += offset;
        (self.start_state, self.end_state)
    }

    /// Concatenates the current NFA with another NFA.
    fn concat(&mut self, mut nfa: Nfa) {
        if self.is_empty() {
            // If the current NFA is empty, set the start and end states of the current NFA to the
            // start and end states of the new NFA
            self.set_start_state(nfa.start_state);
            self.set_end_state(nfa.end_state);
            self.states = nfa.states;
            return;
        }

        // Apply an offset to the state numbers of the given NFA
        let (nfa_start_state, nfa_end_state) = nfa.shift_ids(self.states.len());
        // Move the states of the given NFA to the current NFA
        self.append(nfa);

        // Connect the end state of the current NFA to the start state of the new NFA
        self.add_epsilon_transition(self.end_state, nfa_start_state);

        // Update the end state of the current NFA to the end state of the new NFA
        self.set_end_state(nfa_end_state);
    }

    /// Adds an alternation to the current NFA.
    /// This means that the current NFA can either match the current pattern or the new pattern.
    fn alternation(&mut self, mut nfa: Nfa) {
        if self.is_empty() {
            // If the current NFA is empty, set the start and end states of the current NFA to the
            // start and end states of the new NFA
            self.set_start_state(nfa.start_state);
            self.set_end_state(nfa.end_state);
            self.states = nfa.states;
            return;
        }

        // Apply an offset to the state numbers of the given NFA
        let (nfa_start_state, nfa_end_state) = nfa.shift_ids(self.states.len());

        // Move the states of given the NFA to the current NFA
        self.append(nfa);

        // Create a new start state
        let start_state = self.new_state();
        // Connect the new start state to the start state of the current NFA
        self.add_epsilon_transition(start_state, self.start_state);
        // Connect the new start state to the start state of the new NFA
        self.add_epsilon_transition(start_state, nfa_start_state);

        // Create a new end state
        let end_state = self.new_state();
        // Connect the end state of the current NFA to the new end state
        self.add_epsilon_transition(self.end_state, end_state);
        // Connect the end state of the new NFA to the new end state
        self.add_epsilon_transition(nfa_end_state, end_state);

        // Update the start and end states of the current NFA
        self.set_start_state(start_state);
        self.set_end_state(end_state);
    }

    /// Adds a zero-or-one repetition to the current NFA.
    /// This means that the current NFA can either match the current pattern or not match it at all.
    fn zero_or_one(&mut self) {
        // Create a new start state
        let start_state = self.new_state();
        // Connect the new start state to the start state of the current NFA
        self.add_epsilon_transition(start_state, self.start_state);
        // Connect the new start state to the end state of the current NFA
        self.add_epsilon_transition(start_state, self.end_state);

        // Update the start and end states of the current NFA
        self.set_start_state(start_state);
    }

    /// Adds a zero-or-more repetition to the current NFA.
    /// This means that the current NFA can either match the current pattern zero or more times.
    /// This is done by creating a new start state that connects to the start state and end state
    /// of the current NFA, and a new end state that connects to the end state of the current NFA
    /// and the start state of the current NFA.
    fn zero_or_more(&mut self) {
        // Create a new start state
        let start_state = self.new_state();
        // Connect the new start state to the start state of the current NFA
        self.add_epsilon_transition(start_state, self.start_state);
        // Connect the new start state to the end state of the current NFA
        self.add_epsilon_transition(start_state, self.end_state);

        // Create a new end state
        let end_state = self.new_state();
        // Connect the end state of the current NFA to the new end state
        self.add_epsilon_transition(self.end_state, end_state);
        // Connect the end state of the current NFA to the start state of the current NFA
        self.add_epsilon_transition(self.end_state, self.start_state);

        // Update the start and end states of the current NFA
        self.set_start_state(start_state);
        self.set_end_state(end_state);
    }

    /// Move the states of the given NFA to the current NFA and thereby consume the NFA.
    /// The caller is responsible for ensuring that the states of the NFA are unique.
    fn append(&mut self, mut nfa: Nfa) {
        self.states.append(nfa.states.as_mut());
        // Check the index constraints
        debug_assert!(self.states.iter().enumerate().all(|(i, s)| s.id == i));
    }

    /// Sets the start state of the NFA.
    fn set_start_state(&mut self, start_state: usize) {
        self.start_state = start_state;
    }

    /// Sets the end state of the NFA.
    fn set_end_state(&mut self, end_state: usize) {
        self.end_state = end_state;
    }

    fn add_transition(&mut self, from_state: usize, hir: Hir, to_state: usize) {
        if let Some(state) = self.states.get_mut(from_state) {
            state.add_transition(NfaTransition::new(
                Some(CharacterClassType::HirKind(hir.kind().clone())),
                to_state,
            ));
        } else {
            panic!("State {} does not exist in the NFA.", from_state);
        }
    }

    fn add_epsilon_transition(&mut self, from_state: usize, to_state: usize) {
        if let Some(state) = self.states.get_mut(from_state) {
            state.add_transition(NfaTransition::new(None, to_state));
        } else {
            panic!("State {} does not exist in the NFA.", from_state);
        }
    }

    fn is_empty(&self) -> bool {
        self.start_state == 0
            && self.end_state == 0
            && self.states.len() == 1
            && self.states[0].is_empty()
    }

    /// Calculate the epsilon closure of a state.
    fn epsilon_closure(&self, state: usize) -> Vec<usize> {
        // The state itself is always part of the ε-closure
        let mut closure = vec![state];
        let mut i = 0;
        while i < closure.len() {
            let current_state = closure[i];
            if let Some(state) = self.find_state(current_state) {
                for epsilon_transition in state.epsilon_transitions() {
                    if !closure.contains(&epsilon_transition.target) {
                        closure.push(epsilon_transition.target);
                    }
                }
                i += 1;
            } else {
                panic!("State not found: {:?}", current_state);
            }
        }
        closure.sort_unstable();
        closure.dedup();
        closure
    }

    /// Calculate move(T, a) for a set of states T and a character class a.
    /// This is the set of states that can be reached from T by matching a.
    pub(crate) fn move_set(&self, states: &[usize], char_class: &Hir) -> Vec<usize> {
        let mut move_set = Vec::new();
        for state in states {
            if let Some(state) = self.find_state(*state) {
                for transition in state.transitions() {
                    if let Some(symbol) = transition.character_class.as_ref() {
                        if let CharacterClassType::HirKind(hir_kind) = symbol {
                            // If the transition matches the character class, check if it matches
                            // the given character class.
                            if hir_kind == char_class.kind() {
                                move_set.push(transition.target);
                            }
                        } else if let CharacterClassType::Range(range) = symbol {
                            panic!(
                                "Character class ranges are not supported in NFA move_set: {:?}",
                                range
                            );
                        }
                    }
                }
            } else {
                panic!("State not found: {:?}", state);
            }
        }
        move_set.sort_unstable();
        move_set.dedup();
        move_set
    }

    fn find_state(&self, state: usize) -> Option<&NfaState> {
        self.states.iter().find(|s| s.id == state)
    }

    fn collect_character_classes(&self, character_classes: &mut CharacterClasses) {
        // Collects all character classes from the NFA states and adds them to the
        // `CharacterClasses` data structure.
        for state in &self.states {
            for transition in state.transitions() {
                // The unwrap is save here because we only collect transitions that have a symbol.
                let symbol = transition.character_class.as_ref().unwrap();
                if let CharacterClassType::HirKind(hir_kind) = symbol {
                    // Collect the character class from the HIR kind.
                    character_classes.add_hir(hir_kind.clone());
                } else if let CharacterClassType::Range(range) = symbol {
                    // When this assertion fails, it means that the transitions of this NFA have
                    // already been converted to disjoint character classes.
                    panic!(
                        "Ranges are not supported in collect_character_classes: {:?}",
                        range
                    );
                }
            }
            // If the state is an accepting state, collect the character classes from the
            // lookahead pattern, if it exists.
            if let Some(pattern) = &state.accept_data {
                match &pattern.lookahead {
                    Lookahead::Positive(lookahead_nfa) | Lookahead::Negative(lookahead_nfa) => {
                        for state in &lookahead_nfa.states {
                            for transition in state.transitions() {
                                // The unwrap is save here because we only collect transitions
                                // that have a symbol.
                                let symbol = transition.character_class.as_ref().unwrap();
                                if let CharacterClassType::HirKind(hir_kind) = symbol {
                                    // Collect the character class from the HIR kind.
                                    character_classes.add_hir(hir_kind.clone());
                                } else if let CharacterClassType::Range(range) = symbol {
                                    // When this assertion fails, it means that the transitions
                                    // of this NFA have already been converted to disjoint
                                    // character classes.
                                    panic!(
                                        "Ranges are not supported in collect_character_classes: {:?}",
                                        range
                                    );
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    /// Converts the NFA transitions from `HirKind` to disjoint character classes.
    pub fn convert_to_disjoint_character_classes(&mut self, character_classes: &CharacterClasses) {
        // Replace the transitions in the NFA with the disjoint character classes.
        for state in &mut self.states {
            state.transitions.retain(|t| t.character_class.is_some());
            // Take all transitions from the state and convert each of them to possibly multiple
            // disjoint character classes.
            let old_transitions = std::mem::take(&mut state.transitions);
            state.transitions = old_transitions
                .into_iter()
                .flat_map(|transition| {
                    if let Some(symbol) = &transition.character_class {
                        if let CharacterClassType::HirKind(hir_kind) = symbol {
                            // Convert the HIR kind to disjoint character classes.
                            character_classes
                                .get_disjoint_classes(hir_kind)
                                .iter()
                                .map(|disjoint_class| {
                                    NfaTransition::new(
                                        Some(CharacterClassType::Range(
                                            character_classes.intervals[*disjoint_class].clone(),
                                        )),
                                        transition.target,
                                    )
                                })
                                .collect::<Vec<_>>()
                        } else {
                            panic!(
                                "Character class ranges are not supported in NFA transitions: {:?}",
                                symbol
                            );
                        }
                    } else {
                        vec![transition] // Keep epsilon transitions as they are.
                    }
                })
                .collect();
        }
    }
}

impl TryFrom<Hir> for Nfa {
    type Error = Box<dyn std::error::Error>;

    /// Converts a high-level intermediate representation (HIR) into an NFA.
    /// # Arguments
    /// * `hir` - The high-level intermediate representation (HIR) of the regex syntax.
    /// # Returns
    /// An `Nfa` that represents the NFA of the regex syntax.
    fn try_from(hir: Hir) -> Result<Self> {
        let mut nfa = Nfa::new();
        match hir.kind() {
            HirKind::Empty => Ok(nfa),
            HirKind::Look(look) => match look {
                Look::Start => unsupported!(format!("StartLine {:?}", hir.kind())),
                Look::End => unsupported!(format!("EndLine {:?}", hir.kind())),
                Look::StartLF => unsupported!(format!("StartLF {:?}", hir.kind())),
                Look::EndLF => unsupported!(format!("EndLF {:?}", hir.kind())),
                Look::StartCRLF => unsupported!(format!("StartCRLF {:?}", hir.kind())),
                Look::EndCRLF => unsupported!(format!("EndCRLF {:?}", hir.kind())),
                Look::WordAscii => unsupported!(format!("WordAscii {:?}", hir.kind())),
                Look::WordAsciiNegate => {
                    unsupported!(format!("WordAsciiNegate {:?}", hir.kind()))
                }
                Look::WordUnicode => unsupported!(format!("WordUnicode {:?}", hir.kind())),
                Look::WordUnicodeNegate => {
                    unsupported!(format!("WordUnicodeNegate {:?}", hir.kind()))
                }
                Look::WordStartAscii => {
                    unsupported!(format!("WordStartAscii {:?}", hir.kind()))
                }
                Look::WordEndAscii => unsupported!(format!("WordEndAscii {:?}", hir.kind())),
                Look::WordStartUnicode => {
                    unsupported!(format!("WordStartUnicode {:?}", hir.kind()))
                }
                Look::WordEndUnicode => {
                    unsupported!(format!("WordEndUnicode {:?}", hir.kind()))
                }
                Look::WordStartHalfAscii => {
                    unsupported!(format!("WordStartHalfAscii {:?}", hir.kind()))
                }
                Look::WordEndHalfAscii => {
                    unsupported!(format!("WordEndHalfAscii {:?}", hir.kind()))
                }
                Look::WordStartHalfUnicode => {
                    unsupported!(format!("WordStartHalfUnicode {:?}", hir.kind()))
                }
                Look::WordEndHalfUnicode => {
                    unsupported!(format!("WordEndHalfUnicode {:?}", hir.kind()))
                }
            },
            HirKind::Literal(literal) => {
                let mut start_state = nfa.end_state;
                let chars = std::str::from_utf8(&literal.0)?;
                chars.char_indices().for_each(|(_, c)| {
                    let end_state = nfa.new_state();
                    nfa.set_end_state(end_state);
                    let hir = regex_syntax::hir::Hir::literal(char_to_bytes(c));
                    nfa.add_transition(start_state, hir.clone(), end_state);
                    start_state = end_state;
                });
                nfa.set_end_state(start_state);
                Ok(nfa)
            }
            HirKind::Class(_) => {
                let start_state = nfa.end_state;
                let end_state = nfa.new_state();
                nfa.set_end_state(end_state);
                nfa.add_transition(start_state, hir.clone(), end_state);
                Ok(nfa)
            }
            HirKind::Repetition(repetition) => {
                if !repetition.greedy {
                    unsupported!(format!(
                        "{}: Non-greedy repetitions. Consider using different scanner modes instead.",
                        hir
                    ))?;
                }
                let nfa2: Nfa = Self::try_from((*repetition.sub).clone())?;
                // At least min repetitions
                for _ in 0..repetition.min {
                    nfa.concat(nfa2.clone());
                }
                let mut nfa_zero_or_one: Nfa = nfa2.clone();
                nfa_zero_or_one.zero_or_one();
                if let Some(max) = repetition.max {
                    // At most max-min repetitions are optional
                    for _ in repetition.min..max {
                        nfa.concat(nfa_zero_or_one.clone());
                    }
                } else {
                    // Unbounded repetition
                    let mut nfa_zero_or_more: Nfa = nfa2.clone();
                    nfa_zero_or_more.zero_or_more();
                    nfa.concat(nfa_zero_or_more);
                }
                Ok(nfa)
            }
            HirKind::Capture(capture) => {
                let nfa = Self::try_from((*capture.sub).clone())?;
                Ok(nfa)
            }
            HirKind::Concat(hirs) => {
                for hir in hirs.iter() {
                    let nfa2: Nfa = Self::try_from(hir.clone())?;
                    nfa.concat(nfa2);
                }
                Ok(nfa)
            }
            HirKind::Alternation(hirs) => {
                for hir in hirs.iter() {
                    let nfa2: Nfa = Self::try_from(hir.clone())?;
                    nfa.alternation(nfa2);
                }
                Ok(nfa)
            }
        }
    }
}

fn char_to_bytes(c: char) -> Vec<u8> {
    let mut buffer = [0; 4];
    c.encode_utf8(&mut buffer).as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::pattern::Lookahead;

    use super::*;

    #[test]
    fn test_nfa_from_hir() {
        let hir = regex_syntax::parse(r"\d").unwrap();
        let nfa: Nfa = Nfa::try_from(hir).unwrap();
        // assert_eq!(nfa.pattern, r"\d");
        assert_eq!(nfa.states.len(), 2);
        assert_eq!(nfa.start_state, 0);
        assert_eq!(nfa.end_state, 1);
        assert!(!nfa.is_empty());
    }

    #[test]
    // Test building an NFA from a regex pattern
    fn test_nfa_build() {
        let pattern = r"\d{4}-\d{2}-\d{2}";
        let terminal = 1;
        let pattern = Pattern::new(pattern.to_string(), terminal);
        let nfa = Nfa::build(&pattern).unwrap();
        assert_eq!(nfa.pattern, pattern.pattern);
        assert!(!nfa.is_empty());
        assert_eq!(nfa.start_state, 0);
        assert_eq!(nfa.end_state, 19);
        assert_eq!(nfa.states.len(), 20);
    }

    #[test]
    // Test building an NFA with positive lookahead
    fn test_nfa_build_with_positive_lookahead() {
        let pattern = r"\d{4}-\d{2}-\d{2}";
        let terminal = 1;
        let lookahead = Lookahead::positive(r"\w+".to_string()).unwrap();
        let pattern = Pattern::new(pattern.to_string(), terminal).with_lookahead(lookahead);
        let nfa = Nfa::build(&pattern).unwrap();
        assert!(!nfa.is_empty());
        assert_eq!(nfa.states.len(), 20);
        assert_eq!(nfa.start_state, 0);
        assert_eq!(nfa.end_state, 19);
        assert!(nfa.states[19].accept_data.is_some());
        assert!(matches!(
            nfa.states[19].accept_data.as_ref().unwrap().lookahead,
            Lookahead::Positive(_)
        ));
    }

    #[test]
    // Test building an NFA with negative lookahead
    fn test_nfa_build_with_negative_lookahead() {
        let pattern = r"\d{4}-\d{2}-\d{2}";
        let terminal = 1;
        let lookahead = Lookahead::negative(r"\w+".to_string()).unwrap();
        let pattern = Pattern::new(pattern.to_string(), terminal).with_lookahead(lookahead);
        let nfa = Nfa::build(&pattern).unwrap();
        assert!(!nfa.is_empty());
        assert_eq!(nfa.states.len(), 20);
        assert_eq!(nfa.start_state, 0);
        assert_eq!(nfa.end_state, 19);
        assert!(nfa.states[19].accept_data.is_some());
        assert!(matches!(
            nfa.states[19].accept_data.as_ref().unwrap().lookahead,
            Lookahead::Negative(_)
        ));
    }

    #[test]
    // Test building an NFA from multiple regex patterns
    fn test_nfa_build_from_patterns() {
        let patterns = vec![
            Pattern::new(r"\d{4}-\d{2}-\d{2}".to_string(), 1),
            Pattern::new(r"\w+".to_string(), 2),
        ];
        let nfa = Nfa::build_from_patterns(&patterns).unwrap();
        assert!(!nfa.is_empty());
        assert_eq!(nfa.states.len(), 28);
        assert_eq!(
            nfa.states[19].accept_data,
            Some(Pattern {
                pattern: r"\d{4}-\d{2}-\d{2}".to_string(),
                terminal_type: 1,
                lookahead: Lookahead::None
            })
        );
        assert_eq!(
            nfa.states[25].accept_data,
            Some(Pattern {
                pattern: r"\w+".to_string(),
                terminal_type: 2,
                lookahead: Lookahead::None
            })
        );
        // There should be one accepting state for each pattern
        assert_eq!(
            nfa.states
                .iter()
                .filter(|s| s.accept_data.is_some())
                .count(),
            patterns.len()
        );
    }
}

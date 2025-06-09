use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::collections::{BTreeSet, VecDeque};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    Result,
    ids::{DfaStateID, DisjointCharClassID, NfaStateID, StateIDBase},
    minimizer::Minimizer,
    nfa::Nfa,
    pattern::{AutomatonType, Lookahead, Pattern},
};

/// Represents a Deterministic Finite Automaton (DFA) used for pattern matching.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Dfa {
    /// The states of the DFA.
    pub states: Vec<DfaState>,
}

impl Dfa {
    /// We use a subset construction algorithm to convert an NFA to a DFA.
    /// We also convert the NFA within a possible lookahead to a DFA.
    pub(crate) fn try_from_nfa(nfa: &Nfa) -> Result<Self> {
        match Self::try_from_nfa_not_minimized(nfa) {
            Ok(dfa) => {
                // Minimize the DFA.
                let minimized_dfa = Minimizer::minimize(dfa);
                Ok(minimized_dfa)
            }
            Err(e) => Err(e),
        }
    }

    /// Converts an NFA to a DFA without minimizing it.
    /// This is useful for debugging and testing purposes.
    pub(crate) fn try_from_nfa_not_minimized(nfa: &Nfa) -> Result<Self> {
        // A temporary map to store the state ids of the sets of states.
        let mut state_map: FxHashMap<BTreeSet<NfaStateID>, DfaStateID> = FxHashMap::default();
        // A temporary set to store the transitions of the CompiledDfa.
        // The state ids are numbers of sets of states.
        let mut transitions: FxHashSet<(DfaStateID, DisjointCharClassID, DfaStateID)> =
            FxHashSet::default();
        // Calculate the epsilon closure of the start state.
        let epsilon_closure: BTreeSet<NfaStateID> =
            BTreeSet::from_iter(nfa.epsilon_closure(nfa.start_state));
        // The current state id is always 0.
        let current_state = DfaStateID::new(0);
        // Add the start state to the state map.
        state_map.insert(epsilon_closure.clone(), current_state);
        // The list of target states not yet processed.
        let mut queue: VecDeque<DfaStateID> = VecDeque::new();
        queue.push_back(current_state);
        while let Some(current_state) = queue.pop_front() {
            let epsilon_closure = state_map
                .iter()
                .find(|(_, v)| **v == current_state)
                .unwrap()
                .0
                .clone();
            let target_states = nfa.get_match_transitions(epsilon_closure.iter().cloned());
            let old_state_id = current_state;
            // Group target states by character class
            let mut cc_to_targets: FxHashMap<DisjointCharClassID, FxHashSet<NfaStateID>> =
                FxHashMap::default();
            for (cc, target_state) in target_states {
                cc_to_targets.entry(cc).or_default().insert(target_state);
            }

            // Process each character class once
            for (cc, targets) in cc_to_targets {
                // Calculate epsilon closure of all targets
                let mut combined_epsilon_closure = BTreeSet::new();
                for target in targets {
                    combined_epsilon_closure.extend(nfa.epsilon_closure(target));
                }

                // Create a new DFA state for this combined set
                let new_state_id_candidate = state_map.len() as StateIDBase;
                let new_state_id = *state_map
                    .entry(combined_epsilon_closure.clone())
                    .or_insert_with(|| {
                        let new_state_id = DfaStateID::new(new_state_id_candidate);
                        queue.push_back(new_state_id);
                        new_state_id
                    });
                // Add transitions
                transitions.insert((old_state_id, cc, new_state_id));
            }
        }
        // The transitions of the CompiledDfa.
        let mut states: Vec<DfaState> = vec![DfaState::default(); state_map.len()];
        for (nfa_states, dfa_id) in state_map.iter() {
            // Update accepting states if the epsilon closure contains the end state
            for nfa_state in nfa_states {
                if let Some(accept_data) = nfa.states[*nfa_state].accept_data.as_ref() {
                    let dfa_state = &mut states[*dfa_id];
                    // Only set the accept data if there isn't one already
                    // or if this one has higher priority (lower priority value)
                    if dfa_state.accept_data.is_none()
                        || (accept_data.priority < dfa_state.accept_data.as_ref().unwrap().priority)
                    {
                        // If the NFA state is accepting, add the accept data to the DFA state.
                        let mut accept_data = accept_data.clone();
                        // Convert the Nfa of the pattern's lookahead to a Dfa too.
                        let lookahead = std::mem::take(&mut accept_data.lookahead);
                        match lookahead {
                            Lookahead::None => {}
                            Lookahead::Positive(AutomatonType::Nfa(nfa)) => {
                                // Convert the NFA in the lookahead to a DFA.
                                let dfa_lookahead = Dfa::try_from(&nfa)?;
                                accept_data.lookahead =
                                    Lookahead::Positive(AutomatonType::Dfa(dfa_lookahead));
                            }
                            Lookahead::Negative(AutomatonType::Nfa(nfa)) => {
                                // Convert the NFA in the lookahead to a DFA.
                                let dfa_lookahead = Dfa::try_from(&nfa)?;
                                accept_data.lookahead =
                                    Lookahead::Negative(AutomatonType::Dfa(dfa_lookahead));
                            }
                            _ => {
                                panic!(
                                    "Unexpected lookahead type in DFA conversion: {:?}",
                                    lookahead
                                );
                            }
                        }
                        // Add the accept data to the accepting states.
                        dfa_state.set_accept_data(accept_data);
                    }
                }
            }
        }
        for (from, cc, to) in transitions {
            states[from].transitions.push(DfaTransition::new(cc, to));
        }
        // Create the CompiledDfa from the states and patterns.
        Ok(Dfa { states })
    }
}

impl TryFrom<&Nfa> for Dfa {
    type Error = crate::Error;

    /// Converts an NFA to a DFA.
    ///
    /// # Arguments
    /// * `nfa` - The NFA to convert.
    fn try_from(nfa: &Nfa) -> Result<Self> {
        // Conversion logic from NFA to DFA goes here.
        Self::try_from_nfa(nfa)
    }
}

impl ToTokens for Dfa {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let states = self.states.iter().map(|s| s.to_token_stream());
        tokens.extend(quote! {
            Dfa {
                states: &[#(#states),*],
            }
        });
    }
}

/// Represents a state in the DFA.
/// The id of the state is the index in the `states` vector of the DFA.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DfaState {
    /// The set of transitions from this state.
    pub transitions: Vec<DfaTransition>,
    /// The terminal types, the priorities and patterns if it is an accepting state.
    pub accept_data: Option<Pattern>,
}

impl DfaState {
    /// Creates a new DFA state with the given ID.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the accept data for this state.
    ///
    /// # Arguments
    /// * `accept_data` - The pattern that represents the accept data for this state.
    pub fn set_accept_data(&mut self, accept_data: Pattern) {
        self.accept_data = Some(accept_data);
    }
}

impl ToTokens for DfaState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DfaState {
            transitions,
            accept_data,
        } = self;
        let transitions = transitions.iter().map(|t| t.to_token_stream());
        let accept_data = accept_data
            .as_ref()
            .map_or_else(|| quote! { None }, |ad| quote! { Some(#ad) });
        tokens.extend(quote! {
            DfaState {
                transitions: &[#(#transitions),*],
                accept_data: #accept_data,
            }
        });
    }
}

/// Represents a transition in the DFA.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DfaTransition {
    /// The index of the elementary interval in the character class that this transition
    /// corresponds to.
    pub elementary_interval_index: DisjointCharClassID,
    /// The target state of the transition.
    pub target: DfaStateID,
}

impl DfaTransition {
    /// Creates a new DFA transition with the given character class ID and target state ID.
    ///
    /// # Arguments
    /// * `cc` - The character class ID for this transition.
    /// * `target` - The target state ID for this transition.
    pub fn new(elementary_interval_index: DisjointCharClassID, target: DfaStateID) -> Self {
        Self {
            elementary_interval_index,
            target,
        }
    }
}

impl ToTokens for DfaTransition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DfaTransition {
            elementary_interval_index,
            target,
        } = self;
        let elementary_interval_index = elementary_interval_index.as_usize().to_token_stream();
        let target = target.as_usize().to_token_stream();
        tokens.extend(quote! {
            DfaTransition {
                char_class: #elementary_interval_index,
                to: #target,
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::character_classes::CharacterClasses;

    use super::*;

    #[test]
    fn test_dfa_from_nfa() {
        let patterns = vec![
            Pattern::new(r"\r\n|\r|\n".to_string(), 1.into())
                .with_lookahead(Lookahead::positive("!".to_string()).unwrap()),
            Pattern::new(r"[\s--\r\n]+".to_string(), 2.into()),
            Pattern::new(r#","#.to_string(), 5.into()),
            Pattern::new(r"0|[1-9][0-9]*".to_string(), 6.into()),
        ];
        let mut nfa = Nfa::build_from_patterns(&patterns).unwrap();
        let mut character_classes = CharacterClasses::new();
        nfa.collect_character_classes(&mut character_classes);
        // Generate disjoint character classes
        character_classes.create_disjoint_character_classes();
        // Convert the NFA to use disjoint character classes
        nfa.convert_to_disjoint_character_classes(&character_classes);

        let dfa = Dfa::try_from_nfa(&nfa).expect("Failed to convert NFA to DFA");
        assert_eq!(dfa.states.len(), 8, "DFA should have states");

        // There should be at least one accepting state for each pattern
        let mut terminals = dfa
            .states
            .iter()
            .filter_map(|s| s.accept_data.as_ref().map(|ad| ad.terminal_type))
            .collect::<Vec<_>>();
        terminals.sort();
        terminals.dedup();

        assert!(
            patterns
                .iter()
                .all(|p| { terminals.contains(&p.terminal_type) }),
            "DFA should have accepting states for all patterns"
        );
    }
}

use std::{
    collections::{BTreeMap, BTreeSet},
    vec,
};

use log::trace;

use crate::{
    dfa::{Dfa, DfaState, DfaTransition},
    ids::{DfaStateID, DisjointCharClassID, StateGroupID, StateGroupIDBase, StateIDBase},
    pattern::Pattern,
};

// The type definitions for the subset construction algorithm.

// A state group is a sorted set of states that are in the same partition group.
type StateGroup = BTreeSet<DfaStateID>;
// A partition is a vector of state groups.
type Partition = Vec<StateGroup>;

// A transition map is a map of state ids to a map of character class ids to state set ids.
type TransitionMap = BTreeMap<DfaStateID, BTreeMap<DisjointCharClassID, Vec<DfaStateID>>>;

// A data type that is calculated from the transitions of a DFA state so that for each character
// class the target state is mapped to the partition group it belongs to.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TransitionsToPartitionGroups(pub(crate) Vec<(DisjointCharClassID, StateGroupID)>);

impl TransitionsToPartitionGroups {
    pub(crate) fn new() -> Self {
        TransitionsToPartitionGroups::default()
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        TransitionsToPartitionGroups(Vec::with_capacity(capacity))
    }

    pub(crate) fn insert(
        &mut self,
        char_class: DisjointCharClassID,
        partition_group: StateGroupID,
    ) {
        self.0.push((char_class, partition_group));
    }
}

// The minimizer is a struct that is used to minimize the number of states in a DFA.
#[derive(Debug)]
pub(crate) struct Minimizer;

impl Minimizer {
    /// Minimize the DFA.
    /// The minimization is done using the subset construction algorithm.
    /// The method takes a DFA and returns a minimized DFA.
    pub(crate) fn minimize(dfa: Dfa) -> Dfa {
        trace!("Minimize DFA ----------------------------");
        trace!("Initial DFA:\n{dfa:?}");
        // The transitions of the DFA in a convenient data structure.
        let transitions = Self::calculate_transitions(&dfa);

        trace!("Transitions: {transitions:?}");

        // The initial partition is created.
        let mut partition_old = Self::calculate_initial_partition(&dfa);
        Self::trace_partition("initial", &partition_old);
        let mut partition_new = Partition::new();
        let mut changed = true;
        while changed {
            partition_new = Self::calculate_new_partition(&partition_old, &transitions);
            Self::trace_partition("new", &partition_new);
            changed = partition_new != partition_old;
            partition_old.clone_from(&partition_new);
        }

        Self::create_from_partition(dfa, &partition_new, &transitions)
    }

    /// The start partition is created as follows:
    /// 1. The accepting states are put each in a separate group with group id set to terminal
    ///    id + 1.
    ///    This follows from the constraint of the DFA that multiple patterns can match.
    ///    If a state has multiple accepting patterns, it is put in the group of the first
    ///    accepting pattern.
    /// 2. The non-accepting states are put together in one group with the id 0.
    ///
    /// The partitions are stored in a vector of vectors.
    fn calculate_initial_partition(dfa: &Dfa) -> Partition {
        let mut acceptance_groups: BTreeMap<String, StateGroup> = BTreeMap::new();
        let mut non_accepting_group = StateGroup::new();

        for (id, state) in dfa.states.iter().enumerate() {
            let state_id = (id as StateIDBase).into();
            if state.accept_data.is_empty() {
                non_accepting_group.insert(state_id);
            } else {
                let key = format!("{:?}", state.accept_data);
                acceptance_groups.entry(key).or_default().insert(state_id);
            }
        }

        let mut initial_partition = Vec::new();
        initial_partition.push(non_accepting_group);
        for group in acceptance_groups.into_values() {
            initial_partition.push(group);
        }
        initial_partition
    }

    /// Calculate the new partition based on the old partition.
    /// We try to split the groups of the partition based on the transitions of the DFA.
    /// The new partition is calculated by iterating over the old partition and the states
    /// in the groups. For each state in a group we check if the transitions to the states in the
    /// old partition's groups are the same. If the transitions are the same, the state is put in
    /// the same group as the other states with the same transitions. If the transitions are
    /// different, the state is put in a new group.
    /// The new partition is returned.
    fn calculate_new_partition(partition: &[StateGroup], transitions: &TransitionMap) -> Partition {
        let mut new_partition = Partition::new();
        for (index, group) in partition.iter().enumerate() {
            // The new group receives the states from the old group which are distinguishable from
            // the other states in group.
            Self::split_group(index, group, partition, transitions)
                .into_iter()
                .for_each(|new_group| {
                    new_partition.push(new_group);
                });
        }
        new_partition
    }

    fn split_group(
        group_index: usize,
        group: &StateGroup,
        partition: &[StateGroup],
        transitions: &TransitionMap,
    ) -> Partition {
        // If the group contains only one state, the group can't be split further.
        if group.len() == 1 {
            return vec![group.clone()];
        }
        trace!("Split group {group_index}: {group:?}");
        let mut transition_map_to_states: BTreeMap<TransitionsToPartitionGroups, StateGroup> =
            BTreeMap::new();
        for state_id in group {
            let transitions_to_partition =
                Self::build_transitions_to_partition_group(*state_id, partition, transitions);
            transition_map_to_states
                .entry(transitions_to_partition)
                .or_default()
                .insert(*state_id);
        }
        transition_map_to_states
            .into_values()
            .collect::<Partition>()
    }

    /// Build a modified transition data structure of a given DFA state that maps states to the
    /// partition group.
    /// The partition group is the index of the group in the partition.
    /// The modified transition data structure is returned.
    /// The modified transition data structure is used to determine if two states are distinguish
    /// based on the transitions of the DFA.
    fn build_transitions_to_partition_group(
        state_id: DfaStateID,
        partition: &[StateGroup],
        transitions: &TransitionMap,
    ) -> TransitionsToPartitionGroups {
        if let Some(transitions_of_state) = transitions.get(&state_id) {
            let mut transitions_to_partition_groups =
                TransitionsToPartitionGroups::with_capacity(transitions_of_state.len());
            for transition in transitions_of_state {
                for target_state in transition.1.iter() {
                    let partition_group = Self::find_group(*target_state, partition).unwrap();
                    transitions_to_partition_groups.insert(*transition.0, partition_group);
                }
            }
            Self::trace_transitions_to_groups(state_id, &transitions_to_partition_groups);
            transitions_to_partition_groups
        } else {
            trace!("** State {state_id} has no transitions.");
            TransitionsToPartitionGroups::new()
        }
    }

    fn find_group(state_id: DfaStateID, partition: &[StateGroup]) -> Option<StateGroupID> {
        partition
            .iter()
            .position(|group| group.iter().any(|s| *s == state_id))
            .map(|id| (id as StateGroupIDBase).into())
    }

    /// Create a DFA from a partition.
    /// If a StateGroup contains more than one state, the states are merged into one state.
    /// The transitions are updated accordingly.
    /// The accepting states are updated accordingly.
    /// The new DFA is returned.
    fn create_from_partition(
        dfa: Dfa,
        partition: &[StateGroup],
        transitions: &TransitionMap,
    ) -> Dfa {
        trace!("Create DFA ------------------------------");
        trace!("from partition {partition:?}");
        let Dfa { states, .. } = dfa;
        let mut dfa = Dfa {
            states: vec![DfaState::new(); partition.len()],
        };
        // Calculate the end states of the DFA.
        let end_states = states
            .iter()
            .map(|state| {
                (
                    state.accept_data.is_empty() == false,
                    state.accept_data.clone(),
                )
            })
            .collect::<Vec<_>>();

        // Reorder the groups so that the start state is in the first group (0).
        // The representative state of the first group must be the start state of the minimized DFA,
        // even after minimization.
        let mut partition = partition.to_vec();
        partition.sort_by(|a, b| {
            if a.iter().any(|s| *s == DfaStateID::default()) {
                return std::cmp::Ordering::Less;
            }
            if b.iter().any(|s| *s == DfaStateID::default()) {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Equal
        });

        // Then add the representative states to the DFA from the other groups.
        for (id, group) in partition.iter().enumerate() {
            // For each group we add a representative state to the DFA.
            // It's id is the index of the group in the partition.
            // This function also updates the accepting states of the DFA.
            Self::add_representative_state(
                &mut dfa,
                (id as StateGroupIDBase).into(),
                group,
                &end_states,
            );
        }

        // Then renumber the states in the transitions.
        Self::update_transitions(&mut dfa, &partition, transitions);

        trace!("Minimized DFA:\n{dfa:?}");

        dfa
    }

    /// Add a representative state to the DFA.
    /// The representative state is the first state in the group.
    /// The accepting states are used to determine if the DFA state is an accepting state.
    /// The new state id is returned.
    fn add_representative_state(
        dfa: &mut Dfa,
        group_id: StateGroupID,
        group: &BTreeSet<DfaStateID>,
        end_states: &[(bool, Vec<Pattern>)],
    ) -> DfaStateID {
        let state_id = DfaStateID::new(group_id.id() as StateIDBase);
        let state = DfaState::new();
        dfa.states[state_id] = state;

        // First state in group is the representative state.
        let representative_state_id = group.first().unwrap();

        trace!(
            "Add representative state {} with id {}",
            representative_state_id.as_usize(),
            state_id.as_usize()
        );

        // Insert the representative state into the accepting states if any state in its group is
        // an accepting state.
        for state_in_group in group.iter() {
            if end_states[*state_in_group].0 {
                for pattern in &end_states[*state_in_group].1 {
                    dfa.states[state_id].add_accept_data(pattern.clone());
                }
            }
        }

        state_id
    }

    fn update_transitions(dfa: &mut Dfa, partition: &[StateGroup], transitions: &TransitionMap) {
        // Create a vector because we dont want to mess the transitions map while renumbering.
        let mut transitions = transitions
            .iter()
            .map(|(s, t)| (*s, t.clone()))
            .collect::<Vec<_>>();

        Self::merge_transitions(partition, &mut transitions);
        Self::renumber_states_in_transitions(partition, &mut transitions);

        // Update the transitions of the DFA.
        for (state_id, transitions_of_state) in transitions {
            let state_id = state_id.as_usize();
            for (char_class, target_states) in transitions_of_state.iter() {
                for target_state in target_states {
                    trace!("Add transition {state_id} --{char_class}--> {target_state}");
                    let new_transition = DfaTransition::new(*char_class, target_state.id().into());
                    if !dfa.states[state_id].transitions.contains(&new_transition) {
                        dfa.states[state_id].transitions.push(new_transition);
                    }
                }
            }
        }
    }

    fn merge_transitions(
        partition: &[StateGroup],
        transitions: &mut Vec<(DfaStateID, BTreeMap<DisjointCharClassID, Vec<DfaStateID>>)>,
    ) {
        // Remove all transitions that do not belong to the representative states of a group.
        // The representative states are the first states in the groups.
        for group in partition {
            debug_assert!(!group.is_empty());
            if group.len() == 1 {
                continue;
            }
            let representative_state_id = group.first().unwrap();
            for state_id in group.iter().skip(1) {
                Self::merge_transitions_of_state(*state_id, *representative_state_id, transitions);
            }
        }
    }

    fn merge_transitions_of_state(
        state_id: DfaStateID,
        representative_state_id: DfaStateID,
        transitions: &mut Vec<(DfaStateID, BTreeMap<DisjointCharClassID, Vec<DfaStateID>>)>,
    ) {
        if let Some(rep_pos) = transitions
            .iter()
            .position(|(s, _)| *s == representative_state_id)
        {
            let mut rep_trans = transitions.get_mut(rep_pos).unwrap().1.clone();
            if let Some(pos) = transitions.iter().position(|(s, _)| *s == state_id) {
                let (_, transitions_of_state) = transitions.get_mut(pos).unwrap();
                for (char_class, target_states) in transitions_of_state.iter() {
                    rep_trans
                        .entry(*char_class)
                        .and_modify(|e| {
                            for s in target_states {
                                if !e.contains(s) {
                                    e.push(*s)
                                }
                            }
                        })
                        .or_insert(target_states.clone());
                }
                // Remove the transitions of the state that is merged into the representative state.
                transitions.remove(pos);
            }
            transitions[rep_pos].1 = rep_trans;
        }
    }

    fn renumber_states_in_transitions(
        partition: &[StateGroup],
        transitions: &mut [(DfaStateID, BTreeMap<DisjointCharClassID, Vec<DfaStateID>>)],
    ) {
        let find_group_of_state = |state_id: DfaStateID| -> DfaStateID {
            for (group_id, group) in partition.iter().enumerate() {
                if group.contains(&state_id) {
                    return DfaStateID::new(group_id as StateIDBase);
                }
            }
            panic!("State {} not found in partition.", state_id.as_usize());
        };

        for transition in transitions.iter_mut() {
            transition.0 = find_group_of_state(transition.0);
            for target_states in transition.1.values_mut() {
                for target_state in target_states.iter_mut() {
                    *target_state = find_group_of_state(*target_state);
                }
            }
        }
    }

    /// Trace out a partition of the DFA.
    #[allow(dead_code)]
    fn trace_partition(context: &str, partition: &[StateGroup]) {
        trace!("Partition {context}:");
        for (i, group) in partition.iter().enumerate() {
            trace!("Group {i}: {group:?}");
        }
    }

    #[allow(dead_code)]
    fn trace_transitions_to_groups(
        state_id: DfaStateID,
        transitions_to_groups: &TransitionsToPartitionGroups,
    ) {
        trace!("  Transitions of state {} to groups:", state_id.as_usize());
        for (char_class, group) in &transitions_to_groups.0 {
            trace!("    cc# {char_class} -> gr# {group}");
        }
    }

    /// Transform the transitions of a DFA into a map that groups transitions via character class
    /// by state ids.
    fn calculate_transitions(
        dfa: &Dfa,
    ) -> BTreeMap<DfaStateID, BTreeMap<DisjointCharClassID, Vec<DfaStateID>>> {
        let mut transitions = TransitionMap::new();
        dfa.states.iter().enumerate().for_each(|(id, state)| {
            transitions.entry((id as StateIDBase).into()).or_default();
            for t in &state.transitions {
                let t_of_s = transitions.get_mut(&(id as StateIDBase).into()).unwrap();
                t_of_s
                    .entry(t.elementary_interval_index)
                    .or_default()
                    .push(t.target);
                t_of_s.get_mut(&t.elementary_interval_index).unwrap().sort();
                t_of_s
                    .get_mut(&t.elementary_interval_index)
                    .unwrap()
                    .dedup();
            }
        });
        transitions
    }
}

#[cfg(test)]
mod tests {
    use crate::{character_classes::CharacterClasses, nfa::Nfa, pattern::Pattern};

    use super::*;

    #[test]
    fn test_calculate_initial_partition() {
        let pattern = Pattern::new(r"(A*B|AC)D".to_string(), 0.into());
        let mut nfa: Nfa = Nfa::build(&pattern).unwrap();
        assert_eq!(nfa.start_state.as_usize(), 9);
        assert_eq!(nfa.end_state.as_usize(), 12);
        assert_eq!(nfa.states.len(), 13);
        let accepting_states = nfa
            .states
            .iter()
            .filter(|s| s.accept_data.is_some())
            .count();
        const EXPECTED_ACCEPTING_STATES: usize = 1;
        assert_eq!(accepting_states, EXPECTED_ACCEPTING_STATES);

        let mut character_classes = CharacterClasses::new();
        nfa.collect_character_classes(&mut character_classes);
        const EXPECTED_CHARACTER_CLASSES: usize = 4;
        assert_eq!(character_classes.classes.len(), EXPECTED_CHARACTER_CLASSES);

        character_classes.create_disjoint_character_classes();
        const EXPECTED_DISJOINT_CLASSES: usize = 4;
        assert_eq!(character_classes.intervals.len(), EXPECTED_DISJOINT_CLASSES);

        // eprintln!("Nfa: {:#?}", nfa);

        nfa.convert_to_disjoint_character_classes(&character_classes);

        // eprintln!("Nfa: {:#?}", nfa);

        assert_eq!(nfa.states.len(), 13);
        let transition_count = nfa
            .states
            .iter()
            .map(|s| s.transitions.len())
            .sum::<usize>();
        const EXPECTED_TRANSITIONS: usize = 15;
        assert_eq!(transition_count, EXPECTED_TRANSITIONS);

        let dfa = Dfa::try_from_nfa_not_minimized(&nfa).unwrap();
        const EXPECTED_DFA_STATES: usize = 6;
        assert_eq!(dfa.states.len(), EXPECTED_DFA_STATES);
        let accepting_states = dfa
            .states
            .iter()
            .filter(|s| !s.accept_data.is_empty())
            .count();
        const EXPECTED_DFA_ACCEPTING: usize = 1;
        assert_eq!(accepting_states, EXPECTED_DFA_ACCEPTING);

        let partition = Minimizer::calculate_initial_partition(&dfa);
        // eprintln!("Initial partition: {:#?}", partition);
        const EXPECTED_PARTITION_LEN: usize = 2;
        assert_eq!(partition.len(), EXPECTED_PARTITION_LEN);
    }

    #[test]
    fn test_calculate_new_partition() {
        let pattern = Pattern::new(r"(A*B|AC)D".to_string(), 0.into());
        let mut nfa: Nfa = Nfa::build(&pattern).unwrap();

        let mut character_classes = CharacterClasses::new();
        nfa.collect_character_classes(&mut character_classes);
        character_classes.create_disjoint_character_classes();

        nfa.convert_to_disjoint_character_classes(&character_classes);

        let dfa = Dfa::try_from_nfa_not_minimized(&nfa).unwrap();
        let initial_partition = Minimizer::calculate_initial_partition(&dfa);
        // eprintln!("Initial partition: {:#?}", initial_partition);

        let transitions = Minimizer::calculate_transitions(&dfa);
        // eprintln!("Transitions: {:#?}", transitions);
        assert!(!transitions.is_empty());
        assert_eq!(transitions.len(), 6); // Example: 6 states in the DFA

        let new_partition = Minimizer::calculate_new_partition(&initial_partition, &transitions);
        // eprintln!("New partition: {:#?}", new_partition);
        assert_eq!(new_partition.len(), 4); // Example: 4 groups in the new partition
    }

    #[test]
    fn test_minimize_dfa() {
        let pattern = Pattern::new(r"(A*B|AC)D".to_string(), 0.into());
        let mut nfa: Nfa = Nfa::build(&pattern).unwrap();

        let mut character_classes = CharacterClasses::new();
        nfa.collect_character_classes(&mut character_classes);
        character_classes.create_disjoint_character_classes();
        // eprintln!("Character classes: {:#?}", character_classes);

        nfa.convert_to_disjoint_character_classes(&character_classes);

        let dfa = Dfa::try_from_nfa_not_minimized(&nfa).unwrap();
        // eprintln!("DFA: {:#?}", dfa);
        assert_eq!(dfa.states.len(), 6); // Example: 6 states in the DFA

        let minimized_dfa = Minimizer::minimize(dfa);
        // eprintln!("Minimized DFA: {:#?}", minimized_dfa);
        assert_eq!(minimized_dfa.states.len(), 5); // Example: 5 states in the minimized DFA

        let accepting_states = minimized_dfa
            .states
            .iter()
            .filter(|s| !s.accept_data.is_empty())
            .count();
        assert_eq!(accepting_states, 1); // Example: 1 accepting state

        assert!(!minimized_dfa.states[4].accept_data.is_empty());
    }
}

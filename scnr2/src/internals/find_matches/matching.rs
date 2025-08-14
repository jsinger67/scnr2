//! Matching logic for FindMatches iterators.

use crate::{
    Dfa, Lookahead,
    internals::{
        match_types::{Match, MatchEnd, MatchStart},
        position::{Position, Positions},
    },
};

/// Evaluates the lookahead condition for the current match.
pub(crate) fn evaluate_lookahead<F: super::traits::FindMatchesTrait + Clone>(
    mut find_matches: F,
    accept_data: &crate::AcceptData,
) -> (bool, usize) {
    match &accept_data.lookahead {
        Lookahead::None => unreachable!("Lookahead::None should not be evaluated here"),
        Lookahead::Positive(dfa) => {
            if let Some(ma) = find_next(&mut find_matches, dfa) {
                (true, ma.span.len())
            } else {
                (false, 0)
            }
        }
        Lookahead::Negative(dfa) => {
            if find_next(&mut find_matches, dfa).is_some() {
                (false, 0)
            } else {
                (true, 0)
            }
        }
    }
}

/// Returns the next match in the input, if available.
pub(crate) fn next_match<F: super::traits::FindMatchesTrait + Clone>(
    find_matches: &mut F,
) -> Option<Match> {
    let dfa: &Dfa = find_matches.current_dfa();
    loop {
        if let Some(ma) = find_next(find_matches, dfa) {
            find_matches.handle_mode_transition(ma.token_type);
            return Some(ma);
        }
        if !find_matches.advance_char_iter() {
            return None;
        }
    }
}

/// Simulates the DFA on the given input.
pub(crate) fn find_next<F: super::traits::FindMatchesTrait + Clone>(
    find_matches: &mut F,
    dfa: &Dfa,
) -> Option<Match> {
    let mut state = 0;
    let mut match_start = MatchStart::default();
    let mut match_end = MatchEnd::default();
    let mut start_set = false;
    let mut end_set = false;

    find_matches.save_char_iter();

    while let Some(char_item) = find_matches.peek() {
        let character_class = find_matches.get_disjoint_class(char_item.ch);

        let Some(class_idx) = character_class else {
            break;
        };

        let state_data = &dfa.states[state];
        if let Some(Some(next_state)) = state_data.transitions.get(class_idx) {
            state = next_state.to;
        } else {
            break;
        }
        let state_data = &dfa.states[state];

        find_matches.advance_char_iter();

        if !start_set {
            match_start = MatchStart::new(char_item.byte_index).with_position(char_item.position);
            start_set = true;
        }

        if let Some(accept_data) = &state_data.accept_data {
            let (lookahead_satisfied, lookahead_len) =
                if !matches!(accept_data.lookahead, Lookahead::None) {
                    evaluate_lookahead(find_matches.clone(), accept_data)
                } else {
                    (true, 0)
                };
            if lookahead_satisfied {
                let new_byte_index = char_item.byte_index + lookahead_len + char_item.ch.len_utf8();
                let new_len = new_byte_index - match_start.byte_index;
                let update = !end_set || {
                    let old_len = match_end.byte_index - match_start.byte_index;
                    new_len > old_len
                        || (new_len == old_len && accept_data.priority < match_end.priority)
                };
                if update {
                    match_end =
                        MatchEnd::new(new_byte_index, accept_data.token_type, accept_data.priority)
                            .with_position(
                                char_item
                                    .position
                                    .map(|p| Position::new(p.line, p.column + 1)),
                            );
                    end_set = true;
                    find_matches.save_char_iter();
                }
            }
        }
    }

    if end_set {
        let span: crate::Span = match_start.byte_index..match_end.byte_index;
        find_matches.restore_saved_char_iter();
        Some(
            Match::new(span, match_end.token_type).with_positions(
                match_start
                    .position
                    .zip(match_end.position)
                    .map(|(start, end)| Positions::new(start, end)),
            ),
        )
    } else {
        find_matches.restore_saved_char_iter();
        None
    }
}

//! Trait for finding matches in a string slice using a DFA.

pub trait FindMatchesTrait {
    fn current_dfa(&self) -> &'static crate::Dfa;
    fn handle_mode_transition(&self, token_type: usize);
    fn peek(&mut self) -> Option<crate::internals::char_iter::CharItem>;
    fn get_disjoint_class(&self, ch: char) -> Option<usize>;
    fn advance_char_iter(&mut self) -> bool;
    fn save_char_iter(&mut self);
    fn restore_saved_char_iter(&mut self);
}

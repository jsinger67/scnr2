use syn::braced;

use crate::{pattern::Pattern, scanner_mode::ScannerMode};

macro_rules! parse_ident {
    ($input:ident, $name:ident) => {
        $input.parse().map_err(|e| {
            syn::Error::new(
                e.span(),
                concat!("expected identifier `", stringify!($name), "`"),
            )
        })?
    };
}

#[derive(Debug, Clone)]
pub enum TransitionToNumericMode {
    /// A transition to a new scanner mode triggered by a token type number.
    /// The first element is a non-empty sequence of token type numbers, and the second element is
    /// the new scanner mode index.
    /// This transition is used to set the current scanner mode.
    SetMode(Vec<usize>, usize),
    /// A transition to a new scanner mode triggered by a token type number.
    /// The first element is a non-empty sequence of token type numbers, and the second element is
    /// the new scanner mode index.
    /// This transition is used to push the current mode on the mode stack o be able to return to it later.
    PushMode(Vec<usize>, usize),
    /// A transition back to a formerly pushed scanner mode triggered by a token type number.
    /// The single element is a non-empty sequence of token type numbers, and the second element is
    /// the new scanner mode index.
    /// This transition is used to pop the current scanner mode from the stack.
    PopMode(Vec<usize>),
}

impl TransitionToNumericMode {
    /// Returns the token type number of this transition.
    pub fn token_types(&self) -> &[usize] {
        match self {
            TransitionToNumericMode::SetMode(token_type, _)
            | TransitionToNumericMode::PushMode(token_type, _)
            | TransitionToNumericMode::PopMode(token_type) => token_type,
        }
    }
}

impl PartialEq for TransitionToNumericMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TransitionToNumericMode::SetMode(a, b), TransitionToNumericMode::SetMode(c, d)) => {
                a == c && b == d
            }
            (TransitionToNumericMode::PushMode(a, b), TransitionToNumericMode::PushMode(c, d)) => {
                a == c && b == d
            }
            (TransitionToNumericMode::PopMode(a), TransitionToNumericMode::PopMode(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for TransitionToNumericMode {}

// impl PartialOrd for TransitionToNumericMode {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match (self, other) {
//             (TransitionToNumericMode::SetMode(a, _), TransitionToNumericMode::SetMode(b, _)) => {
//                 a.partial_cmp(b)
//             }
//             (TransitionToNumericMode::PushMode(a, _), TransitionToNumericMode::PushMode(b, _)) => {
//                 a.partial_cmp(b)
//             }
//             (TransitionToNumericMode::PopMode(a), TransitionToNumericMode::PopMode(b)) => {
//                 a.partial_cmp(b)
//             }
//             _ => None,
//         }
//     }
// }

// impl Ord for TransitionToNumericMode {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match (self, other) {
//             (TransitionToNumericMode::SetMode(a, _), TransitionToNumericMode::SetMode(b, _)) => {
//                 a.cmp(b)
//             }
//             (TransitionToNumericMode::PushMode(a, _), TransitionToNumericMode::PushMode(b, _)) => {
//                 a.cmp(b)
//             }
//             (TransitionToNumericMode::PopMode(a), TransitionToNumericMode::PopMode(b)) => a.cmp(b),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionToNamedMode {
    /// A transition to a new scanner mode triggered by a token type number.
    /// The first element is a non-empty sequence of token type numbers, and the second element is
    /// the new scanner mode index.
    /// This transition is used to set the current scanner mode.
    SetMode(Vec<usize>, String),
    /// A transition to a new scanner mode triggered by a token type number.
    /// The first element is a non-empty sequence of token type numbers, and the second element is
    /// the new scanner mode index.
    /// This transition is used to push the current mode on the mode stack o be able to return to it later.
    PushMode(Vec<usize>, String),
    /// A transition back to a formerly pushed scanner mode triggered by a token type number.
    /// The single element is a non-empty sequence of token type numbers, and the second element is
    /// the new scanner mode index.
    /// This transition is used to pop the current scanner mode from the stack.
    PopMode(Vec<usize>),
}

#[derive(Debug)]
pub struct ScannerModeWithNamedTransitions {
    /// The name of the scanner mode.
    pub(crate) name: String,
    /// The regular expressions that are valid token types in this mode, bundled with their token
    /// type numbers.
    /// The priorities of the patterns are determined by their order in the vector. Lower indices
    /// have higher priority if multiple patterns match the input and have the same length.
    pub(crate) patterns: Vec<Pattern>,

    /// The transitions between the scanner modes triggered by a token type number.
    /// The entries are sorted by token type number.
    pub(crate) transitions: Vec<TransitionToNamedMode>,
}

impl ScannerModeWithNamedTransitions {
    /// Converts the scanner mode with named transitions to a scanner mode with numeric transitions.
    /// Returns a vector of tuples of the token type numbers and the new scanner mode ID.
    pub(crate) fn convert_transitions(
        &self,
        scanner_names: &[&str],
    ) -> Vec<TransitionToNumericMode> {
        let mut transitions = Vec::new();
        for transition in &self.transitions {
            match transition {
                TransitionToNamedMode::SetMode(token_types, new_mode) => {
                    let new_mode_id = scanner_names
                        .iter()
                        .position(|name| name == new_mode)
                        .unwrap_or_else(|| panic!("Scanner mode '{}' not found", new_mode));
                    transitions.push(TransitionToNumericMode::SetMode(
                        token_types.clone(),
                        new_mode_id,
                    ));
                }
                TransitionToNamedMode::PushMode(token_type, new_mode) => {
                    let new_mode_id = scanner_names
                        .iter()
                        .position(|name| name == new_mode)
                        .unwrap_or_else(|| panic!("Scanner mode '{}' not found", new_mode));
                    transitions.push(TransitionToNumericMode::PushMode(
                        token_type.clone(),
                        new_mode_id,
                    ));
                }
                TransitionToNamedMode::PopMode(token_type) => {
                    transitions.push(TransitionToNumericMode::PopMode(token_type.clone()));
                }
            }
        }
        transitions.sort_by_key(|t| match t {
            TransitionToNumericMode::SetMode(token_type, _)
            | TransitionToNumericMode::PushMode(token_type, _)
            | TransitionToNumericMode::PopMode(token_type) => {
                token_type.first().cloned().unwrap_or(0)
            }
        });
        transitions
    }
}

/// This is used to create a scanner mode from a part of a macro input.
/// The macro input looks like this:
/// ```text
/// mode INITIAL {
///     token r"\r\n|\r|\n" => 1;
///     token r"[\s--\r\n]+" => 2;
///     token r"//.*(\r\n|\r|\n)?" => 3;
///     token r"/\*([^*]|\*[^/])*\*/" => 4;
///     token r#"""# => 8;
///     token r"Hello" => 9;
///     token r"World" => 10;
///     token r"World" followed by r"!" => 11;
///     token r"!" => 12;
///     token r"[a-zA-Z_]\w*" => 13;
///     token r"." => 14;
///
///     on 8 enter STRING; // Transition to the STRING mode when token type 8 is encountered.
///     on 8 push STRING;  // Push the current mode on the mode stack when token type 8 is encountered and enter STRING mode.
///     on 8 pop; // Pop the current mode from the mode stack when token type 8 is encountered.
/// }
/// ```
/// where there must be at least one token entries which are parsed with the help of the `Pattern`
/// struct's `parse` method. Zero or more `transition` entries can exist.
/// The `transition` entries are tuples of the token type numbers and the new scanner mode name.
/// The scanner mode name is later converted to the scanner mode ID and the transitions are sorted
/// by token type number.
///
impl syn::parse::Parse for ScannerModeWithNamedTransitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mode: syn::Ident = parse_ident!(input, mode);
        if mode != "mode" {
            return Err(input.error("expected 'mode'"));
        }
        let name: syn::Ident = parse_ident!(input, mode_name);
        let name = name.to_string();
        if name.is_empty() {
            return Err(input.error("expected a mode name"));
        }
        let content;
        braced!(content in input);
        let mut patterns = Vec::new();
        let mut transitions = Vec::new();
        while !content.is_empty() {
            let token_or_transition: syn::Ident = parse_ident!(content, token_or_transition);
            if token_or_transition == "token" {
                let pattern: Pattern = content.parse()?;
                patterns.push(pattern);
            } else if token_or_transition == "on" {
                // Parse a non-empty sequence of token type numbers separated by commas with
                // optional trailing comma.

                let mut token_types: Vec<usize> = vec![];
                let token_type: syn::LitInt = content.parse()?;
                let token_type = token_type.base10_parse::<usize>()?;
                token_types.push(token_type);
                while content.peek(syn::Token![,]) {
                    content.parse::<syn::Token![,]>()?;
                    if content.peek(syn::LitInt) {
                        let token_type: syn::LitInt = content.parse()?;
                        let token_type = token_type.base10_parse::<usize>()?;
                        token_types.push(token_type);
                    } else {
                        return Err(content.error("expected a token type number"));
                    }
                }
                let transition_kind: syn::Ident = parse_ident!(content, transition_kind);
                match transition_kind.to_string().as_str() {
                    "enter" => {
                        let new_mode: syn::Ident = parse_ident!(content, new_mode);
                        let new_mode = new_mode.to_string();
                        if new_mode.is_empty() {
                            return Err(content.error("expected a mode name"));
                        }
                        transitions.push(TransitionToNamedMode::SetMode(token_types, new_mode));
                    }
                    "push" => {
                        let new_mode: syn::Ident = parse_ident!(content, new_mode);
                        let new_mode = new_mode.to_string();
                        if new_mode.is_empty() {
                            return Err(content.error("expected a mode name"));
                        }
                        transitions.push(TransitionToNamedMode::PushMode(token_types, new_mode));
                    }
                    "pop" => {
                        transitions.push(TransitionToNamedMode::PopMode(token_types));
                    }
                    _ => {
                        return Err(content.error("expected 'enter', 'push' or 'pop'"));
                    }
                }
                // Parse the semicolon at the end of the transition.
                if content.peek(syn::Token![;]) {
                    content.parse::<syn::Token![;]>()?;
                } else {
                    return Err(content.error("expected ';'"));
                }
            } else {
                return Err(content.error("expected 'token' or 'transition'"));
            }
        }
        Ok(ScannerModeWithNamedTransitions {
            name,
            patterns,
            transitions,
        })
    }
}

#[derive(Debug)]
pub struct ScannerData {
    /// The scanner name.
    pub name: String,
    /// The scanner modes.
    pub modes: Vec<ScannerModeWithNamedTransitions>,
}
impl ScannerData {
    pub fn build_scanner_modes(&self) -> syn::Result<Vec<ScannerMode>> {
        let mut scanner_modes = Vec::new();
        let mut scanner_names = self
            .modes
            .iter()
            .map(|mode| mode.name.as_str())
            .collect::<Vec<_>>();
        for mode in &self.modes {
            let transitions = mode.convert_transitions(&scanner_names);
            let scanner_mode = ScannerMode::new(&mode.name, mode.patterns.clone(), transitions);
            scanner_modes.push(scanner_mode);
            scanner_names.push(&mode.name);
        }
        Ok(scanner_modes)
    }
}

/// This is used to create a scanner from a part of a macro input.
/// The macro input looks like this:
/// ```text
/// HelloWorld {
///     // One or more scanner modes
/// }
impl syn::parse::Parse for ScannerData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = parse_ident!(input, scanner_name);
        let name = name.to_string();
        if name.is_empty() {
            return Err(input.error("expected a scanner name"));
        }
        let content;
        braced!(content in input);
        let mut modes = Vec::new();
        // Parse at least one scanner mode
        let initial_mode: ScannerModeWithNamedTransitions = content.parse()?;
        modes.push(initial_mode);
        while !content.is_empty() {
            let mode: ScannerModeWithNamedTransitions = content.parse()?;
            modes.push(mode);
        }
        Ok(ScannerData { name, modes })
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::Lookahead;

    use super::*;

    #[test]
    fn test_parse_scanner_data() {
        let input = quote::quote! {
            HelloWorld {
                mode INITIAL {
                    token r"\r\n|\r|\n" => 1;
                    token r"[\s--\r\n]+" => 2;
                    token r"//.*(\r\n|\r|\n)?" => 3;
                    token r"/\*([^*]|\*[^/])*\*/" => 4;
                    token r#"""# => 8;
                    token r"Hello" => 9;
                    token r"World" => 10;
                    token r"World" followed by r"!" => 11;
                    token r"!" not followed by r"!" => 12;
                    token r"[a-zA-Z_]\w*" => 13;
                    token r"." => 14;

                    on 8 enter STRING;
                }
                mode STRING {
                    token r#"\\[\"\\bfnt]"# => 5;
                    token r"\\[\s--\n\r]*\r?\n" => 6;
                    token r#"[^\"\]+"# => 7;
                    token r#"""# => 8;
                    token r"." => 14;

                    on 8 enter INITIAL;
                }
            }
        };
        let scanner_data: ScannerData = syn::parse2(input).unwrap();
        assert_eq!(scanner_data.name, "HelloWorld");
        assert_eq!(scanner_data.modes.len(), 2);
        let mode_initial = &scanner_data.modes[0];
        assert_eq!(mode_initial.name, "INITIAL");
        assert_eq!(mode_initial.patterns.len(), 11);
        assert_eq!(mode_initial.transitions.len(), 1);
        assert_eq!(
            TransitionToNamedMode::SetMode(vec![8], "STRING".to_string()),
            mode_initial.transitions[0]
        );
        let mode_initial_patterns = &mode_initial.patterns;
        assert_eq!(mode_initial_patterns[0].pattern, r"\r\n|\r|\n");
        assert_eq!(mode_initial_patterns[1].pattern, r"[\s--\r\n]+");
        assert_eq!(mode_initial_patterns[2].pattern, r"//.*(\r\n|\r|\n)?");
        assert_eq!(mode_initial_patterns[3].pattern, r"/\*([^*]|\*[^/])*\*/");
        assert_eq!(mode_initial_patterns[4].pattern, r#"""#);
        assert_eq!(mode_initial_patterns[5].pattern, r"Hello");
        assert_eq!(mode_initial_patterns[6].pattern, r"World");
        assert_eq!(mode_initial_patterns[7].pattern, r"World");
        assert_eq!(mode_initial_patterns[8].pattern, r"!");
        assert_eq!(mode_initial_patterns[9].pattern, r"[a-zA-Z_]\w*");
        assert_eq!(mode_initial_patterns[10].pattern, r".");
        assert_eq!(mode_initial_patterns[0].terminal_type, 1.into());
        assert_eq!(mode_initial_patterns[1].terminal_type, 2.into());
        assert_eq!(mode_initial_patterns[2].terminal_type, 3.into());
        assert_eq!(mode_initial_patterns[3].terminal_type, 4.into());
        assert_eq!(mode_initial_patterns[4].terminal_type, 8.into());
        assert_eq!(mode_initial_patterns[5].terminal_type, 9.into());
        assert_eq!(mode_initial_patterns[6].terminal_type, 10.into());
        assert_eq!(mode_initial_patterns[7].terminal_type, 11.into());
        assert_eq!(mode_initial_patterns[8].terminal_type, 12.into());
        assert_eq!(mode_initial_patterns[9].terminal_type, 13.into());
        assert_eq!(mode_initial_patterns[10].terminal_type, 14.into());
        assert_eq!(mode_initial_patterns[0].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[1].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[2].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[3].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[4].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[5].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[6].lookahead, Lookahead::None);
        assert_eq!(
            mode_initial_patterns[7].lookahead,
            Lookahead::positive("!".to_string()).unwrap()
        );
        assert_eq!(
            mode_initial_patterns[8].lookahead,
            Lookahead::negative("!".to_string()).unwrap()
        );
        assert_eq!(mode_initial_patterns[9].lookahead, Lookahead::None);
        assert_eq!(mode_initial_patterns[10].lookahead, Lookahead::None);

        let mode_string = &scanner_data.modes[1];
        assert_eq!(mode_string.name, "STRING");
        assert_eq!(mode_string.patterns.len(), 5);
        assert_eq!(mode_string.transitions.len(), 1);
        assert_eq!(
            TransitionToNamedMode::SetMode(vec![8], "INITIAL".to_string()),
            mode_string.transitions[0]
        );
        let mode_string_patterns = &mode_string.patterns;
        assert_eq!(mode_string_patterns[0].pattern, r#"\\[\"\\bfnt]"#);
        assert_eq!(mode_string_patterns[1].pattern, r"\\[\s--\n\r]*\r?\n");
        assert_eq!(mode_string_patterns[2].pattern, r#"[^\"\]+"#);
        assert_eq!(mode_string_patterns[3].pattern, r#"""#);
        assert_eq!(mode_string_patterns[4].pattern, r".");
        assert_eq!(mode_string_patterns[0].terminal_type, 5.into());
        assert_eq!(mode_string_patterns[1].terminal_type, 6.into());
        assert_eq!(mode_string_patterns[2].terminal_type, 7.into());
        assert_eq!(mode_string_patterns[3].terminal_type, 8.into());
        assert_eq!(mode_string_patterns[4].terminal_type, 14.into());
        assert_eq!(mode_string_patterns[0].lookahead, Lookahead::None);
        assert_eq!(mode_string_patterns[1].lookahead, Lookahead::None);
        assert_eq!(mode_string_patterns[2].lookahead, Lookahead::None);
        assert_eq!(mode_string_patterns[3].lookahead, Lookahead::None);
        assert_eq!(mode_string_patterns[4].lookahead, Lookahead::None);
    }
}

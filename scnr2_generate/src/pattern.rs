//! A pattern as a data structure that is used during the construction of the NFA.
//! It contains the pattern string and the associated metadata.
//! Metadata includes the terminal type and a possibly empty lookahead constraint.
use crate::{
    Result,
    dfa::{Dfa, DfaWithNumberOfCharacterClasses},
    ids::{TerminalID, TerminalIDBase},
    nfa::Nfa,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

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

/// The type of the automaton, which can be either an NFA or a DFA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutomatonType {
    /// The NFA type of the automaton.
    Nfa(Nfa),
    /// The DFA type of the automaton.
    Dfa(Dfa),
}

/// The lookahead constraint is used to ensure that the pattern matches only if it is followed by a
/// specific regex pattern, a so called positive lookahead. It is also possible to demand that the
/// pattern is not followed by a specific regex pattern. In this case the lookahead is negative.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Lookahead {
    /// No lookahead constraint is applied. This is used when no specific lookahead is required.
    #[default]
    None,
    /// A positive lookahead constraint that requires the pattern to be followed by a specific regex
    /// pattern.
    Positive(AutomatonType),
    /// A negative lookahead constraint that requires the pattern to not be followed by a specific
    /// regex pattern.
    Negative(AutomatonType),
}

impl Lookahead {
    /// Creates a new positive lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must follow the main pattern.
    pub fn positive(pattern: String) -> Result<Self> {
        // Convert the string pattern into an NFA.
        // The `usize::MAX` is used to indicate that the pattern has no associated terminal type.
        let nfa = Nfa::build(&Pattern::new(pattern, TerminalIDBase::MAX.into()))
            .map_err(|e| format!("Failed to create NFA from regex pattern: {e}"))?;
        Ok(Lookahead::Positive(AutomatonType::Nfa(nfa)))
    }

    /// Creates a new negative lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must not follow the main pattern.
    pub fn negative(pattern: String) -> Result<Self> {
        // Convert the string pattern into an NFA.
        // The `usize::MAX` is used to indicate that the pattern has no associated terminal type.
        let nfa = Nfa::build(&Pattern::new(pattern, TerminalIDBase::MAX.into()))
            .map_err(|e| format!("Failed to create NFA from regex pattern: {e}"))?;
        Ok(Lookahead::Negative(AutomatonType::Nfa(nfa)))
    }
}

/// This is used to create a lookahead from a part of a macro input.
/// The macro input looks like this:
/// ```text
/// followed by r"!";
/// ```
/// for positive lookahead
/// or
/// ```text
/// not followed by r"!";
/// ```
/// for negative lookahead.
impl syn::parse::Parse for Lookahead {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let followed_or_not: syn::Ident = parse_ident!(input, followed_or_not);
        if followed_or_not != "followed" && followed_or_not != "not" {
            return Err(input.error("expected 'followed' or 'not'"));
        }
        let mut is_positive = true;
        if followed_or_not == "not" {
            is_positive = false;
            let followed: syn::Ident = parse_ident!(input, followed);
            if followed != "followed" {
                return Err(input.error("expected 'followed'"));
            }
        }
        // Otherwise followed_or_not is "followed" and we are in the positive case.
        // Now we have to parse the "by" keyword.
        let by: syn::Ident = parse_ident!(input, by);
        if by != "by" {
            return Err(input.error("expected 'by'"));
        }
        // And finally the pattern.
        let pattern: syn::LitStr = input.parse().map_err(|e| {
            syn::Error::new(
                e.span(),
                "expected a string literal for the lookahead pattern",
            )
        })?;
        let pattern = pattern.value();
        Ok(if is_positive {
            Lookahead::positive(pattern).map_err(|e| {
                syn::Error::new(
                    input.span(),
                    format!("Failed to create positive lookahead: {e}"),
                )
            })?
        } else {
            Lookahead::negative(pattern).map_err(|e| {
                syn::Error::new(
                    input.span(),
                    format!("Failed to create negative lookahead: {e}"),
                )
            })?
        })
    }
}

#[derive(Debug)]
pub(crate) struct LookaheadWithNumberOfCharacterClasses {
    /// The lookahead constraint.
    pub lookahead: Lookahead,
    /// The number of character classes in the lookahead automaton.
    pub character_classes: usize,
}

impl LookaheadWithNumberOfCharacterClasses {
    /// Creates a new `LookaheadWithNumberOfCharacterClasses` with the given lookahead and character
    /// classes.
    pub fn new(lookahead: Lookahead, character_classes: usize) -> Self {
        Self {
            lookahead,
            character_classes,
        }
    }
}

impl ToTokens for LookaheadWithNumberOfCharacterClasses {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lookahead_tokens = match &self.lookahead {
            Lookahead::None => quote! { Lookahead::None },
            Lookahead::Positive(AutomatonType::Dfa(dfa)) => {
                let dfa_with_classes =
                    DfaWithNumberOfCharacterClasses::new(dfa, self.character_classes);
                quote! { Lookahead::Positive(#dfa_with_classes) }
            }
            Lookahead::Negative(AutomatonType::Dfa(dfa)) => {
                let dfa_with_classes =
                    DfaWithNumberOfCharacterClasses::new(dfa, self.character_classes);
                quote! { Lookahead::Negative(#dfa_with_classes) }
            }
            _ => panic!("Unexpected lookahead type in Lookahead: {self:?}"),
        };
        tokens.extend(lookahead_tokens);
    }
}

/// A pattern is a data structure that is used during the construction of the NFA.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Pattern {
    /// The pattern string that is used to match input tokens.
    pub pattern: String,
    /// The terminal type associated with the pattern.
    pub terminal_type: TerminalID,
    /// The priority of the pattern, used to resolve conflicts between patterns.
    /// Patterns with priority lower value are preferred over those with higher priority value.
    pub priority: usize,
    /// The lookahead constraint for the pattern, which can be positive, negative, or none.
    pub lookahead: Lookahead,
}

impl Pattern {
    /// Creates a new pattern with the given pattern string, terminal type, and optional lookahead.
    ///
    /// # Arguments
    /// * `pattern` - The pattern string.
    /// * `terminal_type` - The terminal type associated with the pattern.
    pub fn new(pattern: String, terminal_type: TerminalID) -> Self {
        const DEFAULT_PRIORITY: usize = 0;
        Self {
            pattern,
            terminal_type,
            priority: DEFAULT_PRIORITY,
            lookahead: Lookahead::None,
        }
    }

    /// Sets the lookahead constraint for the pattern while consuming the current pattern.
    /// # Arguments
    /// * `lookahead` - The lookahead constraint to set.
    pub fn with_lookahead(mut self, lookahead: Lookahead) -> Self {
        self.lookahead = lookahead;
        self
    }

    /// Sets the priority of the pattern.
    /// # Arguments
    /// * `priority` - The priority to set.
    pub fn with_priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }
}

/// This is used to create a pattern from a part of a macro input.
/// The macro input looks like this:
/// ```text
/// token r"World" followed by r"!" => 11;
/// ```
/// where the lookahead part can be either
/// ```text
/// followed by r"!";
/// ```text
/// or
/// ```text
/// not followed by r"!";
/// ```text
/// or it can be omitted completely.
///
/// The lookahead part should be parsed with the help of the `Lookahead` struct's `parse` method.
///
/// Note that the `token` keyword is not part of the pattern, but it is used to identify the
/// pattern.
impl syn::parse::Parse for Pattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pattern: syn::LitStr = input.parse().map_err(|e| {
            syn::Error::new(
                e.span(),
                format!("expected a string literal for the pattern: {input:?}"),
            )
        })?;
        let pattern = pattern.value();
        let mut lookahead: Option<Lookahead> = None;
        // Check if there is a lookahead and parse it.
        if input.peek(syn::Ident) {
            // The parse implementation of the Lookahead struct will check if the ident is
            // `followed` or `not`.
            // If it is neither, it will return an error.
            lookahead = Some(input.parse()?);
        }
        input.parse::<syn::Token![=>]>()?;
        let token_type: syn::LitInt = input.parse()?;
        let token_type: TerminalIDBase = token_type.base10_parse()?;
        let mut pattern = Pattern::new(pattern, token_type.into());
        // Parse the semicolon at the end of the pattern.
        if input.peek(syn::Token![;]) {
            input.parse::<syn::Token![;]>()?;
        } else {
            return Err(input.error("expected ';'"));
        }

        // If a lookahead was parsed, set it on the pattern.
        let lookahead = lookahead.unwrap_or(Lookahead::None);
        pattern = pattern.with_lookahead(lookahead);

        Ok(pattern)
    }
}

#[derive(Debug)]
pub(crate) struct PatternWithNumberOfCharacterClasses<'a> {
    /// The pattern itself.
    pub pattern: &'a Pattern,
    /// The number of character classes in the pattern.
    pub character_classes: usize,
}

impl<'a> PatternWithNumberOfCharacterClasses<'a> {
    /// Creates a new `PatternWithNumberOfCharacterClasses` with the given pattern and character
    /// classes.
    pub fn new(pattern: &'a Pattern, character_classes: usize) -> Self {
        Self {
            pattern,
            character_classes,
        }
    }
}

impl ToTokens for PatternWithNumberOfCharacterClasses<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PatternWithNumberOfCharacterClasses {
            pattern,
            character_classes,
        } = self;
        let terminal_type = pattern.terminal_type.as_usize().to_token_stream();
        let priority = pattern.priority.to_token_stream();
        let lookahead_with_number_of_character_classes = LookaheadWithNumberOfCharacterClasses::new(
            pattern.lookahead.clone(),
            *character_classes,
        );
        tokens.extend(quote! {
            AcceptData {
                token_type: #terminal_type,
                priority: #priority,
                lookahead: #lookahead_with_number_of_character_classes,
            }
        });
    }
}

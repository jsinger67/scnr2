//! A pattern as a data structure that is used during the construction of the NFA.
//! It contains the pattern string and the associated metadata.
//! Metadata includes the terminal type and a possibly empty lookahead constraint.
use crate::{Result, nfa::Nfa};

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
    Positive(Nfa),
    /// A negative lookahead constraint that requires the pattern to not be followed by a specific
    /// regex pattern.
    Negative(Nfa),
}

impl Lookahead {
    /// Creates a new positive lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must follow the main pattern.
    pub fn positive(pattern: String) -> Result<Self> {
        // Convert the string pattern into an NFA.
        // The `usize::MAX` is used to indicate that the pattern has no associated terminal type.
        let nfa = Nfa::build(&Pattern::new(pattern, usize::MAX))
            .map_err(|e| format!("Failed to create NFA from regex pattern: {}", e))?;
        Ok(Lookahead::Positive(nfa))
    }

    /// Creates a new negative lookahead constraint with the given regex pattern.
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern that must not follow the main pattern.
    pub fn negative(pattern: String) -> Result<Self> {
        // Convert the string pattern into an NFA.
        // The `usize::MAX` is used to indicate that the pattern has no associated terminal type.
        let nfa = Nfa::build(&Pattern::new(pattern, usize::MAX))
            .map_err(|e| format!("Failed to create NFA from regex pattern: {}", e))?;
        Ok(Lookahead::Negative(nfa))
    }

    // /// Checks if the lookahead is empty, meaning it has no constraints.
    // pub fn is_empty(&self) -> bool {
    //     matches!(self, Lookahead::None)
    // }

    // /// Checks if the lookahead is positive, meaning it has a positive lookahead constraint.
    // /// Returns `true` if the lookahead is positive, `false` otherwise.
    // pub fn is_positive(&self) -> bool {
    //     matches!(self, Lookahead::Positive(_))
    // }

    // /// Checks if the lookahead is negative, meaning it has a negative lookahead constraint.
    // /// Returns `true` if the lookahead is negative, `false` otherwise.
    // pub fn is_negative(&self) -> bool {
    //     matches!(self, Lookahead::Negative(_))
    // }
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
                    format!("Failed to create positive lookahead: {}", e),
                )
            })?
        } else {
            Lookahead::negative(pattern).map_err(|e| {
                syn::Error::new(
                    input.span(),
                    format!("Failed to create negative lookahead: {}", e),
                )
            })?
        })
    }
}

/// A pattern is a data structure that is used during the construction of the NFA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub pattern: String,
    pub terminal_type: usize,
    pub lookahead: Lookahead,
}

impl Pattern {
    /// Creates a new pattern with the given pattern string, terminal type, and optional lookahead.
    ///
    /// # Arguments
    /// * `pattern` - The pattern string.
    /// * `terminal_type` - The terminal type associated with the pattern.
    pub fn new(pattern: String, terminal_type: usize) -> Self {
        Self {
            pattern,
            terminal_type,
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

    // /// Sets the lookahead constraint for the pattern.
    // /// # Arguments
    // /// * `lookahead` - The lookahead constraint to set.
    // pub fn set_lookahead(&mut self, lookahead: Lookahead) {
    //     self.lookahead = lookahead;
    // }
}

/// This is used to create a pattern from a part of a macro input.
/// The macro input looks like this:
/// ```text
/// token r"World" => 11 followed by r"!";
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
        let pattern: syn::LitStr = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "expected a string literal for the pattern"))?;
        let pattern = pattern.value();
        input.parse::<syn::Token![=>]>()?;
        let token_type: syn::LitInt = input.parse()?;
        let token_type = token_type.base10_parse()?;
        let mut pattern = Pattern::new(pattern, token_type);
        // Check if there is a lookahead and parse it.
        if input.peek(syn::Ident) {
            // The parse implementation of the Lookahead struct will check if the ident is
            // `followed` or `not`.
            // If it is neither, it will return an error.
            let lookahead: Lookahead = input.parse()?;
            pattern = pattern.with_lookahead(lookahead);
        }
        // Parse the semicolon at the end of the pattern.
        if input.peek(syn::Token![;]) {
            input.parse::<syn::Token![;]>()?;
        } else {
            return Err(input.error("expected ';'"));
        }
        Ok(pattern)
    }
}

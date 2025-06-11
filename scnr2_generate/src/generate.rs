use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse2;

use crate::{
    character_classes::CharacterClasses, dfa::Dfa, nfa::Nfa, scanner_data::ScannerData,
    scanner_mode::ScannerMode,
};

/// This function generates the scanner code from the input token stream.
/// It parses the input token stream into a `ScannerData` struct and then generates the scanner
/// code.
/// It returns a `TokenStream` containing the generated code.
/// The input token stream is expected to contain the scanner definition, including the regex
/// patterns and actions.
/// The macro syntax is expected to be used in the following way:
/// ```ignore
/// use scnr_macro::scanner;
///
/// scanner! {
///     ExampleScanner {
///         mode INITIAL {
///             token r"\r\n|\r|\n" => 1;
///             token r"[\s--\r\n]+" => 2;
///             token r"//.*(\r\n|\r|\n)?" => 3;
///             token r"/\*([^*]|\*[^/])*\*/" => 4;
///             token r#"""# => 8;
///             token r"Hello" => 9;
///             token r"World" => 10;
///             token r"World" => 11 followed by r"!";
///             token r"!" => 12 not followed by r"!";
///             token r"[a-zA-Z_]\w*" => 13;
///             token r"." => 14;
///
///             transition 8 => STRING;
///         }
///         mode STRING {
///             token r#"\\[\"\\bfnt]"# => 5;
///             token r"\\[\s--\n\r]*\r?\n" => 6;
///             token r#"[^\"\]+"# => 7;
///             token r#"""# => 8;
///             token r"." => 14;
///
///             transition 8 => INITIAL;
//          }
///     }
/// }
/// ```
/// where there must be at least one scanner mode with at least one `token` entry.
/// A `token` entry is a regex pattern followed by an arrow and a token type number.
/// Optional `not` and `followed by` modifiers can be used to specify positive and negative
/// lookaheads.
/// Zero or more `transition` entries can exist.
/// The `transition` entries are tuples of the token type numbers and the new scanner mode name.
///
/// The generated code will include the scanner implementation.
/// The generated scanner in this example will be a struct named `ExampleScanner` which implements
/// the `ScannerTrait`.
pub fn generate(input: TokenStream) -> TokenStream {
    let scanner_data: ScannerData = parse2(input).expect("Failed to parse input");
    let scanner_modes: Vec<ScannerMode> = scanner_data
        .build_scanner_modes()
        .expect("Failed to build scanner modes");

    // Generate NFAs for each scanner mode
    let mut nfas = scanner_modes
        .iter()
        .map(|mode| {
            // Build the NFA for each pattern in the scanner mode
            Nfa::build_from_patterns(&mode.patterns).expect("Failed to build NFA for pattern")
        })
        .collect::<Vec<_>>();

    let mut character_classes = CharacterClasses::new();
    // For each NFA, generate the character classes
    for nfa in &nfas {
        nfa.collect_character_classes(&mut character_classes)
    }
    // Generate disjoint character classes
    character_classes.create_disjoint_character_classes();
    // Convert the NFA to use disjoint character classes
    for nfa in &mut nfas {
        nfa.convert_to_disjoint_character_classes(&character_classes);
    }

    // Convert the nfas into DFAs
    let dfas = nfas
        .into_iter()
        .try_fold(Vec::new(), |mut acc, nfa| -> Result<Vec<Dfa>, syn::Error> {
            // Convert the NFA to a DFA
            let dfa = Dfa::try_from(&nfa).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to convert NFA to DFA: {}", e),
                )
            })?;
            // Add the DFA to the accumulator
            acc.push(dfa);
            Ok(acc)
        })
        .expect("Failed to convert NFAs to DFAs");

    // Convert the scanner name to snake case for the module name
    let module_name = to_snake_case(&scanner_data.name);
    let module_name_ident = syn::Ident::new(&module_name, proc_macro2::Span::call_site());

    // Make the scanner name an syn::Ident
    let scanner_name = syn::Ident::new(&scanner_data.name, proc_macro2::Span::call_site());
    let match_function_code = character_classes.generate("match_function");

    let modes = scanner_modes.into_iter().enumerate().map(|(index, mode)| {
        let transitions = mode.transitions.iter().map(|(token_type, new_mode_index)| {
            quote! {
                (#token_type, #new_mode_index)
            }
        });
        let states = dfas[index]
            .states
            .iter()
            .map(|state| state.to_token_stream());
        let mode_name = mode.name;
        quote! {
            ScannerMode {
                name: #mode_name,
                transitions: &[#(#transitions),*],
                dfa: Dfa { states: &[#(#states),*] }
            }
        }
    });

    let output = quote! {
        pub mod #module_name_ident {
            use scnr2::*;
            pub const MODES: &'static [ScannerMode] = &[
                #(
                    #modes
                ),*
            ];
            pub struct #scanner_name {
                pub current_mode: usize,
                current_state: usize,
            }
            impl #scanner_name {
                pub fn new() -> Self {
                    #scanner_name {
                        current_mode: 0,
                        current_state: 0,
                    }
                }
                #match_function_code
            }
        }
    };

    output
}

/// Converts a string from PascalCase or camelCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars = s.chars().peekable();

    for c in chars {
        if c.is_uppercase() {
            // Add underscore if not at the beginning and not after an underscore
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    use crate::Result;
    use std::path::Path;

    use std::process::Command;

    /// Tries to format the source code of a given file.
    fn try_format(path_to_file: &Path) -> Result<()> {
        Command::new("rustfmt")
            .args([path_to_file])
            .status()
            .map(|_| ())
            .map_err(|e| {
                std::io::Error::new(e.kind(), format!("Failed to format file: {}", e)).into()
            })
    }

    #[test]
    fn test_generate() {
        let input = quote::quote! {
            TestScanner {
                mode INITIAL {
                    token r"\r\n|\r|\n" => 1;
                    token r"[\s--\r\n]+" => 2;
                    token r"//.*(\r\n|\r|\n)?" => 3;
                    token r"/\*([^*]|\*[^/])*\*/" => 4;
                    token r#"""# => 8;
                    token r"Hello" => 9;
                    token r"World" => 10;
                    token r"World" => 11 followed by r"!";
                    token r"!" => 12 not followed by r"!";
                    token r"[a-zA-Z_]\w*" => 13;
                    token r"." => 14;

                    transition 8 => STRING;
                }
                mode STRING {
                    token r#"\\[\"\\bfnt]"# => 5;
                    token r"\\[\s--\r\n]*\r?\n" => 6;
                    token r#"[^\"\\]+"# => 7;
                    token r#"""# => 8;
                    token r"." => 14;

                    transition 8 => INITIAL;
                }
            }
        };
        let code = generate(input).to_string();

        // Create a temporary file
        let mut temp_file =
            tempfile::NamedTempFile::new().expect("Failed to create temporary file");

        // Write the generated code to the temporary file
        temp_file
            .write_all(code.as_bytes())
            .expect("Failed to write to temporary file");

        // Optionally, print the file path for debugging
        println!("Temporary file created at: {:?}", temp_file.path());

        // Format the file (if needed)
        try_format(temp_file.path()).expect("Failed to format the temporary file");

        // Load the formatted code and convert possible \r\n to \n for easier comparison
        let formatted_code = std::fs::read_to_string(temp_file.path())
            .expect("Failed to read the formatted temporary file")
            .replace("\r\n", "\n");

        let expected_code = std::fs::read_to_string("data/expected_generated_code.rs")
            .expect("Failed to read the expected code file")
            .replace("\r\n", "\n");
        assert_eq!(formatted_code, expected_code);
    }
}

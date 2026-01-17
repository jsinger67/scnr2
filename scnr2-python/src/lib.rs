#![allow(unsafe_op_in_unsafe_fn)]
use pyo3::prelude::*;
use scnr2_generate::{
    character_classes::CharacterClasses, dfa::Dfa as GenDfa, nfa::Nfa, scanner_data::ScannerData,
    scanner_mode::ScannerMode as GenScannerMode,
};
use std::cell::RefCell;
use std::ops::RangeInclusive;
use std::rc::Rc;

#[pyclass]
#[derive(Clone)]
pub struct TokenMatch {
    #[pyo3(get)]
    pub token_type: usize,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub start_line: Option<usize>,
    #[pyo3(get)]
    pub start_column: Option<usize>,
    #[pyo3(get)]
    pub end_line: Option<usize>,
    #[pyo3(get)]
    pub end_column: Option<usize>,
}

#[pyclass(unsendable)]
pub struct Scanner {
    scanner_impl: Rc<RefCell<::scnr2::ScannerImpl>>,
    match_function: &'static (dyn Fn(char) -> Option<usize> + Sync + Send),
}

fn convert_dfa(gen_dfa: &GenDfa, num_classes: usize) -> ::scnr2::Dfa {
    let states: Vec<::scnr2::DfaState> = gen_dfa
        .states
        .iter()
        .map(|s| {
            // Create a dense transition vector
            let mut transition_opts = vec![None; num_classes];
            for transition in &s.transitions {
                transition_opts[transition.elementary_interval_index.as_usize()] =
                    Some(::scnr2::DfaTransition {
                        to: transition.target.as_usize(),
                    });
            }
            let transitions: &'static [Option<::scnr2::DfaTransition>] =
                Box::leak(transition_opts.into_boxed_slice());

            let accept_data = s.accept_data.as_ref().map(|ad| {
                let lookahead = match &ad.lookahead {
                    scnr2_generate::pattern::Lookahead::None => ::scnr2::Lookahead::None,
                    scnr2_generate::pattern::Lookahead::Positive(
                        scnr2_generate::pattern::AutomatonType::Dfa(d),
                    ) => ::scnr2::Lookahead::Positive(convert_dfa(d, num_classes)),
                    scnr2_generate::pattern::Lookahead::Negative(
                        scnr2_generate::pattern::AutomatonType::Dfa(d),
                    ) => ::scnr2::Lookahead::Negative(convert_dfa(d, num_classes)),
                    _ => ::scnr2::Lookahead::None,
                };
                ::scnr2::AcceptData {
                    token_type: ad.terminal_type.as_usize(),
                    priority: ad.priority,
                    lookahead,
                }
            });

            ::scnr2::DfaState {
                transitions,
                accept_data,
            }
        })
        .collect();
    let states: &'static [::scnr2::DfaState] = Box::leak(states.into_boxed_slice());
    ::scnr2::Dfa { states }
}

#[pymethods]
impl Scanner {
    #[new]
    pub fn new(definition: &str) -> PyResult<Self> {
        let scanner_data: ScannerData = syn::parse_str(definition).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse scanner definition: {}",
                e
            ))
        })?;

        let scanner_modes: Vec<GenScannerMode> =
            scanner_data.build_scanner_modes().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to build scanner modes: {}",
                    e
                ))
            })?;

        // Generate NFAs and then DFAs
        let mut nfas = scanner_modes
            .iter()
            .map(|mode| {
                Nfa::build_from_patterns(&mode.patterns).expect("Failed to build NFA for pattern")
            })
            .collect::<Vec<_>>();

        let mut character_classes = CharacterClasses::new();
        for nfa in &nfas {
            nfa.collect_character_classes(&mut character_classes)
        }
        character_classes.create_disjoint_character_classes();
        for nfa in &mut nfas {
            nfa.convert_to_disjoint_character_classes(&character_classes);
        }

        let gen_dfas = nfas
            .into_iter()
            .map(|nfa| GenDfa::try_from(&nfa).expect("Failed to convert NFA to DFA"))
            .collect::<Vec<_>>();

        let num_character_classes = character_classes.intervals.len();

        let runtime_modes: Vec<::scnr2::ScannerMode> = scanner_modes
            .into_iter()
            .enumerate()
            .map(|(index, mode)| {
                let transitions: Vec<::scnr2::Transition> = mode
                    .transitions
                    .iter()
                    .map(|t| match t {
                        scnr2_generate::scanner_data::TransitionToNumericMode::SetMode(tt, m) => {
                            ::scnr2::Transition::SetMode(*tt, *m)
                        }
                        scnr2_generate::scanner_data::TransitionToNumericMode::PushMode(tt, m) => {
                            ::scnr2::Transition::PushMode(*tt, *m)
                        }
                        scnr2_generate::scanner_data::TransitionToNumericMode::PopMode(tt) => {
                            ::scnr2::Transition::PopMode(*tt)
                        }
                    })
                    .collect();
                let transitions: &'static [::scnr2::Transition] =
                    Box::leak(transitions.into_boxed_slice());

                let dfa = convert_dfa(&gen_dfas[index], num_character_classes);

                ::scnr2::ScannerMode {
                    name: Box::leak(mode.name.into_boxed_str()),
                    transitions,
                    dfa,
                }
            })
            .collect();

        let runtime_modes: &'static [::scnr2::ScannerMode] =
            Box::leak(runtime_modes.into_boxed_slice());

        // Match function using binary search over elementary intervals
        let elementary_intervals = character_classes.elementary_intervals.clone();
        let grouped_intervals: Vec<Vec<RangeInclusive<char>>> = character_classes.intervals.clone();

        let match_func = move |c: char| -> Option<usize> {
            use std::cmp::Ordering;
            let interval_idx = match elementary_intervals.binary_search_by(|interval| {
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

            let interval = &elementary_intervals[interval_idx];
            grouped_intervals
                .iter()
                .position(|group| group.contains(interval))
        };

        let match_func: Box<dyn Fn(char) -> Option<usize> + Sync + Send> = Box::new(match_func);
        let match_func: &'static (dyn Fn(char) -> Option<usize> + Sync + Send) =
            Box::leak(match_func);

        Ok(Scanner {
            scanner_impl: Rc::new(RefCell::new(::scnr2::ScannerImpl::new(runtime_modes))),
            match_function: match_func,
        })
    }

    pub fn find_matches(&self, input: String) -> Vec<TokenMatch> {
        let sc = self.scanner_impl.clone();
        let it = ::scnr2::ScannerImpl::find_matches(sc, &input, 0, self.match_function);
        it.map(|m| TokenMatch {
            token_type: m.token_type,
            start: m.span.start,
            end: m.span.end,
            text: input[m.span.clone()].to_string(),
            start_line: m.positions.as_ref().map(|p| p.start_position.line),
            start_column: m.positions.as_ref().map(|p| p.start_position.column),
            end_line: m.positions.as_ref().map(|p| p.end_position.line),
            end_column: m.positions.as_ref().map(|p| p.end_position.column),
        })
        .collect()
    }

    pub fn find_matches_with_position(&self, input: String) -> Vec<TokenMatch> {
        let sc = self.scanner_impl.clone();
        let it =
            ::scnr2::ScannerImpl::find_matches_with_position(sc, &input, 0, self.match_function);
        it.map(|m| TokenMatch {
            token_type: m.token_type,
            start: m.span.start,
            end: m.span.end,
            text: input[m.span.clone()].to_string(),
            start_line: m.positions.as_ref().map(|p| p.start_position.line),
            start_column: m.positions.as_ref().map(|p| p.start_position.column),
            end_line: m.positions.as_ref().map(|p| p.end_position.line),
            end_column: m.positions.as_ref().map(|p| p.end_position.column),
        })
        .collect()
    }
}

#[pymodule]
fn scnr2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scanner>()?;
    m.add_class::<TokenMatch>()?;
    Ok(())
}

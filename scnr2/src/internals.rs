//! Internals of the scanner module.

/// Scanner implementation details.
pub struct ScannerImpl {
    pub(crate) current_mode: usize,
    pub(crate) current_state: usize,
    pub(crate) modes: &'static [crate::ScannerMode],
}

impl ScannerImpl {
    /// Creates a new scanner implementation with the given modes.
    pub fn new(modes: &'static [crate::ScannerMode]) -> Self {
        ScannerImpl {
            current_mode: 0,
            current_state: 0,
            modes,
        }
    }
}

macro_rules! impl_id {
    ($name:ident, $tp:ty) => {
        /// The ID type $name.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name($tp);

        impl $name {
            /// Create a new id.
            #[inline]
            pub const fn new(index: $tp) -> Self {
                $name(index)
            }

            /// Get the id as $tp.
            #[allow(dead_code)]
            #[inline]
            pub fn as_usize(&self) -> usize {
                self.0 as usize
            }

            /// Get the id as $tp.
            #[allow(dead_code)]
            #[inline]
            pub fn id(&self) -> $tp {
                self.0
            }
        }

        impl core::ops::Add<$tp> for $name {
            type Output = $name;

            #[inline]
            fn add(self, rhs: $tp) -> Self::Output {
                $name(self.0 + rhs)
            }
        }

        impl core::ops::AddAssign<$tp> for $name {
            #[inline]
            fn add_assign(&mut self, rhs: $tp) {
                self.0 = self.0 + rhs;
            }
        }

        impl<T> std::ops::Index<$name> for [T] {
            type Output = T;

            #[inline]
            fn index(&self, index: $name) -> &Self::Output {
                &self[index.0 as usize]
            }
        }

        impl<T> std::ops::IndexMut<$name> for [T] {
            #[inline]
            fn index_mut(&mut self, index: $name) -> &mut T {
                &mut self[index.0 as usize]
            }
        }

        impl<T> std::ops::Index<$name> for Vec<T> {
            type Output = T;

            #[inline]
            fn index(&self, index: $name) -> &Self::Output {
                &self[index.0 as usize]
            }
        }

        impl<T> std::ops::IndexMut<$name> for Vec<T> {
            #[inline]
            fn index_mut(&mut self, index: $name) -> &mut T {
                &mut self[index.0 as usize]
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$tp> for $name {
            fn from(index: $tp) -> Self {
                $name::new(index)
            }
        }
    };
}

/// The ID types for automata states. Used in Finite State Automata.
pub type StateIDBase = u32;
impl_id!(NfaStateID, StateIDBase);
impl_id!(DfaStateID, StateIDBase);

/// The ID type for character classes. This is the index of the character class in the character
/// class registry which in turn is used for all DFAs in the scanner.
pub type CharClassIDBase = u32;
impl_id!(CharClassID, CharClassIDBase);

// The ID type for disjoint character classes. This is the id type for disjoint character classes
// used in the scanner. It is used to represent character classes that are disjoint from each
// other, meaning that they do not overlap in the characters they match.
impl_id!(DisjointCharClassID, CharClassIDBase);

/// The ID type for patterns. Actually the index of the pattern in the pattern vector of a scanner
/// mode. It determines the priority of the pattern, i.e. lower indices have higher priority.
pub type PatternIDBase = usize;
impl_id!(PatternID, PatternIDBase);

/// The ID type for terminals. This is the token type number associated with a pattern and used in
/// the scanner over all scanner modes.
pub type TerminalIDBase = u32;
impl_id!(TerminalID, TerminalIDBase);

/// The ID type for scanner modes. This is the index of the scanner mode in the scanner mode vector
/// of the scanner.
pub type ScannerModeIDBase = usize;
impl_id!(ScannerModeID, ScannerModeIDBase);

/// The ID type for groups in Partitions. This is the index of the group in the partition vector of
/// the minimizer.
pub type StateGroupIDBase = u16;
impl_id!(StateGroupID, StateGroupIDBase);

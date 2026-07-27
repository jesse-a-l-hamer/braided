use crate::{Sign, Strand};
use std::ops::Neg;

/// Struct representing a generator of the standard Artin braid group.
///
/// Geometrically, a positive (negative) Artin generator represents a positive (negative) crossing
/// of two adjacent strands. Thus if we think of the braid strands as stacked vertically and
/// oriented left-to-right, then a positive (negative) Artin generator corresponds to the crossing
/// of a strand under (over) the strand immediately above it.
#[derive(Debug, PartialEq, Eq)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    /// Constructor for ArtinGenerator
    pub fn new(foot: Strand, sign: Sign) -> Self {
        Self { foot, sign }
    }

    /// Accessor for `foot` strand field
    pub fn foot(&self) -> Strand {
        self.foot
    }
    /// Accessor for `sign` field
    pub fn sign(&self) -> Sign {
        self.sign
    }
}

impl Neg for ArtinGenerator {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.foot, -self.sign)
    }
}

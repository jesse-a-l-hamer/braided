use anyhow::Context;

use crate::{BraidIndex, Sign, Strand};
use std::ops::Neg;

#[derive(Debug, thiserror::Error)]
pub enum ArtinValidationError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

/// Struct representing a generator of the standard Artin braid group.
///
/// Geometrically, a positive (negative) Artin generator represents a positive (negative) crossing
/// of two adjacent strands. Thus if we think of the braid strands as stacked vertically and
/// oriented left-to-right, then a positive (negative) Artin generator corresponds to the crossing
/// of a strand under (over) the strand immediately above it.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArtinGenerator {
    foot: Strand,
    sign: Sign,
}

impl ArtinGenerator {
    /// Constructor for ArtinGenerator
    pub fn new(foot: u16, sign: Sign) -> Result<Self, ArtinValidationError> {
        let foot = Strand::new(foot).context("Failed to construct foot strand.")?;
        Ok(Self { foot, sign })
    }

    /// Accessor for `foot` strand field
    pub fn foot(&self) -> Strand {
        self.foot
    }
    /// Accessor for `sign` field
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Compute minimal required braid index for this generator
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.foot.index() + 1).unwrap()
    }
}

impl Neg for ArtinGenerator {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.foot.index(), -self.sign).unwrap()
    }
}

macro_rules! artin {
    ($foot:expr; 0) => {};
    ($foot:expr; +) => {};
    ($foot:expr; -) => {};
    ($foot:expr; +$power:expr) => {};
    ($foot:expr; -$power:expr) => {};
}

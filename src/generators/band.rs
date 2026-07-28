use anyhow::Context;

use crate::{ArtinGenerator, BraidIndex, Sign, Strand};
use std::ops::Neg;

/// Error type representing failures that may occur during construction of `BandGenerator`
#[derive(thiserror::Error, Debug)]
pub enum BandValidationError {
    #[error("{0}")]
    ArtinConversionFailure(String),
    #[error("foot strand and head strand are the same ({0:?})")]
    FootOnHead(Strand),
    #[error("foot strand ({foot:?}) is over head strand ({head:?})")]
    FootOverHead { foot: Strand, head: Strand },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Struct representing a generator in the band presentation of a braid group.
///
/// Geometrically, a positive (negative) band generator may be thought of as the crossing of a
/// "head" strand over (under) a "foot" strand, where the index of the head strand is _at least_
/// one greater than that of the foot strand, and where the two interchanging strands pass _over_
/// all intermediate strands. Thus, the standard Artin braid generators are simply band generators
/// where the index of the head strand is exactly one greater than that of the foot strand.
///
/// Algebraically, band generators and Artin generators are related as follows. Suppose that
/// $b_{f, h}^{\pm 1}$ denotes a band generator from foot strand with index $f$ to head strand with
/// index $h$. Suppose that $a_i$ denotes the Artin generator in which, geometrically, strand
/// $(i+1)$ passes over strand $i$. There are in fact $h - f$ ways to decompose $b_{f, h}^{\pm 1}$
/// as a product of Artin generators, but we shall employ the following convention by default:
/// $$b_{f, h}^{\pm 1}=a_f^{-1}a_{f+1}^{-1}\cdots a_{h-1}^{-1}a_h^{\pm 1}a_{h-1}\cdots a_{f+1}a_f.$$
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BandGenerator {
    foot: Strand,
    head: Strand,
    sign: Sign,
}

impl BandGenerator {
    /// Constructor for `BandGenerator`
    pub fn new(foot: u16, head: u16, sign: Sign) -> Result<Self, BandValidationError> {
        let foot = Strand::new(foot).context("Failed to construct foot strand.")?;
        let head = Strand::new(head).context("Failed to construct head strand.")?;
        if foot == head {
            return Err(BandValidationError::FootOnHead(foot));
        }
        if foot > head {
            return Err(BandValidationError::FootOverHead { foot, head });
        }
        Ok(Self { foot, head, sign })
    }
    pub fn from_artin(band_parts: &[ArtinGenerator]) -> Result<Self, BandValidationError> {
        let mut band_parts = band_parts;
        let mut upper_staircase = Vec::new();
        let mut lower_staircase = Vec::new();

        while let Some(left_step) = band_parts.first() {
            if left_step.sign() == Sign::Positive {
                upper_staircase.push(left_step)
            } else {
                lower_staircase.push(left_step)
            }

            band_parts = &band_parts[1..];

            if let (Some(upper_step), Some(lower_step)) =
                (upper_staircase.last(), lower_staircase.last())
                && upper_step.foot() == lower_step.foot() + 2
            {
                break;
            }
        }

        let crossing = if let Some(crossing) = band_parts.first() {
            crossing
        } else {
            return Err(BandValidationError::ArtinConversionFailure(
                "Valid crossing generator not found.".to_string(),
            ));
        };
        let foot = if let Some(step) = lower_staircase.first() {
            step.foot()
        } else {
            crossing.foot()
        };
        let head = if let Some(step) = upper_staircase.first() {
            step.foot() + 1
        } else {
            crossing.foot() + 1
        };

        while let Some(right_step) = band_parts.first() {
            let left_step = if right_step.sign() == Sign::Positive {
                lower_staircase.pop()
            } else {
                upper_staircase.pop()
            };

            if left_step.is_none_or(|s| *s != -*right_step) {
                return Err(BandValidationError::ArtinConversionFailure(format!(
                    "Generators should be inverses: {:?} != -{:?}",
                    left_step, right_step,
                )));
            }

            band_parts = &band_parts[1..]
        }

        if !lower_staircase.is_empty() || !upper_staircase.is_empty() {
            return Err(BandValidationError::ArtinConversionFailure(
                "List of band parts is not balanced with respect to crossing.".to_string(),
            ));
        }

        Self::new(foot.index(), head.index(), crossing.sign())
    }

    /// Accessor for `foot` strand field
    pub fn foot(&self) -> Strand {
        self.foot
    }
    /// Accessor for `head` strand field
    pub fn head(&self) -> Strand {
        self.head
    }
    /// Accessor for `sign` field
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Computes the height of the `BandGenerator`, that is, the difference in indices of its head
    /// and foot strands.
    pub fn height(&self) -> u16 {
        self.head.index() - self.foot.index()
    }
    /// Computes whether the band generator is equivalent to an Artin generator.
    pub fn is_artin(&self) -> bool {
        self.height() == 1
    }
    /// The minimal braid index required to define the braid.
    pub fn minimal_required_braid_index(&self) -> BraidIndex {
        BraidIndex::new(self.head.index()).unwrap()
    }
    pub fn artin_length(&self) -> u16 {
        1 + (self.height() - 1) * 2
    }
}

impl Neg for BandGenerator {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.foot.index(), self.head.index(), -self.sign).unwrap()
    }
}

macro_rules! _band {
    ($foot:expr, $head:expr; 0) => {
        Vec::<BandGenerator>::new()
    };
    ($foot:expr, $head:expr; +) => {
        BandGenerator::new($foot, $head, Sign::Positive)
    };
    ($foot:expr, $head:expr; -) => {
        BandGenerator::new($foot, $head, Sign::Negative)
    };
    ($foot:expr, $head:expr; +$power:expr) => {
        {
            let mut word = Vec::<BandGenerator>::new();
            for _ in ..$power {
                word.push(band![$foot, $head; +]);
            }
            word
        }
    };
    ($foot:expr, $head:expr; -$power:expr) => {
        {
            let mut word = Vec::<BandGenerator>::new();
            for _ in ..$power {
                word.push(band![$foot, $head; -]);
            }
            word
        }

    };
}

use crate::{BraidIndex, Sign, Strand};
use std::ops::Neg;

/// Error type representing failures that may occur during construction of `BandGenerator`
#[derive(thiserror::Error, Debug)]
pub enum BandValidationError {
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
    pub fn new(foot: Strand, head: Strand, sign: Sign) -> Result<Self, BandValidationError> {
        if foot == head {
            return Err(BandValidationError::FootOnHead(foot));
        }
        if foot > head {
            return Err(BandValidationError::FootOverHead { foot, head });
        }
        Ok(Self { foot, head, sign })
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
        Self::new(self.foot, self.head, -self.sign).unwrap()
    }
}

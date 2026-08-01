#![warn(missing_docs)]
//! Define and manipulate [mathematical braids](https://en.wikipedia.org/wiki/Braid_group).
//!
//! # Braids and Generating Sets
//!
//! We think of a braid as an _algebraic encoding_ of _geometric_ information: namely, a pattern of
//! weaving among a collection of disjoint strands (or strings). The [`Braid`] struct encodes this
//! information, and is thus the heart of the crate. Specifically, a [`Braid`] consists of a
//! [`BraidIndex`]-valued [`Braid::index()`] field, which is a wrapped `u16` measuring the
//! number of [strands][Strand], as well as a _word_ (which amounts to a [`Vec`]) in one of two
//! generating sets, which we now describe.
//!
//! An _Artin generator_, represented by the [`ArtinGenerator`] struct, encodes the crossing of two
//! adjacent strands. Thus every [`ArtinGenerator`] is specified by a single [`Strand`], together
//! with a [`Sign`]: positive or negative. If we think of the strands of a braid as being arranged
//! as a vertical stack of parallel lines in the plane (except for where the crossings occur),
//! oriented left-to-right, then the [`Strand`] contained in an [`ArtinGenerator`] corresponds to
//! the [`Strand`] which is initially on bottom (we refer to this as the
//! [foot](ArtinGenerator::foot()) of the generator), before exchanging positions with the strand
//! above it (referred to as the _head strand_). The [sign](ArtinGenerator::sign()) of the generator
//! determines whether the foot strand passes under (a [positive][Sign::Positive] crossing) or over
//! (a [negative][Sign::Negative] crossing) the head strand.
//!
//! While the Artin generators are _canonical_ for presenting braid groups, internally we actually
//! use a different generating set for our [braids](Braid): the _band generators_, represented by
//! the [`BandGenerator`] struct. A band generator is similar to an Artin generator, except that the
//! requirement that the two crossing strands be adjacent is relaxed: every [`BandGenerator`]
//! contains both a [foot strand](BandGenerator::foot()) and an explicit
//! [head strand](BandGenerator::head()) (which can be arbitrarily many strands _above_ the foot
//! strand), as well as a [sign](BandGenerator::sign()). The semantics
//! of the [sign][Sign] are the same as with Artin generators: a [positive][Sign::Positive] sign
//! indicates that the foot strand passes _under_ the head strand, while a
//! [negative][Sign::Negative] sign indicates that the foot strand passes _over_ the head strand;
//! regardless of sign, the interchanging strands should be understood as passing _over_ all of the
//! intermediate strands.
//!
//! Note that every Artin generator is a band generator where the foot and head strands are distance
//! 1 apart. Likewise, if we think of concatenation of generators as simply placing their geometric
//! representatives side-by-side, then every band generator decomposes (non-uniquely) into a product
//! of 2*([height](BandGenerator::height()) - 1) + 1 Artin generators, where
//! [height](BandGenerator::height()) is the index of the head strand minus that of the foot strand.
//!
//! # Defining Braids
//!
//! Braids can be defined by first specifying a list of generators in a particular generating set
//! and then passing it to the appropriate constructor:
//!
//! ```
//! use braided::{ArtinGenerator, BandGenerator, Braid, Sign, artin, band};
//!
//! // You can define Artin generators using the ArtinGenerator::new function, or the artin! macro:
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! let my_artin = ArtinGenerator::new(1, Sign::Negative).unwrap();
//! let my_two_artins = artin![3; 2].unwrap(); // Vec<ArtinGenerator>
//!
//! let braid_from_artins = Braid::from_artin(4, &[
//!     vec![my_artin],              // a crossing of strand 1 over strand 2
//!     artin![2; 1].unwrap(),       // a crossing of strand 3 over stand 2
//!     artin![1; 1].unwrap(),       // a crossing of strand 2 over strand 1
//!     my_two_artins,               // two consecutive crossings of strand 4 over strand 3
//!     artin![2; -2].unwrap(),      // two consecutive crossings of strand 2 over stand 3
//! ].concat()).unwrap();
//!
//! // You can define band generators using the BandGenerator::new function, or the band! macro:
//! let my_band = BandGenerator::new(1, 3, Sign::Positive).unwrap();
//! let my_three_bands = band![2 => 4; 3].unwrap(); // Vec<BandGenerator>
//!
//! let braid_from_bands = Braid::from_bands(4, &[
//!     vec![my_band],               // a crossing of strand 1 over strand 3
//!     my_three_bands,              // three consecutive crossings of strand 4 over strand 2
//!     band![1 => 4; -4].unwrap(),  // four consecutive crossings of strand 1 over strand 4
//!     // Band generators are a superset of Artin generators:
//!     band![2 => 3; -1].unwrap(),  // same as artin![2; -1]
//! ].concat()).unwrap();
//!
//! // Even though we defined them using different generating sets, the two braids are still
//! // interoperable:
//! assert_eq!(braid_from_artins.artin_length(), 7);
//! assert_eq!(braid_from_artins.band_length(), 5);
//! assert_eq!(braid_from_bands.artin_length(), 33);
//! assert_eq!(braid_from_bands.band_length(), 9);
//! # }
//! ```
//!
//! There is also a [braid!] macro which allows defining a braid from any combination of generators:
//!
//! ```
//! use braided::{BraidIndex, braid};

//! // The syntax of the braid! macro requires first the index, then the sequence of generators.
//! // The generator syntax is similar to that of the corresponding generator constructor macros:
//! //     `[foot; power]` -> `power` consecutive Artin generators w/ foot index `foot`;
//! //     `[foot => head; power]` -> `power` consecutive bands w/ foot (head) index `foot` (`head`)
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! let my_cool_braid = braid![5;
//!     [1; 7],
//!     [2 => 4; 3],
//!     [3; -2],
//!     [1 => 4; -5],
//! ].unwrap();
//!
//! // The given index can be larger that strictly required by the given generators.
//! assert_eq!(my_cool_braid.index(), BraidIndex::new(5).unwrap());
//! assert_eq!(my_cool_braid.minimal_required_braid_index(), BraidIndex::new(4).unwrap());
//! # }
//! ```
//!
//! # Braid Arithmetic
//!
//! The collection of braids of a fixed index `N` forms a
//! [group](https://en.wikipedia.org/wiki/Group_(mathematics)), meaning that braids can be
//! associatively "multiplied" and "inverted", and moreover the multiplication operation has an
//! identity element: namely, the _trivial braid_ with `N` strands. The multiplication operation
//! is simply concatenation of generator words.
//!
//! <div class="warning">
//!
//! As of this writing, `braided` is still in an early phase with respect to braid arithmetic. In
//! particular, there is not yet any functionality to apply simple algebraic manipulations to a
//! given braid, outside of the above-mentioned multiplication and inversion. In particular:
//!
//! 1. There is no way to cancel an adjacent pair of inverse generators.
//! 2. There is no way to apply the "far commutativity" relations in order to swap two adjacent
//!    non-interacting generators.
//! 3. There is no way to apply the "braid" relations (i.e., a
//!    [Reidemeister III move](https://en.wikipedia.org/wiki/Reidemeister_move)).
//!
//! To be clear: all of these and many other geometrically inspired manipulations are planned for
//! implementation.
//!
//! </div>
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! let braid_1 = braid![4; [1 => 4; 1], [1 => 3; -2]].unwrap();
//! let braid_2 = braid![5; [2 => 5; 3], [1; -4]].unwrap();
//! // Braids in `braided` needn't have the same index when multiplying: the product will have the
//! // greater of the two indices.
//! assert_eq!(
//!     braid_1.clone() * braid_2.clone(),
//!     braid![5; [1 => 4; 1], [1 => 3; -2], [2 => 5; 3], [1; -4]].unwrap(),
//! );
//!
//! // Inversion reverses the word and inverts the sign of each generator:
//! assert_eq!(braid_1.clone().inverse(), braid![4; [1 => 3; 2], [1 => 4; -1]].unwrap());
//! assert_eq!(
//!     (braid_2.clone() * braid_1.clone()).inverse(),
//!     braid_1.clone().inverse() * braid_2.clone().inverse(),
//! );
//!
//! // While auto-simplification is not yet implemented, we can still see the effect of inversion
//! // on the "writhe" of the braid (i.e., the sum of all exponents of the braid):
//! assert_eq!(braid_1.inverse().writhe(), 1);
//! assert_eq!((braid_2.clone() * braid_2.inverse()).writhe(), 0);
//! # }
//! ```

mod braid;
mod generators;
mod index;
mod macros;
mod sign;
mod strand;

pub use braid::{Braid, BraidValidationError};
pub use generators::{
    ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError, artin_to_band,
    band_to_artin,
};
pub use index::{BraidIndex, IndexValidationError};
pub use sign::Sign;
pub use strand::{Strand, StrandValidationError};

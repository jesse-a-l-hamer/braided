#![warn(missing_docs)]
//! A library for working with [mathematical braids](https://en.wikipedia.org/wiki/Braid_group).
//!
//! Braids have several equivalent definitions of varying degrees of mathematical sophistication.
//! Here is a sample of their many interpretations:
//!
//! - Geometric objects, being _weaving patters_ among a collection of disjoint strands.
//! - Algebraic objects, being elements of some formal presentation of a _braid group_.
//! - Physical objects, being motions of a set of distinct points in the plane, where the set of
//!   points at the start of the motion is the same as that at the end.
//! - A [symmetry](https://en.wikipedia.org/wiki/Mapping_class_group) of a disk with several punctures.
//!
//! In `braided`, the first two interpretations are emphasized: namely, the implementation focuses on
//! the _algebraic_ nature of braids, as this lends itself to computational applications; yet my
//! goal in implementing this library is to support applications that require _vizualizing_ braids,
//! Thus, while I would eventually like this library to be as comprehensive a computational resource
//! for braids as possible, if in the earlier stages of development certain features or properties
//! of braids are prioritized over others, know that this goal is likely playing a part.
//!
//! # Quick Start
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! use braided::{braid, letter, word};
//!
//! // Use the letter! macro to define individual letters of a braid word:
//!
//! // Artin letters are generators in the standard (i.e., "Artin") presentation of the braid group:
//! let artin_letter = letter![1; +]; // crossing of strand 1 under strand 2
//! let other_artin_letter = letter![2; -]; // crossing of strand 2 over strand 3
//!
//! // Band letters are generalized Artin generators, representing crossings of arbitrary strands:
//! let band_letter = letter![2 => 3; -]; // same as `other_artin_letter`
//! let other_band_letter = letter![1 => 3; +]; // crossing of strand 1 under strand 3
//!
//! // Letters can be multiplied to form words: formal sequences of letters
//! let artin_word = artin_letter * other_artin_letter;
//! let band_word = band_letter * other_band_letter;
//!
//! // Two words can also be multiplied; mixing generator sets is fine
//! let combined_word = artin_word * band_word;
//!
//! // We can also define words directly using the word! macro:
//! assert_eq!(combined_word, word![[1; 1], [2; -2], [1 => 3; 1]]);
//!
//! // A braid consists of a braid index and a word:
//! let my_9_braid = braid![(9); [1; 2], [2 => 5; -7], [3; 3], [1 => 4; 2]];
//!
//! // The index can also be inferred from the given word:
//! let my_other_9_braid = braid![(); [1 => 8; 3], [2 => 9; -4]];
//!
//! // Two braids can be multiplied...
//! assert_eq!(
//!     &my_9_braid * my_other_9_braid,
//!     braid![(); [1; 2], [2 => 5; -7], [3; 3], [1 => 4; 2], [1 => 8; 3], [2 => 9; -4]]
//! );
//!
//! // ... but only if the braid indices match!
//! use braided::{BraidValidationError, BraidIndex};
//!
//! assert_eq!(
//!     *(my_9_braid * braid![(); [1; 1]]), // See note below about the deref operator here
//!     Err(BraidValidationError::UnequalIndices {
//!             left: BraidIndex::try_new(9).unwrap(),
//!             right: BraidIndex::try_new(2).unwrap(),
//!     }),
//! );
//!
//! // In order for the multiplication operation to be as ergonomic as possible, the return type is
//! // actually a newtype `BraidResult` wrapping a `Result<Braid, BraidValidationError>`. The
//! // `std::ops::Deref` trait is implemented on `BraidResult`, allowing for easy access to the
//! // wrapped inner `Result<_, _>`.
//! # }
//! ```
//!
//! # Less Quick Start
//!
//! The easiest and quickest way to define a braid is with the [`braid!`] macro:
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! use braided::braid;
//!
//! let cool_braid = braid![
//!     (3);
//!     [1; 4],
//!     [1 => 3; -3],
//!     [2; -1],
//! ];
//! # }
//! ```
//!
//! Let's break down what's happening in the previous statement. The macro [braid!] attempts to
//! construct a [`Braid`] struct, which is the central object of the library. I say "attempts", as
//! this process is fallible: for more extensive documentation on the myriad ways in which
//! constructing a braid can go wrong, see the documentation for [`BraidValidationError`]. See also
//! [`BraidResult`] for the actual return type which wraps a
//! [`Result<Braid, BraidValidationError>`].
//!
//! The first line of input to [`braid!`] defines the [braid index](BraidIndex) of the braid, which
//! may be thought of as the number of strands of which the braid is comprised. In this case, we
//! have defined a braid index of 3:
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! # use braided::braid;
//! #
//! # let cool_braid = braid![
//! #     (3);
//! #     [1; 4],
//! #     [1 => 3; -3],
//! #     [2; -1],
//! # ];
//! #
//! use braided::BraidIndex;
//!
//! assert_eq!(cool_braid.clone_unwrap().braid_index(), BraidIndex::try_new(3).unwrap());
//! # }
//! ```
//!
//! <div class="warning">
//!
//! You may have noticed that in the previous codeblock we call `cool_braid.clone_unwrap` instead of
//! `cool_braid.unwrap`. This is because the actual type of `cool_braid` is [`BraidResult`] instead
//! of [`Result<Braid, BraidValidationError>`]. [`BraidResult`] is a wrapper around
//! [`Result<Braid, BraidValidationError>`] which implements [`std::ops::Deref`], but because
//! [`Braid`] is not [`Copy`], calling `.unwrap()` directly becomes cumbersome. This is why we have
//! the [`BraidResult::clone_unwrap`] method, which essentially clones the inner
//! [`Result<Braid, BraidValidationError>`] value before unwrapping it. Similarly, we can use
//! [`BraidResult::clone_unwrap_err`] to unwrap a contained [`Err`] variant.
//!
//! See also [`WordResult`], which implements similar functionality, but for [words](Word).
//!
//! </div>
//!
//! In addition to a [braid index](BraidIndex), a [braid](Braid) also contains a [_word_](Word),
//! which is what describes the "weaving" pattern of the braid. This is what the remaining three
//! lines of input to the [`braid!`] macro are specifying, so let's now try to understand how this
//! syntax works.
//!
//! Imagine that the three strands of `cool_braid` are three literal pieces of string which we have
//! initially laid out before us as three parallel, horizontal lines. Let us label each string with
//! a number, starting with 1 for the bottom strand, 2 in the middle, and 3 at the top. Now we start
//! interweaving the strands, keeping track of which string crosses over which; importantly, when
//! two strands interchange positions via a crossing, the strands adopt the label of the _position_
//! into which they have been placed. Thus, for example, if we interchange the bottom string with
//! the middle string, the new label for the botton string becomes "2", while the new label for the
//! middle string becomes "1". In this way, we can assign a label to every possible crossing of two
//! strands, which we call a [letter](Letter) of the braid.
//!
//! Letters come in two varieties: [Artin letters](Letter::Artin) and [band letters](Letter::Band),
//! both of which can be constructed with the [`letter!`] macro:
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! use braided::letter;
//!
//! let artin_letter = letter![1; +];
//! let band_letter = letter![1 => 3; -];
//! # }
//! ```
//!
//! An [Artin letter](Letter::Artin), also called an [Artin generator](ArtinGenerator), describes a
//! crossing of two _adjacent_ strands, and is specified by two pieces of data: a positive integer
//! and a sign (+ or -). The integer, called the _foot strand_ of the letter, denotes the index of
//! the strand which is on bottom _before_ the crossing (the other strand involved in the crossing
//! is called the _head strand_). The sign indicates which strand crosses over which: if the sign is
//! +, then the foot strand crosses _under_ the head strand, while in a - crossing the foot strand
//! crosses _over_ the head (this naming convention comes from the right hand rule, which one may
//! verify is respected by choosing any orientation on the parallel strands in which both are
//! oriented in the same direction). Thus in the code block above, `artin_letter` is an _Artin
//! letter_ denoting the crossing of strand 2 _over_ strand 1.
//!
//! A [Band letter](Letter::Band), also called a [Band generator](BandGenerator), describes a
//! crossing of two strands of arbitrary distance apart. Thus we need _three_ pieces of data to
//! describe a band letter: a foot strand index, a sign, and now an _explicitly_ given head strand
//! index. In the code block above, `band_letter` is a _band letter_ denoting the crossing of strand
//! 1 _under_ strand 3.
//!
//! One may note (e.g., if one ir following along with literal string), that it is not possible to
//! cross strand 1 over strand 3 without crossing both over strand 2 (well, unless one's braids
//! exist on a sphere rather than a plane)! Indeed, this observation amounts to the fact that the
//! Artin letters suffice to _generate_ the braid group: so that in particular, every band letter
//! must be expressible in terms of Artin letters. For example, one way to
//! [decompose](Braid::decompose) the band letter `[1 => 3; -]` is as the sequence of Artin letters
//! `[1, -], [2, -], [1, +]`.
//!
//! A natural question then arises: why even bother with band generators if they're really just
//! sequences of Artin generators? Well, the answer is: convenience. To be sure, if one only wants
//! to denote a crossing of adjacent strands, then the head strand does not need to be specified:
//! Artin generators are more efficient here. However, as is illustrated with `[1 => 3; -]` above,
//! in general it takes `2 * height - 1` Artin letters to denote any given band letter, where
//! `height` denotes the distance between the foot and head strands. The relative efficiency of band
//! letters thus becomes apparent if one wants to describe any crossing of non-adjacent strands.
//!
//! Let us now return to our braid `cool_braid` defined above. We have already discussed the meaning
//! of the [braid index](BraidIndex), which is defined within [`braid!`] as the parenthetical
//! expression `(3);`. The remaining inputs to [`braid!`]---the three bracketed
//! expressions---collectively define the [_word_](Word) of `cool_braid`, or the sequence of
//! [letters](Letter) describing the braid's "weaving pattern". Every such bracketed expression
//! denotes a single [letter](Letter) of the braid [word](Word) together with an _exponent_ which
//! describes both the [sign](Sign) of the [letter](Letter) as well as the number of times it is
//! repeated at that position in the braid word. Thus `[1; 4]` denotes the _Artin_ letter `[1; +]`
//! repeated four times; `[1 => 3; -3]` denotes the _band_ letter `[1 => 3, -]` repeated three
//! times, and `[2; -1]` denotes a single occurrence of the letter `[2; -]`:
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! # use braided::{braid, letter};
//! #
//! # let cool_braid = braid![
//! #     (3);
//! #     [1; 4],
//! #     [1 => 3; -3],
//! #     [2; -1],
//! # ];
//! #
//! assert_eq!(cool_braid.clone_unwrap().letters(), vec![
//!     letter![1; +].unwrap(), // Letter construction is fallible too, so we must unwrap
//!     letter![1; +].unwrap(),
//!     letter![1; +].unwrap(),
//!     letter![1; +].unwrap(),
//!     letter![1 => 3; -].unwrap(),
//!     letter![1 => 3; -].unwrap(),
//!     letter![1 => 3; -].unwrap(),
//!     letter![2; -].unwrap(),
//! ]);
//! # }
//! ```
//!
//! Note that the [braid index](BraidIndex) can be omitted in the [`braid!`] macro, in which case
//! the index is inferred from the given [word](Word):
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! # use braided::{braid, letter};
//! #
//! # let cool_braid = braid![
//! #     (3);
//! #     [1; 4],
//! #     [1 => 3; -3],
//! #     [2; -1],
//! # ];
//! #
//! let implicit_index_cool_braid = braid![
//!     (); // You still have to write the parentheses!
//!     [1; 4],
//!     [1 => 3; -3],
//!     [2; -1],
//! ];
//!
//! assert_eq!(implicit_index_cool_braid, cool_braid);
//! # }
//! ```
//!
//! You might have noticed that, as a braid word is really just a sequence of letters, we could
//! easily extend our "weaving pattern" by extending `cool_braid` with some additional letters at
//! its start or end. Indeed, this is because braids (of a fixed braid index) collectively form a
//! [group](https://en.wikipedia.org/wiki/Group_(mathematics)). That is:
//!
//! - You can _multiply_ two braids together.
//! - This multiplication operation is
//!   [associative](https://en.wikipedia.org/wiki/Associative_property).
//! - There is a multiplicative identity, meaning a braid which, when multiplied by some other
//!   braid, leaves that other braid unchanged.
//! - Every braid has an inverse with respect to the braid product, that is, an element both of
//!   whose products with the original braid yield the identity.
//!
//! The multiplication operation between two `N`-stranded braid simply concatenates their braid
//! words in the same order as the braids in the product (thus braid multiplication is _not_
//! commutative!).
//!
//! Geometrically, we can imagine that we have two sets of `N` interwoven strands sitting
//! side-by-side. To "multiply" these two geometric braids together is simply to "glue" adjacent
//! strands together so that the result is again a _single_ collection of `N` interwoven strands.
//!
//! Let's see how this structure is reflected in `braided`.
//!
//! First, two braids can be multiplied with the usual `*` operator:
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! use braided::braid;
//!
//! // Note that both braids have braid index 9, even though the word of each does not make use of
//! // all 9 strands; the word only enforces a minimality constraint on the braid.
//! let braid1 = braid![(9); [2 => 5; 3], [1; -2], [4; 3], [2 => 4; -7]];
//! let braid2 = braid![(9); [1; 3], [1 => 3; -4], [2 => 3; 1]];
//!
//! assert_eq!(
//!     &braid1 * &braid2, // Multiplication consumes operands unless you explicitly borrow
//!     braid![(9); [2 => 5; 3], [1; -2], [4; 3], [2 => 4; -7], [1; 3], [1 => 3; -4], [2 => 3; 1]],
//! );
//!
//! assert_eq!(
//!     braid2 * braid1,
//!     braid![(9); [1; 3], [1 => 3; -4], [2 => 3; 1], [2 => 5; 3], [1; -2], [4; 3], [2 => 4; -7]],
//! );
//! # }
//! ```
//!
//! The multiplicative identity is the _trivial braid_ of a given [braid index](BraidIndex), which
//! is the braid of the given index whose [word](Word) is empty. Geometrically, the trivial braid
//! with `N` strands amounts to `N` parallel strands, with no crossings between them. We construct a
//! trivial braid by simply omitting the word in the [`braid!`] macro (the braid index must be
//! explicitly specified though!):
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! use braided::braid;
//!
//! let trivial_9_braid = braid![(9)].clone_unwrap();
//!
//! assert!(trivial_9_braid.is_trivial());
//!
//! let some_9_braid = braid![(9); [1 => 8; 1], [2 => 9; -1]];
//!
//! assert_eq!(&trivial_9_braid * &some_9_braid, some_9_braid);
//! assert_eq!(&some_9_braid * &trivial_9_braid, some_9_braid);
//! # }
//! ```
//!
//! The multiplicative inverse of a [braid](Braid) can be obained by reversing its [word](Word) and
//! negating the exponent of each [letter](Letter). We can easily compute inverses using the
//! [inverse](Braid::inverse) method defined on the braid:
//!
//! ```
//! # #[macro_use] extern crate braided;
//! # fn main() {
//! use braided::braid;
//!
//! let braid = braid![(); [1 => 3; 4], [2 => 5; -3], [4; -7], [3; 1]].clone_unwrap();
//!
//! assert_eq!(
//!     braid.inverse(),
//!     braid![(); [3; -1], [4; 7], [2 => 5; 3], [1 => 3; -4]].clone_unwrap()
//! );
//! # }
//! ```
//!
//! Note however that there is currently no simplification mechanism (though this will eventually be
//! fixed). Thus inverses are purely formal at the moment---the multiplication of a braid with its
//! inverse will not (at the moment) simplify to a trivial braid.
//!
//! Thus concludes our brief introduction to defining and manipulating braids in `braided`! There's
//! much more to explore in the library, and much more to braids that I plan to implement into the
//! library! For example, the above discussion is completely devoid of any mention of the defining
//! relations that make braids _braids_, such as _far commutativity_ and the _braid relations_. Fear
//! not: one of the earliest features I plan to implement is a system for both recognizing and
//! applying "moves" to braids. This system of "moves" will include both of the above-mentioned
//! relations, along with many others!
//!
//! For the reader wishing to dive deeper into the current state of the library, I
//! recommend starting with the docs for the [Braid] struct and exploring from there. One may also
//! wish to look at the docs for the [braid!] macro to better understand its syntax.
//!
//! # Planned Features & Improvements
//!
//! `braided` is still very much in development, and the API should be considered unstable until a
//! `v1.0.0` release. Here is a checklist of planned features and improvements which I would like to
//! see implemented before bumping the version to `v1.0.0`:
//!
//! - [x] Low-level braid component types
//!   - [x] [`Sign`]
//!   - [x] [`Strand`]
//!   - [x] [`BraidIndex`]
//!   - [x] [`ArtinGenerator`]
//!   - [x] [`BandGenerator`]
//! - [x] High-level braid component & braid types
//!   - [x] [`Letter`]
//!   - [x] [`Word`]
//!   - [x] [`Braid`]
//! - [x] High-level constructor macros
//!   - [x] [`letter!`]
//!   - [x] [`word!`]
//!   - [x] [`braid!`]
//! - [x] Multiplication
//!   - [x] impl [`std::ops::Mul`] for [`Letter`], [`Word`], [`Braid`], and their borrowed variants.
//!   - [x] impl [`std::ops::Mul`] for [`LetterResult`], [`WordResult`],[`BraidResult`], and their
//!     borrowed variants.
//! - [ ] A `BraidMove` trait that encodes the notion of a geometric/algebraic manipulation
//!   transforming one braid into another in some controlled way.
//! - [ ] Concrete types implementing `BraidMove`:
//!   - [ ] Simplification/cancellation
//!   - [ ] Far commutativity
//!   - [ ] Braid relations
//!   - [ ] Band slides
//!   - [ ] Conjugation
//!   - [ ] Subword cycling (a variant of conjugation in which a subword at one end of a braid is
//!     "cycled" to the other end).
//!   - [ ] Strand cycling (i.e., moving the top strand to the bottom by "wrapping around the back
//!     of the sphere"; could also be called a "sphere move")
//!   - [ ] Others?
//! - [ ] An `Isotopy` type that represents a sequence of moves transforming one braid into another.
//! - [ ] Implementation of algorithms for producing various "normal forms" which enable solutions
//!   to algebraic problems, such as the "word" problem or "conjugacy" problem in braid groups.
//! - [ ] Maybe a few other things that I'm forgetting at the moment... this list is subject to
//!   _growing_, but I don't anticipate that much will be removed.

mod braid;
mod error;
#[doc(hidden)]
pub mod generators;
mod index;
mod letter;
mod multiplication;
mod result;
mod sign;
mod strand;
mod word;

mod macros;

pub use braid::Braid;
pub use error::{
    ArtinValidationError, BandValidationError, BraidValidationError, IndexValidationError,
    LetterValidationError, StrandValidationError, WordValidationError,
};
pub use generators::{ArtinGenerator, BandGenerator};
pub use index::BraidIndex;
pub use letter::Letter;
pub use result::{
    ArtinResult, BandResult, BraidResult, IndexResult, LetterResult, StrandResult, WordResult,
};
pub use sign::Sign;
pub use strand::Strand;
pub use word::Word;

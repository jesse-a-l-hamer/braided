#![warn(missing_docs)]
//! This is a library for working with [mathematical braids](https://en.wikipedia.org/wiki/Braid_group).
//!

mod braid;
mod generators;
mod index;
mod letter;
mod macros;
mod sign;
mod strand;
mod word;

pub use braid::{Braid, BraidValidationError};
pub use generators::{ArtinGenerator, ArtinValidationError, BandGenerator, BandValidationError};
pub use index::{BraidIndex, IndexValidationError};
pub use letter::{Letter, LetterValidationError};
pub use sign::Sign;
pub use strand::{Strand, StrandValidationError};
pub use word::{Word, WordValidationError};

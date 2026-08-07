# Braided

[![Crates.io](https://img.shields.io/crates/v/braided.svg)](https://crates.io/crates/braided)
[![Documentation](https://docs.rs/braided/badge.svg)](https://docs.rs/braided)
[![Build Status](https://github.com/jesse-a-l-hamer/braided/workflows/CI/badge.svg)](https://github.com/jesse-a-l-hamer/braided/actions)

A library for working with [mathematical braids](https://en.wikipedia.org/wiki/Braid_group), written in Rust.

## Quick Start

> [!NOTE]
> If you are new to the theory of braids, it may help to instead see the _Less Quick Start_ given in the [docs](https://docs.rs/braided).

```rust
use braided::{braid, letter, word};

// Use the letter! macro to define individual letters of a braid word:

// Artin letters are generators in the standard (i.e., "Artin") presentation of the braid group:
let artin_letter = letter![1; +].unwrap(); // crossing of strand 1 under strand 2
let other_artin_letter = letter![2; -].unwrap(); // crossing of strand 2 over strand 3

// Band letters are generalized Artin generators, representing crossings of arbitrary strands:
let band_letter = letter![2 => 3; -].unwrap(); // same as `other_artin_letter`
let other_band_letter = letter![1 => 3; +].unwrap(); // crossing of strand 1 under strand 3

// Letters can be multiplied to form words: formal sequences of letters
let artin_word = (artin_letter * other_artin_letter).unwrap();
let band_word = (band_letter * other_band_letter).unwrap();

// Two words can also be multiplied; mixing generator sets is fine
let combined_word = artin_word * band_word;

// We can also define words directly using the word! macro:
assert_eq!(combined_word, word![[1; 1], [2; -2], [1 => 3; 1]]);

// Multiplication automatically cancels adjacent pairs of opposite-sign letters:
assert_eq!(
    (artin_letter * band_letter).unwrap() * letter![2; +].unwrap(), // word * letter is valid
    word![[1; 1]], // multiplication always produces a word, even if the result is one letter
);

// Words can be formally inverted, and the multiplication detects this:
let some_word = word![[1; 2], [2 => 5; -7], [3; 3], [1 => 4; 2]].unwrap();
assert_eq!(some_word.inverse() * some_word, word![]); // the product is trivial

// A braid consists of a braid index and a word:
let my_9_braid = braid![(9); [1; 2], [2 => 5; -7], [3; 3], [1 => 4; 2]].unwrap();

// The index can also be inferred from the given word:
let my_other_9_braid = braid![(); [1 => 8; 3], [2 => 9; -4]].unwrap();

// Two braids can be multiplied...
assert_eq!(
    my_9_braid.clone() * my_other_9_braid,
    braid![(); [1; 2], [2 => 5; -7], [3; 3], [1 => 4; 2], [1 => 8; 3], [2 => 9; -4]]
);

// ... but only if the braid indices match!
use braided::{BraidValidationError, BraidIndex};

assert_eq!(
    my_9_braid * braid![(); [1; 1]].unwrap(),
    Err(BraidValidationError::UnequalIndices {
            left: BraidIndex::new(9).unwrap(),
            right: BraidIndex::new(2).unwrap(),
    }),
);
```

## Planned Features & Improvements

`braided` is still very much in development, and the API should be considered unstable until a
`v1.0.0` release. Here is a checklist of planned features and improvements which I would like to
see implemented before bumping the version to `v1.0.0`:

- [x] Low-level braid component types
  - [x] `Sign`
  - [x] `Strand`
  - [x] `BraidIndex`
  - [x] `ArtinGenerator`
  - [x] `BandGenerator`
- [x] High-level braid component & braid types
  - [x] `Letter`
  - [x] `Word`
  - [x] `Braid`
- [x] High-level constructor macros
  - [x] `letter!`
  - [x] `word!`
  - [x] `braid!`
- [ ] Multiplication
  - [x] impl `std::ops::Mul` for `Letter`, `Word`, and `Braid`
  - [x] implement auto-cancellation for multiplication
  - [ ] impl `std::ops::Mul` for `Result<Letter, _>`, `Result<Word, _>`,
        and `Result<Braid, _>`
- [ ] A `BraidMove` trait that encodes the notion of a geometric/algebraic manipulation
      transforming one braid into another in some controlled way.
- [ ] Concrete moves which implement `BraidMove`:
  - [ ] Far commutativity
  - [ ] Braid relations
  - [ ] Band slides
  - [ ] Conjugation
  - [ ] Subword cycling (a variant of conjugation in which a subword at one end of a braid is
        "cycled" to the other end).
  - [ ] Strand cycling (i.e., moving the top strand to the bottom by "wrapping around the back
        of the sphere"; could also be called a "sphere move")
  - [ ] Others?
- [ ] An `Isotopy` type that contains a sequence of moves.
- [ ] Implementation of algorithms for producing various "normal forms" which enable solutions
      to algebraic problems, such as the "word" problem or "conjugacy" problem in braid groups.
- [ ] Maybe a few other things that I'm forgetting at the moment... this list is subject to
      _growing_, but I don't anticipate that much will be removed.

## Installation

Simply add `braided` to your `Cargo.toml`:

```toml
# Cargo.toml
# ...
[dependencies]
braided = "*"
```

## Documentation

- [docs.rs](https://docs.rs/braided)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## License

Licensed under the [MIT](./LICENSE) license.

## Acknowledgements

- The article [Production-Ready Rust Project Setup: From Zero to CI/CD](https://dev.to/ajitkumar/production-ready-rust-project-setup-from-zero-to-cicd-jp4) by Ajit Kumar was very helpful for getting the repo off the ground. In particular, the initial [CHANGELOG.md](./CHANGELOG.md) and [CONTRIBUTING.md](./CONTRIBUTING.md) files were taken almost verbatim from this article.
- This is my first real Rust project, and it would not have been possible without the wonderful book that is Luca Palmieri's [Zero To Production In Rust](https://www.zero2prod.com). Aside from its value as a learning resource, I have also made use in this repository of several project setup suggestions given in the text, in particular, the CI workflows.

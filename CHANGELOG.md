# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.0](https://github.com/jesse-a-l-hamer/braided/compare/v0.1.2...v0.2.0) - 2026-08-10

### Added

- *(multiplication::braid)* implement `Mul` for `&BraidResult`
- *(multiplication::word)* implement `Mul` for `&WordResult`
- *(result)* new `ResultWrapper` trait implemented for each `*Result` type to make accessing inner values easier
- *(multiplication::braid)* impl `Mul` for all combinations of borrowed and moved `Letter`, `Word`, `Braid`
- *(multiplication::word)* impl Mul for all combinations of borrowed/moved Letter and Word

### Other

- *(result)* formatting
- *(braid,error)* fix broken links
- *(README)* quick start fixes and roadmap updates
- *(braid,word)* remove unnecessary clones in some doctests involving multiplication
- *(lib)* fixup front page docs
- *(word)* change line in `Word::coalesce_decomposed` to better communicate unreachability
- *(braid)* add missing `try_from_letters` error case
- *(multiplication)* add unit tests covering all cases with error operands
- *(result)* write unit tests for `*Result` structs
- *(result)* document all `*Result` types
- *(error)* fixup existing docs
- *(index)* fixup existing docs
- *(strand)* fixup existing docs
- *(artin)* fixup existing docs
- *(band)* fixup existing docs
- *(letter)* fixup existing docs
- *(word)* fixup existing docs
- *(braid)* fixup existing docs
- *(braid)* [**breaking**] `Braid::try_new` now requires an explicit index and `Braid::try_from_letters` accepts an optional index
- *(macros)* fixup existing docs
- *(macros)* use `*Result::clone_unwrap()` and `*Result::clone_unwrap_err()` where appropriate
- *(braid)* [**breaking**] rename `Braid::from_data` to `Braid::try_from_data`
- *(result)* [**breaking**] replace `ResultWrapper` trait with `clone_unwrap` and `clone_unwrap_err` methods on `WordResult` and `BraidResult`
- *(macros, multiplication)* fixed macro calls broken due to new result types; all unit tests passing
- checkpoint commit - all fallible constructors now return a special `*Result` newtype struct
- *(multiplication::braid)* add unit tests to check multiplication with result types
- *(multiplication::word)* add unit tests to check multiplication with result types
- *(multiplication::letter)* add unit tests to check multiplication with result types
- *(multiplication)* add `result` module with `Result<_, _>` newtypes
- *(lib, braid, word)* replace unnecessary clones with borrows in multiplication doctests
- *(README)* Add link to "Less Quick Start" section of docs within README
- *(braid, letter, word)* factored out all impls of `Mul` into dedicated `multiplication` module

## [0.1.2](https://github.com/jesse-a-l-hamer/braided/compare/v0.1.1...v0.1.2) - 2026-08-08

### Other

- *(Cargo.toml)* exclude .github/ from packaged files
- *(gitignore)* delete lcov.info file and add its name to .gitignore
- *(gitignore)* remove existing coverage/ directory and add coverage/ to gitignore
- *(PULL_REQUEST_TEMPLATE)* add "Other" option to "Type of Change" checklist

## [0.1.1](https://github.com/jesse-a-l-hamer/braided/compare/v0.1.0...v0.1.1) - 2026-08-08

### Other

- add release-plz.yml workflow

## [0.1.0](https://github.com/jesse-a-l-hamer/braided/releases/tag/v0.1.0) - 2026-08-08

This is the first release of the project! I consider this an MVP for a library about braids. See the following sections for auto-generated updates from commit messages. Here are some of the highlights:

- Implementation of core types for representing braids:
  - `Sign`: enum representing the sign of a crossing.
  - `Strand`: wrapped `u16` representing a braid strand. Can be added/subtracted.
  - `BraidIndex`: wrapped `u16` representing the total number of strands in a braid.
  - `ArtinGenerator`: represents a crossing of adjacent strands.
  - `BandGenerator`: represents a crossing of arbitrarily distant strands.
  - `Letter`: enum abstracting over generators; the basis of a braid.
  - `Word`: wrapper struct around a `Vec<Letter>`; represents a formal braid word.
  - `Braid`: struct consisting of a `BraidIndex` and a `Word`.
- For each of the types above (except for `Sign`), implemented a `...ValidationErr` type (e.g., `StrandValidationError`, `BraidValidationError`) to keep track of possible construction failures.
- Implement constructor macros `letter!`, `word!`, and `braid!` which expose a much more ergonomic interface for defining the corresponding objects.
- Implemented basic braid multiplication:
  - Can multiply two `Letter`, producing a `Word`.
  - Can multiply a `Letter` and `Word`, producing a `Word`.
  - Can multiply two `Word`, producing a `Word`.
  - Can multiply a `Letter` and a `Braid`, producing a `Braid`.
  - Can multiply a `Word` and a `Braid`, producing a `Braid`.
  - Can multiply a `Braid` and a `Braid`, producing a `Braid`; requires that both braids have the same `BraidIndex`.
- Helper functions for constructing trivial objects (`Word` or `Braid` of given index).
- Helper functions for computing multiplicative inverses (e.g., `Braid::inverse()`).
- Can compute several basic properties of braids, like `Braid::writhe`, `Braid::artin_length`, etc.
- All public items documented.
- Crate root documented.


### Added

- *(word)* panic on seemingly unreachable code path in Mul impls for Word & Letter
- *(braid)* impl `TryFrom<Vec<L>>` and `TryFrom<&[L]>` on `Braid` where `L: TryInto<Letter>`
- *(word)* impl `Default` for `Word` as `Word::trivial`
- *(generators::artin)* remove needless `ArtinValidationError::Infallible` variant
- *(letter)* implement `Letter::is_artin`
- *(lib)* re-export `index::IndexValidationError` and `strand::StrandValidationError`
- *(braid)* derive `Clone` for `Braid` struct
- *(braid)* implement `Braid::minimal_required_braid_index` method
- *(license)* add MIT license and update Cargo.toml
- *(braid)* reworked braid macro to accept mixed generators
- *(braid)* first draft implmentation of constructor macro
- add readme
- *(generators::artin)* export artin macro and improve hygiene
- *(generators::{artin,band})* define constructor macros
- *(generators::conversion)* untested implementation of artin_to_band
- *(generators::band)* implement `BandGenerator::from_artin` constructor
- *(strand)* impl Add
- implemented logic to convert between band/artin generators
- initial commit; basic structure and minimal functionality laid out

### Fixed

- *(macros)* fix macro `word!` so that all branches (including trivial) return a result
- *(word)* multiplying two long words which cancel to a short one now returns Ok instead of Err
- *(letter)* bug in equality comparison `Letter::Band == Letter::Artin`
- *(braid)* fix use of `u16::max` instead of `u16::MAX`
- *(word)* replaced broken recursive word multiplication algorithm with much simpler and faster one
- *(word)* fix bug in Word multiplication impls where error message was reporting incorrect length
- *(generators::band)* imposed maximum band height to prevent construction of bands with Artin length exceeding `u16::MAX`
- *(macros)* change needlessly large `i64` to `i32` for `TryInto` of `$exponent` in `word!`
- *(strand)* properly implemented `AsRef<u16>` for Strand
- *(macros)* [**breaking**] fix various type checking issues; macros now functional
- *(lib)* fixed bad paths in `artin!` and `band!` macros, allowing us to remove pub modifier on generators module
- *(lib)* make generators module public and re-export error types
- *(macros)* fix bad hygiene in `braid!` macro
- *(README)* fix broken link in acknowledgements
- *(README)* update default badge URLs
- *(generators::conversion)* replaced old artin_to_band algorith with much simpler (and more functional) one
- *(generators::band)* fix/simplify band! constructor macro

### Other

- *(CHANGELOG)* prune rough draft to make way for release-plz output
- *(coverage)* update report
- fix broken links and weird spacing issues
- *(README)* add "Quick Start" and roadmap sections to README
- *(lib)* add front-page library docs
- add todos to implement Mul for result types
- *(braid)* write doctests
- *(word)* write doctests
- *(braid)* document `BraidValidationError` and `Braid` except for doctests
- *(word)* documented `Word` and `WordValidationError`, except for doctests
- *(coverage)* update report
- *(braid)* implement unit test suite
- *(braid)* [**breaking**] rename `Braid::index()` to `Braid::braid_index()` to avoid collision with container fn
- *(coverage)* update report
- *(word)* implement unit test suite
- *(braid)* remove useless bindings in Mul impls for Braid
- *(word)* removed redundant pass through iterator in `impl TryFrom<Vec<L>> for Word`
- *(braid,word)* revert `TryFrom` impls back to using `Into<Letter>`
- *(word)* change bound on L in `TryFrom` impls to `TryInto<Letter>` from `Into<Letter>`
- *(letter)* docmuent `LetterValidationError` and `Letter`
- *(coverage)* update report
- *(letter)* implement unit test suite
- derive `Clone, Copy` on all error types
- *(letter)* derive `Copy` trait on `Letter`
- *(letter,word)* move impl of `Mul<Word>` for `Letter` into `word` module
- *(braid,letter,word)* removed unnecessary `Infallible` error variants and trait bounds
- *(generators::band)* document `BandGenerator`
- *(generators::band)* documented `BandValidationError`
- *(coverage)* update report
- *(generators::band)* implement unit test suite
- *(coverage)* update report
- *(generators::artin)* documented `ArtinGenerator`
- *(generators::artin)* documented `ArtinValidationError`
- *(generators::artin)* implement unit test suite
- *(coverage)* update report
- *(index)* documented `BraidIndex` and `IndexValidationError`
- *(coverage)* update report
- *(strand)* documented `Strand` and `StrandValidationError`
- *(strand)* implement unit test suite
- *(macros)* add documentation for `braid!`
- *(macros)* remove unnecessary/wasteful coercions to `isize`
- *(macros)* add documentation for `word!`
- *(coverage)* update report
- *(macros)* add documentation for `letter!` macro
- *(unit/macros)* implement unit test suite for all three macros
- *(coverage)* update report
- [**breaking**] complete overhaul of generator and constructor interfaces
- [**breaking**] refactor error types to remove opaque `anyhow::Error` variants
- *(sign)* add documentation for `Sign` and its variants
- *(lib)* re-export `generators::artin_to_band` and `generators::band_to_artin`
- *(generators::macros)* add documentation for `band!` macro
- *(macros,generators::macros)* rename "power" -> "exp" in `artin!`, `band!`, and `braid!` macros
- *(generators::macros)* add documentation for `artin!` macro
- *(macros)* add documentation for `braid!` macro
- *(lib)* add front-page documentation
- *(README)* fix sign error in quickstart example
- *(generators::{artin,band})* move `artin!` and `band!` macros into `generators::macros` module
- *(generators::band)* remove existing docstrings
- *(generators::artin)* remove existing docstrings
- *(braid)* move `braid!` macro into `macros` module
- *(braid)* remove existing docstrings
- *(braid)* sort imports
- *(README)* fix missing `.unwrap()` calls in quick start
- Set package-ecosystem to 'cargo' in dependabot config
- add pull request template
- add issue templates
- *(coverage)* add coverage report
- add GitHub actions CI workflow and security audit workflow
- *(README)* add Luca Palmieri to acknowledgements
- *(gitignore)* stop ignoring Cargo.lock
- *(Cargo.toml)* update cargo toml with repository info
- *(gitignore)* flesh out .gitignore
- *(CHANGELOG)* add CHANGELOG.md and corresponding section of README
- *(README.md)* add quick start
- *(CONTRIBUTING)* add basic instructions
- *(README)* Add disclaimer and shields
- *(braid)* rename `Braid::word` to `Braid::band_word` and `Braid::length` to `Braid::band_length`
- *(braid)* [**breaking**] rename `Braid::new` to `Braid::from_bands`
- *(generators::band)* [**breaking**] refactor band macro to match on `[$foot => $head; $power]`
- *(generators::band)* [**breaking**] remove sign-only match variants from band macro
- *(generators::artin)* [**breaking**] remove sign-only match variants from artin macro
- *(braid)* wrote unit test stubs
- *(braid,generators)* replace several impls of `Neg` with a more appropriate `inverse` method
- *(braid)* [**breaking**] constructors now expect raw u16 instead of BraidIndex
- *(generators::conversion)* write unit tests for converters
- update Cargo.toml package metadata
- *(generators::band)* write unit test suite for `BandGenerator`
- *(generators::artin)* update several tests to not assume outcome of result
- *(generators::artin)* add unit tests for artin generators
- *(braid,generators)* initialize remaining unit test modules
- *(index)* add unit tests
- *(strand)* add unit tests
- *(sign)* add unit test
- *(dev-dependencies)* add googletest
- *(generators::{artin, band})* [**breaking**] refactor constructors to be more ergonomic

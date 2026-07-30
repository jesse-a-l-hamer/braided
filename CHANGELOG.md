# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Initial project setup
- Basic component types: `Sign`, `Strand`, `BraidIndex`
- Generators:
  - `artin`: represents crossings of adjacent strands
    - `ArtinGenerator`: main type
    - `artin!`: constructor macro
  - `band`: represents crossings of arbitrarily distant strands
    - `BandGenerator`: main type
    - `band!`: constructor macro
  - `conversion`: tools for converting between generating sets
    - `artin_to_band`: collects Artin generators into maximal bands
    - `band_to_artin`: decomposes bands
- Core module: `braid`:
  - Implement `Braid` type with constructors from both generating sets
  - Can multiply braids with overloaded `*` operator.
    - Automatic simplification is NOT yet implemented. Right now multiplication is just
      concatenation.
  - Can invert braids with `inverse` method.
  - Can compute simple numeric properties:
    - `band_length`
    - `artin_length`
    - `writhe`
    - `index`
  - `braid!`: constructor macro whose syntax accepts any combination of generators (Artin, band,
    or both).

## [0.1.0] - TBD

First release!

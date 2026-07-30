# Braided

> [!WARNING]
> Braided is still very much a work in progress! While basic functionality has been implemented,
> the project is still far from mature and lacking any real documentation. As such, the user
> experience right now is likely to be both limited and painful.
>
> That said, please stay tuned, as I plan to update this page with a feature roadmap ASAP, so that
> users can at least have some idea of where I intend to take this project.

[![Crates.io](https://img.shields.io/crates/v/my-calculator.svg)](https://crates.io/crates/my-calculator)
[![Documentation](https://docs.rs/my-calculator/badge.svg)](https://docs.rs/my-calculator)
[![Build Status](https://github.com/YOUR_USERNAME/my-calculator/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/my-calculator/actions)

A library for defining and manipulating [mathematical braids](https://en.wikipedia.org/wiki/Braid_group), written in Rust.

## Quick Start

```rust
use braided::{ArtinGenerator, BandGenerator, Braid, braid};
use braided::{artin, band};

// Braids can be expressed in two different generating sets:

// 1. The standard Artin generators, representing crossings of adjacent strands.
let artin_generators = [
    artin![1; -1],      // a crossing of strand 1 over strand 2
    artin![2; 1],       // a crossing of strand 3 over stand 2
    artin![1; -1],      // a crossing of strand 2 over strand 1
    artin![3; 3],       // three consecutive crossings of strand 4 over strand 3
    artin![2; -2],      // two consecutive crossings of strand 2 over stand 3
].concat()

// 2. The band generators, representing crossings of potentially distant strands.
let band_generators = [
    band![1 => 3; 1],   // a crossing of strand 1 over strand 3
    band![2 => 4; 3],   // three consecutive crossings of strand 4 over strand 2
    band![1 => 4; -4],  // four consecutive crossings of strand 1 over strand 4
    // Band generators are a superset of artin generators:
    band![2 => 3; -1],   // a crossing of strand 2 over strand 3; same as artin![2; -1]
].concat()

// A braid, then, consists of an index (number of strands) together with a word of generators.
let braid_from_artins = Braid::from_artin(4, &artin_generators).unwrap();
let braid_from_bands = Braid::from_bands(4, &band_generators).unwrap();

// Note that the index only needs to be sufficiently large:
let braid_with_unlinked_strand = Braid::(5, &band_generators).unwrap();

// Using the braid! macro, you can define a braid from a mixed set of generators:
let braid_from_mixed = braid![4; [1; 7], [2 => 4; 3], [3; -2], [1 => 4; -5]].unwrap();

// Regardless of the original generating set, you can still perform braid arithmetic:
let artins_times_inverted_bands = braid_from_artins * braid_from_bands.inverse();
// You can multiply braids of differing indices; the result will have the larger index
let mixed_times_5_braid = braid_from_mixed * braid_with_unlinked_strand;

// NOTE: As of writing this braid multiplication is simple word concatenation.
// No automatic simplification is performed, though this is early on the roadmap.

// And you can compute basic properties of the braid:
assert_eq!(braid_from_artins.band_length(), 4);
assert_eq!(braid_from_bands.artin_length(), 33);
assert_eq!(artins_times_inverted_bands.writhe(), 1);
assert_eq!(mixed_times_5_braid.index(), 5);

```

## Installation

Simply add `braided` to your `Cargo.toml`:

```toml
# Cargo.toml
# ...
[dependencies]
braided = "0.1"
```

## Documentation

TODO

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

Licensed under the [MIT](./LICENSE) license.

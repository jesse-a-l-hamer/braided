# Braided

> [!WARNING]
> Braided is still very much a work in progress! While basic functionality has been implemented,
> the project is still far from mature and lacking any real documentation. As such, the user
> experience right now is likely to be both limited and painful.
>
> That said, please stay tuned, as I plan to update this page with a feature roadmap ASAP, so that
> users can at least have some idea of where I intend to take this project.

[![Crates.io](https://img.shields.io/crates/v/braided.svg)](https://crates.io/crates/braided)
[![Documentation](https://docs.rs/braided/badge.svg)](https://docs.rs/braided)
[![Build Status](https://github.com/jesse-a-l-hamer/braided/workflows/CI/badge.svg)](https://github.com/jesse-a-l-hamer/braided/actions)

A library for working with [mathematical braids](https://en.wikipedia.org/wiki/Braid_group), written in Rust.

## Quick Start

```rust

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

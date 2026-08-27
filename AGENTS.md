# AGENTS.md

Rust workspace (library `braided` + test-only `braided_utils`). Library source lives in `crates/braided/` (declared via `[lib] path`, not the default location); integration/property tests live in `tests/braided/`; benchmarks in `benches/`.

## Commands

```
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test                       # runs doctests + tests/braided suite
cargo test --test braided_tests <name>   # single test target
cargo test --doc                 # doctests
cargo bench                      # 5 criterion benches, harness=false
```

`cargo llvm-cov --all-features --workspace --lcov` for coverage (needs `cargo-llvm-cov`). CI runs build/test on ubuntu+macos, stable+beta; keep docs compiling with `cargo doc --no-deps --all-features`.

## Conventions

- Tests use `googletest` (`#[gtest]`, `assert_that!`, `expect_that!`) and `proptest` (driven manually via `TestRunner`, not `#[proptest]` proptest-macro style). Property-test case generators live in `braided_utils::arbitrary::valid`/`invalid`.
- Call `braided_utils::telemetry::start_tracing()` at the top of each test (tracing with `log` feature; tests may log).
- Constructor macros (`letter!`, `word!`, `braid!`) return `*Result` newtypes (e.g. `BraidResult`) wrapping `Result<T, *ValidationError>`; they implement `Deref` to the inner `Result`, but since `Braid` etc. are not `Copy`, use `.clone_unwrap()` / `.clone_unwrap_err()` in tests/code rather than `.unwrap()`.
- Multiplication (`Mul`) consumes operands unless explicitly borrowed; products of unequal braid indices yield `Err(BraidValidationError::UnequalIndices)`.
- API is unstable pre-1.0; public items require docs (`#![warn(missing_docs)]`) and `BraidValidationError` variants are exhaustive-error style.

## Gotchas

- **Multiplication does NOT auto-cancel** adjacent inverse letters (removed deliberately — it broke associativity; see commit `7df97f9`). Inverses are formal: `b * b.inverse()` is not simplified to the trivial braid. Do not write or resurrect tests expecting cancellation.
- `braided!` with an omitted index still requires writing `();` on the first line before the letters.
- `Braided` (root package) is `version.workspace = true`; `braided_utils` has `release = false` in `release-plz.toml` — releases are automated via release-plz on push to `main`, don't manage tags manually.
- Edition 2024, `rust-version` 1.97; CI also tests `beta`.
- `lcov.info` and `coverage/` are local llvm-cov artifacts (gitignored) — don't edit or rely on them.

# e2e_rust

![CI](https://github.com/Summer9212/e2e_rust/workflows/CI/badge.svg)

A small Rust library + CLI demo created to exercise the full toolchain end to end:
`cargo fmt`, `cargo clippy`, unit tests, integration tests (including running the
compiled binary), and doc tests.

## What it does

- `fizzbuzz(n)` — the classic FizzBuzz label for a number.
- `word_counts(text)` — word-frequency map, case-insensitive, punctuation ignored.
- `e2e_rust` (binary) — prints FizzBuzz for 1..=15 and word counts for a sample sentence.

## Run it

```sh
cargo run            # debug build
cargo test           # full test suite (unit + integration + doc tests)
cargo clippy --all-targets -- -D warnings   # lint, warnings as errors
cargo fmt --check    # formatting check
```

Every push and pull request is checked by CI on Ubuntu, macOS, and Windows
(see `.github/workflows/ci.yml`).

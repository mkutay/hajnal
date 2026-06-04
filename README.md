# Hajnal

> [Hajnal, Venetian Snares](https://www.youtube.com/watch?v=FbJ63spk48s)

This is a SAT solver written in Rust. Currently it only implements the very basic DPLL algorithm, but I plan to add more!

Right now it prints a satisfying assignment if one exists, and prints otherwise.

## Usage

```bash
cargo run benchmarks/sample.cnf
```

```bash
cargo test
```

Run the linter:

```bash
cargo clippy -- -D warnings
```

Run the formatter:

```bash
cargo fmt --all
```

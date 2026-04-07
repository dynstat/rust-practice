# Testing and Crate Structure in Rust

Understanding Rust's crate boundaries is essential for knowing where tests go and what they can access.

## Boundary Definitions

1. **Package:** A single `Cargo.toml`.
2. **Crate:** A single compilation unit (a library, a binary, or an integration test).
3. **Module:** A namespace *inside* a crate.

- `src/main.rs` is one crate.
- `src/lib.rs` is another crate.
- `tests/api.rs` is a completely separate crate.

## Where to place tests

### Unit Tests
Because unit tests reside in a `mod tests` block within the same file (e.g., `src/domain.rs`), they are part of the *same* crate and *same* module hierarchy. 
- **Visibility:** Unit tests can see `private` items from the parent module using `use super::*;`.
- **Purpose:** Test internal logic, edge cases of private helpers, and individual functions.

### Integration Tests
Files placed in `tests/*.rs` are compiled completely independently of your library. 
- **Visibility:** They can ONLY see items explicitly marked `pub` in your `src/lib.rs`.
- **Constraint:** They *cannot* import modules directly from a binary crate (`src/bin/server.rs` or `src/main.rs`). If you want to integration-test logic that currently lives in a binary, you must extract that logic into `src/lib.rs` as a public module.
- **Purpose:** Test the public-facing API of your library crate as an external consumer would use it.

## Typical Layout
```text
my_app/
  Cargo.toml
  src/
    lib.rs       <-- Public API, shared logic
    main.rs      <-- Thin executable, calls lib.rs
    config.rs    <-- Tested via unit tests at the bottom of config.rs
  tests/
    api.rs       <-- Integration test, imports `my_app` as external crate
```

## Running Tests
- `cargo test`: Runs all unit tests, integration tests, and doc tests.
- `cargo test --lib`: Runs only the unit tests inside `src/lib.rs`.
- `cargo test --test api`: Runs only the `tests/api.rs` integration test.

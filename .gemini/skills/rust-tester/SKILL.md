---
name: rust-tester
description: Write idiomatic Rust tests and debug test failures. Use when creating unit tests, integration tests, or benchmarks, or when analyzing output from cargo test.
---

# Rust Testing Guidelines

You are an expert in writing and structuring tests for Rust projects. Adhere to these guidelines to ensure tests are idiomatic and respect module boundaries.

## Core Rules

1. **Unit Tests (Module-level):**
   - Unit tests go in the same file as the code they are testing.
   - Place them at the bottom of the file in a `mod tests` block annotated with `#[cfg(test)]`.
   - Use `use super::*;` to bring the parent module's items (including private items) into scope.
   - Example:
     ```rust
     #[cfg(test)]
     mod tests {
         use super::*;

         #[test]
         fn test_example() {
             // Arrange, Act, Assert
         }
     }
     ```

2. **Integration Tests (Crate-level):**
   - Integration tests must be placed in the `tests/` directory at the project root.
   - Each file in `tests/` is compiled as a separate crate.
   - They can ONLY test the public API of the library (`src/lib.rs`).
   - If a binary has no `lib.rs` and only `main.rs`, integration tests cannot easily test internal functions; logic should be extracted to `src/lib.rs` first.

3. **Test Structure:**
   - Follow the **Arrange, Act, Assert** pattern in every test.
   - For tests that can fail, use `Result<(), Box<dyn std::error::Error>>` as the return type to enable the `?` operator.
   - When writing tests that require environment variables, prefer passing explicit configurations rather than mutating global environment variables (to avoid data races during parallel test execution). If `env::set_var` must be used in a test, comment that it's safe because it's a single-threaded test context, or run tests with `--test-threads=1`.

## References
For further guidance on how testing fits into the broader crate boundaries and package layouts, read [TESTING_STRUCTURE.md](references/TESTING_STRUCTURE.md).

# Rust Project Structure Notes

Verified against stable Rust 1.94.0 and Cargo 1.94.0, using edition 2024.

This note explains how Rust projects are organized, how Cargo sees them, how multiple crates fit together, and how modules, visibility, and dependencies work across those boundaries.

## 1. Core Vocabulary

These terms are related, but not interchangeable:

- Package: A project described by one `Cargo.toml`.
- Crate: A single compilation unit. A library crate, a binary crate, an example crate, an integration test crate, and a benchmark crate are all crates.
- Module: A namespace inside a crate.
- Workspace: A set of one or more packages managed together.

Useful rule of thumb:

- Cargo works with packages and workspaces.
- `rustc` compiles crates.
- Your code is organized into modules inside a crate.

## 2. The Most Important Distinction

Many beginners mix up these three layers:

1. Package boundary
2. Crate boundary
3. Module boundary

They are different:

- One package can contain one library crate and multiple binary crates.
- Every binary is a separate crate.
- Every integration test file in `tests/` is a separate crate.
- Modules only exist inside one crate.

That means:

- `src/main.rs` is one crate root.
- `src/lib.rs` is another crate root.
- `src/bin/server.rs` is another crate root.
- `tests/api.rs` is another crate root.

If you define `mod helpers;` inside `src/main.rs`, that module belongs only to that binary crate. It is not automatically visible to `src/bin/server.rs`.

## 3. What Cargo Looks For By Default

Cargo auto-discovers common targets by convention.

### Single binary package

```text
my_app/
  Cargo.toml
  src/
    main.rs
```

This gives you one binary crate.

### Single library package

```text
my_lib/
  Cargo.toml
  src/
    lib.rs
```

This gives you one library crate.

### Package with both a library and a binary

```text
my_app/
  Cargo.toml
  src/
    lib.rs
    main.rs
```

This gives you:

- one library crate from `src/lib.rs`
- one binary crate from `src/main.rs`

The binary can import the library crate by crate name:

```rust
use my_app::config::Config;
```

If your package name in `Cargo.toml` is `my-app`, the Rust crate name becomes `my_app`.

### Package with multiple binaries

```text
my_app/
  Cargo.toml
  src/
    main.rs
    bin/
      server.rs
      client.rs
      admin.rs
```

This gives you multiple binary crates:

- `my_app` from `src/main.rs`
- `server` from `src/bin/server.rs`
- `client` from `src/bin/client.rs`
- `admin` from `src/bin/admin.rs`

Run them with:

```bash
cargo run
cargo run --bin server
cargo run --bin client
```

### Other standard target folders

```text
my_app/
  Cargo.toml
  src/
    lib.rs
    main.rs
  tests/
    api.rs
  examples/
    demo.rs
  benches/
    perf.rs
```

Meaning:

- `tests/api.rs` is an integration test crate.
- `examples/demo.rs` is an example crate.
- `benches/perf.rs` is a benchmark crate.

All of those are separate crates, not modules in your library.

## 4. When To Use `src/lib.rs`

If code needs to be shared by:

- `src/main.rs`
- files in `src/bin/`
- integration tests in `tests/`
- examples in `examples/`

then that shared code usually belongs in `src/lib.rs` and its modules.

This is the standard shape:

```text
my_app/
  Cargo.toml
  src/
    lib.rs
    main.rs
    config.rs
    http/
      mod.rs
      client.rs
      server.rs
    bin/
      admin.rs
      worker.rs
```

Example:

```rust
// src/lib.rs
pub mod config;
pub mod http;
```

```rust
// src/main.rs
use my_app::config::Config;
```

```rust
// src/bin/admin.rs
use my_app::http::client::ApiClient;
```

Without `src/lib.rs`, each binary crate would need to duplicate modules or declare its own separate module tree.

## 5. How Modules Work Inside One Crate

Modules are a way to split one crate into namespaces and files.

### Basic rule

`mod` declares a module.

Example:

```rust
// src/lib.rs or src/main.rs
mod utils;
```

Cargo and the compiler then load the module from one of these file locations:

- `src/utils.rs`
- `src/utils/mod.rs`

Inside that module, a nested declaration:

```rust
mod parser;
```

would load from one of:

- `src/utils/parser.rs`
- `src/utils/parser/mod.rs`

Both styles are valid on Rust 1.94.0:

- `foo.rs` plus `foo/bar.rs`
- `foo/mod.rs` plus `foo/bar.rs`

Modern projects often prefer the first style for top-level modules:

```text
src/
  lib.rs
  utils.rs
  utils/
    parser.rs
    fs.rs
```

But `mod.rs` is still fully supported and common in older or existing codebases.

### Important rule

You declare a child module in its parent exactly once.

Example:

```rust
// src/lib.rs
pub mod utils;
```

```rust
// src/utils.rs
pub mod parser;
pub mod fs;
```

You do not repeat `mod parser;` in unrelated files.

## 6. `mod` vs `use`

This is one of the most important Rust distinctions.

### `mod`

- Defines or loads a module into the current crate's module tree.
- Usually appears in a crate root or parent module file.

Example:

```rust
mod utils;
```

### `use`

- Brings an existing path into scope.
- Does not create or load files.
- Only changes how you refer to names.

Example:

```rust
use crate::utils::parser::parse_config;
```

This means:

- `mod` changes the module tree.
- `use` changes local name lookup.

## 7. Paths In Rust

Common path prefixes:

- `crate::` means from the current crate root.
- `self::` means from the current module.
- `super::` means from the parent module.
- `some_external_crate::` means from a dependency or another crate in scope.

Example:

```rust
use crate::config::Config;
use self::lexer::Token;
use super::ast::Expr;
use serde::Serialize;
```

Inside a package with a library crate named `my_app`, a binary can use:

```rust
use my_app::config::Config;
```

That is not `crate::config::Config`, because the binary crate and the library crate are different crates.

## 8. Visibility: `pub` And Private By Default

Rust items are private by default.

That includes:

- functions
- structs
- enums
- traits
- modules
- constants
- type aliases

### Basic examples

```rust
mod internal {
    pub fn visible_inside_parent() {}
    fn private_inside_module() {}
}
```

`internal` itself is private unless you write `pub mod internal`.

### Common visibility forms

```rust
pub fn public_everywhere_visible() {}
pub(crate) fn visible_anywhere_in_this_crate() {}
pub(super) fn visible_to_parent_module() {}
pub(self) fn visible_only_here() {}
pub(in crate::http) fn visible_only_inside_crate_http() {}
```

### Struct field visibility

```rust
pub struct User {
    pub id: u64,
    name: String,
}
```

The struct is public, but `name` is still private.

### Practical guideline

Prefer the narrowest visibility that works:

- private first
- `pub(crate)` for internal shared code
- `pub` only for real external API

## 9. Re-exports With `pub use`

You can expose items from deeper modules through a cleaner public path.

Example:

```rust
// src/lib.rs
pub mod http;
pub use http::client::ApiClient;
```

Then users can write:

```rust
use my_app::ApiClient;
```

instead of:

```rust
use my_app::http::client::ApiClient;
```

This is common in libraries that want a simple public API while keeping implementation files deeply organized.

## 10. Common File Layout Patterns

### Pattern A: Small CLI or toy project

```text
src/
  main.rs
```

Use this when all code is still small.

### Pattern B: Single binary with internal modules

```text
src/
  main.rs
  cli.rs
  config.rs
  io.rs
```

`main.rs`:

```rust
mod cli;
mod config;
mod io;
```

This is fine while the code is only used by one binary.

### Pattern C: One binary plus nested module folders

```text
src/
  main.rs
  app.rs
  app/
    commands.rs
    state.rs
  infra/
    mod.rs
    db.rs
    cache.rs
```

Good when one binary is growing and you want grouped modules.

### Pattern D: Shared library plus multiple binaries

```text
src/
  lib.rs
  domain.rs
  services/
    mod.rs
    billing.rs
    mail.rs
  main.rs
  bin/
    worker.rs
    admin.rs
```

This is one of the most common production layouts.

### Pattern E: Workspace with multiple packages

```text
Cargo.toml
crates/
  app/
    Cargo.toml
    src/
      main.rs
  core/
    Cargo.toml
    src/
      lib.rs
  shared/
    Cargo.toml
    src/
      lib.rs
```

This is common when different packages need independent versioning, dependency sets, or build targets.

## 11. Cargo Manifest Basics

The file is `Cargo.toml`.

Minimal package:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
```

Important fields:

- `name`: package name
- `version`: package version
- `edition`: currently usually `2024`
- `rust-version`: optional minimum supported Rust version
- `default-run`: optional default binary to run with `cargo run`

Example:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
rust-version = "1.94"
default-run = "server"
```

## 12. Dependencies In `Cargo.toml`

### crates.io dependency

```toml
[dependencies]
serde = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### Rename a dependency locally

```toml
[dependencies]
serde_json_crate = { package = "serde_json", version = "1" }
```

Then in code:

```rust
use serde_json_crate::Value;
```

### Path dependency

```toml
[dependencies]
my-core = { path = "../my-core" }
```

Then in code:

```rust
use my_core::Config;
```

Again, hyphens in package names become underscores in Rust paths.

### Git dependency

```toml
[dependencies]
my-tool = { git = "https://github.com/example/my-tool", tag = "v0.3.0" }
```

### Optional dependency

```toml
[dependencies]
serde = { version = "1", optional = true, features = ["derive"] }
```

### Features

```toml
[features]
default = ["json"]
json = ["dep:serde"]
cli = []
```

### Development-only dependencies

```toml
[dev-dependencies]
assert_cmd = "2"
```

Used by tests, examples, and benchmarks as needed.

### Build-only dependencies

```toml
[build-dependencies]
cc = "1"
```

Used by `build.rs`, not by your runtime code.

## 13. Explicit Library And Binary Targets

You usually do not need to declare `[lib]` or `[[bin]]` if you follow Cargo's default folder conventions.

Use them when:

- the file lives somewhere unusual
- you want a custom target name
- you want to disable autodiscovery and list targets explicitly

### Explicit library target

```toml
[lib]
name = "my_core"
path = "src/lib.rs"
```

Important:

- A package can have at most one library target.
- A package can have many binary targets.

### Explicit binary targets

```toml
[[bin]]
name = "server"
path = "src/apps/server/main.rs"

[[bin]]
name = "worker"
path = "src/apps/worker/main.rs"
```

This is how you support arbitrarily nested binary entry files.

You can also gate a target behind features:

```toml
[[bin]]
name = "admin"
path = "src/bin/admin.rs"
required-features = ["cli"]
```

## 14. Explicit Example, Test, And Bench Targets

Cargo can also auto-discover these by folder convention, but you can declare them explicitly too.

### Example target

```toml
[[example]]
name = "quickstart"
path = "examples/quickstart.rs"
```

### Integration test target

```toml
[[test]]
name = "api"
path = "tests/api.rs"
```

### Benchmark target

```toml
[[bench]]
name = "perf"
path = "benches/perf.rs"
```

### Disabling autodiscovery

Advanced manifests can turn off automatic target discovery and list everything explicitly.

Examples:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"
autobins = false
autoexamples = false
autotests = false
autobenches = false
```

This is useful when a repository has unusual layout rules and you want Cargo to build only the targets you declared.

## 15. Workspaces

Workspaces are for multiple packages under one repository.

Workspace root:

```toml
[workspace]
members = ["crates/app", "crates/core", "crates/shared"]
resolver = "3"
```

Each member still has its own `Cargo.toml`.

Example:

```text
repo/
  Cargo.toml
  crates/
    app/
      Cargo.toml
      src/main.rs
    core/
      Cargo.toml
      src/lib.rs
```

### Why use a workspace

- multiple packages in one repo
- shared lockfile
- shared target directory by default
- run Cargo commands across all members
- local path dependencies between packages

### Shared dependency versions

Workspace root:

```toml
[workspace]
members = ["crates/app", "crates/core"]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Member package:

```toml
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

This keeps versions consistent across packages.

## 16. How Crates In One Workspace Depend On Each Other

Example:

```text
repo/
  Cargo.toml
  crates/
    app/
      Cargo.toml
      src/main.rs
    core/
      Cargo.toml
      src/lib.rs
```

`crates/core/Cargo.toml`:

```toml
[package]
name = "core-lib"
version = "0.1.0"
edition = "2024"
```

`crates/app/Cargo.toml`:

```toml
[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
core-lib = { path = "../core" }
```

Then in `crates/app/src/main.rs`:

```rust
use core_lib::Config;
```

## 17. Target-Specific Dependencies

Sometimes a dependency should only exist on some platforms.

Example:

```toml
[target.'cfg(windows)'.dependencies]
winreg = "0.55"

[target.'cfg(unix)'.dependencies]
nix = "0.30"
```

This keeps platform-specific crates out of builds where they are not needed.

## 18. What Integration Tests Can See

Unit tests inside a module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

These can see private items from the surrounding module.

Integration tests in `tests/*.rs`:

- are separate crates
- can only use the public API of your library crate
- cannot use private items
- cannot import internal modules from a binary crate

That is another reason shared logic should usually live in `src/lib.rs`.

## 19. How Examples And Benches Fit In

Examples in `examples/*.rs`:

- compile as separate crates
- usually import your library crate
- are useful for demos, docs, and manual testing

Benchmarks in `benches/*.rs`:

- compile as separate crates
- usually import your library crate
- may use unstable or third-party benchmarking setups depending on your tooling

## 20. Build Scripts

If a package has `build.rs`, Cargo runs it before compiling the package.

Typical uses:

- compile C code
- generate Rust source
- emit linker flags
- read environment or system information

Example:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"

[build-dependencies]
cc = "1"
```

The build script can write generated files into `OUT_DIR`, and your crate can include them if needed.

## 21. Direct Vs Transitive Dependencies

Suppose your package depends on `tokio`, and `tokio` depends on `bytes`.

That does not mean your code should assume `bytes` is available.

Best practice:

- If your code uses `bytes`, add `bytes` to your own `[dependencies]`.
- Do not rely on transitive dependencies staying present forever.

Cargo resolves all versions and writes the exact result into `Cargo.lock`.

## 22. Where Downloaded Crates And Build Output Live

Common locations:

- Cargo home on Windows: usually `%USERPROFILE%\\.cargo`
- Cargo home on Unix: usually `$HOME/.cargo`
- Registry source cache: under Cargo home `registry/src`
- Registry archives: under Cargo home `registry/cache`
- Git dependencies: under Cargo home `git`
- Build output: `target/` in the workspace or package root

Useful commands:

```bash
cargo tree
cargo metadata --format-version 1
cargo build
cargo check
cargo test
```

## 23. Recommended Structure By Project Size

### Small program

Use:

```text
src/main.rs
```

Add modules only when needed.

### Medium application with one executable

Use:

```text
src/
  main.rs
  config.rs
  cli.rs
  services/
    mod.rs
    mail.rs
    cache.rs
```

### Multiple executables sharing code

Use:

```text
src/
  lib.rs
  main.rs
  bin/
    worker.rs
    admin.rs
```

Put shared logic in the library crate.

### Larger repository with separable domains

Use a workspace:

```text
Cargo.toml
crates/
  api/
  worker/
  domain/
  shared/
```

## 24. Common Mistakes

### Mistake: thinking `use` loads a file

It does not.

`mod` defines or loads a module.
`use` imports a path that already exists.

### Mistake: putting shared code under `src/main.rs`

That only shares code within that one binary crate.

If multiple binaries or integration tests need it, move it into `src/lib.rs`.

### Mistake: making everything `pub`

This weakens module boundaries and makes refactoring harder.

Prefer:

- private by default
- `pub(crate)` for crate-internal helpers
- `pub` only for true public API

### Mistake: confusing package name with crate path

`my-app` in `Cargo.toml` becomes `my_app` in Rust code.

### Mistake: relying on transitive dependencies

Declare every crate your code directly uses.

### Mistake: expecting `tests/foo.rs` to access private items

Integration tests are separate crates and only see public library API.

## 25. A Good Default Layout For Many Apps

If you are not sure what to choose, this is a strong default:

```text
my_app/
  Cargo.toml
  src/
    lib.rs
    main.rs
    config.rs
    error.rs
    domain/
      mod.rs
      user.rs
      order.rs
    services/
      mod.rs
      email.rs
      billing.rs
    bin/
      worker.rs
  tests/
    api.rs
  examples/
    quickstart.rs
```

Why this works well:

- `lib.rs` is the shared API surface
- `main.rs` stays thin
- additional binaries can reuse the library
- integration tests target the public API
- modules are grouped by responsibility

## 26. A Few Cargo Files You Should Recognize

### `Cargo.toml`

The manifest that defines the package, dependencies, targets, features, and metadata.

### `Cargo.lock`

The lockfile with the exact resolved dependency versions used for this build.

### `.cargo/config.toml`

Optional project or user Cargo configuration, often used for:

- aliases
- custom registries
- linker flags
- target-specific settings

## 27. Practical Checklist

When starting a Rust project:

1. Decide if it is one package or a workspace.
2. Decide if shared code should live in a library crate.
3. Keep `main.rs` and `src/bin/*.rs` as entrypoints, not giant logic files.
4. Organize internal code with modules, not with random long files.
5. Use `pub(crate)` more often than `pub`.
6. Put cross-binary shared code in `src/lib.rs`.
7. Declare direct dependencies explicitly in `Cargo.toml`.
8. Use `[[bin]]` only when default folder conventions are not enough.

## 28. Official References

- Cargo package layout: <https://doc.rust-lang.org/cargo/guide/project-layout.html>
- Cargo dependencies: <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html>
- Cargo home and caches: <https://doc.rust-lang.org/cargo/guide/cargo-home.html>
- Build cache and `target/`: <https://doc.rust-lang.org/cargo/reference/build-cache.html>
- The Rust Book, packages/crates/modules: <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>
- The Rust Book, defining modules and scope: <https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html>
- The Rust Reference, `use`: <https://doc.rust-lang.org/reference/items/use-declarations.html>
- Visibility and privacy examples: <https://doc.rust-lang.org/rust-by-example/mod/visibility.html>
- Workspaces: <https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html>

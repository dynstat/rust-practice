---
name: env-manager
description: Manage, read, and validate environment variables in Rust projects safely. Use when creating or modifying code that interacts with std::env, .env files, or requires configuration structs.
---

# Environment Variables Management

You are responsible for safely managing environment variables in Rust applications.

## Core Rules

1. **Never use `unsafe` to mutate environment variables:** Do not use `std::env::set_var` in production code due to data races. Environment variables should be read-only at runtime.
2. **Use `dotenvy`:** Always use `dotenvy::dotenv().ok()` at application startup to load `.env` variables. Do NOT assume they are already loaded.
3. **Graceful Defaults:** When reading optional or flexible variables (e.g., `PORT`, `HOST`), use `.unwrap_or_else()` to provide sensible defaults.
4. **Fail Fast for Secrets:** For required sensitive data (e.g., `API_KEY`), use `.expect("API_KEY must be set")` so the program panics clearly if misconfigured.
5. **Configuration Struct:** When managing multiple settings, prefer implementing a dedicated `Config` struct (see references) to group them rather than reading variables scattered throughout the application.
6. **.gitignore:** Always ensure `.env` and `.env.local` are added to `.gitignore`. Maintain a `.env.example` file that tracks the required structure but omits actual secrets.

## References
For deeper insight into safe reading, structs patterns, or testing with variables:
- Read [ENV_BEST_PRACTICES.md](references/ENV_BEST_PRACTICES.md) for architectural and theoretical details.
- Read [ENV_CHEATSHEET.md](references/ENV_CHEATSHEET.md) for quick code snippets.

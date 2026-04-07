# Environment Variables Best Practices in Rust

## Quick Start: The Simple Way

### 1. Add the dotenv crate

```toml
[dependencies]
dotenvy = "0.15"
```

### 2. Create a `.env` file in your project root

```env
DATABASE_URL=postgres://localhost/myapp
PORT=8080
DEBUG=true
API_KEY=your_secret_key
```

### 3. Load and use environment variables

```rust
use std::env;

fn main() {
    // Load .env file (do this once at startup)
    dotenvy::dotenv().ok();
    
    // Read environment variables
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string());
    
    println!("Running on port {}", port);
}
```

**That's it!** This is 90% of what you need to know.

---

## Core Concepts

### Reading Environment Variables (Safe - No `unsafe` needed)

```rust
use std::env;

// ✅ BEST: With default value
let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

// ✅ GOOD: With pattern matching
match env::var("DATABASE_URL") {
    Ok(url) => println!("Database: {}", url),
    Err(_) => println!("No database configured"),
}

// ✅ GOOD: Optional values
let api_key: Option<String> = env::var("API_KEY").ok();

// ✅ GOOD: Parse to specific types
let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap_or(8080);

// ✅ GOOD: Boolean flags
let debug = env::var("DEBUG")
    .map(|v| v == "true" || v == "1")
    .unwrap_or(false);
```

### DON'T Modify Environment Variables at Runtime

```rust
// ❌ BAD: Don't do this in production code
unsafe {
    env::set_var("MY_VAR", "value");  // Can cause data races!
}

// ✅ GOOD: Set environment variables OUTSIDE your program
// - In your shell: export MY_VAR=value
// - In .env file: MY_VAR=value
// - In Docker: ENV MY_VAR=value
// - In systemd: Environment="MY_VAR=value"
```

**Why?** Modifying env vars at runtime can cause data races in multi-threaded programs. Environment variables should be set before your program starts.

---

## Real-World Patterns

### Pattern 1: Configuration Struct (Recommended)

```rust
use std::env;

struct Config {
    database_url: String,
    port: u16,
    debug: bool,
    api_key: Option<String>,
}

impl Config {
    fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/myapp".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            debug: env::var("DEBUG")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(false),
            api_key: env::var("API_KEY").ok(),
        }
    }
}

fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    
    // Use config throughout your app
    println!("Server running on port {}", config.port);
}
```

### Pattern 2: Lazy Static Config (For Global Access)

```rust
use std::env;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        dotenvy::dotenv().ok();
        Config::from_env()
    })
}

fn main() {
    // Access config anywhere in your app
    println!("Port: {}", config().port);
}
```

### Pattern 3: Environment-Specific Behavior

```rust
fn main() {
    dotenvy::dotenv().ok();
    
    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    
    match env.as_str() {
        "production" => {
            // Production settings
            println!("Running in PRODUCTION mode");
            // - Enable optimizations
            // - Disable debug features
            // - Use production database
        }
        "staging" => {
            println!("Running in STAGING mode");
            // - Test with production-like data
            // - Enable some debug features
        }
        _ => {
            println!("Running in DEVELOPMENT mode");
            // - Hot reload
            // - Debug logging
            // - Use local database
        }
    }
}
```

---

## Working with .env Files

### File Structure

```
your-project/
├── .env                 # Your actual secrets (DON'T COMMIT)
├── .env.example         # Template (COMMIT THIS)
├── .gitignore           # Must include .env
├── Cargo.toml
└── src/
    └── main.rs
```

### .gitignore

```gitignore
# ALWAYS add .env to .gitignore
.env
.env.local
.env.*.local

# Don't ignore .env.example
!.env.example
```

### Multiple .env Files

```rust
// Load different .env files based on environment
fn main() {
    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    
    // Try to load environment-specific .env file first
    let env_file = format!(".env.{}", env);
    dotenvy::from_filename(&env_file).ok();
    
    // Fall back to .env
    dotenvy::dotenv().ok();
}
```

---

## Security Best Practices

### ✅ DO

1. **Use .env files for local development**
   ```env
   DATABASE_URL=postgres://localhost/myapp_dev
   DEBUG=true
   ```

2. **Never commit .env files**
   ```gitignore
   .env
   ```

3. **Provide .env.example template**
   ```env
   DATABASE_URL=postgres://localhost/myapp
   API_KEY=your_api_key_here
   ```

4. **Use environment variables in production**
   - Docker: `ENV API_KEY=xxx`
   - Kubernetes: ConfigMaps and Secrets
   - Cloud platforms: Environment variables panel
   - Systemd: `Environment="API_KEY=xxx"`

5. **Fail fast for required secrets**
   ```rust
   let api_key = env::var("API_KEY")
       .expect("API_KEY must be set");
   ```

### ❌ DON'T

1. **Don't hardcode secrets**
   ```rust
   // ❌ BAD
   let api_key = "sk_live_abcd1234";
   
   // ✅ GOOD
   let api_key = env::var("API_KEY").expect("API_KEY required");
   ```

2. **Don't commit .env files**
   ```bash
   # Always check before committing
   git status
   ```

3. **Don't log sensitive values**
   ```rust
   // ❌ BAD
   println!("API Key: {}", config.api_key);
   
   // ✅ GOOD
   println!("API Key: {}", if config.api_key.is_some() { "SET" } else { "NOT SET" });
   ```

---

## Common Use Cases

### Database Connection

```rust
let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            env::var("DB_PASS").unwrap_or_else(|_| "password".to_string()),
            env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()),
            env::var("DB_NAME").unwrap_or_else(|_| "myapp".to_string()),
        )
    });
```

### Server Configuration

```rust
let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap_or(8080);

println!("Server listening on {}:{}", host, port);
```

### API Client

```rust
struct ApiClient {
    api_key: String,
    endpoint: String,
}

impl ApiClient {
    fn new() -> Result<Self, String> {
        let api_key = env::var("API_KEY")
            .map_err(|_| "API_KEY environment variable required")?;
        
        let endpoint = env::var("API_ENDPOINT")
            .unwrap_or_else(|_| "https://api.example.com".to_string());
        
        Ok(Self { api_key, endpoint })
    }
}
```

### Feature Flags

```rust
let features = vec![
    "FEATURE_NEW_UI",
    "FEATURE_BETA_API",
    "FEATURE_ANALYTICS",
];

for feature in features {
    if env::var(feature).map(|v| v == "true" || v == "1").unwrap_or(false) {
        println!("✓ {} enabled", feature);
    }
}
```

---

## Command Line Arguments vs Environment Variables

### When to use each:

**Command Line Arguments** (like in your `client.rs` and `server.rs`)
```rust
// ✅ USE FOR: Runtime-specific, per-execution values
let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4000".to_string());
```

**Environment Variables**
```rust
// ✅ USE FOR: Configuration, secrets, feature flags
let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "...".to_string());
```

**Example: Combine both**
```rust
fn main() {
    dotenvy::dotenv().ok();
    
    // CLI args override environment variables
    let port = env::args()
        .nth(1)
        .or_else(|| env::var("PORT").ok())
        .unwrap_or_else(|| "3000".to_string());
    
    println!("Using port: {}", port);
}
```

---

## Production Deployment

### Docker

```dockerfile
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/myapp /usr/local/bin/
# Set environment variables
ENV PORT=8080
ENV RUST_ENV=production
CMD ["myapp"]
```

### Kubernetes

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
data:
  PORT: "8080"
  LOG_LEVEL: "info"
---
apiVersion: v1
kind: Secret
metadata:
  name: app-secrets
type: Opaque
stringData:
  DATABASE_URL: "postgres://user:pass@db:5432/myapp"
  API_KEY: "secret_key_here"
---
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: myapp
        envFrom:
        - configMapRef:
            name: app-config
        - secretRef:
            name: app-secrets
```

### Systemd Service

```ini
[Unit]
Description=My Rust Application

[Service]
Type=simple
ExecStart=/usr/local/bin/myapp
Environment="PORT=8080"
Environment="DATABASE_URL=postgres://localhost/myapp"
EnvironmentFile=/etc/myapp/.env

[Install]
WantedBy=multi-user.target
```

---

## Testing with Environment Variables

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_with_env_vars() {
        // Set test environment variables
        std::env::set_var("PORT", "9999");
        std::env::set_var("DEBUG", "true");
        
        let config = Config::from_env();
        assert_eq!(config.port, 9999);
        assert_eq!(config.debug, true);
        
        // Clean up
        std::env::remove_var("PORT");
        std::env::remove_var("DEBUG");
    }
}
```

**Note:** It's safe to use `set_var`/`remove_var` in single-threaded tests, but be careful with parallel test execution.

---

## Summary: What You Actually Need

**For 99% of applications:**

1. **Add dotenvy**: `cargo add dotenvy`

2. **Create .env file**:
   ```env
   DATABASE_URL=postgres://localhost/myapp
   PORT=8080
   ```

3. **Read variables**:
   ```rust
   fn main() {
       dotenvy::dotenv().ok();
       
       let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
       let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "...".to_string());
       
       // Use them!
   }
   ```

4. **Add .env to .gitignore**

**That's it!** You don't need `unsafe`, you don't need to modify env vars at runtime. Just read them at startup. Simple and safe.
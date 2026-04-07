# Environment Variables in Rust - Quick Reference Cheat Sheet

## 🚀 Quick Start (30 seconds)

### 1. Add dependency
```bash
cargo add dotenvy
```

### 2. Create `.env` file
```env
DATABASE_URL=postgres://localhost/myapp
PORT=8080
DEBUG=true
API_KEY=your_secret_key
```

### 3. Use in code
```rust
use std::env;

fn main() {
    // Load .env file
    dotenvy::dotenv().ok();
    
    // Read variables
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("Port: {}", port);
}
```

### 4. Add to `.gitignore`
```gitignore
.env
```

---

## 📖 Reading Environment Variables

```rust
use std::env;

// With default value (MOST COMMON)
let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

// Optional value
let api_key: Option<String> = env::var("API_KEY").ok();

// Pattern matching
match env::var("DATABASE_URL") {
    Ok(url) => println!("DB: {}", url),
    Err(_) => println!("No DB configured"),
}

// Required value (panic if not set)
let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

// Parse to specific type
let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap_or(8080);

// Boolean flags
let debug = env::var("DEBUG")
    .map(|v| v == "true" || v == "1")
    .unwrap_or(false);
```

---

## 🏗️ Common Patterns

### Configuration Struct
```rust
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
    // Use config...
}
```

### CLI Args + Environment Variables
```rust
fn main() {
    dotenvy::dotenv().ok();
    
    // CLI args override env vars
    let port = env::args()
        .nth(1)
        .or_else(|| env::var("PORT").ok())
        .unwrap_or_else(|| "3000".to_string());
}
```

---

## 📝 Common .env File Examples

### Web Server
```env
HOST=127.0.0.1
PORT=8080
RUST_ENV=development
LOG_LEVEL=info
DEBUG=true
```

### Database
```env
DATABASE_URL=postgres://user:pass@localhost:5432/myapp
DB_POOL_SIZE=10
DB_MAX_CONNECTIONS=100
```

### API Keys
```env
API_KEY=your_api_key_here
API_SECRET=your_api_secret
API_ENDPOINT=https://api.example.com
JWT_SECRET=your_jwt_secret
```

### Feature Flags
```env
FEATURE_NEW_UI=true
FEATURE_BETA_API=false
FEATURE_ANALYTICS=true
```

---

## 🔒 Security Best Practices

### ✅ DO
- Use `.env` files for local development
- Add `.env` to `.gitignore`
- Provide `.env.example` template
- Use `Option<String>` for sensitive values
- Set env vars in production via your platform (Docker, K8s, etc.)

### ❌ DON'T
- Never commit `.env` files
- Never hardcode secrets in code
- Never log sensitive values
- Never modify env vars at runtime (avoid `set_var`/`remove_var`)

---

## 🎯 Real-World Use Cases

### Database Connection
```rust
let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
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

### Environment Detection
```rust
let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

match env.as_str() {
    "production" => {
        // Production settings
    }
    "staging" => {
        // Staging settings
    }
    _ => {
        // Development settings
    }
}
```

### Server Configuration
```rust
let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap_or(8080);

println!("Server: {}:{}", host, port);
```

---

## 🐳 Production Deployment

### Docker
```dockerfile
# Set in Dockerfile
ENV PORT=8080
ENV RUST_ENV=production

# Or pass at runtime
docker run -e PORT=8080 -e DATABASE_URL=postgres://... myapp
```

### Docker Compose
```yaml
version: '3'
services:
  app:
    env_file:
      - .env
    environment:
      PORT: 8080
      DATABASE_URL: postgres://db:5432/myapp
```

### Kubernetes
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
data:
  PORT: "8080"
---
apiVersion: v1
kind: Secret
metadata:
  name: app-secrets
stringData:
  DATABASE_URL: "postgres://..."
  API_KEY: "secret"
```

---

## 🧪 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config() {
        // Set test env vars
        std::env::set_var("PORT", "9999");
        
        let config = Config::from_env();
        assert_eq!(config.port, 9999);
        
        // Clean up
        std::env::remove_var("PORT");
    }
}
```

---

## 🛠️ Useful Commands

```bash
# Run with custom env var
PORT=3000 cargo run

# Run with multiple env vars
PORT=3000 DEBUG=true cargo run

# Set env var in current shell
export PORT=3000

# Set env var for Windows (PowerShell)
$env:PORT="3000"

# Set env var for Windows (CMD)
set PORT=3000

# Check if env var is set
echo $PORT                  # Unix/Linux/Mac
echo $env:PORT             # PowerShell
echo %PORT%                # CMD

# Run with .env file
cargo run  # Automatically loads .env if using dotenvy
```

---

## 🎓 Key Takeaways

1. **Just READ variables** - No `unsafe` needed for 99% of use cases
2. **Use `.env` files** - Great for local development
3. **Never commit secrets** - Add `.env` to `.gitignore`
4. **Provide defaults** - Use `unwrap_or_else()` with sensible defaults
5. **Load once at startup** - Call `dotenvy::dotenv().ok()` at the beginning of `main()`
6. **Use config structs** - Group related configuration together
7. **Fail fast** - Use `.expect()` for required values
8. **Keep it simple** - Environment variables are not complicated!

---

## 📚 Additional Resources

- Your project examples: `client.rs` and `server.rs` use `env::args()` (command line args)
- See `simple_env.rs` for complete working examples
- See `.env.example` for template
- See `ENV_BEST_PRACTICES.md` for detailed guide

---

## 🆘 Common Issues

### .env file not loading?
```rust
// Check return value
match dotenvy::dotenv() {
    Ok(path) => println!("Loaded: {:?}", path),
    Err(e) => println!("Error: {}", e),
}

// Make sure .env is in project root (same directory as Cargo.toml)
```

### Variable not found?
```rust
// Print all variables
for (key, value) in std::env::vars() {
    println!("{}: {}", key, value);
}
```

### Wrong value being used?
- CLI args might override env vars
- Check for multiple .env files
- Shell env vars override .env file
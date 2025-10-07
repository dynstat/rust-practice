use std::env;

fn main() {
    println!("=== Simple & Practical Environment Variables in Rust ===\n");

    // Step 1: Load .env file (if it exists)
    // This is typically done at the start of your application
    match dotenvy::dotenv() {
        Ok(path) => println!("✓ Loaded .env file from: {:?}\n", path),
        Err(_) => println!("⚠ No .env file found (that's okay!)\n"),
    }

    // ============================================================
    // READING ENVIRONMENT VARIABLES (This is what you do 99% of the time)
    // ============================================================

    println!("--- Basic Usage ---\n");

    // Method 1: Read with a default value (MOST COMMON)
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("Server will run on port: {}", port);

    // Method 2: Parse to a specific type
    let port_number: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);
    println!("Port as number: {}", port_number);

    // Method 3: Check if a variable exists
    if let Ok(database_url) = env::var("DATABASE_URL") {
        println!("Database URL is set: {}", database_url);
    } else {
        println!("No DATABASE_URL found, using default");
    }

    // Method 4: Boolean flags
    let debug_mode = env::var("DEBUG")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    println!("Debug mode: {}", debug_mode);

    println!("\n--- Real-World Example: Application Config ---\n");

    // This is the pattern used in production apps
    let config = AppConfig::from_env();
    config.print();

    println!("\n--- Command Line Arguments (from your client.rs/server.rs) ---\n");

    // Your current usage - this is perfect!
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:4000".to_string());
    println!("Address from CLI args: {}", addr);
}

// ============================================================
// REAL-WORLD PATTERN: Configuration Struct
// ============================================================

struct AppConfig {
    // Database
    database_url: String,
    db_pool_size: u32,

    // Server
    host: String,
    port: u16,

    // Features
    debug_mode: bool,
    log_level: String,

    // Secrets (API keys, tokens, etc.)
    api_key: Option<String>,
    jwt_secret: Option<String>,
}

impl AppConfig {
    fn from_env() -> Self {
        Self {
            // Database configuration
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost:5432/myapp".to_string()),
            db_pool_size: env::var("DB_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),

            // Server configuration
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),

            // Feature flags
            debug_mode: env::var("DEBUG")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(false),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),

            // Optional secrets (don't have defaults for security)
            api_key: env::var("API_KEY").ok(),
            jwt_secret: env::var("JWT_SECRET").ok(),
        }
    }

    fn print(&self) {
        println!("Application Configuration:");
        println!(
            "  Database URL: {}",
            self.mask_sensitive(&self.database_url)
        );
        println!("  DB Pool Size: {}", self.db_pool_size);
        println!("  Server: {}:{}", self.host, self.port);
        println!("  Debug Mode: {}", self.debug_mode);
        println!("  Log Level: {}", self.log_level);
        println!(
            "  API Key: {}",
            if self.api_key.is_some() {
                "✓ Set"
            } else {
                "✗ Not set"
            }
        );
        println!(
            "  JWT Secret: {}",
            if self.jwt_secret.is_some() {
                "✓ Set"
            } else {
                "✗ Not set"
            }
        );
    }

    fn mask_sensitive(&self, s: &str) -> String {
        if s.len() > 10 {
            format!("{}...{}", &s[..5], &s[s.len() - 3..])
        } else {
            "***".to_string()
        }
    }
}

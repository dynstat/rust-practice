use std::collections::HashMap;
use std::env;

fn main() {
    println!("=== Environment Variable Usage Examples ===\n");

    // 1. Command Line Arguments (what you're using in your project)
    example_command_line_args();

    // 2. Reading Environment Variables
    example_read_env_vars();

    // 3. Setting Environment Variables
    example_set_env_vars();

    // 4. Working with PATH and System Variables
    example_system_paths();

    // 5. Configuration Management
    example_config_management();

    // 6. Real-world Database Connection Example
    example_database_config();

    // 7. API Keys and Secrets Management
    example_api_keys();

    // 8. Feature Flags and Environment-based Behavior
    example_feature_flags();

    // 9. Iterating Over All Environment Variables
    example_iterate_env_vars();
}

// Example 1: Command Line Arguments (like in your client.rs and server.rs)
fn example_command_line_args() {
    println!("1. Command Line Arguments:");
    println!("----------------------------");

    let args: Vec<String> = env::args().collect();
    println!("Program name: {}", args[0]);

    // Your pattern: using nth() with unwrap_or_else
    let server_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let message = env::args()
        .nth(2)
        .unwrap_or_else(|| "default message".to_string());

    println!("Server address: {}", server_addr);
    println!("Message: {}", message);

    // Alternative pattern using matches
    match env::args().nth(3) {
        Some(port) => println!("Custom port provided: {}", port),
        None => println!("No custom port provided, using default"),
    }

    println!();
}

// Example 2: Reading Environment Variables
fn example_read_env_vars() {
    println!("2. Reading Environment Variables:");
    println!("----------------------------------");

    // Method 1: Using var() - panics if not valid UTF-8
    match env::var("HOME") {
        Ok(val) => println!("HOME directory: {}", val),
        Err(e) => println!("Couldn't read HOME: {}", e),
    }

    // Method 2: Using var_os() - returns OsString, handles non-UTF-8
    if let Some(path) = env::var_os("PATH") {
        println!("PATH is set (length: {} bytes)", path.len());
    }

    // Common pattern: provide default value
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string());
    println!("Log level: {}", log_level);

    println!();
}

// Example 3: Setting Environment Variables (for child processes)
fn example_set_env_vars() {
    println!("3. Setting Environment Variables:");
    println!("---------------------------------");

    // Set an environment variable for the current process
    // Note: set_var and remove_var are unsafe because they can cause data races
    // if other threads are reading environment variables
    unsafe {
        env::set_var("MY_CUSTOM_VAR", "Hello from Rust!");
    }

    // Read it back
    if let Ok(val) = env::var("MY_CUSTOM_VAR") {
        println!("MY_CUSTOM_VAR = {}", val);
    }

    // Remove an environment variable
    unsafe {
        env::remove_var("MY_CUSTOM_VAR");
    }
    println!("MY_CUSTOM_VAR removed");

    // Verify it's gone
    match env::var("MY_CUSTOM_VAR") {
        Ok(_) => println!("Variable still exists!"),
        Err(_) => println!("Variable successfully removed"),
    }

    println!();
}

// Example 4: Working with System Paths
fn example_system_paths() {
    println!("4. System Paths and Directories:");
    println!("---------------------------------");

    // Current directory
    match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    }

    // Current executable path
    match env::current_exe() {
        Ok(path) => println!("Current executable: {}", path.display()),
        Err(e) => println!("Error getting executable path: {}", e),
    }

    // Temporary directory
    println!("Temp directory: {}", env::temp_dir().display());

    // Split PATH variable (cross-platform)
    if let Ok(path) = env::var("PATH") {
        let paths: Vec<String> = env::split_paths(&path)
            .take(3) // Just show first 3 for brevity
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();
        println!("First 3 PATH entries: {:?}", paths);
    }

    println!();
}

// Example 5: Configuration Management Pattern
fn example_config_management() {
    println!("5. Configuration Management:");
    println!("----------------------------");

    #[derive(Debug)]
    struct AppConfig {
        database_url: String,
        port: u16,
        debug_mode: bool,
        max_connections: usize,
    }

    impl AppConfig {
        fn from_env() -> Self {
            Self {
                database_url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://localhost/myapp".to_string()),
                port: env::var("PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
                debug_mode: env::var("DEBUG")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(false),
                max_connections: env::var("MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
            }
        }
    }

    let config = AppConfig::from_env();
    println!("App configuration: {:?}", config);
    println!();
}

// Example 6: Real-world Database Connection
fn example_database_config() {
    println!("6. Database Configuration Pattern:");
    println!("----------------------------------");

    // Pattern 1: Single DATABASE_URL
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Fallback: construct from individual components
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let name = env::var("DB_NAME").unwrap_or_else(|_| "myapp".to_string());
        let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let pass = env::var("DB_PASS").unwrap_or_else(|_| "password".to_string());

        format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, name)
    });

    println!(
        "Database URL: {}",
        database_url.replace(|c: char| c == ':' || c == '@', "*")
    );

    // Pattern 2: Connection pool configuration
    let pool_size = env::var("DB_POOL_SIZE")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u32>()
        .unwrap_or(10);

    println!("Connection pool size: {}", pool_size);
    println!();
}

// Example 7: API Keys and Secrets
fn example_api_keys() {
    println!("7. API Keys and Secrets Management:");
    println!("------------------------------------");

    // Never hardcode API keys! Always use environment variables
    struct ApiClient {
        api_key: String,
        api_secret: Option<String>,
        endpoint: String,
    }

    impl ApiClient {
        fn from_env() -> Result<Self, String> {
            let api_key =
                env::var("API_KEY").map_err(|_| "API_KEY environment variable is required")?;

            let api_secret = env::var("API_SECRET").ok();

            let endpoint =
                env::var("API_ENDPOINT").unwrap_or_else(|_| "https://api.example.com".to_string());

            Ok(Self {
                api_key,
                api_secret,
                endpoint,
            })
        }
    }

    match ApiClient::from_env() {
        Ok(client) => {
            println!("API client configured");
            println!("Endpoint: {}", client.endpoint);
            println!("API key present: {}", !client.api_key.is_empty());
            println!("API secret present: {}", client.api_secret.is_some());
        }
        Err(e) => {
            println!("Warning: {}", e);
            println!("Set API_KEY environment variable to use API features");
        }
    }

    println!();
}

// Example 8: Feature Flags and Environment-based Behavior
fn example_feature_flags() {
    println!("8. Feature Flags and Environment Detection:");
    println!("--------------------------------------------");

    // Detect environment (development, staging, production)
    let environment = env::var("RUST_ENV")
        .or_else(|_| env::var("ENVIRONMENT"))
        .unwrap_or_else(|_| "development".to_string());

    println!("Current environment: {}", environment);

    // Feature flags
    let features = vec![
        ("FEATURE_NEW_UI", "Enable new UI design"),
        ("FEATURE_BETA_API", "Use beta API endpoints"),
        ("FEATURE_ANALYTICS", "Enable analytics tracking"),
        ("FEATURE_DEBUG_PANEL", "Show debug panel"),
    ];

    println!("Active features:");
    for (flag, description) in features {
        if env::var(flag)
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false)
        {
            println!("  ✓ {} - {}", flag, description);
        } else {
            println!("  ✗ {} - {}", flag, description);
        }
    }

    // Environment-specific behavior
    match environment.as_str() {
        "production" => {
            println!("Running in production mode:");
            println!("  - Logging level: ERROR");
            println!("  - Optimizations: ENABLED");
            println!("  - Debug features: DISABLED");
        }
        "staging" => {
            println!("Running in staging mode:");
            println!("  - Logging level: INFO");
            println!("  - Optimizations: ENABLED");
            println!("  - Debug features: LIMITED");
        }
        _ => {
            println!("Running in development mode:");
            println!("  - Logging level: DEBUG");
            println!("  - Optimizations: DISABLED");
            println!("  - Debug features: ENABLED");
        }
    }

    println!();
}

// Example 9: Iterating Over All Environment Variables
fn example_iterate_env_vars() {
    println!("9. Listing Environment Variables:");
    println!("---------------------------------");

    // Get all environment variables
    let all_vars: HashMap<String, String> = env::vars().collect();
    println!("Total environment variables: {}", all_vars.len());

    // Filter and display specific prefixes
    println!("\nVariables starting with 'RUST_':");
    for (key, value) in env::vars() {
        if key.starts_with("RUST_") {
            println!("  {} = {}", key, value);
        }
    }

    // Check for common development variables
    println!("\nCommon development variables:");
    let common_vars = ["HOME", "USER", "PATH", "SHELL", "EDITOR", "TERM"];
    for var_name in &common_vars {
        match env::var(var_name) {
            Ok(val) => println!("  {} is set (length: {})", var_name, val.len()),
            Err(_) => println!("  {} is not set", var_name),
        }
    }

    println!();
}

// Additional Real-World Examples

// Example: Docker/Container Environment
fn _example_docker_config() {
    // Common in containerized applications
    let is_docker = env::var("DOCKER_CONTAINER").is_ok();
    let container_id = env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());

    if is_docker {
        println!("Running in Docker container: {}", container_id);
    }
}

// Example: Cloud Provider Detection
fn _example_cloud_detection() {
    // AWS
    if env::var("AWS_EXECUTION_ENV").is_ok() {
        println!("Running on AWS Lambda");
    }

    // Google Cloud
    if env::var("K_SERVICE").is_ok() {
        println!("Running on Google Cloud Run");
    }

    // Azure
    if env::var("WEBSITE_INSTANCE_ID").is_ok() {
        println!("Running on Azure App Service");
    }

    // Kubernetes
    if env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        println!("Running in Kubernetes");
    }
}

// Example: Logging Configuration
fn _example_logging_config() {
    // Rust's env_logger crate pattern
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let log_style = env::var("RUST_LOG_STYLE").unwrap_or_else(|_| "auto".to_string());

    println!("Logging configuration:");
    println!("  Level: {}", log_level);
    println!("  Style: {}", log_style);

    // Custom logging targets
    if let Ok(targets) = env::var("LOG_TARGETS") {
        let targets: Vec<&str> = targets.split(',').collect();
        println!("  Targets: {:?}", targets);
    }
}

// Example: Proxy Configuration
fn _example_proxy_config() {
    // Common proxy environment variables
    let proxies = [
        ("HTTP_PROXY", "HTTP proxy"),
        ("HTTPS_PROXY", "HTTPS proxy"),
        ("NO_PROXY", "Proxy exceptions"),
        ("ALL_PROXY", "General proxy"),
    ];

    for (var, description) in &proxies {
        if let Ok(value) = env::var(var) {
            println!("{}: {}", description, value);
        }
    }
}

// Example: Build-time Information (often set during CI/CD)
fn _example_build_info() {
    let build_info = [
        ("CI", "Running in CI"),
        ("GITHUB_ACTIONS", "GitHub Actions"),
        ("GITLAB_CI", "GitLab CI"),
        ("JENKINS_URL", "Jenkins"),
        ("BUILD_NUMBER", "Build number"),
        ("GIT_COMMIT", "Git commit hash"),
        ("GIT_BRANCH", "Git branch"),
    ];

    for (var, description) in &build_info {
        if let Ok(value) = env::var(var) {
            println!("{}: {}", description, value);
        }
    }
}

use std::env;

fn main() {
    // Read API key from environment during build
    let api_key = env::var("SHELLAPI_API_KEY")
        .unwrap_or_else(|_| "default_api_key".to_string());
    
    // Make the API key available at compile time
    println!("cargo:rustc-env=SHELLAPI_API_KEY={}", api_key);
}
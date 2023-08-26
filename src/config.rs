//! The config module contains the Config struct, which is used to initialize configuration for the API.
//! Currently contains the `API token`, the `user agent` and the `base url`.
//!
//!
//! # Example
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config {
//!     auth : String::from("REPLICATE_API_TOKEN"),
//!     ..Default::default()
//! };
//!
//! let replicate = Replicate::new(config); // config OR Default::default()
//! ```
//! ### Note
//!
//! The Config struct implements the `Default` trait, so you can also use `Default::default()` to initialize the config.
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//!
//! let replicate = Replicate::new(config);
//! ```    
#[derive(Clone, Debug)]
pub struct Config {
    /// The API token to use for authentication.
    pub auth: String,

    /// The user agent to use for the API requests. Defaults to `replicate-rust/{version}`.
    pub user_agent: String,

    /// The base url to use for the API requests. Defaults to `https://api.replicate.com/v1`.
    pub base_url: String,
}

// Default implementation for Client

impl Default for Config {
    /// Create a new Config struct with the default values.
    fn default() -> Self {
        Self {
            auth: std::env::var("REPLICATE_API_TOKEN").unwrap_or_else(|_| String::new()),
            user_agent: format!("replicate-rust/{}", env!("CARGO_PKG_VERSION")),
            base_url: String::from("https://api.replicate.com/v1"),
        }
    }
}

impl Config {
    /// Check if auth is set and exit if not.
    /// The auth token can be set in the environment variable `REPLICATE_API_TOKEN`.
    /// Otherwise, it can be set in the `Config` struct.
    pub fn check_auth(&self) {
        // Check if auth is set.
        if self.auth.is_empty() {
            eprintln!("No API token provided. You need to set the REPLICATE_API_TOKEN environment variable or create a client with `Config {{auth: String::from('REPLICATE_API_TOKEN'), ..Default::default()}}`.

You can find your API key on https://replicate.com");
            std::process::exit(1);
        }
    }
}

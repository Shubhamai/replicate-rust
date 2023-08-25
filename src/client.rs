//! The client module contains the Client struct, which is used to initialize settings for the API client.
//! It contains the `API token`, the `user agent` and the `base url`.
//!

/// The client module contains the Client struct, which is used to initialize settings for the API client.
/// It contains the `API token`, the `user agent` and the `base url`.

///
/// Currently, the API token is the only required parameter.
// TODO : Fix the above line
///
/// The `user agent` is set to `replicate-rust/{CARGO_PKG_VERSION}`.
///
/// The `base url` is set to `https://api.replicate.com/v1`.
#[derive(Clone)]
pub struct Client {
    pub auth: String,
    pub user_agent: String,
    pub base_url: String,
}

impl Client {
    pub fn new() -> Self {
        let api_key = std::env::var("REPLICATE_API_TOKEN").unwrap_or_else(|_| {
            eprintln!("No API token provided. You need to set the REPLICATE_API_TOKEN environment variable or create a client with `replicate.Client(api_token=...)");
            std::process::exit(1)
        });

        Self {
            auth: api_key,
            user_agent: format!("replicate-rust/{}", env!("CARGO_PKG_VERSION")),
            base_url: String::from("https://api.replicate.com/v1"),
        }
    }
}

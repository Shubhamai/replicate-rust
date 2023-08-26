//! Used to interact with the [Collection Endpoints](https://replicate.com/docs/reference/http#collections.get).
//! 
//! The Collection struct is initialized with a Config struct.
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! match replicate.collection.get(String::from("audio-generation")) {
//!     Ok(result) => println!("Success : {:?}", result),
//!     Err(e) => println!("Error : {}", e),
//! }
//!
//!

use crate::api_definitions::{GetCollectionModels, ListCollectionModels};

pub struct Collection {
    /// Holds a reference to a Config struct, which contains the base url,  auth token among other settings.
    pub parent: crate::config::Config,
}

impl Collection {
    /// Create a new Collection struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Get a collection by slug.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// match replicate.collection.get(String::from("audio-generation")) {
    ///    Ok(result) => println!("Success : {:?}", result),
    ///   Err(e) => println!("Error : {}", e),
    /// }
    /// ```
    pub fn get(
        &self,
        collection_slug: String,
    ) -> Result<GetCollectionModels, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/collections/{}",
                self.parent.base_url, collection_slug
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetCollectionModels = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// List all collections present in Replicate.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// match replicate.collection.list() {
    ///   Ok(result) => println!("Success : {:?}", result),
    ///   Err(e) => println!("Error : {}", e),
    /// }
    /// ```
    ///
    pub fn list(&self) -> Result<ListCollectionModels, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/collections", self.parent.base_url))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListCollectionModels = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

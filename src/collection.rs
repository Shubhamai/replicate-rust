//! Used to interact with the [Collection Endpoints](https://replicate.com/docs/reference/http#collections.get).
//!
//! The Collection struct is used to interact with the [Collection Endpoints](https://replicate.com/docs/reference/http#collections.get).
//!
//! The Collection struct is initialized with a Client struct.
//!
//! The Collection struct has two methods:
//!     
//!   - `get`
//!   - `list`
//!
//! # Example
//!
//! ```
//! use replicate_rust::Replicate;
//!
//! let replicate = Replicate::new();
//!
//! match replicate.collection.get(String::from("audio-generation")) {
//!     Ok(result) => println!("Success : {:?}", result),
//!     Err(e) => println!("Error : {}", e),
//! }
//!
//!

use crate::api_definitions::{GetCollectionModels, ListCollectionModels};

pub struct Collection {
    // Holds a reference to a Replicate
    pub parent: crate::client::Client,
}

impl Collection {
    pub fn new(rep: crate::client::Client) -> Self {
        Self { parent: rep }
    }

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

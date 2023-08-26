//! Used to interact with the [Model Versions Endpoints](https://replicate.com/docs/reference/http#models.versions.get).
//!
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! // List the versions of a model.
//! match replicate.models.versions.list(String::from("replicate"), String::from("hello-world")) {
//!        Ok(result) => println!("Success : {:?}", result),
//!        Err(e) => println!("Error : {}", e),
//! };
//!
//! // Get the details of a model version.
//! match replicate.models.versions.get(
//!         String::from("kvfrans"),
//!         String::from("clipdraw"),
//!         String::from("5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b"),
//!     ) {
//!         Ok(result) => println!("Success : {:?}", result),
//!         Err(e) => println!("Error : {}", e),
//! };
//!
//!

use crate::api_definitions::{GetModelVersion, ListModelVersions};

// #[derive(Clone)]
pub struct Version {
    // Holds a reference to a Replicate
    pub parent: crate::config::Config,
}

/// The Version struct is used to interact with the [Model Versions Endpoints](https://replicate.com/docs/reference/http#models.versions.get).
impl Version {
    /// Create a new Version struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Get the details of a model version.
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Get the details of a model version.
    /// match replicate.models.versions.get(
    ///         String::from("kvfrans"),
    ///         String::from("clipdraw"),
    ///         String::from("5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b"),
    ///     ) {
    ///         Ok(result) => println!("Success : {:?}", result),
    ///         Err(e) => println!("Error : {}", e),
    /// };
    /// ```
    pub fn get(
        &self,
        model_owner: String,
        model_name: String,
        version_id: String,
    ) -> Result<GetModelVersion, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}/versions/{}",
                self.parent.base_url, model_owner, model_name, version_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetModelVersion = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// List the versions of a model.
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // List the versions of a model.
    /// match replicate.models.versions.list(String::from("replicate"), String::from("hello-world")) {
    ///        Ok(result) => println!("Success : {:?}", result),
    ///        Err(e) => println!("Error : {}", e),
    /// };
    /// ```
    pub fn list(
        &self,
        model_owner: String,
        model_name: String,
    ) -> Result<ListModelVersions, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}/versions",
                self.parent.base_url, model_owner, model_name
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListModelVersions = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

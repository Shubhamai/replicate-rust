//! # Used to interact with the [Model Endpoints](https://replicate.com/docs/reference/http#models.get).
//!
//! The model module contains all the functionality for interacting with the model endpoints of the Replicate API.
//! Currently supports the following endpoint:
//! * [Get Model](https://replicate.com/docs/reference/http#models.get)
//! * [Get Model Versions](https://replicate.com/docs/reference/http#models.versions.get)
//! * [List Model Versions](https://replicate.com/docs/reference/http#models.versions.list)
//! * [Delete Model Version](https://replicate.com/docs/reference/http#models.versions.delete)
//!
//! # Example
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! match replicate.models.get(String::from("replicate"), String::from("hello-world")) {
//!    Ok(result) => println!("Success : {:?}", result),
//!   Err(e) => println!("Error : {}", e),
//! };
//! ```

use crate::{api_definitions::GetModel, version::Version};

// #[derive(Clone)]
pub struct Model {
    // Holds a reference to a Replicate
    pub parent: crate::config::Config,
    pub versions: Version,
}

/// Model struct contains all the functionality for interacting with the model endpoints of the Replicate API.
/// Currently supports the following endpoint:
/// * [Get Model](https://replicate.com/docs/reference/http#models.get)
/// * [Get Model Versions](https://replicate.com/docs/reference/http#models.versions.get)
/// * [List Model Versions](https://replicate.com/docs/reference/http#models.versions.list)
/// * [Delete Model Version](https://replicate.com/docs/reference/http#models.versions.delete)
///
impl Model {
    /// Create a new Model struct.
    /// # Arguments
    /// * `rep` - The config (`crate::config::Config`) to use for authentication and communication.
    ///
    pub fn new(rep: crate::config::Config) -> Self {
        let versions = Version::new(rep.clone());
        Self {
            parent: rep,
            versions,
        }
    }

    /// Get the details of a model.
    /// # Arguments
    /// * `model_owner` - The owner of the model.
    /// * `model_name` - The name of the model.
    ///
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// match replicate.models.get(String::from("replicate"), String::from("hello-world")) {
    ///    Ok(result) => println!("Success : {:?}", result),
    ///    Err(e) => println!("Error : {}", e),
    /// };
    pub fn get(
        &self,
        model_owner: String,
        model_name: String,
    ) -> Result<GetModel, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}",
                self.parent.base_url, model_owner, model_name
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetModel = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

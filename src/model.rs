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
//! match replicate.models.get("replicate", "hello-world") {
//!    Ok(result) => println!("Success : {:?}", result),
//!   Err(e) => println!("Error : {}", e),
//! };
//! ```

use crate::{api_definitions::GetModel, version::Version};

// #[derive(Clone)]
/// Used to interact with the [Model Endpoints](https://replicate.com/docs/reference/http#models.get).
pub struct Model {
    /// Holds a reference to a Configuration struct, which contains the base url,  auth token among other settings.
    pub parent: crate::config::Config,

    /// Holds a reference to a Version struct, which contains the functionality for interacting with the version endpoints of the Replicate API.
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
    /// match replicate.models.get("replicate", "hello-world") {
    ///    Ok(result) => println!("Success : {:?}", result),
    ///    Err(e) => println!("Error : {}", e),
    /// };
    pub fn get(
        &self,
        model_owner: &str,
        model_name: &str,
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

#[cfg(test)]
mod tests {
    use crate::{config::Config, Replicate};

    use httpmock::{Method::GET, MockServer};
    use serde_json::json;

    #[test]
    fn test_get() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/models/replicate/hello-world");
            then.status(200).json_body_obj(&json!( {
                "url": "https://replicate.com/replicate/hello-world",
                "owner": "replicate",
                "name": "hello-world",
                "description": "A tiny model that says hello",
                "visibility": "public",
                "github_url": "https://github.com/replicate/cog-examples",
                "paper_url": None::<String>,
                "license_url": None::<String>,
                "run_count": 12345,
                "cover_image_url": "",
                "default_example": {},
                "latest_version": {}
            }
            ));
        });

        let config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let result = replicate
            .models
            .get("replicate", "hello-world")?;

        println!("{:?}", result);

        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }
}

//! Rust Client for interacting with the [Replicate API](https://replicate.com/docs/api/). Provides a type-safe interface by deserializing API responses into Rust structs.
//!
//! ### Getting Started
//!
//! Add `replicate_rust` to your `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! replicate_rust = "0.0.3"
//! ```
//!
//! ## Example
//! In this example we will run a model that generates a caption for an image using the [Stable Diffusion](
//! https://replicate.ai/stability-ai/stable-diffusion) model.
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! // Creating the inputs
//! let mut inputs = std::collections::HashMap::new();
//! inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//! let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
//!
//! // Run the model.
//! let result = replicate.run(version, inputs);
//!
//! // Print the result
//! match result {
//!     Ok(result) => println!("Success : {:?}", result.output),
//!     Err(e) => println!("Error : {}", e),
//! }
//!
//! ```

use std::collections::HashMap;

use api_definitions::GetPrediction;
use collection::Collection;
use config::Config;
use model::Model;
use prediction::Prediction;
use training::Training;

pub mod collection;
pub mod config;
pub mod model;
pub mod prediction;
pub mod training;
pub mod version;

pub mod api_definitions;
pub mod prediction_client;
pub mod retry;

pub struct Replicate {
    /// Holds a reference to a Config struct.
    config: Config,

    /// Holds a reference to a Prediction struct. Use to run inference given model inputs and version.
    pub predictions: Prediction,

    /// Holds a reference to a Model struct. Use to get information about a model.
    pub models: Model,

    /// Holds a reference to a Training struct. Use to create a new training run.
    pub trainings: Training,

    /// Holds a reference to a Collection struct. Use to get and list model collections present in Replicate.
    pub collections: Collection,
}

/// Rust Client for interacting with the [Replicate API](https://replicate.com/docs/api/).
impl Replicate {
    /// Create a new Replicate client.
    ///
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        // Check if auth is set.
        config.check_auth();

        // TODO : Maybe reference instead of clone
        let predictions = Prediction::new(config.clone());
        let models = Model::new(config.clone());
        let trainings = Training::new(config.clone());
        let collections = Collection::new(config.clone());

        Self {
            config,
            predictions,
            models,
            trainings,
            collections,
        }
    }

    /// Run a model with the given inputs in a blocking manner.
    /// # Arguments
    /// * `version` - The version of the model to run.
    /// * `inputs` - The inputs to the model in the form of a HashMap.
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Construct the inputs.
    /// let mut inputs = std::collections::HashMap::new();
    /// inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
    ///
    /// let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
    ///
    /// // Run the model.
    /// let result = replicate.run(version, inputs);
    ///
    /// // Print the result.
    /// match result {
    ///    Ok(result) => println!("Success : {:?}", result.output),
    ///   Err(e) => println!("Error : {}", e),
    /// }
    /// ```
    /// # Errors
    ///
    /// TODO : Add errors
    ///
    pub fn run<K: serde::Serialize, V: serde::Serialize>(
        &self,
        version: String,
        inputs: HashMap<K, V>,
        // TODO : Perhaps not Box<dyn std::error::Error> but something more specific?
    ) -> Result<GetPrediction, Box<dyn std::error::Error>> {
        let prediction = Prediction::new(self.config.clone()).create(version, inputs);

        prediction.wait()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{
        Method::{GET, POST},
        MockServer,
    };
    use serde_json::json;

    #[test]
    fn test_run() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        // Mock the POST response
        let post_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/predictions")
                .json_body_obj(&json!({
                    "version": "v1",
                    "input": {"text": "world"}
                }));
            then.status(200).json_body_obj(&json!({
                "id": "p1",
                "version": "v1",
                "urls": {
                    "get": format!("{}/predictions/p1", server.base_url()),
                    "cancel": format!("{}/predictions/p1", server.base_url()),
                },
                "created_at": "2022-04-26T20:00:40.658234Z",
                "completed_at": "2022-04-26T20:02:27.648305Z",
                "source": "api",
                "status": "processing",
                "input": {"text": "world"},
                "output": None::<String>,
                "error": None::<String>,
                "logs": None::<String>,
            }));
        });

        // Mock the GET response
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/predictions/p1");
            then.status(200).json_body_obj(&json!({
                "id": "p1",
                "version": "v1",
                "urls": {
                    "get": format!("{}/predictions/p1", server.base_url()),
                    "cancel": format!("{}/predictions/p1", server.base_url()),
                },
                "created_at": "2022-04-26T20:00:40.658234Z",
                "completed_at": "2022-04-26T20:02:27.648305Z",
                "source": "api",
                "status": "succeeded",
                "input": {"text": "world"},
                "output": "hello world",
                "error": None::<String>,
                "logs": "",
            }));
        });

        let config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let mut inputs = std::collections::HashMap::new();
        inputs.insert("text", "world");

        let version = String::from("test/model:v1");
        let result = replicate.run(version, inputs).unwrap();

        // Assert that the returned value is correct
        assert_eq!(result.output, Some(serde_json::to_value("hello world")?));

        // Ensure the mocks were called as expected
        post_mock.assert();
        get_mock.assert();

        Ok(())
    }
}

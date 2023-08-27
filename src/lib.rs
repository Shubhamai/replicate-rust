//! An Unofficial Rust client for [Replicate](https://replicate.com). Provides a type-safe interface by deserializing API responses into Rust structs.
//!
//! ## Getting Started
//!
//! Add `replicate_rust` to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! replicate-rust = "0.0.4"
//! ```
//!
//! Grab your token from [replicate.com/account](https://replicate.com/account) and set it as an environment variable:
//!
//! ```sh
//! export REPLICATE_API_TOKEN=<your token>
//! ```
//!
//! Here's an example using `replicate_rust` to run a model:
//!
//! ```rust
//! use replicate_rust::{config::Config, Replicate, errors::ReplicateError};
//!
//! fn main() -> Result<(), ReplicateError> {
//!    let config = Config::default();
//!    // Instead of using the default config ( which reads API token from env variable), you can also set the token directly:
//!    // let config = Config {
//!    //     auth: String::from("REPLICATE_API_TOKEN"),
//!    //     ..Default::default()
//!    // };
//!
//!    let replicate = Replicate::new(config);
//!
//!    // Construct the inputs.
//!    let mut inputs = std::collections::HashMap::new();
//!    inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//!    let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
//!
//!    // Run the model.
//!    let result = replicate.run(version, inputs)?;
//!
//!    // Print the result.
//!    println!("{:?}", result.output);
//!    // Some(Array [String("https://pbxt.replicate.delivery/QLDGe2rXuIQ9ByMViQEXrYCkKfDi9I3YWAzPwWsDZWMXeN7iA/out-0.png")])```
//!
//!    Ok(())
//! }
//! ```
//!
//! ## Usage
//!
//! See the [reference docs](https://docs.rs/replicate-rust/) for detailed API documentation.
//!
//! ## Examples
//!
//! - Run a model in the background:
//!     ```rust
//!     // Construct the inputs.
//!     let mut inputs = std::collections::HashMap::new();
//!     inputs.insert("prompt", "a 19th century portrait of a wombat gentleman");
//!
//!     let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
//!
//!     // Run the model.
//!     let mut prediction = replicate.predictions.create(version, inputs)?;
//!
//!     println!("{:?}", prediction.status);
//!     // 'starting'
//!
//!     prediction.reload()?;
//!     println!("{:?}", prediction.status);
//!     // 'processing'
//!
//!     println!("{:?}", prediction.logs);
//!     // Some("Using seed: 3599
//!     // 0%|          | 0/50 [00:00<?, ?it/s]
//!     // 4%|▍         | 2/50 [00:00<00:04, 10.00it/s]
//!     // 8%|▊         | 4/50 [00:00<00:03, 11.56it/s]
//!    
//!
//!     let prediction = prediction.wait()?;
//!
//!     println!("{:?}", prediction.status);
//!     // 'succeeded'
//!
//!     println!("{:?}", prediction.output);
// !    // Some(Array [String("https://pbxt.replicate.delivery/QLDGe2rXuIQ9ByMViQEXrYCkKfDi9I3YWAzPwWsDZWMXeN7iA/out-0.png")])
//!     ```
//!
//! - Cancel a prediction:
//!   ```rust
//!   // Construct the inputs.
//!   let mut inputs = std::collections::HashMap::new();
//!   inputs.insert("prompt", "a 19th century portrait of a wombat gentleman");
//!
//!   let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
//!
//!   // Run the model.
//!   let mut prediction = replicate.predictions.create(version, inputs)?;
//!
//!   println!("{:?}", prediction.status);
//!   // 'starting'
//!
//!   prediction.cancel()?;
//!
//!   prediction.reload()?;
//!
//!   println!("{:?}", prediction.status);
//!   // 'cancelled'
//!   ```
//!
//! - List predictions:
//!   ```rust
//!   let predictions = replicate.predictions.list()?;
//!   println!("{:?}", predictions);
//!   // ListPredictions { ... }
//!   ```
//!
//! - Get model Information:
//!   ```rust
//!   let model = replicate.models.get("replicate", "hello-world")?;
//!   println!("{:?}", model);
//!   // GetModel { ... }
//!   ```
//!
//! - Get Versions List:
//!   ```rust
//!   let versions = replicate.models.versions.list("replicate", "hello-world")?;
//!   println!("{:?}", versions);
//!   // ListModelVersions { ... }
//!   ```
//!
//! - Get Model Version Information:
//!   ```rust
//!   let model = replicate.models.versions.get("kvfrans",
//!   "clipdraw",
//!   "5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b",)?;
//!   println!("{:?}", model);
//!   // GetModelVersion { ... }
//!   ```
//!
//! - Get Collection Information:
//!   ```rust
//!   let collection = replicate.collections.get("audio-generation")?;
//!   println!("{:?}", collection);
//!   // GetCollectionModels { ... }//!   ```
//!    ```
//! 
//! - Get Collection Lists:
//!   ```rust
//!   let collections = replicate.collections.list()?;
//!   println!("{:?}", collections);
//!   // ListCollectionModels { ... }
//!   ```
//!
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::collections::HashMap;

use api_definitions::GetPrediction;
use collection::Collection;
use config::Config;
use errors::ReplicateError;
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
pub mod errors;
pub mod prediction_client;
pub mod retry;

/// Rust Client for interacting with the [Replicate API](https://replicate.com/docs/api/). Currently supports the following endpoints:
/// * [Predictions](https://replicate.com/docs/reference/http#predictions.create)
/// * [Models](https://replicate.com/docs/reference/http#models.get)
/// * [Trainings](https://replicate.com/docs/reference/http#trainings.create)
/// * [Collections](https://replicate.com/docs/reference/http#collections.get)
#[derive(Clone, Debug)]
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
    /// let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
    ///
    /// // Run the model.
    /// let result = replicate.run(version, inputs)?;
    ///
    /// println!("Output : {:?}", result.output);
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn run<K: serde::Serialize, V: serde::Serialize>(
        &self,
        version: &str,
        inputs: HashMap<K, V>,
    ) -> Result<GetPrediction, ReplicateError> {
        let prediction = Prediction::new(self.config.clone()).create(version, inputs)?;

        prediction.wait()
    }
}

#[cfg(test)]
mod tests {
    use crate::api_definitions::OptionSerdeJson;

    use super::*;
    use httpmock::{
        Method::{GET, POST},
        MockServer,
    };
    use serde_json::json;

    #[test]
    fn test_run() -> Result<(), ReplicateError> {
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

        let result = replicate.run("test/model:v1", inputs)?;

        // Assert that the returned value is correct
        assert_eq!(
            result.output,
            OptionSerdeJson(Some(serde_json::to_value("hello world")?))
        );

        // Ensure the mocks were called as expected
        post_mock.assert();
        get_mock.assert();

        Ok(())
    }
}

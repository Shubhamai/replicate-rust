//! Rust Client for interacting with the [Replicate API](https://replicate.com/docs/api/). Provides a type-safe interface by deserializing API responses into Rust structs.
//!
//! ### Getting Started
//!
//! Add `replicate_rust` to your `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! replicate_rust = "0.0.2"
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
    pub training: Training,

    /// Holds a reference to a Collection struct. Use to get and list model collections present in Replicate.
    pub collection: Collection,
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
        let training = Training::new(config.clone());
        let collection = Collection::new(config.clone());

        Self {
            config,
            predictions,
            models,
            training,
            collection,
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

// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

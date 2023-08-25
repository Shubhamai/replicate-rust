//! Rust Client for interacting with the [Replicate API](https://replicate.com/docs/api/).
//!
//! ### Example
//! In this example we will run a model that generates a caption for an image using the [Stable Diffusion](
//! https://replicate.ai/stability-ai/stable-diffusion) model.
//!
//! ```
//! use replicate::Replicate;
//!
//! // Reading the API key from an environment variable.
//! let api_key = std::env::var("REPLICATE_API_TOKEN").unwrap_or_else(|_| {
//!     eprintln!("REPLICATE_API_TOKEN not set");
//!     std::process::exit(1)
//! });
//!
//! // Create a new Replicate client.
//! let replicate = Replicate::new(api_key)
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
use model::Model;
use prediction::Prediction;
use training::Training;

pub mod client;
pub mod collection;
pub mod model;
pub mod prediction;
pub mod training;
pub mod version;

pub mod api_definitions;
pub mod prediction_client;
pub mod retry;

pub struct Replicate {
    client: client::Client,
    pub predictions: Prediction,
    pub models: Model,
    pub training: Training,
    pub collection: Collection,
}

/// Rust Client for interacting with the [Replicate API](https://replicate.com/docs/api/).
///
impl Replicate {
    /// Create a new Replicate client.
    /// # Arguments
    /// * `api_key` - The API key to use for authentication.
    /// # Example
    /// ```
    /// use replicate::Replicate;
    /// let replicate = Replicate::new("api_key");
    /// ```
    pub fn new() -> Self {
        let client = client::Client::new();

        // TODO : Maybe reference instead of clone
        let predictions = Prediction::new(client.clone());
        let models = Model::new(client.clone());
        let training = Training::new(client.clone());
        let collection = Collection::new(client.clone());

        Self {
            client,
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
    /// use replicate::Replicate;
    /// let replicate = Replicate::new("api_key");
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
        let prediction = Prediction::new(self.client.clone()).create(version, inputs);

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

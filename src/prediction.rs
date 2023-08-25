//! Used to interact with the [Prediction Endpoints](https://replicate.com/docs/reference/http#predictions.get).
//!
//! # Example
//!
//! ```
//!let replicate = Replicate::new();
//!
//!// Construct the inputs.
//!let mut inputs = std::collections::HashMap::new();
//!inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//!let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
//!
//! // Print the result.
//! match result {
//!     Ok(result) => println!("Success : {:?}", result.output),
//!     Err(e) => println!("Error : {}", e),
//! }
//!
//! ```
//!
//! ## Another example to showcase other methods
//!
//! ```
//! let replicate = Replicate::new();
//!
//! // Construct the inputs.
//! let mut inputs = std::collections::HashMap::new();
//! inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//! let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
//!
//! // Run the model.
//! let mut prediction = replicate.predictions.create(version, inputs);
//!
//! println!("Prediction : {:?}", prediction.status);
//!
//! // Refetch the prediction using the reload method.
//! let _ = prediction.reload();
//! println!("Prediction : {:?}", prediction.status);
//!
//! // Cancel the prediction.
//! let _ = prediction.cancel();
//! println!("Predictions : {:?}", prediction.status);;
//!
//! // Wait for the prediction to complete (or fail).
//! match prediction.wait() {
//!        Ok(result) => println!("Success : {:?}", result.output),
//!        Err(e) => println!("Error : {}", e),
//!    }
//! ```
//!
//!

use serde::Serialize;
use std::collections::HashMap;

use crate::{api_definitions::ListPredictions, prediction_client::PredictionClient};

#[derive(Serialize)]
pub struct PredictionPayload<K: serde::Serialize, V: serde::ser::Serialize> {
    pub version: String,
    pub input: HashMap<K, V>,
}

// #[derive(Clone)]
pub struct Prediction {
    // Holds a reference to a Replicate
    pub parent: crate::client::Client,
}

impl Prediction {
    pub fn new(rep: crate::client::Client) -> Self {
        Self { parent: rep }
    }

    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        self,
        version: String,
        inputs: HashMap<K, V>,
    ) -> PredictionClient {
        match PredictionClient::create(self.parent, version, inputs) {
            Ok(prediction) => prediction,
            Err(e) => panic!("Error : {}", e),
        }
    }

    pub fn list(&self) -> Result<ListPredictions, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/predictions", self.parent.base_url))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListPredictions = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

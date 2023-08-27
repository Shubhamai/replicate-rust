//! Used to interact with the [Prediction Endpoints](https://replicate.com/docs/reference/http#predictions.get).
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! // Construct the inputs.
//! let mut inputs = std::collections::HashMap::new();
//! inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//! let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
//!
//! // Run the model.
//! let result = replicate.predictions.create(version, inputs)?.wait()?;
//!
//! // Print the result.
//! println!("Result : {:?}", result.output);
//!
//! # Ok::<(), replicate_rust::errors::ReplicateError>(())
//! ```
//!
//! ## Another example to showcase other methods
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! // Construct the inputs.
//! let mut inputs = std::collections::HashMap::new();
//! inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//! let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
//!
//! // Run the model.
//! let mut prediction = replicate.predictions.create(version, inputs)?;
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
//! println!("Prediction : {:?}", prediction.wait()?);
//!
//! # Ok::<(), replicate_rust::errors::ReplicateError>(())
//! ```
//!
//!

use serde::Serialize;
use std::collections::HashMap;

use crate::{
    api_definitions::{GetPrediction, ListPredictions},
    errors::ReplicateError,
    prediction_client::PredictionClient,
};

/// Used to interact with the [Prediction Endpoints](https://replicate.com/docs/reference/http#predictions.get).
#[derive(Serialize)]
pub struct PredictionPayload<K: serde::Serialize, V: serde::ser::Serialize> {
    /// Version of the model used for the prediction
    pub version: String,

    /// Input to the model
    pub input: HashMap<K, V>,
}

/// Used to interact with the [Prediction Endpoints](https://replicate.com/docs/reference/http#predictions.get).
#[derive(Clone, Debug)]
pub struct Prediction {
    /// Holds a reference to a Config struct. Use to get the base url, auth token among other settings.
    pub parent: crate::config::Config,
}

impl Prediction {
    /// Create a new Prediction struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Create a new prediction, by passing in the model version and inputs to PredictionClient.
    /// PredictionClient contains the necessary methods to interact with the prediction such as reload, cancel and wait.
    ///
    /// # Example
    ///
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
    /// let mut prediction = replicate.predictions.create(version, inputs)?;
    ///
    /// println!("Prediction : {:?}", prediction.status);
    ///
    /// // Refetch the prediction using the reload method.
    /// prediction.reload();
    /// println!("Prediction : {:?}", prediction.status);
    ///
    /// // Wait for the prediction to complete (or fail).
    /// println!("Prediction : {:?}", prediction.wait()?);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    ///
    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        &self,
        version: &str,
        inputs: HashMap<K, V>,
    ) -> Result<PredictionClient, ReplicateError> {
        Ok(PredictionClient::create(
            self.parent.clone(),
            version,
            inputs,
        )?)
    }

    /// List all predictions executed in Replicate by the user.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// let predictions = replicate.predictions.list()?;
    /// println!("Predictions : {:?}", predictions);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn list(&self) -> Result<ListPredictions, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/predictions", self.parent.base_url))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: ListPredictions = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// Get a prediction by passing in the prediction id.
    /// The prediction id can be obtained from the PredictionClient struct.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// let prediction = replicate.predictions.get("rrr4z55ocneqzikepnug6xezpe")?;
    /// println!("Prediction : {:?}", prediction);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn get(&self, id: &str) -> Result<GetPrediction, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/predictions/{}", self.parent.base_url, id))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: GetPrediction = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, Replicate};

    use super::*;
    use httpmock::{Method::GET, MockServer};
    use serde_json::json;

    #[test]
    fn test_list() -> Result<(), ReplicateError> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/predictions");
            then.status(200).json_body_obj(&json!( {
                "next": "https://api.replicate.com/v1/predictions?cursor=cD0yMDIyLTAxLTIxKzIzJTNBMTglM0EyNC41MzAzNTclMkIwMCUzQTAw",
                "previous": None::<String>,
                "results": [
                  {
                    "id": "jpzd7hm5gfcapbfyt4mqytarku",
                    "version":
                      "b21cbe271e65c1718f2999b038c18b45e21e4fba961181fbfae9342fc53b9e05",
                    "urls": {
                      "get": "https://api.replicate.com/v1/predictions/jpzd7hm5gfcapbfyt4mqytarku",
                      "cancel":
                        "https://api.replicate.com/v1/predictions/jpzd7hm5gfcapbfyt4mqytarku/cancel",
                    },
                    "created_at": "2022-04-26T20:00:40.658234Z",
                    "started_at": "2022-04-26T20:00:84.583803Z",
                    "completed_at": "2022-04-26T20:02:27.648305Z",
                    "source": "web",
                    "status": "succeeded",
                  },
                ],
              }
            ));
        });

        let config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let mut input = HashMap::new();
        input.insert("text", "...");

        let result = replicate.predictions.list()?;

        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0].id, "jpzd7hm5gfcapbfyt4mqytarku");

        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), ReplicateError> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/predictions/rrr4z55ocneqzikepnug6xezpe");
            then.status(200).json_body_obj(&json!(  {
                "id": "rrr4z55ocneqzikepnug6xezpe",
                "version":
                  "be04660a5b93ef2aff61e3668dedb4cbeb14941e62a3fd5998364a32d613e35e",
                "urls": {
                  "get": "https://api.replicate.com/v1/predictions/rrr4z55ocneqzikepnug6xezpe",
                  "cancel":
                    "https://api.replicate.com/v1/predictions/rrr4z55ocneqzikepnug6xezpe/cancel",
                },
                "created_at": "2022-09-13T22:54:18.578761Z",
                "started_at": "2022-09-13T22:54:19.438525Z",
                "completed_at": "2022-09-13T22:54:23.236610Z",
                "source": "api",
                "status": "succeeded",
                "input": {
                  "prompt": "oak tree with boletus growing on its branches",
                },
                "output": [
                  "https://replicate.com/api/models/stability-ai/stable-diffusion/files/9c3b6fe4-2d37-4571-a17a-83951b1cb120/out-0.png",
                ],
                "error": None::<String>,
                "logs": "Using seed: 36941...",
                "metrics": {
                  "predict_time": 4.484541,
                },
              }
            ));
        });

        let config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let result = replicate.predictions.get("rrr4z55ocneqzikepnug6xezpe")?;

        assert_eq!(result.id, "rrr4z55ocneqzikepnug6xezpe");

        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }
}

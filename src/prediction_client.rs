//! Helper struct for the prediction struct
//!
//! Used to create a prediction, reload for latest info, cancel it and wait for prediction to complete.
//!
//! # Example
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
//! let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
//!
//! // Create a new prediction
//! let mut prediction = replicate.predictions.create(version, inputs);
//!
//! // Reload the prediction to get the latest info and logs
//! prediction.reload().unwrap();
//!
//! // Cancel the prediction
//! // prediction.cancel().unwrap();
//!
//! // Wait for the prediction to complete
//! let result = prediction.wait().unwrap();
//!
//! println!("Result : {:?}", result);
//!
//! ```

use std::collections::HashMap;

use crate::{
    api_definitions::{CreatePrediction, GetPrediction, PredictionStatus, PredictionsUrls},
    prediction::PredictionPayload,
};

use super::retry::{RetryPolicy, RetryStrategy};

/// Parse a model version string into its model and version parts.
pub fn parse_version(s: &str) -> Option<(&str, &str)> {
    // Split the string at the colon.
    let mut parts = s.splitn(2, ':');

    // Extract the model and version parts.
    let model = parts.next()?;
    let version = parts.next()?;

    // Check if the model part contains a slash.
    if !model.contains('/') {
        return None;
    }

    Some((model, version))
}

/// Helper struct for the Prediction struct. Used to create a prediction, reload for latest info, cancel it and wait for prediction to complete.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct PredictionClient {
    /// Holds a reference to a Configuration struct, which contains the base url,  auth token among other settings.
    pub parent: crate::config::Config,

    /// Unique identifier of the prediction
    pub id: String,
    pub version: String,

    pub urls: PredictionsUrls,

    pub created_at: String,

    pub status: PredictionStatus,

    pub input: HashMap<String, serde_json::Value>,

    pub error: Option<String>,

    pub logs: Option<String>,
}

impl PredictionClient {
    /// Run the prediction of the model version with the given input
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Creating the inputs
    /// let mut inputs = std::collections::HashMap::new();
    /// inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
    ///
    /// let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
    ///
    /// // Create a new prediction
    /// let mut prediction = replicate.predictions.create(version, inputs);
    ///
    /// ```
    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        rep: crate::config::Config,
        version: &str,
        inputs: HashMap<K, V>,
    ) -> Result<PredictionClient, Box<dyn std::error::Error>> {
        // Parse the model version string.
        let (_model, version) = parse_version(&version).unwrap();

        // Construct the request payload
        let payload = PredictionPayload {
            version: version.to_string(),
            input: inputs,
        };

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/predictions", rep.base_url))
            .header("Authorization", format!("Token {}", rep.auth))
            .header("User-Agent", &rep.user_agent)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let result: CreatePrediction = response.json()?;

            Ok(Self {
                parent: rep,
                id: result.id,
                version: result.version,
                urls: result.urls,
                created_at: result.created_at,
                status: result.status,
                input: result.input,
                error: result.error,
                logs: result.logs,
            })
        } else {
            let error_message = response.text()?;
            Err(error_message.into())
        }
    }

    /// Returns the latest info of the prediction
    // # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Creating the inputs
    /// let mut inputs = std::collections::HashMap::new();
    /// inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
    ///
    /// let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
    ///
    /// // Create a new prediction
    /// let mut prediction = replicate.predictions.create(version, inputs);
    ///
    /// // Reload the prediction to get the latest info and logs
    /// prediction.reload().unwrap();
    ///
    /// println!("Prediction : {:?}", prediction.status);
    ///
    /// ```
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/predictions/{}", self.parent.base_url, self.id))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetPrediction = serde_json::from_str(&response_string)?;

        self.id = response_struct.id;
        self.version = response_struct.version;
        self.urls = response_struct.urls;
        self.created_at = response_struct.created_at;
        self.status = response_struct.status;
        self.input = response_struct.input;
        self.error = response_struct.error;
        self.logs = response_struct.logs;

        Ok(())
    }

    /// Cancel the prediction
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Creating the inputs
    /// let mut inputs = std::collections::HashMap::new();
    /// inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
    ///
    /// let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
    ///
    /// // Create a new prediction
    /// let mut prediction = replicate.predictions.create(version, inputs);
    ///
    /// // Cancel the prediction
    /// prediction.cancel().unwrap();
    ///
    /// // Wait for the prediction to complete (or fail).
    /// let result = prediction.wait().unwrap();
    ///
    /// println!("Result : {:?}", result);
    ///
    /// ```
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        client
            .post(format!(
                "{}/predictions/{}/cancel",
                self.parent.base_url, self.id
            ))
            .header("Authorization", format!("Token {}", &self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        self.reload()?;

        Ok(())
    }

    /// Blocks until the predictions are ready and returns the predictions
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Creating the inputs
    /// let mut inputs = std::collections::HashMap::new();
    /// inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
    ///
    /// let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";
    ///
    /// // Create a new prediction
    /// let mut prediction = replicate.predictions.create(version, inputs);
    ///
    /// // Wait for the prediction to complete (or fail).
    /// let result = prediction.wait().unwrap();
    ///
    /// println!("Result : {:?}", result);
    ///
    ///
    /// ```
    pub fn wait(&self) -> Result<GetPrediction, Box<dyn std::error::Error>> {
        // TODO : Implement a retry policy
        let retry_policy = RetryPolicy::new(5, RetryStrategy::FixedDelay(1000));
        let client = reqwest::blocking::Client::new();

        loop {
            let response = client
                .get(format!("{}/predictions/{}", self.parent.base_url, self.id))
                .header("Authorization", format!("Token {}", self.parent.auth))
                .header("User-Agent", &self.parent.user_agent)
                .send()?;

            let response_string = response.text()?;
            let response_struct: GetPrediction = serde_json::from_str(&response_string)?;

            match response_struct.status {
                PredictionStatus::succeeded
                | PredictionStatus::failed
                | PredictionStatus::canceled => {
                    return Ok(response_struct);
                }
                PredictionStatus::processing | PredictionStatus::starting => {
                    // Retry
                    // TODO : Fix the retry implementation
                    retry_policy.step();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, Replicate};

    use super::*;
    use httpmock::{Method::POST, MockServer};
    use serde_json::json;

    #[test]
    fn test_create() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        let post_mock = server.mock(|when, then| {
            when.method(POST).path("/predictions");
            then.status(200).json_body_obj(&json!(  {
                "id": "ufawqhfynnddngldkgtslldrkq",
                "version":
                  "5c7d5dc6dd8bf75c1acaa8565735e7986bc5b66206b55cca93cb72c9bf15ccaa",
                "urls": {
                  "get": "https://api.replicate.com/v1/predictions/ufawqhfynnddngldkgtslldrkq",
                  "cancel":
                    "https://api.replicate.com/v1/predictions/ufawqhfynnddngldkgtslldrkq/cancel",
                },
                "created_at": "2022-04-26T22:13:06.224088Z",
                "started_at": None::<String>,
                "completed_at": None::<String>,
                "status": "starting",
                "input": {
                  "text": "Alice",
                },
                "output": None::<String>,
                "error": None::<String>,
                "logs": None::<String>,
                "metrics": {},
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
        input.insert("text", "Alice");

        let result = replicate.predictions.create(
            "owner/model:632231d0d49d34d5c4633bd838aee3d81d936e59a886fbf28524702003b4c532",
            input,
        );
        assert_eq!(result.id, "ufawqhfynnddngldkgtslldrkq");

        // Ensure the mocks were called as expected
        post_mock.assert();

        Ok(())
    }
}

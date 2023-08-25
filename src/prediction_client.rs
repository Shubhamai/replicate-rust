//! Helper struct for the prediction struct

use std::collections::HashMap;

use crate::{
    api_definitions::{CreatePrediction, GetPrediction, PredictionStatus, PredictionsUrls},
    prediction::PredictionPayload,
};

use super::retry::{RetryPolicy, RetryStrategy};

///Parse a model version string into its model and version parts.
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

/// Helper struct for the prediction struct
pub struct PredictionClient {
    // Holds a reference to a Replicate
    pub parent: crate::client::Client,

    // Unique identifier of the prediction
    // id: String,
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
    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        // &mut self,
        rep: crate::client::Client,
        version: String,
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
                // TODO : Need to do this better
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
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/predictions/{}", self.parent.base_url, self.id))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetPrediction = serde_json::from_str(&response_string)?;

        // TODO : Need to make this cleaner
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
    pub fn wait(self) -> Result<GetPrediction, Box<dyn std::error::Error>> {
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

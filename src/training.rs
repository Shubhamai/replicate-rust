//! Used to interact with the [Training Endpoints](https://replicate.com/docs/reference/http#trainings.create).
//!
//!
//! # Example
//!
//! TODO
//!
//!

use std::collections::HashMap;

use crate::api_definitions::{CreateTraining, GetTraining, ListTraining, WebhookEvents};

pub struct TrainingOptions {
    pub destination: String,

    pub input: HashMap<String, String>,

    pub webhook: String,
    _webhook_events_filter: Option<WebhookEvents>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTrainingPayload {
    pub destination: String,

    pub input: HashMap<String, String>,

    pub webhook: String,
}

// #[derive(Clone)]
pub struct Training {
    // Holds a reference to a Replicate
    pub parent: crate::config::Config,
}

impl Training {
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    pub fn create(
        &self,
        model_owner: String,
        model_name: String,
        version_id: String,
        options: TrainingOptions,
    ) -> Result<CreateTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let payload = CreateTrainingPayload {
            destination: options.destination,
            input: options.input,
            webhook: options.webhook,
        };

        let response = client
            .post(format!(
                "{}/models/{}/{}/versions/{}/trainings",
                self.parent.base_url, model_owner, model_name, version_id,
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .json(&payload)
            .send()?;

        let response_string = response.text()?;
        let response_struct: CreateTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    pub fn get(&self, training_id: String) -> Result<GetTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/trainings/{}",
                self.parent.base_url, training_id,
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    pub fn list(&self) -> Result<ListTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/trainings", self.parent.base_url,))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    // Perhaps the training_id should be automatically derives, just like prediction one
    pub fn cancel(&self, training_id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        client
            .get(format!(
                "{}/trainings/{}/cancel",
                self.parent.base_url, training_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        Ok(())
    }
}

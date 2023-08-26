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
    pub fn cancel(&self, training_id: String) -> Result<GetTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(format!(
                "{}/trainings/{}/cancel",
                self.parent.base_url, training_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;
        let response_string = response.text()?;
        let response_struct: GetTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

#[cfg(test)]
mod tests {
    use crate::{api_definitions::PredictionStatus, config::Config, Replicate};

    use super::*;
    use httpmock::{
        Method::{GET, POST},
        MockServer,
    };
    use serde_json::json;

    #[test]
    fn test_create() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        let post_mock = server.mock(|when, then| {
            when.method(POST).path("/models/owner/model/versions/632231d0d49d34d5c4633bd838aee3d81d936e59a886fbf28524702003b4c532/trainings");
            then.status(200).json_body_obj(&json!( {
                "id": "zz4ibbonubfz7carwiefibzgga",
                "version": "{version}",
                "status": "starting",
                "input": {
                  "text": "...",
                },
                "output": None::<String>,
                "error": None::<String>,
                "logs": None::<String>,
                "started_at": None::<String>,
                "created_at": "2023-03-28T21:47:58.566434Z",
                "completed_at": None::<String>,
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
        input.insert(String::from("text"), String::from("..."));

        let result = replicate.trainings.create(
            String::from("owner"),
            String::from("model"),
            String::from("632231d0d49d34d5c4633bd838aee3d81d936e59a886fbf28524702003b4c532"),
            TrainingOptions {
                destination: String::from("new_owner/new_model"),
                input,
                webhook: String::from("webhook"),
                _webhook_events_filter: None,
            },
        );

        assert_eq!(result?.id, "zz4ibbonubfz7carwiefibzgga");
        // Ensure the mocks were called as expected
        post_mock.assert();

        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/trainings/zz4ibbonubfz7carwiefibzgga");
            then.status(200).json_body_obj(&json!( {
                "id": "zz4ibbonubfz7carwiefibzgga",
                "version": "{version}",
                "status": "succeeded",
                "input": {
                  "text": "...",
                  "param" : "..."
                },
                "output": {
                    "version": "...",
                  },
                "error": None::<String>,
                "logs": None::<String>,
                "webhook_completed": None::<String>,
                "started_at": None::<String>,
                "created_at": "2023-03-28T21:47:58.566434Z",
                "completed_at": None::<String>,
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
            .trainings
            .get(String::from("zz4ibbonubfz7carwiefibzgga"));

        assert_eq!(result?.status, PredictionStatus::succeeded);
        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }

    #[test]
    fn test_cancel() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/trainings/zz4ibbonubfz7carwiefibzgga/cancel");
            then.status(200).json_body_obj(&json!( {
                "id": "zz4ibbonubfz7carwiefibzgga",
                "version": "{version}",
                "status": "canceled",
                "input": {
                  "text": "...",
                  "param1" : "..."
                },
                "output": {
                    "version": "...",
                  },
                "error": None::<String>,
                "logs": None::<String>,
                "webhook_completed": None::<String>,
                "started_at": None::<String>,
                "created_at": "2023-03-28T21:47:58.566434Z",
                "completed_at": None::<String>,
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
            .trainings
            .cancel(String::from("zz4ibbonubfz7carwiefibzgga"))?;

        assert_eq!(result.status, PredictionStatus::canceled);
        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }

    #[test]
    fn test_list() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/trainings");
            then.status(200).json_body_obj(&json!( {
                "next": "https://api.replicate.com/v1/trainings?cursor=cD0yMDIyLTAxLTIxKzIzJTNBMTglM0EyNC41MzAzNTclMkIwMCUzQTAw",
                "previous": None::<String>,
                "results": [
                  {
                    "id": "jpzd7hm5gfcapbfyt4mqytarku",
                    "version": "b21cbe271e65c1718f2999b038c18b45e21e4fba961181fbfae9342fc53b9e05",
                    "urls": {
                      "get": "https://api.replicate.com/v1/trainings/jpzd7hm5gfcapbfyt4mqytarku",
                      "cancel": "https://api.replicate.com/v1/trainings/jpzd7hm5gfcapbfyt4mqytarku/cancel"
                    },
                    "created_at": "2022-04-26T20:00:40.658234Z",
                    "started_at": "2022-04-26T20:00:84.583803Z",
                    "completed_at": "2022-04-26T20:02:27.648305Z",
                    "source": "web",
                    "status": "succeeded"
                  }
                ]
              }
              
            ));
        });

        let config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let result = replicate.trainings.list()?;

        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0].id, "jpzd7hm5gfcapbfyt4mqytarku");

        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }
}

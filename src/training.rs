//! Used to interact with the [Training Endpoints](https://replicate.com/docs/reference/http#trainings.create).
//!
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config, training::TrainingOptions};
//! use std::collections::HashMap;
//! 
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//! 
//! let mut input = HashMap::new();
//! input.insert(String::from("train_data"), String::from("https://example.com/70k_samples.jsonl"));
//!
//! let result = replicate.trainings.create(
//!     "owner",
//!     "model",
//!     "632231d0d49d34d5c4633bd838aee3d81d936e59a886fbf28524702003b4c532",
//!     TrainingOptions {
//!         destination: String::from("new_owner/new_name"),
//!         input,
//!         webhook: String::from("https://example.com/my-webhook"),
//!         _webhook_events_filter: None,
//!     },
//! )?;
//! # Ok::<(), replicate_rust::errors::ReplicateError>(())
//! ```
//!
//!

use std::collections::HashMap;

use crate::{api_definitions::{CreateTraining, GetTraining, ListTraining, WebhookEvents}, errors::ReplicateError};

/// Contains all the options for creating a training.
pub struct TrainingOptions {

    /// A string representing the desired model to push to in the format {destination_model_owner}/{destination_model_name}. This should be an existing model owned by the user or organization making the API request. If the destination is invalid, the server returns an appropriate 4XX response.
    pub destination: String,

    /// An object containing inputs to the Cog model's train() function.
    pub input: HashMap<String, String>,

    /// An HTTPS URL for receiving a webhook when the training completes. The webhook will be a POST request where the request body is the same as the response body of the get training operation. If there are network problems, we will retry the webhook a few times, so make sure it can be safely called more than once.
    pub webhook: String,

    /// TO only send specifc events to the webhook, use this field. If not specified, all events will be sent. TODO : Add this to the API 
    pub _webhook_events_filter: Option<WebhookEvents>,
}


/// Data to be sent to the API when creating a training.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTrainingPayload {

    /// A string representing the desired model to push to in the format {destination_model_owner}/{destination_model_name}. This should be an existing model owned by the user or organization making the API request. If the destination is invalid, the server returns an appropriate 4XX response.
    pub destination: String,

    /// An object containing inputs to the Cog model's train() function.
    pub input: HashMap<String, String>,

    /// An HTTPS URL for receiving a webhook when the training completes. The webhook will be a POST request where the request body is the same as the response body of the get training operation. If there are network problems, we will retry the webhook a few times, so make sure it can be safely called more than once.
    pub webhook: String,
}

/// Used to interact with the [Training Endpoints](https://replicate.com/docs/reference/http#trainings.create).
#[derive(Clone, Debug)]
pub struct Training {
    /// Holds a reference to a Configuration struct, which contains the base url, auth token among other settings.
    pub parent: crate::config::Config,
}

/// Training struct contains all the functionality for interacting with the training endpoints of the Replicate API.
impl Training {

    /// Create a new Training struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Create a new training.
    /// 
    /// # Arguments
    /// * `model_owner` - The name of the user or organization that owns the model.
    /// * `model_name` - The name of the model.
    /// * `version_id` - The ID of the version.
    /// * `options` - The options for creating a training.
    ///     * `destination` - A string representing the desired model to push to in the format {destination_model_owner}/{destination_model_name}. This should be an existing model owned by the user or organization making the API request. If the destination is invalid, the server returns an appropriate 4XX response.
    ///    * `input` - An object containing inputs to the Cog model's train() function.
    ///   * `webhook` - An HTTPS URL for receiving a webhook when the training completes. The webhook will be a POST request where the request body is the same as the response body of the get training operation. If there are network problems, we will retry the webhook a few times, so make sure it can be safely called more than once.
    ///  * `_webhook_events_filter` - TO only send specifc events to the webhook, use this field. If not specified, all events will be sent. The following events are supported:
    /// 
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config, training::TrainingOptions};
    /// use std::collections::HashMap;
    /// 
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    /// 
    /// let mut input = HashMap::new();
    /// input.insert(String::from("training_data"), String::from("https://example.com/70k_samples.jsonl"));
    /// 
    /// let result = replicate.trainings.create(
    ///    "owner",
    ///    "model",
    ///   "632231d0d49d34d5c4633bd838aee3d81d936e59a886fbf28524702003b4c532",
    ///  TrainingOptions {
    ///     destination: String::from("new_owner/new_name"),
    ///     input,
    ///     webhook: String::from("https://example.com/my-webhook"),
    ///     _webhook_events_filter: None,
    /// },
    /// )?;
    /// 
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    /// 
    pub fn create(
        &self,
        model_owner: &str,
        model_name: &str,
        version_id: &str,
        options: TrainingOptions,
    ) -> Result<CreateTraining, ReplicateError> {
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

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: CreateTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }


    /// Get the details of a training.
    /// 
    /// # Arguments
    /// * `training_id` - The ID of the training you want to get.
    /// 
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    /// 
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    /// 
    /// let training = replicate.trainings.get("zz4ibbonubfz7carwiefibzgga")?;
    /// println!("Training : {:?}", training);
    /// 
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ``` 
    pub fn get(&self, training_id: &str) -> Result<GetTraining, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/trainings/{}",
                self.parent.base_url, training_id,
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
                .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: GetTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// Get a paginated list of trainings that you've created with your account. Returns 100 records per page.
    /// 
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    /// 
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    /// 
    /// let trainings = replicate.trainings.list()?;
    /// println!("Trainings : {:?}", trainings);
    /// 
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn list(&self) -> Result<ListTraining, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/trainings", self.parent.base_url,))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
                .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: ListTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// Cancel a training.
    /// 
    /// # Arguments
    /// * `training_id` - The ID of the training you want to cancel.
    /// 
    /// # Example
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    /// 
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    /// 
    /// let result =  replicate.trainings.cancel("zz4ibbonubfz7carwiefibzgga")?;
    /// println!("Result : {:?}", result);
    /// 
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn cancel(&self, training_id: &str) -> Result<GetTraining, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(format!(
                "{}/trainings/{}/cancel",
                self.parent.base_url, training_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
                .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }
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
    fn test_create() -> Result<(), ReplicateError> {
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
        input.insert(String::from("text"),String::from("..."));

        let result = replicate.trainings.create(
            "owner",
            "model",
            "632231d0d49d34d5c4633bd838aee3d81d936e59a886fbf28524702003b4c532",
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
    fn test_get() -> Result<(), ReplicateError> {
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
            .get("zz4ibbonubfz7carwiefibzgga");

        assert_eq!(result?.status, PredictionStatus::succeeded);
        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }

    #[test]
    fn test_cancel() -> Result<(), ReplicateError> {
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
            .cancel("zz4ibbonubfz7carwiefibzgga")?;

        assert_eq!(result.status, PredictionStatus::canceled);
        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }

    #[test]
    fn test_list() -> Result<(), ReplicateError> {
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

//! This module contains the definition of the API responses by the Replicate API.
//! The responses the documented in the [HTTP API reference](https://replicate.com/docs/reference/http).
//!
//! The API responses are defined as structs that implement the `serde::Deserialize` trait.
//!

// Allow rustdoc::bare_urls for the whole module
#![allow(rustdoc::bare_urls)]
#![allow(missing_docs)]

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// If the object is empty, return None
pub fn object_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux<T> {
        T(T),
        Empty(Empty),
        Null,
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::T(t) => Ok(Some(t)),
        Aux::Empty(_) | Aux::Null => Ok(None),
    }
}

/// GET https://api.replicate.com/v1/models/{model_owner}/{model_name}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetModel {
    pub url: String,

    pub owner: String,
    pub name: String,
    pub description: String,
    pub visibility: String,

    pub github_url: Option<String>,
    pub paper_url: Option<String>,
    pub license_url: Option<String>,

    pub run_count: Option<u32>,

    pub cover_image_url: Option<String>,

    #[serde(deserialize_with = "object_empty_as_none")]
    pub default_example: Option<GetPrediction>,

    #[serde(deserialize_with = "object_empty_as_none")]
    pub latest_version: Option<GetModelVersion>,
}

/// GET https://api.replicate.com/v1/collections/{collection_slug}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetCollectionModels {
    pub name: String,
    pub slug: String,

    pub description: String,

    pub models: Vec<GetModel>,
}

/// Prediction urls to iether cancel or get the prediction
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PredictionsUrls {
    pub cancel: String,
    pub get: String,
}

/// POST https://api.replicate.com/v1/predictions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPrediction {
    // Unique identifier of the prediction
    pub id: String,

    // Version of the model used for the prediction
    pub version: String,

    // Urls to cancel or get the prediction
    pub urls: PredictionsUrls,

    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,

    pub source: Option<PredictionSource>,

    // Status of the prediction
    pub status: PredictionStatus,

    // Input and Outputs of the prediction
    pub input: HashMap<String, serde_json::Value>,

    // Either a vector of string or a simple string
    pub output: Option<serde_json::Value>,

    pub error: Option<String>,
    pub logs: Option<String>,

    pub metrics: Option<HashMap<String, serde_json::Value>>,
}

/// GET https://api.replicate.com/v1/trainings/{training_id}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetTraining {
    pub id: String,
    pub version: String,

    pub status: PredictionStatus,

    pub input: Option<HashMap<String, String>>,
    pub output: Option<HashMap<String, String>>,

    pub error: Option<String>,
    pub logs: Option<String>,
    pub webhook_completed: Option<String>,

    pub started_at: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// POST https://api.replicate.com/v1/models/{model_owner}/{model_name}/versions/{version_id}/trainings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateTraining {
    pub id: String,
    pub version: String,

    pub status: PredictionStatus,

    pub input: Option<HashMap<String, String>>,
    pub output: Option<HashMap<String, String>>,

    pub logs: Option<String>,

    pub started_at: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// POST https://api.replicate.com/v1/predictions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreatePrediction {
    pub id: String,
    pub version: String,

    pub urls: PredictionsUrls,

    pub created_at: String,

    pub status: PredictionStatus,

    pub input: HashMap<String, serde_json::Value>,

    pub error: Option<String>,

    pub logs: Option<String>,
}

/// GET https://api.replicate.com/v1/models/{model_owner}/{model_name}/versions/{version_id}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetModelVersion {
    pub id: String,
    pub created_at: String,

    pub cog_version: String,

    pub openapi_schema: HashMap<String, serde_json::Value>,
}

/// Each item of the list of collections
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListCollectionModelsItem {
    pub name: String,
    pub slug: String,
    pub description: String,
}

/// GET https://api.replicate.com/v1/collections
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListCollectionModels {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<ListCollectionModelsItem>,
}

/// Represents a prediction in the list of predictions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PredictionsListItem {
    pub id: String,
    pub version: String,

    pub urls: PredictionsUrls,

    pub created_at: String,
    pub started_at: String,
    pub completed_at: Option<String>,

    pub source: Option<PredictionSource>,

    pub status: PredictionStatus,
}

/// GET https://api.replicate.com/v1/predictions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListPredictions {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<PredictionsListItem>,
}

/// GET https://api.replicate.com/v1/models/{model_owner}/{model_name}/versions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListModelVersions {
    pub previous: Option<String>,

    pub next: Option<String>,

    pub results: Vec<GetModelVersion>,
}

/// Each item of the list of trainings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListTrainingItem {
    pub id: String,

    pub version: String,

    pub urls: PredictionsUrls,

    pub created_at: String,
    pub started_at: String,
    pub completed_at: String,

    pub source: PredictionSource,
    pub status: PredictionStatus,
}

/// GET https://api.replicate.com/v1/trainings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListTraining {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<ListTrainingItem>,
}

///////////////////////////////////////////////////////////

/// Source of the prediction, either from the API or from the web
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PredictionSource {
    api,
    web,
}

/// Status of the prediction, either starting, processing, succeeded, failed or canceled
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PredictionStatus {
    starting,
    processing,
    succeeded,
    failed,
    canceled,
}

/// Events of the webhook, either start, output, logs or completed
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum WebhookEvents {
    start,
    output,
    logs,
    completed,
}

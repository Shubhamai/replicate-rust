//! This module contains the definition of the API responses by the Replicate API.
//! The responses the documented in the [HTTP API reference](https://replicate.com/docs/reference/http).
//!
//! The API responses are defined as structs that implement the `serde::Deserialize` trait.
//!
//!

use serde::Deserialize;
use std::collections::HashMap;

/// GET https://api.replicate.com/v1/models/{model_owner}/{model_name}
#[derive(Deserialize, Debug)]
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

    pub default_example: Option<GetPrediction>,

    pub latest_version: Option<GetModelVersion>,
}

/// GET https://api.replicate.com/v1/collections/{collection_slug}
#[derive(serde::Deserialize, Debug)]
pub struct GetCollectionModels {
    pub name: String,
    pub slug: String,

    pub description: String,

    pub models: Vec<GetModel>,
}

#[derive(serde::Deserialize, Debug)]
pub struct PredictionsUrls {
    pub cancel: String,
    pub get: String,
}

/// POST https://api.replicate.com/v1/predictions
#[derive(serde::Deserialize, Debug)]
pub struct GetPrediction {
    // Unique identifier of the prediction
    pub id: String,

    // Version of the model used for the prediction
    pub version: String,

    // Urls to cancel or get the prediction
    pub urls: PredictionsUrls,

    pub created_at: String,
    pub started_at: String,
    pub completed_at: Option<String>,

    pub source: Option<PredictionSource>,

    // Status of the prediction
    pub status: PredictionStatus,

    // Input and Outputs of the prediction
    pub input: HashMap<String, serde_json::Value>,

    // Either a vector of string or a simple string
    // TODO : previous it was a Option<String>
    pub output: Option<serde_json::Value>,

    pub error: Option<String>,
    pub logs: Option<String>,

    pub metrics: Option<HashMap<String, serde_json::Value>>,
}

/// GET https://api.replicate.com/v1/trainings/{training_id}
#[derive(Debug, serde::Deserialize)]
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
#[derive(Debug, serde::Deserialize)]
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

#[derive(serde::Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct GetModelVersion {
    pub id: String,
    pub created_at: String,

    pub cog_version: String,

    pub openapi_schema: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct ListCollectionModelsItem {
    pub name: String,
    pub slug: String,
    pub description: String,
}

/// GET https://api.replicate.com/v1/collections
#[derive(Deserialize, Debug)]
pub struct ListCollectionModels {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<ListCollectionModelsItem>,
}

#[derive(serde::Deserialize, Debug)]
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
#[derive(serde::Deserialize, Debug)]
pub struct ListPredictions {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<PredictionsListItem>,
}

/// GET https://api.replicate.com/v1/models/{model_owner}/{model_name}/versions
#[derive(serde::Deserialize, Debug)]
pub struct ListModelVersions {
    pub previous: Option<String>,

    pub next: Option<String>,

    pub results: Vec<GetModelVersion>,
}

/// Each item of the list of trainings
#[derive(Debug, serde::Deserialize)]
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
#[derive(Debug, serde::Deserialize)]
pub struct ListTraining {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<ListTrainingItem>,
}

///////////////////////////////////////////////////////////

/// Source of the prediction, either from the API or from the web
#[derive(serde::Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PredictionSource {
    api,
    web,
}

/// Status of the prediction, either starting, processing, succeeded, failed or canceled
#[derive(serde::Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PredictionStatus {
    starting,
    processing,
    succeeded,
    failed,
    canceled,
}

/// Events of the webhook, either start, output, logs or completed
#[allow(non_camel_case_types)]
pub enum WebhookEvents {
    start,
    output,
    logs,
    completed,
}

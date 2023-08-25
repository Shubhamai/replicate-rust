use super::PredictionsUrls::PredictionsUrls;
use crate::enums::{PredictionSource::PredictionSource, PredictionStatus::PredictionStatus};
use std::collections::HashMap;

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

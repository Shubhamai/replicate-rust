use super::PredictionsUrls::PredictionsUrls;
use crate::enums::PredictionStatus::PredictionStatus;
use std::collections::HashMap;

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

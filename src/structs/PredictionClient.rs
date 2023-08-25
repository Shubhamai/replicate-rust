use std::collections::HashMap;

use super::PredictionsUrls::PredictionsUrls;
use crate::enums::PredictionStatus::PredictionStatus;

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

use std::collections::HashMap;
use crate::enums::PredictionStatus::PredictionStatus;

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

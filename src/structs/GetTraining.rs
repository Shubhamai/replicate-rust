use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct GetTraining {
    pub id: String,
    pub version: String,

    pub status: crate::enums::PredictionStatus::PredictionStatus,

    pub input: Option<HashMap<String, String>>,
    pub output: Option<HashMap<String, String>>,

    pub error: Option<String>,
    pub logs: Option<String>,
    pub webhook_completed: Option<String>,

    pub started_at: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

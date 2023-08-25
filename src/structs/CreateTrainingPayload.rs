use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTrainingPayload {
    pub destination: String,

    pub input: HashMap<String, String>,

    pub webhook: String,
}

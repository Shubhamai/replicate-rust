use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct PredictionPayload<K: serde::Serialize, V: serde::ser::Serialize> {
    pub version: String,
    pub input: HashMap<K, V>,
}

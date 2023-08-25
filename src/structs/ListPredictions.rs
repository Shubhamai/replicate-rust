use super::PredictionsUrls::PredictionsUrls;
use crate::enums::{PredictionSource::PredictionSource, PredictionStatus::PredictionStatus};

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
#[derive(serde::Deserialize, Debug)]
pub struct ListPredictions {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<PredictionsListItem>,
}

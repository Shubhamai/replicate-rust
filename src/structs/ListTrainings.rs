use super::PredictionsUrls::PredictionsUrls;

#[derive(Debug, serde::Deserialize)]
pub struct ListTrainingItem {
    pub id: String,

    pub version: String,

    pub urls: PredictionsUrls,

    pub created_at: String,
    pub started_at: String,
    pub completed_at: String,

    pub source: crate::enums::PredictionSource::PredictionSource,
    pub status: crate::enums::PredictionStatus::PredictionStatus,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListTraining {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<ListTrainingItem>,
}

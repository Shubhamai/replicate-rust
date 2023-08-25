use super::{GetModelVersion::GetModelVersion, GetPrediction::GetPrediction};
use serde::Deserialize;

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

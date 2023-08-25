#[derive(serde::Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PredictionStatus {
    starting,
    processing,
    succeeded,
    failed,
    canceled,
}
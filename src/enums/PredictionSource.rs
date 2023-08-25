#[derive(serde::Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PredictionSource {
    api,
    web,
}
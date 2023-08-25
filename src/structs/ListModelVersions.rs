#[derive(serde::Deserialize, Debug)]
pub struct ListModelVersions {
    pub previous: Option<String>,

    pub next: Option<String>,

    pub results: Vec<crate::structs::GetModelVersion::GetModelVersion>,
}

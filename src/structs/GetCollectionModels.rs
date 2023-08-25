use super::GetModel::GetModel;

#[derive(serde::Deserialize, Debug)]
pub struct GetCollectionModels {
    pub name: String,
    pub slug: String,

    pub description: String,

    pub models: Vec<GetModel>,
}

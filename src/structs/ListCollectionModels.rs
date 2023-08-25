use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ListCollectionModelsItem {
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct ListCollectionModels {
    pub previous: Option<String>,
    pub next: Option<String>,

    pub results: Vec<ListCollectionModelsItem>,
}

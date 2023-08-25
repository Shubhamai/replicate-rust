use crate::structs::{
    Collection::Collection, GetCollectionModels::GetCollectionModels,
    ListCollectionModels::ListCollectionModels,
};

impl Collection {
    pub fn new(rep: crate::client::Client) -> Self {
        Self { parent: rep }
    }

    pub fn get(
        &self,
        collection_slug: String,
    ) -> Result<GetCollectionModels, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/collections/{}",
                self.parent.base_url, collection_slug
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetCollectionModels = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    pub fn list(&self) -> Result<ListCollectionModels, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/collections", self.parent.base_url))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListCollectionModels = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

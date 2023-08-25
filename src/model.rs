use crate::structs::{GetModel::GetModel, Model::Model};

impl Model {
    pub fn new(rep: crate::client::Client) -> Self {
        let versions = crate::structs::Version::Version::new(rep.clone());
        Self {
            parent: rep,
            versions,
        }
    }

    // Get the model by name
    pub fn get(
        &self,
        model_owner: String,
        model_name: String,
    ) -> Result<GetModel, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}",
                self.parent.base_url, model_owner, model_name
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetModel = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

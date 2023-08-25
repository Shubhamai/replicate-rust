use crate::structs::{
    GetModelVersion::GetModelVersion, ListModelVersions::ListModelVersions, Version::Version,
};

impl Version {
    pub fn new(rep: crate::client::Client) -> Self {
        Self { parent: rep }
    }

    pub fn get(
        &self,
        model_owner: String,
        model_name: String,
        version_id: String,
    ) -> Result<GetModelVersion, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}/versions/{}",
                self.parent.base_url, model_owner, model_name, version_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetModelVersion = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    pub fn list(
        &self,
        model_owner: String,
        model_name: String,
    ) -> Result<ListModelVersions, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}/versions",
                self.parent.base_url, model_owner, model_name
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListModelVersions = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

use crate::structs::{
    ListPredictions::ListPredictions, Prediction::Prediction, PredictionClient::PredictionClient,
};
use std::collections::HashMap;
// use crate::structs::Prediction::;

impl Prediction {
    pub fn new(rep: crate::client::Client) -> Self {
        Self { parent: rep }
    }

    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        self,
        version: String,
        inputs: HashMap<K, V>,
    ) -> PredictionClient {
        match PredictionClient::create(self.parent, version, inputs) {
            Ok(prediction) => prediction,
            Err(e) => panic!("Error : {}", e),
        }
    }

    pub fn list(&self) -> Result<ListPredictions, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/predictions", self.parent.base_url))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListPredictions = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

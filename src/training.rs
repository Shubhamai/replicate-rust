use crate::structs::{
    CreateTraining::CreateTraining, CreateTrainingPayload::CreateTrainingPayload,
    GetTraining::GetTraining, ListTrainings::ListTraining, Training::Training,
    TrainingOptions::TrainingOptions,
};

impl Training {
    pub fn new(rep: crate::client::Client) -> Self {
        Self { parent: rep }
    }

    pub fn create(
        &self,
        model_owner: String,
        model_name: String,
        version_id: String,
        options: TrainingOptions,
    ) -> Result<CreateTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let payload = CreateTrainingPayload {
            destination: options.destination,
            input: options.input,
            webhook: options.webhook,
        };

        let response = client
            .post(format!(
                "{}/models/{}/{}/versions/{}/trainings",
                self.parent.base_url, model_owner, model_name, version_id,
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .json(&payload)
            .send()?;

        let response_string = response.text()?;
        let response_struct: CreateTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    pub fn get(&self, training_id: String) -> Result<GetTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/trainings/{}",
                self.parent.base_url, training_id,
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    pub fn list(&self) -> Result<ListTraining, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!("{}/trainings", self.parent.base_url,))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: ListTraining = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    // Perhaps the training_id should be automatically derives, just like prediction one
    pub fn cancel(&self, training_id: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        client
            .get(format!(
                "{}/trainings/{}/cancel",
                self.parent.base_url, training_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        Ok(())
    }
}
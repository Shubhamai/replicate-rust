use std::collections::HashMap;

pub mod prediction;

// Parse a model version string into its model and version parts.
fn parse_version(s: &str) -> Option<(&str, &str)> {
    // Split the string at the colon.
    let mut parts = s.splitn(2, ':');

    // Extract the model and version parts.
    let model = parts.next()?;
    let version = parts.next()?;

    // Check if the model part contains a slash.
    if !model.contains('/') {
        return None;
    }

    Some((model, version))
}
pub struct Replicate {
    auth: String,
    user_agent: String,
    base_url: String,
}

impl Replicate {
    pub fn new(auth: String) -> Self {
        Self {
            auth,
            user_agent: format!("replicate-rust/{}", env!("CARGO_PKG_VERSION")),
            base_url: String::from("https://api.replicate.com/v1/predictions"),
        }
    }

    pub fn run<K: serde::Serialize, V: serde::Serialize>(
        &self,
        version: String,
        inputs: HashMap<K, V>,
        // TODO : Perhaps not Box<dyn std::error::Error> but something more specific?
    ) -> Result<prediction::GetPrediction, Box<dyn std::error::Error>> {
        let prediction = prediction::CreatePredictionStruct::new(self).create(version, inputs);

        prediction.wait()
    }

    pub fn predictions(&self) -> prediction::CreatePredictionStruct {
        prediction::CreatePredictionStruct::new(self)
    }
}

// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Payload<K: serde::Serialize, V: serde::ser::Serialize> {
    version: String,
    input: HashMap<K, V>,
}

struct RetryPolicy {
    max_retries: u32,
    strategy: RetryStrategy,
}

enum RetryStrategy {
    // Retry with a fixed delay.
    FixedDelay(u32),
    // Retry with an exponential backoff.
    // ExponentialBackoff(u32),
}

// Parse a model version string into its model and version parts.
fn parse_model_version(s: &str) -> Option<(&str, &str)> {
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
    pub fn new(auth: String, user_agent: String, base_url: String) -> Self {
        Self {
            auth,
            user_agent,
            base_url,
        }
    }

    pub async fn run<K: serde::Serialize, V: serde::ser::Serialize>(
        &self,
        model_version: String,
        inputs: HashMap<K, V>,
        // TODO : Perhaps not Box<dyn std::error::Error> but something more specific?
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        // Parse the model version string.
        let (model, version) = parse_model_version(&model_version).unwrap();

        // Construct the request payload
        let payload = Payload {
            version: version.to_string(),
            input: inputs,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&self.base_url)
            .header("Authorization", format!("Token {}", &self.auth))
            .header("User-Agent", &self.user_agent)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let result: HashMap<String, serde_json::Value> = response.json().await?;
            // println!("{:?}", result);
            Ok(result)
        } else {
            let error_message = response.text().await?;
            // println!("Error message: {}", error_message);

            Err(error_message.into())
        }
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

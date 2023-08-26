//! Used to interact with the [Prediction Endpoints](https://replicate.com/docs/reference/http#predictions.get).
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! // Construct the inputs.
//! let mut inputs = std::collections::HashMap::new();
//! inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//! let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
//! 
//! // Run the model.
//! let result = replicate.predictions.create(version, inputs).wait();
//! 
//! // Print the result.
//! match result {
//!     Ok(result) => println!("Success : {:?}", result.output),
//!     Err(e) => println!("Error : {}", e),
//! }
//!
//! ```
//!
//! ## Another example to showcase other methods
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! // Construct the inputs.
//! let mut inputs = std::collections::HashMap::new();
//! inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
//!
//! let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
//!
//! // Run the model.
//! let mut prediction = replicate.predictions.create(version, inputs);
//!
//! println!("Prediction : {:?}", prediction.status);
//!
//! // Refetch the prediction using the reload method.
//! let _ = prediction.reload();
//! println!("Prediction : {:?}", prediction.status);
//!
//! // Cancel the prediction.
//! let _ = prediction.cancel();
//! println!("Predictions : {:?}", prediction.status);;
//!
//! // Wait for the prediction to complete (or fail).
//! match prediction.wait() {
//!        Ok(result) => println!("Success : {:?}", result.output),
//!        Err(e) => println!("Error : {}", e),
//!    }
//! ```
//!
//!

use serde::Serialize;
use std::collections::HashMap;

use crate::{api_definitions::ListPredictions, prediction_client::PredictionClient};

#[derive(Serialize)]
pub struct PredictionPayload<K: serde::Serialize, V: serde::ser::Serialize> {
    pub version: String,
    pub input: HashMap<K, V>,
}

#[derive(Clone)]
pub struct Prediction {
    // Holds a reference to a Config struct. Use to get the base url, auth token among other settings.
    pub parent: crate::config::Config,
}

impl Prediction {
    /// Create a new Prediction struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Create a new prediction, by passing in the model version and inputs to PredictionClient.
    /// PredictionClient contains the necessary methods to interact with the prediction such as reload, cancel and wait.
    /// 
    /// # Example
    /// 
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    /// 
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    /// 
    /// // Construct the inputs.
    /// let mut inputs = std::collections::HashMap::new();
    /// inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");
    /// 
    /// let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");
    /// 
    /// // Run the model.
    /// let mut prediction = replicate.predictions.create(version, inputs);
    /// 
    /// println!("Prediction : {:?}", prediction.status);
    /// 
    /// // Refetch the prediction using the reload method.
    /// prediction.reload();
    /// println!("Prediction : {:?}", prediction.status);
    /// 
    /// // Wait for the prediction to complete (or fail).
    /// match prediction.wait() {
    ///    Ok(result) => println!("Success : {:?}", result.output),
    ///    Err(e) => println!("Error : {}", e),
    /// }
    /// ```
    /// 
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

    /// List all predictions executed in Replicate by the user.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// match replicate.predictions.list() {
    ///    Ok(result) => println!("Success : {:?}", result),
    ///    Err(e) => println!("Error : {}", e),
    /// };
    /// ```
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

// mod tests {
//     use super::*;

//     #[test]
//     fn test_predict() {
//         // Start a local mock server.
//         let server = MockServer::start();

//         // Define mock for the POST request
//         let post_mock = Mock::new()
//             .expect_method(POST)
//             .expect_path("/v1/predictions")
//             .expect_body(json!({
//                 "version": "v1",
//                 "input": {"text": "world"}
//             }))
//             .return_status(200)
//             .return_json_body(&json!({
//                 "id": "p1",
//                 "version": "v1",
//                 "urls": {
//                     "get": format!("{}/v1/predictions/p1", server.base_url()),
//                     "cancel": format!("{}/v1/predictions/p1/cancel", server.base_url()),
//                 },
//                 // ... the rest of the response
//             }))
//             .create_on(&server);

//         // Define mock for the GET request
//         let get_mock = Mock::new()
//             .expect_method(GET)
//             .expect_path("/v1/predictions/p1")
//             .return_status(200)
//             .return_json_body(&json!({
//                 "id": "p1",
//                 "version": "v1",
//                 // ... the rest of the response
//                 "output": "hello world",
//             }))
//             .create_on(&server);

//         // Actual code that would send the requests (assuming reqwest is being used).
//         // This is just a simplistic representation and your actual predict function would be more complex.
//         let client = reqwest::blocking::Client::new();
//         let resp: serde_json::Value = client
//             .post(&format!("{}/v1/predictions", server.base_url()))
//             .json(&json!({
//                 "version": "v1",
//                 "input": {"text": "world"}
//             }))
//             .send()
//             .unwrap()
//             .json()
//             .unwrap();

//         let prediction_url = resp["urls"]["get"].as_str().unwrap();
//         let prediction: serde_json::Value =
//             client.get(prediction_url).send().unwrap().json().unwrap();

//         assert_eq!(prediction["output"].as_str().unwrap(), "hello world");

//         // Verify that the mocks were called correctly
//         post_mock.assert();
//         get_mock.assert();
//     }
// }

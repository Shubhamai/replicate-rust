use std::collections::HashMap;

pub mod client;
pub mod collection;
pub mod model;
pub mod prediction;
pub mod training;
pub mod version;

pub mod enums;
pub mod libs;
pub mod structs;
mod utils;

use structs::Collection::Collection;
use structs::Model::Model;
use structs::Training::Training;
use structs::{GetPrediction::GetPrediction, Prediction::Prediction};

pub struct Replicate {
    client: client::Client,
    pub predictions: Prediction,
    pub models: Model,
    pub training: Training,
    pub collection: Collection,
}

impl Replicate {
    pub fn new(auth: String) -> Self {
        let client = client::Client::new(auth);

        // TODO : Maybe reference instead of clone
        let predictions = Prediction::new(client.clone());
        let models = Model::new(client.clone());
        let training = Training::new(client.clone());
        let collection = Collection::new(client.clone());

        Self {
            client,
            predictions,
            models,
            training,
            collection,
        }
    }

    pub fn run<K: serde::Serialize, V: serde::Serialize>(
        &self,
        version: String,
        inputs: HashMap<K, V>,
        // TODO : Perhaps not Box<dyn std::error::Error> but something more specific?
    ) -> Result<GetPrediction, Box<dyn std::error::Error>> {
        let prediction = Prediction::new(self.client.clone()).create(version, inputs);

        prediction.wait()
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

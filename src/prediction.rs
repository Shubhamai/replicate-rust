use reqwest::Response;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Payload<K: serde::Serialize, V: serde::ser::Serialize> {
    pub version: String,
    pub input: HashMap<K, V>,
}

pub enum RetryStrategy {
    // Retry with a fixed delay.
    FixedDelay(u64),
    // Retry with an exponential backoff.
    // ExponentialBackoff(u32),
}

pub struct RetryPolicy {
    pub max_retries: u32,
    pub strategy: RetryStrategy,
    step: u32,
}

impl RetryPolicy {
    fn new(max_retries: u32, strategy: RetryStrategy) -> Self {
        Self {
            max_retries,
            strategy,
            step: 0,
        }
    }

    fn step(&self) {
        match self.strategy {
            RetryStrategy::FixedDelay(delay) => {
                std::thread::sleep(std::time::Duration::from_millis(delay))
            } // RetryStrategy::ExponentialBackoff(delay) => delay * attempt,
        }
    }
}

pub struct Prediction<'a> {
    // Holds a reference to a Replicate
    parent: &'a crate::Replicate,

    // Unique identifier of the prediction
    // id: String,
    pub id: String,
    pub version: String,

    pub urls: PredictionsUrls,

    pub created_at: String,

    pub status: PredictionStatus,

    pub input: HashMap<String, serde_json::Value>,

    pub error: Option<String>,

    pub logs: String,
}

#[derive(serde::Deserialize, Debug)]
struct CreatePrediction {
    id: String,
    version: String,

    urls: PredictionsUrls,

    created_at: String,

    status: PredictionStatus,

    input: HashMap<String, serde_json::Value>,

    error: Option<String>,

    logs: String,
}

#[derive(serde::Deserialize, Debug)]
pub enum PredictionStatus {
    starting,
    processing,
    succeeded,
    failed,
    canceled,
}

#[derive(serde::Deserialize, Debug)]
pub enum PredictionSource {
    api,
    web,
}

#[derive(serde::Deserialize, Debug)]
pub struct PredictionsUrls {
    pub cancel: String,
    pub get: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GetPrediction {
    // Unique identifier of the prediction
    pub id: String,

    // Version of the model used for the prediction
    pub version: String,

    // Urls to cancel or get the prediction
    pub urls: PredictionsUrls,

    pub created_at: String,
    pub started_at: String,
    pub completed_at: Option<String>,

    pub source: Option<PredictionSource>,

    // Status of the prediction
    pub status: PredictionStatus,

    // Input and Outputs of the prediction
    pub input: HashMap<String, serde_json::Value>,
    pub output: Option<Vec<String>>,
    pub error: Option<String>,
    pub logs: String,

    pub metrics: Option<HashMap<String, serde_json::Value>>,
}

pub struct PredictionsListItem {
    id: String,
    version: String,

    urls: PredictionsUrls,

    created_at: String,
    started_at: String,
    completed_at: Option<String>,

    source: Option<PredictionSource>,

    status: PredictionStatus,
}
pub struct ListPredictions {
    previous: Option<String>,
    next: Option<String>,

    results: Vec<PredictionsListItem>,
}

// type ListPredictions = Vec<GetPrediction>;

impl<'a> Prediction<'a> {
    // pub fn new(rep: &'a crate::Replicate) -> Self {
    //     Prediction { parent: rep }
    // }

    // Run the prediction of the model version with the given input
    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        // &mut self,
        rep: &'a crate::Replicate,
        version: String,
        inputs: HashMap<K, V>,
    ) -> Result<Prediction<'a>, Box<dyn std::error::Error>> {
        // Parse the model version string.
        let (model, version) = crate::parse_version(&version).unwrap();

        // Construct the request payload
        let payload = Payload {
            version: version.to_string(),
            input: inputs,
        };

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&rep.base_url)
            .header("Authorization", format!("Token {}", rep.auth))
            .header("User-Agent", &rep.user_agent)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let result: CreatePrediction = response.json()?;

            Ok(Self {
                parent: rep,
                // TODO : Need to do this better
                id: result.id,
                version: result.version,
                urls: result.urls,
                created_at: result.created_at,
                status: result.status,
                input: result.input,
                error: result.error,
                logs: result.logs,
            })
        } else {
            let error_message = response.text()?;
            Err(error_message.into())
        }
    }

    // Returns the latest status of the prediction
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "https://api.replicate.com/v1/predictions/{}",
                self.id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        let response_struct: GetPrediction = serde_json::from_str(&response_string)?;

        // TODO : Need to make this cleaner
        self.id = response_struct.id;
        self.version = response_struct.version;
        self.urls = response_struct.urls;
        self.created_at = response_struct.created_at;
        self.status = response_struct.status;
        self.input = response_struct.input;
        self.error = response_struct.error;
        self.logs = response_struct.logs;

        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!(
                "https://api.replicate.com/v1/predictions/{}/cancel",
                self.id
            ))
            .header("Authorization", format!("Token {}", &self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        self.reload()?;

        Ok(())
    }

    // Blocks until the predictions are ready and returns the predictions
    pub fn wait(self) -> Result<GetPrediction, Box<dyn std::error::Error>> {
        // TODO : Implement a retry policy
        let retry_policy = RetryPolicy::new(5, RetryStrategy::FixedDelay(1000));
        let client = reqwest::blocking::Client::new();

        loop {
            let response = client
                .get(format!(
                    "https://api.replicate.com/v1/predictions/{}",
                    self.id
                ))
                .header("Authorization", format!("Token {}", self.parent.auth))
                .header("User-Agent", &self.parent.user_agent)
                .send()?;

            let response_string = response.text()?;
            let response_struct: GetPrediction = serde_json::from_str(&response_string)?;

            match response_struct.status {
                PredictionStatus::succeeded => {
                    // println!("Success : {:?}", response_string);
                    return Ok(response_struct);
                }
                PredictionStatus::failed => {
                    // println!("Failed : {:?}", response_string);
                    return Err(response_string.into());
                }
                PredictionStatus::processing | PredictionStatus::starting => {
                    // Retry
                    // TODO : Fix the retry implementation
                    retry_policy.step();
                }
                PredictionStatus::canceled => {
                    // println!("Canceled : {:?}", response_string);
                    return Err(response_string.into());
                }
            }
        }
    }
}

pub struct CreatePredictionStruct<'a> {
    // Holds a reference to a Replicate
    parent: &'a crate::Replicate,
}

impl<'a> CreatePredictionStruct<'a> {
    pub fn new(rep: &'a crate::Replicate) -> Self {
        Self { parent: rep }
    }

    pub fn create<K: serde::Serialize, V: serde::ser::Serialize>(
        self,
        version: String,
        inputs: HashMap<K, V>,
    ) -> Prediction<'a> {
        match Prediction::create(&self.parent, version, inputs) {
            Ok(prediction) => prediction,
            Err(e) => panic!("Error : {}", e),
        }
    }

    pub fn list(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get("https://api.replicate.com/v1/predictions")
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        let response_string = response.text()?;
        // let response_struct: ListPredictions = serde_json::from_str(&response_string)?;
        println!("Response : {:?}", response_string);

        Ok(())
    }
}

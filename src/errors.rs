//! Custom errors for the crate.

use thiserror::Error;

/// Errors related to sending requests to the API.
#[derive(Error, Debug)]
pub enum ReplicateError {
    /// Error occues when sending the api request results in an error.
    #[error("failed to send the api request: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Error occues when the api returns a non 200 response.
    #[error("Received a non 200 response from the api: {0}")]
    ResponseError(String),

    /// Error occues when parsing the api response into a struct results in an error.
    #[error("failed to parse the api response : {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Invalid version string provided.
    #[error("Invalid version string: {0}")]
    InvalidVersionString(String),
}

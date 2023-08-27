//! Used to interact with the [Model Versions Endpoints](https://replicate.com/docs/reference/http#models.versions.get).
//!
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! let versions = replicate.models.versions.list("replicate", "hello-world")?;
//! println!("Versions : {:?}", versions);
//!
//!
//! // Get the details of a model version.
//! let version = replicate.models.versions.get(
//!         "kvfrans",
//!         "clipdraw",
//!         "5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b",
//!    )?;
//! println!("Version : {:?}", version);
//!
//! # Ok::<(), replicate_rust::errors::ReplicateError>(())
//! ```
//!

use crate::{
    api_definitions::{GetModelVersion, ListModelVersions},
    errors::ReplicateError,
};

/// Used to interact with the [Model Versions Endpoints](https://replicate.com/docs/refer   ence/http#models.versions.get).
#[derive(Clone, Debug)]
pub struct Version {
    /// Holds a reference to a Configuration struct, which contains the base url,  auth token among other settings.
    pub parent: crate::config::Config,
}

/// The Version struct is used to interact with the [Model Versions Endpoints](https://replicate.com/docs/reference/http#models.versions.get).
impl Version {
    /// Create a new Version struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Get the details of a model version.
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// // Get the details of a model version.
    /// let version = replicate.models.versions.get(
    ///         "kvfrans",
    ///         "clipdraw",
    ///         "5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b",
    ///     )?;
    /// println!("Version : {:?}", version);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn get(
        &self,
        model_owner: &str,
        model_name: &str,
        version_id: &str,
    ) -> Result<GetModelVersion, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}/versions/{}",
                self.parent.base_url, model_owner, model_name, version_id
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: GetModelVersion = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// List the versions of a model.
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// let versions = replicate.models.versions.list("replicate", "hello-world")?;
    /// println!("Versions : {:?}", versions);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn list(
        &self,
        model_owner: &str,
        model_name: &str,
    ) -> Result<ListModelVersions, ReplicateError> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "{}/models/{}/{}/versions",
                self.parent.base_url, model_owner, model_name
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: ListModelVersions = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

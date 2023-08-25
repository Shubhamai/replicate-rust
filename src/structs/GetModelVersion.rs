use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetModelVersion {
    pub id: String,
    pub created_at: String,

    pub cog_version: String,

    pub openapi_schema: HashMap<String, serde_json::Value>,
}

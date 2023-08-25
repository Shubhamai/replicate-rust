use crate::enums::WebhookEvents::WebhookEvents;
use std::collections::HashMap;

pub struct TrainingOptions {
    pub destination: String,

    pub input: HashMap<String, String>,

    pub webhook: String,
    _webhook_events_filter: Option<WebhookEvents>,
}

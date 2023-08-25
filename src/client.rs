// TODO : Do these need to be public

#[derive(Clone)]
pub struct Client {
    pub auth: String,
    pub user_agent: String,
    pub base_url: String,
}

impl Client {
    pub fn new(auth: String) -> Self {
        Self {
            auth,
            user_agent: format!("replicate-rust/{}", env!("CARGO_PKG_VERSION")),
            base_url: String::from("https://api.replicate.com/v1"),
        }
    }
}

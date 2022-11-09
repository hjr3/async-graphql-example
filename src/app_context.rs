pub struct AppContext {
    pub http_client: reqwest::Client,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}

use std::time::Instant;

use reqwest::IntoUrl;

pub struct Datasource {
    http_client: reqwest::Client,
    dogstatsd: dogstatsd::Client,
}

impl Datasource {
    pub fn new(dogstatsd: dogstatsd::Client) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            dogstatsd,
        }
    }

    pub async fn get<U: IntoUrl>(&self, url: U) -> Result<String, reqwest::Error> {
        let now = Instant::now();
        let response = self.http_client.get(url).send().await?;

        let body = response.text().await?;

        let timing = now.elapsed().as_millis();
        self.dogstatsd
            .timing("my_timing", timing as i64, &["tag:datasource"])
            .unwrap();

        Ok(body)
    }
}

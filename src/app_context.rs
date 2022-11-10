use crate::datasource::Datasource;

pub struct AppContext {
    pub datasource: Datasource,
}

impl AppContext {
    pub fn new(dogstatsd: dogstatsd::Client) -> Self {
        Self {
            datasource: Datasource::new(dogstatsd),
        }
    }
}

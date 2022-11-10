use crate::datasource::IDatasource;

pub struct AppContext {
    pub datasource: Box<dyn IDatasource>,
}

impl AppContext {
    pub fn new(datasource: Box<dyn IDatasource>) -> Self {
        Self { datasource }
    }
}

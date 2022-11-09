use async_graphql::Object;

#[derive(Default)]
pub struct HelloQuery;

#[Object]
impl HelloQuery {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

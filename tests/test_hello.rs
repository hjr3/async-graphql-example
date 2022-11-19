use anyhow::Result;
use axum::http::StatusCode;
use serde_json::json;
use tower::ServiceExt;

use async_graphql_example::app;

mod graphql {
    use anyhow::Result;
    use axum::http;
    use hyper::{Body, Method, Request};
    use serde_json::{json, Value};

    /// Create a GraphQL query request
    pub fn query(query: &str, variables: Value) -> Result<Request<Body>> {
        let req = Request::builder()
            .method(Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref());

        let body = serde_json::to_string(&json!({ "query": query, "variables": variables }))?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }
}

#[tokio::test]
async fn test_hello() -> Result<()> {
    pretty_env_logger::try_init()?;
    let app = app();

    let query = r#"
        query TestAdd {
          add(a: 1, b: 2)
        }
    "#;

    let variables = json!({});

    let request = graphql::query(query, variables)?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;

    let expected = b"{\"data\":{\"add\":3}}";
    assert_eq!(&body[..], expected);

    Ok(())
}

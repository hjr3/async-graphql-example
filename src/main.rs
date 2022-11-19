use axum::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use async_graphql_example::app;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "async_graphql_example=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app();
    let port = 8000;
    let s_addr = format!("0.0.0.0:{}", port);
    let server = Server::bind(&s_addr.parse().unwrap()).serve(app.into_make_service());

    let addr = server.local_addr();

    tracing::debug!("GraphiQL IDE: http://localhost:{}", addr.port());

    server.await.unwrap();
}

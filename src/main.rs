use async_graphql_example::app;
use axum::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = app();
    let port = 8000;
    let s_addr = format!("0.0.0.0:{}", port);
    let server = Server::bind(&s_addr.parse().unwrap()).serve(app.into_make_service());

    let addr = server.local_addr();

    println!("GraphiQL IDE: http://localhost:{}", addr.port());

    server.await.unwrap();
}

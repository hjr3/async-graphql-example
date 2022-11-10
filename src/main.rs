mod app_context;
mod cat;
mod datasource;
mod hello;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use cat::CatQuery;
use hello::HelloQuery;

#[derive(MergedObject, Default)]
struct Query(CatQuery, HelloQuery);

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let dogstatsd = dogstatsd::Client::new(dogstatsd::Options::default()).unwrap();
    let app_context = app_context::AppContext::new(dogstatsd);
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(app_context)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

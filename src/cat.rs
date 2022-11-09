use async_graphql::{Context, Object, SimpleObject};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::app_context::AppContext;

#[derive(SimpleObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cat {
    tags: Vec<String>,
    created_at: String,
    updated_at: String,
    validated: bool,
    owner: String,
    file: String,
    mimetype: String,
    size: usize,
    #[serde(rename = "_id")]
    id: String,
    url: String,
}

#[derive(Default)]
pub struct CatQuery;

#[Object]
impl CatQuery {
    async fn cat<'a>(&self, ctx: &Context<'a>) -> Result<Cat, StatusCode> {
        let app_context = ctx.data_unchecked::<AppContext>();

        let response = app_context
            .http_client
            .get("https://cataas.com/cat?json=true")
            .send()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let body = response
            .text()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let cat: Cat = serde_json::from_str(&body).map_err(|err| {
            log::error!("Failed to deserialize {}", err);

            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(cat)
    }
}


use async_trait::async_trait;
use axum::extract::{
    rejection::{self, JsonRejection},
    Extension, FromRequest, RequestParts,
};
use axum::ws::{ws, Message, WebSocket};
use axum::{
    //extract::ExtractUserAgent,
    prelude::*,
    routing,
    service::ServiceExt,
    AddExtensionLayer,
};
//use bb8::Pool;
//use bb8_postgres::PostgresConnectionManager;
use http::{
    header::{HeaderValue, USER_AGENT},
    StatusCode,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
//use tokio_postgres::{Config, NoTls};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

#[derive(Debug)]
struct ExtractGraphql(String);

#[async_trait]
impl<B> FromRequest<B> for ExtractGraphql
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        //    let full_body = hyper::body::to_bytes(req.into_body()).await?;
        if let Some(body) = req.take_body() {
            Ok(ExtractGraphql("body".into()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}

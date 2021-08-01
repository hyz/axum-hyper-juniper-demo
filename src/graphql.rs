pub mod schema;
#[cfg(feature = "schema-language")]
pub mod schema_language;

use crate::db::Extension;
use crate::db::PoolPg; //= sqlx::Pool<sqlx::postgres::Postgres>;
use axum::extract::{FromRequest, RequestParts};
use futures::TryFutureExt;
use std::{convert::Infallible, sync::Arc};

use http::request::Request;
use hyper::{Body, Method, Response, StatusCode};
use juniper::DefaultScalarValue;
use juniper::{
    graphql_interface,
    graphql_object,
    graphql_subscription,
    http::{GraphQLBatchRequest, GraphQLRequest as JuniperGraphQLRequest, GraphQLRequest},
    //tests::fixtures::starwars::schema::{Database, Query},
    Context,
    EmptyMutation,
    EmptySubscription,
    GraphQLEnum,

    GraphQLSubscriptionType,
    GraphQLType,
    GraphQLTypeAsync,
    InputValue,
    RootNode,
    ScalarValue,
};

pub async fn iql_() -> Response<Body> {
    juniper_hyper::graphiql("/graphql", None).await
}

// pub async fn graphql(
//     Extension(pool): Extension<PoolPg>,
//     Extension(root): Extension<Arc<RootNode<_>>>,
// ) -> Response<Body> {
//     juniper_hyper::graphql(root, ctx, req).await
// }
type Root = RootNode<
    'static,
    schema::Query,
    EmptyMutation<schema::Database>,
    EmptySubscription<schema::Database>,
    DefaultScalarValue,
>;
//pub type PoolPg = sqlx::Pool<sqlx::postgres::Postgres>;

pub async fn graphql(
    Extension(root): Extension<Arc<Root>>,
    Extension(ctx): Extension<Arc<schema::Database>>,
    req: Request<Body>,
) -> Response<Body> {
    //let body = Body::empty();
    //let req = Request::new(body);
    juniper_hyper::graphql(root, ctx, req).await
}

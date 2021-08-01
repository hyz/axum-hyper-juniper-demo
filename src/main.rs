use axum::{
    //extract::ExtractUserAgent,
    prelude::*,
    routing,
    service::ServiceExt,
    AddExtensionLayer,
};
use axum_ex1 as lib;
use http::StatusCode;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::{convert::Infallible, sync::Arc};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(err) = dotenv::dotenv() {
        eprintln!(".env file {:?}", err);
    }

    let database_url = std::env::var("DATABASE_URL").unwrap(); // "host=localhost user=postgres"
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Pool<Postgres> fail");
    let root = RootNode::new(
        lib::graphql::schema::Query,
        EmptyMutation::<lib::graphql::schema::Database>::new(),
        EmptySubscription::<lib::graphql::schema::Database>::new(),
    );
    let database = lib::graphql::schema::Database::new();

    // routes are matched from bottom to top, so we have to put `nest` at the top since it matches all routes
    let app = routing::nest(
        "/",
        axum::service::get(
            ServeDir::new("public")
                .append_index_html_on_directories(true)
                .handle_error(|error: std::io::Error| {
                    Ok::<_, std::convert::Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled interal error: {}", error),
                    ))
                }),
        ),
    )
    .route("/graphiql", get(lib::graphql::iql_))
    .route("/graphql", post(lib::graphql::graphql))
    .route("/pub", get(lib::db::pub_))
    .route("/sub", axum::ws::ws(lib::ws::sub_))
    .layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
    )
    .layer(AddExtensionLayer::new(pool))
    .layer(AddExtensionLayer::new(Arc::new(root)))
    .layer(AddExtensionLayer::new(Arc::new(database)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// use axum::routing::Layered;
// use axum::routing::Route;
// use axum::routing::RoutingDsl;
// use http::request::Request;
// use tower_layer::Layer;
// use tower_service::Service;

// fn layer<This, L>(this: This, layer: L) -> Layered<L::Service>
// where
//     L: Layer<This>,
// {
//     Layered::new(layer.layer(this))
// }

// fn route<This, T, B>(this: This, description: &str, svc: T) -> Route<T, This>
// where
//     T: Service<Request<B>> + Clone,
// {
//     Route {
//         pattern: PathPattern::new(description),
//         svc,
//         fallback: this,
//     }
// }

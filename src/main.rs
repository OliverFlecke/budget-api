use std::sync::Arc;

use axum::{
    http::{StatusCode, Uri},
    Router,
};
use budget_api::budget::budget_router;
use sqlx::postgres::PgPool;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{warn, Level};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL")
        .expect("Missing environment variable 'DATABASE_URL' provided with a connection string");
    let pool = Arc::new(PgPool::connect(&url).await.unwrap());

    setup_tracing();
    let budget_router = budget_router(&pool);
    let app = Router::new()
        .nest("/budget", budget_router)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback(not_found);

    let port = std::env::var("PORT")
        .map(|p| p.parse::<usize>().expect("PORT is not a valid integer"))
        .unwrap_or(4000);
    let host = format!("0.0.0.0:{port}").parse().unwrap();

    tracing::event!(Level::INFO, "Server running at {host}");
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "budget_api=debug,hyper=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    warn!("Path not found {uri}");
    (StatusCode::NOT_FOUND, format!("No route for '{uri}'"))
}

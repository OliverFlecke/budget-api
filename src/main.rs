use std::sync::Arc;

use axum::{
    http::{StatusCode, Uri},
    Router,
};
use budget_api::budget::budget_router;
use sqlx::postgres::PgPool;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "budget-api=debug,hyper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let url = std::env::var("DATABASE_URL")
        .expect("Missing environment variable 'DATABASE_URL' provided with a connection string");
    let pool = Arc::new(PgPool::connect(&url).await.unwrap());
    info!("Hello");

    let budget_router = budget_router(&pool);
    let app = Router::new()
        .nest("/budget", budget_router)
        .fallback(not_found);

    // run it with hyper on localhost:3000
    let host = "0.0.0.0:4000".parse().unwrap();
    println!("Server running at {host}");
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for '{uri}'"))
}

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use budget_api::budget::endpoints::{create_budget, get_all_budgets, get_budget};
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() {
    let url = "postgres://postgres:password@localhost:5432/finance";
    let pool = Arc::new(PgPool::connect(url).await.unwrap());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/budget", get(get_all_budgets))
        .with_state(pool.clone())
        .route("/budget", post(create_budget))
        .with_state(pool.clone())
        .route("/budget/:id", get(get_budget))
        .with_state(pool.clone());

    // run it with hyper on localhost:3000
    let host = "0.0.0.0:3000".parse().unwrap();
    println!("Server running at {host}");
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

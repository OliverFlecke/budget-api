use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() {
    let url = "postgres://postgres:password@localhost/finance";
    let pool = Arc::new(PgPool::connect(url).await.unwrap());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/budget", post(create_budget))
        .with_state(pool);

    // run it with hyper on localhost:3000
    let host = "0.0.0.0:3000".parse().unwrap();
    println!("Server running at {host}");
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_budget(State(pool): State<Arc<PgPool>>) {}

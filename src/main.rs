use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPool,
    types::{time::PrimitiveDateTime, Uuid},
};

#[tokio::main]
async fn main() {
    let url = "postgres://postgres:password@localhost:5432/finance";
    let pool = Arc::new(PgPool::connect(url).await.unwrap());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .route("/budget", get(get_budget))
        // .with_state(pool)
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

// #[derive(Serialize)]
struct Budget {
    id: Uuid,
    user_id: String,
    title: String,
    created_at: PrimitiveDateTime,
}

async fn get_budget(State(pool): State<Arc<PgPool>>, id: Uuid) -> Json<Budget> {
    let budget = sqlx::query_as!(Budget, "SELECT * FROM budget")
        .fetch_one(pool.as_ref())
        .await;
    Json(budget.unwrap())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateBudget {
    title: String,
}

async fn create_budget(State(pool): State<Arc<PgPool>>, Json(payload): Json<CreateBudget>) {
    let user_id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO budget (user_id, title) VALUES ($1, $2)",
        user_id,
        payload.title
    )
    .execute(pool.as_ref())
    .await
    .unwrap();
    // sqlx::query("INSERT INTO budget (user_id, title) VALUES (?, ?)")
    //     .bind(user_id)
    //     .bind(payload.title)
    //     .execute(pool.as_ref())
    //     .await
    //     .unwrap();
}

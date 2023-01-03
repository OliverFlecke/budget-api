mod dto;
mod model;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

pub fn budget_router(pool: &Arc<PgPool>) -> Router {
    let item_router = Router::new()
        .route("/", post(endpoints::add_item_to_budget))
        .with_state(pool.clone())
        .route("/:item_id", put(endpoints::update_item))
        .with_state(pool.clone())
        .route("/:item_id", delete(endpoints::delete_item))
        .with_state(pool.clone());

    Router::new()
        .route("/", get(endpoints::get_all_budgets))
        .with_state(pool.clone())
        .route("/", post(endpoints::create_budget))
        .with_state(pool.clone())
        .route("/:id", get(endpoints::get_budget))
        .with_state(pool.clone())
        .nest("/:id/item", item_router)
}

// Endpoints
mod endpoints {
    use std::sync::Arc;

    use axum::{
        extract::{Path, State},
        http::StatusCode,
        Json,
    };
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{
        auth::ExtractUserId,
        budget::{
            dto::{self},
            model::{self},
        },
    };

    use super::dto::AddItemToBudgetRequest;

    /// Get a budget from a given ID.
    pub async fn get_budget(
        Path(budget_id): Path<Uuid>,
        State(pool): State<Arc<PgPool>>,
    ) -> Result<Json<dto::BudgetWithItems>, StatusCode> {
        println!("Get budget {budget_id}");

        let query = sqlx::query_as!(
            model::BudgetWithItems,
            r#"SELECT b.*,
array_agg((i.id, i.budget_id, i.category, i.name, i.amount, i.created_at, i.modified)) as "items!: Vec<model::Item>"
FROM budget AS b
LEFT JOIN item AS i ON b.id = i.budget_id
WHERE b.id = $1
GROUP BY b.id
"#,
            budget_id
        );

        match query.fetch_one(pool.as_ref()).await {
            Ok(budget) => Ok(Json((&budget).into())),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    /// Get all budgets in the database.
    ///
    /// NOTE: This will not continue to be exposed to end users.
    pub async fn get_all_budgets(State(pool): State<Arc<PgPool>>) -> Json<Vec<dto::Budget>> {
        let query = sqlx::query_as!(model::Budget, "SELECT * FROM budget");

        Json(
            query
                .fetch_all(pool.as_ref())
                .await
                .unwrap()
                .iter()
                .map(|x| x.into())
                .collect::<Vec<dto::Budget>>(),
        )
    }

    /// Create a new budget.
    pub async fn create_budget(
        State(pool): State<Arc<PgPool>>,
        ExtractUserId(user_id): ExtractUserId,
        Json(payload): Json<dto::CreateBudget>,
    ) {
        sqlx::query!(
            "INSERT INTO budget (user_id, title) VALUES ($1, $2)",
            user_id,
            payload.title
        )
        .execute(pool.as_ref())
        .await
        .unwrap();
    }

    // pub async fn get_item(State(pool): State<Arc<PgPool>>, Path(item_id): Path<Uuid>) {
    //     let query = sqlx::query_as!(model::Item, "SELECT * FROM item WHERE id = $1", item_id);

    //     query.fetch_one(pool.as_ref()).await;
    // }

    /// Add a new item to a budget.
    pub async fn add_item_to_budget(
        State(pool): State<Arc<PgPool>>,
        Path(budget_id): Path<Uuid>,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> StatusCode {
        let query = sqlx::query!(
            "INSERT INTO item (budget_id, category, name, amount) VALUES ($1, $2, $3, $4)",
            budget_id,
            payload.category,
            payload.name,
            payload.amount
        );

        match query.execute(pool.as_ref()).await {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Update an item on a budget
    pub async fn update_item(
        State(pool): State<Arc<PgPool>>,
        Path((_budget_id, item_id)): Path<(Uuid, Uuid)>,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> StatusCode {
        // TODO: Validate user has access to budget (necessary for more than just this endpoint)

        let query = sqlx::query!(
            "UPDATE item SET category = $1, amount = $2, name = $3 WHERE id = $4",
            payload.category,
            payload.amount,
            payload.name,
            item_id
        );

        match query.execute(pool.as_ref()).await {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Delete an item.
    pub async fn delete_item(
        State(pool): State<Arc<PgPool>>,
        Path((_, item_id)): Path<(Uuid, Uuid)>,
    ) -> StatusCode {
        let query = sqlx::query!("DELETE FROM item WHERE id = $1", item_id);

        match query.execute(pool.as_ref()).await {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }
}

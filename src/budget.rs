mod dto;
mod model;

use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;

pub fn budget_router(pool: &Arc<PgPool>) -> Router {
    let item_router = Router::new()
        .route("/", post(endpoints::add_item_to_budget))
        .with_state(pool.clone())
        .route("/:item_id", put(endpoints::update_item))
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

    use crate::budget::{
        dto::{BudgetDto, CreateBudget},
        model::Budget,
    };

    use super::dto::{AddItemToBudgetRequest, UpdateItemOnBudgetRequest};

    /// Get a budget from a given ID.
    pub async fn get_budget(
        Path(budget_id): Path<Uuid>,
        State(pool): State<Arc<PgPool>>,
    ) -> Result<Json<BudgetDto>, StatusCode> {
        let query = sqlx::query_as!(Budget, "SELECT * FROM budget WHERE id = $1", budget_id);

        match query.fetch_one(pool.as_ref()).await {
            Ok(budget) => Ok(Json((&budget).into())),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    /// Get all budgets in the database.
    ///
    /// NOTE: This will not continue to be exposed to end users.
    pub async fn get_all_budgets(State(pool): State<Arc<PgPool>>) -> Json<Vec<BudgetDto>> {
        let query = sqlx::query_as!(Budget, "SELECT * FROM budget");

        Json(
            query
                .fetch_all(pool.as_ref())
                .await
                .unwrap()
                .iter()
                .map(|x| x.into())
                .collect::<Vec<BudgetDto>>(),
        )
    }

    /// Create a new budget.
    pub async fn create_budget(State(pool): State<Arc<PgPool>>, Json(payload): Json<CreateBudget>) {
        let user_id = Uuid::new_v4().to_string(); // TODO: Get user id from auth token
        sqlx::query!(
            "INSERT INTO budget (user_id, title) VALUES ($1, $2)",
            user_id,
            payload.title
        )
        .execute(pool.as_ref())
        .await
        .unwrap();
    }

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
}

mod dto;
mod model;
mod repository;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use self::repository::BudgetRepository;

pub fn budget_router(pool: &Arc<PgPool>) -> Router {
    let budget_repository = Arc::new(BudgetRepository::new(pool.clone()));

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
        .with_state(budget_repository.clone())
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
        budget::{dto, model},
    };

    use super::{dto::AddItemToBudgetRequest, repository::BudgetRepository};

    /// Create a new budget.
    pub async fn create_budget(
        State(repository): State<Arc<BudgetRepository>>,
        ExtractUserId(user_id): ExtractUserId,
        Json(payload): Json<dto::CreateBudget>,
    ) -> StatusCode {
        match repository
            .create_budget(user_id.as_str(), &payload.title)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Get a budget from a given ID.
    pub async fn get_budget(
        State(repository): State<Arc<BudgetRepository>>,
        Path(budget_id): Path<Uuid>,
        ExtractUserId(user_id): ExtractUserId,
    ) -> Result<Json<dto::BudgetWithItems>, StatusCode> {
        println!("Get budget {budget_id} and user: {user_id}");

        match repository.get_budget(&user_id, &budget_id).await {
            Some(budget) => Ok(Json((&budget).into())),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    /// Get all budgets in the database.
    ///
    /// NOTE: This will not continue to be exposed to end users.
    pub async fn get_all_budgets(
        State(pool): State<Arc<PgPool>>,
        ExtractUserId(user_id): ExtractUserId,
    ) -> Json<Vec<dto::Budget>> {
        let query = sqlx::query_as!(
            model::Budget,
            "SELECT * FROM budget WHERE user_id = $1",
            user_id
        );

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

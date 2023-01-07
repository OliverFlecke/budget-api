mod dto;
mod item_repository;
mod model;
mod repository;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use self::{item_repository::ItemRepository, repository::BudgetRepository};

pub fn budget_router(pool: &Arc<PgPool>) -> Router {
    let budget_repository = Arc::new(BudgetRepository::new(pool.clone()));
    let item_repository = Arc::new(ItemRepository::new(pool.clone()));

    let item_router = Router::new()
        .route("/", post(endpoints::add_item_to_budget))
        .with_state(item_repository.clone())
        .route("/:item_id", put(endpoints::update_item))
        .with_state(item_repository.clone())
        .route("/:item_id", delete(endpoints::delete_item))
        .with_state(item_repository);

    Router::new()
        .route("/", get(endpoints::get_all_budgets))
        .with_state(budget_repository.clone())
        .route("/", post(endpoints::create_budget))
        .with_state(budget_repository.clone())
        .route("/:id", get(endpoints::get_budget))
        .with_state(budget_repository)
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
    use uuid::Uuid;

    use crate::{auth::ExtractUserId, budget::dto};

    use super::{
        dto::AddItemToBudgetRequest, item_repository::ItemRepository, repository::BudgetRepository,
    };

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
        State(repository): State<Arc<BudgetRepository>>,
        ExtractUserId(user_id): ExtractUserId,
    ) -> Json<Vec<dto::Budget>> {
        Json(
            repository
                .get_all_budgets_for_user(&user_id)
                .await
                .iter()
                .map(|x| x.into())
                .collect::<Vec<dto::Budget>>(),
        )
    }

    /// Add a new item to a budget.
    pub async fn add_item_to_budget(
        State(repository): State<Arc<ItemRepository>>,
        Path(budget_id): Path<Uuid>,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> Result<Json<Uuid>, StatusCode> {
        match repository.add_item_to_budget(budget_id, payload).await {
            Ok(id) => Ok(Json(id)),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Update an item on a budget
    pub async fn update_item(
        State(repository): State<Arc<ItemRepository>>,
        Path((_budget_id, item_id)): Path<(Uuid, Uuid)>,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> StatusCode {
        // TODO: Validate user has access to budget (necessary for more than just this endpoint)

        match repository.update_item(item_id, payload).await {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Delete an item.
    pub async fn delete_item(
        State(repository): State<Arc<ItemRepository>>,
        Path((budget_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> StatusCode {
        match repository.delete_item(budget_id, item_id).await {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }
}

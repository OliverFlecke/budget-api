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
        .route("/:id", delete(endpoints::delete_budget))
        .with_state(budget_repository.clone())
        .route("/:id", get(endpoints::get_budget))
        .with_state(budget_repository)
        .nest("/:id/item", item_router)
}

// Endpoints
mod endpoints {
    use std::sync::Arc;
    use tracing::{event, Level};

    use axum::{
        extract::{Path, State},
        http::StatusCode,
        Json,
    };
    use uuid::Uuid;

    use crate::{auth::Claims, budget::dto};

    use super::{
        dto::AddItemToBudgetRequest, item_repository::ItemRepository, repository::BudgetRepository,
    };

    /// Create a new budget.
    pub async fn create_budget(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
        Json(payload): Json<dto::CreateBudget>,
    ) -> Result<String, StatusCode> {
        event!(Level::INFO, "Creating budget");

        match repository
            .create_budget(claims.user_id(), &payload.title)
            .await
        {
            Ok(id) => Ok(id.to_string()),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Delete a budget for a user
    pub async fn delete_budget(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
        Path(budget_id): Path<Uuid>,
    ) -> Result<(), StatusCode> {
        event!(
            Level::INFO,
            "Deleting budget '{budget_id}' for user '{}'",
            claims.user_id()
        );

        match repository.delete_budget(claims.user_id(), &budget_id).await {
            Ok(_) => Ok(()),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Get a budget from a given ID.
    pub async fn get_budget(
        State(repository): State<Arc<BudgetRepository>>,
        Path(budget_id): Path<Uuid>,
        claims: Claims,
    ) -> Result<Json<dto::BudgetWithItems>, StatusCode> {
        event!(
            Level::INFO,
            "Get budget {budget_id} and user: {}",
            claims.user_id()
        );

        match repository.get_budget(claims.user_id(), &budget_id).await {
            Some(budget) => Ok(Json((&budget).into())),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    /// Get all budgets in the database.
    ///
    /// NOTE: This will not continue to be exposed to end users.
    pub async fn get_all_budgets(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
    ) -> Json<Vec<dto::Budget>> {
        event!(Level::INFO, "Get all budgets for user {}", claims.user_id());

        Json(
            repository
                .get_all_budgets_for_user(claims.user_id())
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
        claims: Claims,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> Result<String, StatusCode> {
        event!(
            Level::INFO,
            "User '{}' add item to budget {budget_id}. Payload: {payload:?}",
            claims.user_id()
        );
        match repository.add_item_to_budget(budget_id, payload).await {
            Ok(id) => Ok(id.to_string()),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Update an item on a budget
    pub async fn update_item(
        State(repository): State<Arc<ItemRepository>>,
        Path((budget_id, item_id)): Path<(Uuid, Uuid)>,
        claims: Claims,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> StatusCode {
        event!(
            Level::INFO,
            "User '{}' update item {item_id} on budget {budget_id}. Payload: {payload:?}",
            claims.user_id()
        );
        // TODO: Validate user has access to budget (necessary for more than just this endpoint)

        match repository.update_item(item_id, payload).await {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Delete an item.
    pub async fn delete_item(
        State(repository): State<Arc<ItemRepository>>,
        claims: Claims,
        Path((budget_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> StatusCode {
        event!(
            Level::INFO,
            "User '{}' delete item {item_id} on budget {budget_id}",
            claims.user_id()
        );

        match repository
            .delete_item(claims.user_id(), budget_id, item_id)
            .await
        {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }
}

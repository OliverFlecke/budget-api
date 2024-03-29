mod dto;
pub(crate) mod item_repository;
mod model;
pub(crate) mod repository;

use crate::app_state::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(endpoints::get_all_budgets))
        .route("/", post(endpoints::create_budget))
        .route("/:id", delete(endpoints::delete_budget))
        .route("/:id", get(endpoints::get_budget))
        .route("/:id", put(endpoints::update_budget))
        .with_state(state.clone())
        .nest(
            "/:id/item",
            Router::new()
                .route("/", post(endpoints::add_item_to_budget))
                .route("/:item_id", put(endpoints::update_item))
                .route("/:item_id", delete(endpoints::delete_item))
                .with_state(state),
        )
}

mod endpoints {
    use super::{
        dto::AddItemToBudgetRequest, item_repository::ItemRepository, repository::BudgetRepository,
    };
    use crate::{app_state::AppState, auth::Claims, budget::dto};
    use axum::{
        debug_handler,
        extract::{Path, State},
        http::StatusCode,
        Json,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    /// Create a new budget.
    #[debug_handler(state = AppState)]
    pub async fn create_budget(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
        Json(payload): Json<dto::CreateBudget>,
    ) -> Result<String, StatusCode> {
        tracing::info!("Creating budget");

        match repository
            .create_budget(claims.user_id(), &payload.title)
            .await
        {
            Ok(id) => Ok(id.to_string()),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Delete a budget for a user
    #[debug_handler(state = AppState)]
    pub async fn delete_budget(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
        Path(budget_id): Path<Uuid>,
    ) -> Result<(), StatusCode> {
        tracing::info!(
            "Deleting budget '{budget_id}' for user '{}'",
            claims.user_id()
        );

        match repository.delete_budget(claims.user_id(), &budget_id).await {
            Ok(_) => Ok(()),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Get a budget from a given ID.
    #[debug_handler(state = AppState)]
    pub async fn get_budget(
        State(repository): State<Arc<BudgetRepository>>,
        Path(budget_id): Path<Uuid>,
        claims: Claims,
    ) -> Result<Json<dto::BudgetWithItems>, StatusCode> {
        tracing::info!("Get budget {budget_id} and user: {}", claims.user_id());

        match repository.get_budget(claims.user_id(), &budget_id).await {
            Some(budget) => Ok(Json((&budget).into())),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    /// Get all budgets in the database.
    ///
    /// NOTE: This will not continue to be exposed to end users.
    #[debug_handler(state = AppState)]
    pub async fn get_all_budgets(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
    ) -> Json<Vec<dto::Budget>> {
        tracing::info!("Get all budgets for user {}", claims.user_id());

        Json(
            repository
                .get_all_budgets_for_user(claims.user_id())
                .await
                .iter()
                .map(|x| x.into())
                .collect::<Vec<dto::Budget>>(),
        )
    }

    /// Update the name of a budget.
    pub async fn update_budget(
        State(repository): State<Arc<BudgetRepository>>,
        claims: Claims,
        Path(budget_id): Path<Uuid>,
        Json(payload): Json<dto::UpdateBudget>,
    ) -> Result<(), StatusCode> {
        tracing::info!("Updating budget for user '{}'", claims.user_id());

        repository
            .update_budget(claims.user_id(), &budget_id, &payload.title)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)
    }

    /// Add a new item to a budget.
    #[debug_handler(state = AppState)]
    pub async fn add_item_to_budget(
        State(repository): State<Arc<ItemRepository>>,
        Path(budget_id): Path<Uuid>,
        claims: Claims,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> Result<String, StatusCode> {
        tracing::info!(
            "User '{}' add item to budget {budget_id}. Payload: {payload:?}",
            claims.user_id()
        );
        match repository
            .add_item_to_budget(claims.user_id(), budget_id, payload)
            .await
        {
            Ok(id) => Ok(id.to_string()),
            Err(_) => Err(StatusCode::BAD_REQUEST),
        }
    }

    /// Update an item on a budget
    #[debug_handler(state = AppState)]
    pub async fn update_item(
        State(repository): State<Arc<ItemRepository>>,
        Path((budget_id, item_id)): Path<(Uuid, Uuid)>,
        claims: Claims,
        Json(payload): Json<AddItemToBudgetRequest>,
    ) -> StatusCode {
        tracing::info!(
            "User '{}' update item {item_id} on budget {budget_id}. Payload: {payload:?}",
            claims.user_id()
        );

        match repository
            .update_item(claims.user_id(), budget_id, item_id, payload)
            .await
        {
            Ok(_) => StatusCode::ACCEPTED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Delete an item.
    #[debug_handler(state = AppState)]
    pub async fn delete_item(
        State(repository): State<Arc<ItemRepository>>,
        claims: Claims,
        Path((budget_id, item_id)): Path<(Uuid, Uuid)>,
    ) -> StatusCode {
        tracing::info!(
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

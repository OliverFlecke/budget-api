use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// Models

struct Budget {
    id: Uuid,
    user_id: String,
    title: String,
    created_at: NaiveDateTime,
}

// DTOs

#[derive(Debug, Serialize)]
pub struct BudgetDto {
    id: Uuid,
    user_id: String,
    title: String,
    created_at: DateTime<Utc>,
}

impl From<&Budget> for BudgetDto {
    fn from(from: &Budget) -> Self {
        Self {
            id: from.id,
            user_id: from.user_id.to_owned(),
            title: from.title.to_owned(),
            created_at: DateTime::from_utc(from.created_at, Utc),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBudget {
    title: String,
}

pub fn budget_router(pool: &Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(endpoints::get_all_budgets))
        .with_state(pool.clone())
        .route("/", post(endpoints::create_budget))
        .with_state(pool.clone())
        .route("/:id", get(endpoints::get_budget))
        .with_state(pool.clone())
}

// Endpoints
pub mod endpoints {
    use std::sync::Arc;

    use axum::{
        extract::{Path, State},
        http::StatusCode,
        Json,
    };
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{Budget, BudgetDto, CreateBudget};

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
}

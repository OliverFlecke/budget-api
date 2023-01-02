use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
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

// Endpoints
pub mod endpoints {
    use std::sync::Arc;

    use axum::{
        extract::{Path, State},
        Json,
    };
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{Budget, BudgetDto, CreateBudget};

    pub async fn get_budget(
        Path(budget_id): Path<Uuid>,
        State(pool): State<Arc<PgPool>>,
    ) -> Json<BudgetDto> {
        let budget = sqlx::query_as!(Budget, "SELECT * FROM budget WHERE id = $1", budget_id)
            .fetch_one(pool.as_ref())
            .await;
        // TODO: Handle not found
        Json((&budget.unwrap()).into())
    }

    pub async fn get_all_budgets(State(pool): State<Arc<PgPool>>) -> Json<Vec<BudgetDto>> {
        let budget = sqlx::query_as!(Budget, "SELECT * FROM budget")
            .fetch_all(pool.as_ref())
            .await;

        Json(
            budget
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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::Budget;

#[derive(Debug, Serialize)]
pub struct BudgetDto {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
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
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddItemToBudgetRequest {
    pub category: String,
    pub name: String,
    pub amount: i32,
}

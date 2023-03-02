use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model;

#[derive(Debug, Serialize)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
}
impl From<&model::Budget> for Budget {
    fn from(from: &model::Budget) -> Self {
        Self {
            id: from.id,
            user_id: from.user_id.to_owned(),
            title: from.title.to_owned(),
            created_at: DateTime::from_utc(from.created_at, Utc),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BudgetWithItems {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub items: Vec<Item>,
}

impl From<&model::BudgetWithItems> for BudgetWithItems {
    fn from(from: &model::BudgetWithItems) -> Self {
        Self {
            id: from.id,
            user_id: from.user_id.to_owned(),
            title: from.title.to_owned(),
            created_at: DateTime::from_utc(from.created_at, Utc),
            items: from.items.iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBudget {
    pub title: String,
}

/// DTO for the basic item that can be returned to the user.
/// This mirrors the database object closely.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub category: String,
    pub name: String,
    pub amount: i32,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl From<&model::Item> for Item {
    fn from(from: &model::Item) -> Self {
        Self {
            id: from.id,
            budget_id: from.budget_id,
            category: from.category.to_owned(),
            name: from.name.to_owned(),
            amount: from.amount,
            created_at: DateTime::from_utc(from.created_at, Utc),
            modified_at: DateTime::from_utc(from.modified_at, Utc),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(derive_new::new))]
pub struct AddItemToBudgetRequest {
    pub category: String,
    pub name: String,
    pub amount: i32,
}

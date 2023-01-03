use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct BudgetWithItems {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub items: Vec<Item>,
}

/// Datamodel for the `Budget` table
#[derive(Debug, sqlx::Type)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub created_at: NaiveDateTime,
}

/// Datamodel for the `Item` table.
#[derive(Debug, sqlx::Type)]
pub struct Item {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub category: String,
    pub name: String,
    pub amount: i32,
    pub created_at: NaiveDateTime,
    pub modified: NaiveDateTime,
}

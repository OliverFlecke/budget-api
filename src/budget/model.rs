use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Item<'a> {
    pub budget_id: Uuid,
    pub category: &'a str,
    pub name: &'a str,
    pub amount: u64,
}

use std::sync::Arc;

use sqlx::PgPool;

pub struct BudgetRepository {
    db_pool: Arc<PgPool>,
}

impl BudgetRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn create_budget(&self, user_id: &str, title: &str) -> Result<(), ()> {
        match sqlx::query!(
            "INSERT INTO budget (user_id, title) VALUES ($1, $2)",
            user_id,
            title
        )
        .execute(self.db_pool.as_ref())
        .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn create_a_new_budget(pool: PgPool) -> sqlx::Result<()> {
        let repo = BudgetRepository::new(Arc::new(pool));

        assert_eq!(repo.create_budget("Alice", "My first budget").await, Ok(()));

        Ok(())
    }
}

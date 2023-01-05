use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use super::model;

pub struct BudgetRepository {
    db_pool: Arc<PgPool>,
}

impl BudgetRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn create_budget(&self, user_id: &str, title: &str) -> Result<Uuid, ()> {
        match sqlx::query_scalar!(
            r#"INSERT INTO budget (user_id, title) VALUES ($1, $2) RETURNING id"#,
            user_id,
            title
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        {
            Ok(id) => Ok(id),
            Err(_) => Err(()),
        }
    }

    pub async fn get_budget(
        &self,
        user_id: &str,
        budget_id: &Uuid,
    ) -> Option<model::BudgetWithItems> {
        let query = sqlx::query_as!(
            model::BudgetWithItems,
            r#"SELECT b.*,
CASE
    WHEN count(i) = 0 THEN '{}'
    ELSE
        array_agg(
            (i.id, i.budget_id, i.category, i.name, i.amount, i.created_at, i.modified)
        )
    END as "items!: Vec<model::Item>"
FROM budget AS b
LEFT JOIN item AS i ON b.id = i.budget_id
WHERE b.id = $1 AND b.user_id = $2
GROUP BY b.id
"#,
            budget_id,
            user_id
        );

        match query.fetch_one(self.db_pool.as_ref()).await {
            Ok(budget) => Some(budget),
            Err(err) => {
                println!("Error: {err:?}");
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const USER_ID: &str = "Alice";

    #[sqlx::test]
    async fn create_a_new_budget(pool: PgPool) -> sqlx::Result<()> {
        let repo = BudgetRepository::new(Arc::new(pool));

        assert!(repo.create_budget(USER_ID, "My first budget").await.is_ok());

        Ok(())
    }

    #[sqlx::test]
    async fn get_budget_that_is_not_there(pool: PgPool) -> sqlx::Result<()> {
        let repo = BudgetRepository::new(Arc::new(pool));
        let budget_id = Uuid::new_v4();

        assert_eq!(repo.get_budget(USER_ID, &budget_id).await, None);

        Ok(())
    }

    #[sqlx::test]
    async fn get_budget_without_any_items(pool: PgPool) -> sqlx::Result<()> {
        let repo = BudgetRepository::new(Arc::new(pool));
        // Setup data
        let budget_title = "some budget_name";
        let budget_id = repo.create_budget(USER_ID, budget_title).await.unwrap();

        // Act
        let budget = repo.get_budget(USER_ID, &budget_id).await.unwrap();

        // Assert
        assert_eq!(budget.id, budget_id);
        assert_eq!(budget.title, budget_title);
        assert_eq!(budget.user_id, USER_ID);

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    async fn get_budget_with_items(pool: PgPool) -> sqlx::Result<()> {
        let repo = BudgetRepository::new(Arc::new(pool));
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();

        // Act
        let budget = repo.get_budget(USER_ID, &budget_id).await.unwrap();

        // Assert
        assert_eq!(budget.title, "My budget with items");
        assert_eq!(budget.items.len(), 3);
        assert_eq!(budget.items[0].category, "Income");
        assert_eq!(budget.items[0].name, "Paycheck");
        assert_eq!(budget.items[0].amount, 100);
        assert_eq!(budget.items[1].category, "Home");
        assert_eq!(budget.items[1].name, "Rent");
        assert_eq!(budget.items[1].amount, 50);
        assert_eq!(budget.items[2].category, "Food");
        assert_eq!(budget.items[2].name, "Restaurants");
        assert_eq!(budget.items[2].amount, 10);

        Ok(())
    }
}

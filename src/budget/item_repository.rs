use std::sync::Arc;

use sqlx::PgPool;
use tracing::{event, Level};
use uuid::Uuid;

use super::{dto, model};

#[derive(Debug, PartialEq, Eq)]
pub enum ItemRepositoryError {
    Database,
    NotFound,
    Unauthorized(String),
}

/// Repository to access items.
/// Abstracts away the DB interations for items.
#[derive(Debug)]
pub struct ItemRepository {
    db_pool: Arc<PgPool>,
}

impl ItemRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    /// Get the item by its id.
    #[allow(dead_code)]
    pub async fn get_item(&self, budget_id: Uuid, item_id: Uuid) -> Option<model::Item> {
        let query = sqlx::query_as!(
            model::Item,
            r#"SELECT * FROM item WHERE id = $1 AND budget_id = $2 "#,
            item_id,
            budget_id
        );

        match query.fetch_one(self.db_pool.as_ref()).await {
            Ok(item) => Some(item),
            Err(_) => None,
        }
    }

    /// Add a new item to a budget.
    pub async fn add_item_to_budget(
        &self,
        user_id: &str,
        budget_id: Uuid,
        payload: dto::AddItemToBudgetRequest,
    ) -> Result<Uuid, ItemRepositoryError> {
        if !self.check_access(budget_id, user_id).await {
            return Err(ItemRepositoryError::Unauthorized(user_id.to_string()));
        }

        let query = sqlx::query_scalar!(
            "INSERT INTO item (budget_id, category, name, amount) VALUES ($1, $2, $3, $4) RETURNING id",
            budget_id,
            payload.category,
            payload.name,
            payload.amount
        );

        match query.fetch_one(self.db_pool.as_ref()).await {
            Ok(id) => Ok(id),
            Err(err) => {
                event!(Level::ERROR, "Error adding item to budget: {err:?}");
                Err(ItemRepositoryError::Database)
            }
        }
    }

    /// Delete an item.
    pub async fn delete_item(
        &self,
        user_id: &str,
        budget_id: Uuid,
        item_id: Uuid,
    ) -> Result<(), ItemRepositoryError> {
        event!(Level::TRACE, "[item_repository] User '{user_id}' deleting item '{item_id}' from budget '{budget_id}'");
        let query = sqlx::query!(
            r#"with deleted as 
            (delete from item 
               where id = $1 
                 and exists(select * from budget where id = $2 and user_id = $3) 
               returning *)
            select count(*) from deleted"#,
            item_id,
            budget_id,
            user_id
        );

        match query.fetch_one(self.db_pool.as_ref()).await {
            Ok(x) => match x.count {
                Some(1) => Ok(()),
                _ => {
                    event!(Level::ERROR, "Item '{item_id}' does not exists");
                    Err(ItemRepositoryError::NotFound)
                }
            },
            Err(err) => {
                event!(Level::ERROR, "Error: {err:?}");
                Err(ItemRepositoryError::Database)
            }
        }
    }

    /// Update an item. Can be provided with a new name, category, or amount.
    pub async fn update_item(
        &self,
        user_id: &str,
        budget_id: Uuid,
        item_id: Uuid,
        request: dto::AddItemToBudgetRequest,
    ) -> Result<(), ItemRepositoryError> {
        if !self.check_access(budget_id, user_id).await {
            return Err(ItemRepositoryError::Unauthorized(user_id.to_string()));
        }

        let query = sqlx::query!(
            "UPDATE item SET category = $1, amount = $2, name = $3 WHERE id = $4",
            request.category,
            request.amount,
            request.name,
            item_id
        );

        match query.execute(self.db_pool.as_ref()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                event!(Level::ERROR, "Error: {err:?}");
                Err(ItemRepositoryError::Database)
            }
        }
    }

    async fn check_access(&self, budget_id: Uuid, user_id: &str) -> bool {
        let budget_query = sqlx::query!(
            "SELECT * FROM budget WHERE id = $1 AND user_id = $2",
            budget_id,
            user_id,
        );

        match budget_query.fetch_optional(self.db_pool.as_ref()).await {
            Ok(Some(_)) => {
                event!(Level::TRACE, "User '{user_id}' has access to '{budget_id}'");
                true
            }
            Ok(None) => {
                event!(
                    Level::WARN,
                    "User '{user_id}' does not have access to '{budget_id}'"
                );
                false
            }
            Err(err) => {
                event!(
                    Level::ERROR,
                    "Error check access for user '{user_id}' to budget '{budget_id}': {err:?}"
                );
                false
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    async fn get_item_with_an_id(pool: PgPool) -> sqlx::Result<()> {
        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();
        let id = Uuid::parse_str("d831821b-1b50-41fc-a01e-19a1243c334a").unwrap();

        // Act
        let item = repo.get_item(budget_id, id).await.unwrap();

        // Assert
        assert_eq!(item.category, "Food");
        assert_eq!(item.name, "Restaurants");
        assert_eq!(item.amount, 10);

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    async fn add_a_new_item_to_a_budget(pool: PgPool) -> sqlx::Result<()> {
        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let user_id = "Alice";
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();

        let request = dto::AddItemToBudgetRequest::new(
            "Some category".to_string(),
            "Some name".to_string(),
            123,
        );

        // Act
        let item_id = repo
            .add_item_to_budget(user_id, budget_id, request.clone())
            .await
            .unwrap();

        // Assert
        // Get the item that was just created
        let item = repo.get_item(budget_id, item_id).await.unwrap();
        assert_eq!(item.category, request.category);
        assert_eq!(item.name, request.name);
        assert_eq!(item.amount, request.amount);

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    #[traced_test]
    async fn try_add_a_new_item_to_a_budget_as_other_user(pool: PgPool) -> sqlx::Result<()> {
        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let user_id = "Bob";
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();

        let request = dto::AddItemToBudgetRequest::new(
            "Some category".to_string(),
            "Some name".to_string(),
            123,
        );

        // Act
        let item = repo
            .add_item_to_budget(user_id, budget_id, request.clone())
            .await;

        // Assert
        assert!(item.is_err());

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    async fn delete_an_item(pool: PgPool) -> sqlx::Result<()> {
        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();
        let item_id = Uuid::parse_str("d831821b-1b50-41fc-a01e-19a1243c334a").unwrap();
        let user_id = "Alice";

        // Act
        assert!(repo.delete_item(user_id, budget_id, item_id).await.is_ok());

        // Assert
        assert_eq!(repo.get_item(budget_id, item_id).await, None);

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    async fn try_delete_an_item_for_another_user(pool: PgPool) -> sqlx::Result<()> {
        // "Bob" is trying to delete an item from "Alice"'s budget

        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();
        let item_id = Uuid::parse_str("d831821b-1b50-41fc-a01e-19a1243c334a").unwrap();
        let user_id = "Bob";

        // Act
        assert_eq!(
            repo.delete_item(user_id, budget_id, item_id)
                .await
                .unwrap_err(),
            ItemRepositoryError::NotFound
        );

        // Assert
        assert_ne!(repo.get_item(budget_id, item_id).await, None);

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    async fn update_item_with_new_fields(pool: PgPool) -> sqlx::Result<()> {
        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let user_id = "Alice";
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();
        let item_id = Uuid::parse_str("d831821b-1b50-41fc-a01e-19a1243c334a").unwrap();
        let request = dto::AddItemToBudgetRequest::new(
            "Updated category".to_string(),
            "Updated name".to_string(),
            999,
        );

        // Act
        assert!(repo
            .update_item(user_id, budget_id, item_id, request.clone())
            .await
            .is_ok());

        // Assert
        let item = repo.get_item(budget_id, item_id).await.unwrap();
        assert_eq!(item.category, request.category);
        assert_eq!(item.name, request.name);
        assert_eq!(item.amount, request.amount);

        Ok(())
    }

    #[sqlx::test(fixtures("budget_with_items"))]
    #[cfg_attr(not(feature = "db_test"), ignore)]
    #[traced_test]
    async fn try_update_item_with_new_fields_as_other_user(pool: PgPool) -> sqlx::Result<()> {
        // Arrange
        let repo = ItemRepository::new(Arc::new(pool));
        let user_id = "Bob";
        let budget_id = Uuid::parse_str("b8d6ff4e-c12f-416b-a611-8ad0c90669fe").unwrap();
        let item_id = Uuid::parse_str("d831821b-1b50-41fc-a01e-19a1243c334a").unwrap();
        let request = dto::AddItemToBudgetRequest::new(
            "Updated category".to_string(),
            "Updated name".to_string(),
            999,
        );

        // Act
        let error = repo
            .update_item(user_id, budget_id, item_id, request.clone())
            .await
            .unwrap_err();

        // Assert
        assert_eq!(
            error,
            ItemRepositoryError::Unauthorized(user_id.to_string())
        );
        let item = repo.get_item(budget_id, item_id).await.unwrap();
        assert_ne!(item.category, request.category);
        assert_ne!(item.name, request.name);
        assert_ne!(item.amount, request.amount);

        Ok(())
    }
}

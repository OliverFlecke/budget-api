use crate::{
    auth::{config::AuthConfig, jwk::JwkRepository},
    budget::{item_repository::ItemRepository, repository::BudgetRepository},
};
use anyhow::Result;
use axum::extract::FromRef;
use derive_getters::Getters;
use duplicate::duplicate_item;
use sqlx::PgPool;
use std::sync::Arc;

/// Represents the global app state for Axum.
/// Can also be considered as the DoI container for the application.
#[derive(Debug, Clone, Getters)]
pub struct AppState {
    jwks_repository: Arc<JwkRepository>,
    budget_repository: Arc<BudgetRepository>,
    item_repository: Arc<ItemRepository>,
}

impl AppState {
    pub async fn initialize() -> Result<Self> {
        tracing::trace!("Initializing application services");
        let url = std::env::var("DATABASE_URL").expect(
            "Missing environment variable 'DATABASE_URL' provided with a connection string",
        );
        let pool = Arc::new(PgPool::connect(&url).await.unwrap());

        let auth_config = AuthConfig::from_env();
        let jwks_repository = Arc::new(JwkRepository::new(auth_config).await?);

        Ok(Self {
            jwks_repository,
            budget_repository: Arc::new(BudgetRepository::new(pool.clone())),
            item_repository: Arc::new(ItemRepository::new(pool.clone())),
        })
    }
}

#[duplicate_item(
    service_type         field;
    [ BudgetRepository ] [ budget_repository ];
    [ ItemRepository ]   [ item_repository ];
    [ JwkRepository ]    [ jwks_repository ];
)]
impl FromRef<AppState> for Arc<service_type> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.field.clone()
    }
}

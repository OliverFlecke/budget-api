use std::error::Error;

use derive_getters::Getters;
use jsonwebtoken::DecodingKey;
use serde::Deserialize;

use super::config::AuthConfig;

static JWKS_ENDPOINT: &str = ".well-known/jwks.json";

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

impl JwksResponse {
    /// Fetch JWKS keys from the and endpoint
    async fn fetch(auth_server: &str) -> Result<JwksResponse, reqwest::Error> {
        reqwest::get(format!("{auth_server}{JWKS_ENDPOINT}"))
            .await?
            .json::<JwksResponse>()
            .await
    }
}

#[derive(Debug, Clone, Deserialize, Getters, PartialEq, Eq)]
pub struct Jwk {
    n: String,
    e: String,
}

impl Jwk {
    /// Get the current JWK to use for decoding tokens.
    pub async fn fetch(auth_server: &str) -> Result<Jwk, Box<dyn Error>> {
        tracing::event!(tracing::Level::DEBUG, "Fetching jwk from identity host");

        let jwks = JwksResponse::fetch(auth_server).await?;

        Ok(jwks.keys.first().unwrap().clone())
    }
}

impl From<Jwk> for DecodingKey {
    fn from(value: Jwk) -> Self {
        DecodingKey::from_rsa_components(value.n(), value.e()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct JwkRepository {
    auth_config: AuthConfig,
    keys: Vec<Jwk>,
}

impl From<AuthConfig> for JwkRepository {
    fn from(auth_config: AuthConfig) -> Self {
        Self {
            auth_config,
            keys: vec![],
        }
    }
}

impl JwkRepository {
    pub async fn new(auth_config: AuthConfig) -> Result<Self, Box<dyn Error>> {
        let jwks = JwksResponse::fetch(auth_config.issuer()).await?;

        Ok(Self {
            auth_config,
            keys: jwks.keys,
        })
    }

    /// Get the current JWK to use.
    pub fn get_key(&self) -> Option<Jwk> {
        self.keys.first().cloned()
    }

    /// Refresh JWKs and returns the current one.
    pub async fn get_key_with_refresh(&mut self) -> Result<Jwk, Box<dyn Error>> {
        // TODO: Should be update at a certain frequency.
        if self.keys.is_empty() {
            self.update_keys().await?;
        }

        Ok(self.get_key().expect("a key to have been fetched"))
    }

    /// Updates the internal, local storage of the JWKs.
    async fn update_keys(&mut self) -> Result<(), Box<dyn Error>> {
        self.keys = JwksResponse::fetch(self.auth_config.issuer()).await?.keys;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::auth::config::AuthConfig;

    use super::*;

    #[tokio::test]
    async fn fetch_jwk_from_auth0() {
        let config = AuthConfig::default();
        let jwk = Jwk::fetch(config.issuer()).await.unwrap();

        assert_eq!(jwk.e, "AQAB");
        assert_ne!(jwk.n, ""); // Don't want to test the exact value of the key here, so it's enough to just verify that its not empty.
    }

    #[tokio::test]
    async fn repository_update_keys() {
        let mut repository = JwkRepository::from(AuthConfig::default());

        repository.update_keys().await.expect("update to work");
        assert_ne!(repository.get_key(), None);
    }

    #[tokio::test]
    async fn repository_fetch_on_create() {
        let repository = JwkRepository::new(AuthConfig::default())
            .await
            .expect("to be able to create the repository");

        assert_ne!(repository.get_key(), None);
    }
}

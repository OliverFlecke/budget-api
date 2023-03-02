use std::error::Error;

use derive_getters::Getters;
use jsonwebtoken::DecodingKey;
use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize, Getters)]
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
}

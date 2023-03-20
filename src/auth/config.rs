use derive_getters::Getters;
use jsonwebtoken::{Algorithm, Validation};

pub(crate) static ISSUER: &str = "https://oliverflecke.eu.auth0.com/";
pub(crate) static AUDIENCE: &str = "https://finance.oliverflecke.me/";

#[derive(Debug, Getters, Clone)]
pub struct AuthConfig {
    issuer: String,
    audience: String,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            issuer: std::env::var("ISSUER").expect("variable 'ISSUER' to be set"),
            audience: std::env::var("AUDIENCE").expect("variable 'AUDIENCE' to be set"),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            issuer: ISSUER.to_string(),
            audience: AUDIENCE.to_string(),
        }
    }
}

impl From<&AuthConfig> for Validation {
    fn from(value: &AuthConfig) -> Self {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[value.issuer()]);
        validation.set_audience(&[value.audience()]);

        validation
    }
}

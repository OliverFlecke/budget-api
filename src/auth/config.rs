use derive_getters::Getters;

pub(crate) static ISSUER: &str = "https://oliverflecke.eu.auth0.com/";
pub(crate) static AUDIENCE: &str = "https://finance.oliverflecke.me/";

#[derive(Debug, Getters)]
pub struct AuthConfig {
    issuer: String,
    audience: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            issuer: ISSUER.to_string(),
            audience: AUDIENCE.to_string(),
        }
    }
}

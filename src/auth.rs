pub mod config;
mod jwk;
mod utils;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};

/// Extractor to get the user ID from the request.
///
/// TODO: This is only a temporary way to get the user id.
/// At some point this should be migrated to actually use a JWT or similar.
pub struct ExtractUserId(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUserId
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.headers.get(AUTHORIZATION) {
            Some(authorization) => {
                println!("Got authorization as user: {authorization:?}");
                Ok(ExtractUserId(authorization.to_str().unwrap().to_string()))
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

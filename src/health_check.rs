use axum::{http::StatusCode, routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/", get(is_alive))
}

async fn is_alive() -> StatusCode {
    StatusCode::OK
}

use std::error::Error;

use axum::{
    http::{StatusCode, Uri},
    Router,
};
use budget_api::{app_state::AppState, budget::create_budget_router};

use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, trace, warn, Level};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize services
    setup_tracing();
    trace!("Initialize services");

    let port = std::env::var("PORT")
        .map(|p| p.parse::<usize>().expect("PORT is not a valid integer"))
        .unwrap_or(4000);
    let host = format!("0.0.0.0:{port}").parse().unwrap();

    let app_state = AppState::initialize().await?;

    // Build app
    trace!("Building app");
    let app = Router::new()
        .nest("/budget", create_budget_router(app_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback(not_found);

    // Run app
    info!("Server running at {host}");
    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "budget_api=debug,hyper=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    warn!("Path not found {uri}");
    (StatusCode::NOT_FOUND, format!("No route for '{uri}'"))
}

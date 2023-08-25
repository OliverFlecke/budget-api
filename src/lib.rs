use crate::app_state::AppState;
use anyhow::Result;
use axum::{
    http::{StatusCode, Uri},
    Router, Server,
};
use std::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

pub mod app_state;
pub mod auth;
pub mod budget;
mod health_check;

#[derive(Debug)]
pub struct App {
    router: Router,
}

impl App {
    pub async fn create() -> Result<Self> {
        // Initialize services
        setup_tracing()?;
        tracing::trace!("Initialize services");

        let app_state = AppState::initialize().await?;
        let router = Self::build_router(app_state);

        Ok(Self { router })
    }

    pub async fn serve(self, host: TcpListener) -> Result<()> {
        tracing::info!("Server running at {host:#?}");
        Server::from_tcp(host)?
            .serve(self.router.into_make_service())
            .await?;
        Ok(())
    }

    /// Builder the router for the application.
    fn build_router(app_state: AppState) -> Router {
        tracing::trace!("Building app");
        Router::new()
            .nest("/health", health_check::create_router())
            .nest("/budget", budget::create_router(app_state))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .fallback(not_found)
    }
}

fn setup_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("budget_api=debug".parse()?)
                .add_directive("hyper=info".parse()?)
                .add_directive("tower_http=info".parse()?),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    Ok(())
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    tracing::warn!("Path not found {uri}");
    (StatusCode::NOT_FOUND, format!("No route for '{uri}'"))
}

mod agents;
mod api;
mod auth;
mod events;
mod models;
mod runtime;
mod storage;
mod tools;

use std::net::SocketAddr;

use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug)]
pub struct AppState {
    pub service_name: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let state = AppState {
        service_name: "nodepilot-backend".to_string(),
    };

    let app = build_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!(%addr, "starting backend server");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("server exited unexpectedly");
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(api::health::health_check))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

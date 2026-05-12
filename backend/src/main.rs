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
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug)]
pub struct AppState {
    pub service_name: String,
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let db = match storage::init_pool_from_env().await {
        Ok(pool) => pool,
        Err(err) => {
            error!(error = %err, "failed to connect to sqlite");
            return;
        }
    };

    if let Err(err) = storage::initialize_schema(&db).await {
        error!(error = %err, "failed to initialize sqlite schema");
        return;
    }

    let state = AppState {
        service_name: "nodepilot-backend".to_string(),
        db,
    };

    let app = build_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    info!(%addr, "starting backend server");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            error!(error = %err, "failed to bind tcp listener");
            return;
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        error!(error = %err, "server exited unexpectedly");
    }
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

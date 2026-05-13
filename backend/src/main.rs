mod agents;
mod api;
mod auth;
mod events;
mod models;
mod runtime;
mod storage;
mod tools;

use std::{net::SocketAddr, path::PathBuf};

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::events::broadcaster::EventBroadcaster;
use crate::runtime::RuntimeSettings;

#[derive(Clone, Debug)]
pub struct AppState {
    pub service_name: String,
    pub db: SqlitePool,
    pub events: EventBroadcaster,
    pub workspace_root: PathBuf,
    pub runtime: RuntimeSettings,
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

    let workspace_root = std::env::var("WORKSPACES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../workspaces"));

    let state = AppState {
        service_name: "nodepilot-backend".to_string(),
        db,
        events: EventBroadcaster::new(),
        workspace_root,
        runtime: RuntimeSettings::from_env(),
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
        .route(
            "/agents",
            post(agents::handlers::create_agent).get(agents::handlers::list_agents),
        )
        .route(
            "/agents/{id}/provision",
            post(agents::handlers::provision_agent),
        )
        .route("/agents/{id}/chat", post(agents::handlers::chat_with_agent))
        .route(
            "/agents/{id}/events",
            get(agents::handlers::stream_agent_events),
        )
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

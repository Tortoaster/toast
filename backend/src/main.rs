use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayerBuilder;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::{config::AppConfig, error::AppError, state::AppState};

mod api;
mod config;
mod dto;
mod error;
mod repository;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    let config = AppConfig::get();

    tracing_subscriber::fmt()
        .with_env_filter(config.env_filter())
        .init();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_prefix("toast")
        .with_default_metrics()
        .build_pair();

    let state = AppState::new().await;

    let app = Router::new()
        .route("/projects", get(api::list_projects))
        .route("/projects/{id}", get(api::get_project))
        .fallback(async || AppError::NotFound)
        .route("/metrics", get(async move || metric_handle.render()))
        .layer(prometheus_layer)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

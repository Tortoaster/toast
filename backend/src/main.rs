use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, response::IntoResponse, routing::get};
use axum_oidc::{error::MiddlewareError, OidcLoginLayer};
use axum_prometheus::PrometheusMetricLayerBuilder;
use tokio::{net::TcpListener, signal, task::AbortHandle};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{cookie::SameSite, ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{config::AppConfig, error::AppError, state::AppState, utils::claims::AppClaims};

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

    let (state, oidc_auth_layer) = AppState::new().await;

    let session_store = PostgresStore::new(state.pool.clone())
        .with_schema_name("sessions")
        .unwrap()
        .with_table_name("sessions")
        .unwrap();

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(Duration::from_secs(1800)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(time::Duration::minutes(30)));

    let toast_handle_error_layer =
        HandleErrorLayer::new(|e: MiddlewareError| async { AppError::Session(e).into_response() });

    let oidc_login_service = ServiceBuilder::new()
        .layer(toast_handle_error_layer.clone())
        .layer(OidcLoginLayer::<AppClaims>::new());

    let oidc_auth_service = ServiceBuilder::new()
        .layer(toast_handle_error_layer)
        .layer(oidc_auth_layer);

    let (app, api) = OpenApiRouter::new()
        // Login required
        .merge(api::projects::protected_router())
        .merge(api::comments::protected_router())
        .merge(api::files::protected_router())
        .routes(routes!(api::users::login))
        .layer(oidc_login_service)
        // Login optional
        .routes(routes!(api::users::logout))
        .layer(oidc_auth_service)
        .layer(session_layer)
        // Publicly available
        .merge(api::projects::public_router())
        .merge(api::comments::public_router())
        .fallback(async || AppError::NotFound)
        .route("/metrics", get(async move || metric_handle.render()))
        .layer(prometheus_layer)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
        .split_for_parts();
    let app = app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown([deletion_task.abort_handle()]))
        .await
        .unwrap();
}

pub async fn graceful_shutdown(abort_handles: impl IntoIterator<Item = AbortHandle>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { for handle in abort_handles { handle.abort() } },
        _ = terminate => { for handle in abort_handles { handle.abort() } },
    }
}

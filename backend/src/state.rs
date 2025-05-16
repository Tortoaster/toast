use std::time::Duration;

use axum::extract::FromRef;
use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use sqlx::SqlitePool;
use tracing::{info, warn};

use crate::{config::AppConfig, repository::ProjectRepository};

#[derive(Clone, Debug)]
pub struct AppState {
    pool: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        let config = AppConfig::get();

        let pool = init_pool(config)
            .await
            .expect("failed to connect to database");

        AppState { pool }
    }
}

async fn init_pool(config: &AppConfig) -> Result<SqlitePool, &'static str> {
    let pool = backoff::future::retry_notify(
        backoff_config(),
        || async {
            let pool = SqlitePool::connect_with(config.sqlite_connect_options()).await?;
            Ok(pool)
        },
        |error, duration: Duration| {
            warn!("failed to connect to database: {error}");
            warn!("retrying in {} seconds", duration.as_secs());
        },
    )
    .await
    .map_err(|_| "failed to connect to database")?;

    info!("connected to database");

    Ok(pool)
}

fn backoff_config() -> ExponentialBackoff {
    ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_secs(1))
        .with_multiplier(2.0)
        .with_max_interval(Duration::from_secs(5))
        .with_randomization_factor(0.0)
        .with_max_elapsed_time(Some(Duration::from_secs(32)))
        .build()
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for ProjectRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.pool.clone())
    }
}

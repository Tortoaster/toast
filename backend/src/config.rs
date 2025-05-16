use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::OnceLock,
};

use config::Config;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use sqlx::sqlite::SqliteConnectOptions;
use tracing_subscriber::EnvFilter;

#[serde_inline_default]
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde_inline_default(IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub host: IpAddr,
    #[serde_inline_default(8000)]
    pub port: u16,
    #[serde_inline_default("info".to_owned())]
    rust_log: String,
    database: DatabaseConfig,
}

impl AppConfig {
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AppConfig> = OnceLock::new();

        CONFIG.get_or_init(|| {
            Config::builder()
                .add_source(config::File::with_name("Config.toml").required(false))
                .add_source(config::Environment::default().separator("__"))
                .build()
                .expect("invalid config settings")
                .try_deserialize()
                .expect("failed to deserialize config")
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn env_filter(&self) -> EnvFilter {
        EnvFilter::new(&self.rust_log)
    }

    pub fn sqlite_connect_options(&self) -> SqliteConnectOptions {
        self.database
            .url
            .parse::<SqliteConnectOptions>()
            .expect("invalid database url")
    }
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
}

use std::time::Duration;

use derive_more::Deref;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DbConfig {
    /// Database-specific connection and configuration URL.
    pub url: String,
    /// Minimum number of connections to maintain in the pool.
    pub min_connections: Option<u32>,
    /// Maximum number of connections to maintain in the pool.
    pub max_connections: usize,
    /// Number of seconds to wait for a connection before timing out.
    pub connect_timeout: u64,
    /// Maximum number of seconds to keep a connection alive for.
    pub idle_timeout: Option<u64>,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://default.sqlite3".to_string(),
            min_connections: Some(1),
            max_connections: 16,
            connect_timeout: 5,
            idle_timeout: Some(5),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("FigmentErr {0}")]
    Figment(#[from] figment::Error),
    #[error("SeaOrmDbErr {0}")]
    SeaOrmDb(#[from] DbErr),
    #[error("MaxConnectionsOverflowErr: {0}")]
    MaxConnectionsOverflow(#[from] std::num::TryFromIntError),
}

/// Our apps database newtype
#[derive(Clone, Debug, Deref)]
pub struct Db {
    conn: DatabaseConnection,
}

impl Db {
    pub async fn connect(config: &DbConfig) -> Result<Self, Error> {
        let mut options: ConnectOptions = config.url.clone().into();

        options
            .max_connections(config.max_connections.try_into()?)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));

        info!("Connecting to database: {:?}", config);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = Database::connect(options).await?;

        Ok(Self {
            conn,
        })
    }

    pub async fn run_migrations(&self) -> std::result::Result<(), Error> {
        migration::Migrator::up(self.conn(), None).await?;
        Ok(())
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}

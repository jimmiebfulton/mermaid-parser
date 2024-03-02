use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

const DEFAULT_DATABASE_URL: &str = "postgres://test@localhost/example-service";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersistenceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    migrate: Option<bool>,
    database: DatabaseSettings,
}

impl PersistenceSettings {
    pub fn temp_db(&self) -> Option<bool> {
        self.temporary
    }

    pub fn set_temp_db(&mut self, temporary: bool) -> &mut PersistenceSettings {
        self.temporary = Some(temporary);
        self
    }

    pub fn with_temp_db(mut self, temporary: bool) -> PersistenceSettings {
        self.temporary = Some(temporary);
        self
    }

    pub fn database(&self) -> &DatabaseSettings {
        &self.database
    }

    pub fn migrate(&self) -> Option<bool> {
        self.migrate
    }

    pub fn set_migrate(&mut self, migrate: Option<bool>) {
        self.migrate = migrate;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseSettings {
    url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_timeout_millis: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_lifetime_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_sql: Option<bool>,
}

impl DatabaseSettings {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn with_url(mut self, url: &Url) -> Self {
        self.url = url.clone();
        self
    }

    pub fn max_connections(&self) -> Option<u32> {
        self.max_connections
    }

    pub fn min_connections(&self) -> Option<u32> {
        self.min_connections
    }

    pub fn connect_timeout(&self) -> Option<Duration> {
        self.connect_timeout_millis.map(Duration::from_millis)
    }

    pub fn idle_timeout(&self) -> Option<Duration> {
        self.idle_timeout_seconds.map(Duration::from_secs)
    }

    pub fn max_lifetime(&self) -> Option<Duration> {
        self.max_lifetime_seconds.map(Duration::from_secs)
    }

    pub fn log_sql(&self) -> bool {
        self.log_sql.unwrap_or(false)
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings {
            url: Url::parse(DEFAULT_DATABASE_URL).expect("Improperly formed Database URL"),
            max_connections: None,
            min_connections: None,
            connect_timeout_millis: None,
            idle_timeout_seconds: None,
            max_lifetime_seconds: None,
            log_sql: None,
        }
    }
}
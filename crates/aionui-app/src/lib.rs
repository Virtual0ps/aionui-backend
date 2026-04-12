use axum::{Json, Router, routing::get};
use serde::Serialize;

/// Application configuration parsed from CLI arguments.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    /// Format as `host:port` for socket binding.
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: aionui_common::constants::DEFAULT_HOST.to_string(),
            port: aionui_common::constants::DEFAULT_PORT,
        }
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// Create the application router with all routes registered.
pub fn create_router() -> Router {
    Router::new().route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 25808);
    }

    #[test]
    fn test_app_config_socket_addr() {
        let config = AppConfig {
            host: "0.0.0.0".to_string(),
            port: 3000,
        };
        assert_eq!(config.socket_addr(), "0.0.0.0:3000");
    }

    #[test]
    fn test_app_config_socket_addr_default() {
        let config = AppConfig::default();
        assert_eq!(config.socket_addr(), "127.0.0.1:25808");
    }
}

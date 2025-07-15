//! # Configuration Module
//!
//! Configuration management for the UltraFast MCP Sequential Thinking project.
//!
//! This module provides configuration structures and loading functionality
//! for both server and client components.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::thinking::client::ClientThinkingConfig;

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Transport type (stdio, http)
    pub transport: String,
    /// Port for HTTP transport
    pub port: u16,
    /// Thinking configuration
    pub thinking: ThinkingConfig,
    /// Export configuration
    pub export: ExportConfig,
    /// Analytics configuration
    pub analytics: AnalyticsConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Security configuration
    pub security: SecurityConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "ultrafast-sequential-thinking".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            transport: "stdio".to_string(),
            port: 8080,
            thinking: ThinkingConfig::default(),
            export: ExportConfig::default(),
            analytics: AnalyticsConfig::default(),
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

/// Client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Server URL
    pub server_url: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Number of retry attempts
    pub retry_attempts: u32,
    /// Thinking configuration
    pub thinking: ClientThinkingConfig,
    /// Connection configuration
    pub connection: ConnectionConfig,
    /// UI configuration
    pub ui: UIConfig,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_url: "stdio://".to_string(),
            timeout_seconds: 30,
            retry_attempts: 3,
            thinking: ClientThinkingConfig::default(),
            connection: ConnectionConfig::default(),
            ui: UIConfig::default(),
        }
    }
}

/// Thinking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Maximum thoughts per session
    pub max_thoughts_per_session: u32,
    /// Maximum branches per session
    pub max_branches_per_session: u32,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Whether to enable analytics
    pub enable_analytics: bool,
    /// Whether to enable thought logging
    pub enable_thought_logging: bool,
    /// Maximum thought length
    pub max_thought_length: usize,
    /// Minimum thought length
    pub min_thought_length: usize,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
}

impl Default for ThinkingConfig {
    fn default() -> Self {
        Self {
            max_thoughts_per_session: 100,
            max_branches_per_session: 10,
            session_timeout_seconds: 3600,
            enable_analytics: true,
            enable_thought_logging: true,
            max_thought_length: 10000,
            min_thought_length: 10,
            rate_limiting: RateLimitingConfig::default(),
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Maximum requests per minute
    pub requests_per_minute: u32,
    /// Maximum thoughts per minute
    pub thoughts_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
    /// Whether rate limiting is enabled
    pub enabled: bool,
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            thoughts_per_minute: 100,
            burst_size: 10,
            enabled: true,
        }
    }
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Supported export formats
    pub formats: Vec<String>,
    /// Whether to auto-export
    pub auto_export: bool,
    /// Export directory
    pub export_directory: String,
    /// Export filename template
    pub filename_template: String,
    /// Whether to include metadata
    pub include_metadata: bool,
    /// Whether to include statistics
    pub include_statistics: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            formats: vec![
                "json".to_string(),
                "markdown".to_string(),
                "pdf".to_string(),
            ],
            auto_export: false,
            export_directory: "./exports".to_string(),
            filename_template: "session_{session_id}_{timestamp}".to_string(),
            include_metadata: true,
            include_statistics: true,
        }
    }
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Whether analytics is enabled
    pub enabled: bool,
    /// Analytics endpoint
    pub endpoint: String,
    /// Analytics API key
    pub api_key: Option<String>,
    /// Metrics collection interval in seconds
    pub collection_interval: u64,
    /// Whether to collect detailed metrics
    pub detailed_metrics: bool,
    /// Retention period for metrics in days
    pub retention_days: u32,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:9090".to_string(),
            api_key: None,
            collection_interval: 60,
            detailed_metrics: true,
            retention_days: 30,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log file path
    pub file_path: Option<String>,
    /// Whether to log to console
    pub console: bool,
    /// Whether to log to file
    pub file: bool,
    /// Log format (json, text)
    pub format: String,
    /// Whether to include timestamps
    pub include_timestamps: bool,
    /// Whether to include thread IDs
    pub include_thread_ids: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_path: Some("./logs/sequential-thinking.log".to_string()),
            console: true,
            file: true,
            format: "text".to_string(),
            include_timestamps: true,
            include_thread_ids: false,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Whether authentication is required
    pub require_auth: bool,
    /// Allowed origins for CORS
    pub allowed_origins: Vec<String>,
    /// API key validation
    pub api_key_validation: bool,
    /// Rate limiting enabled
    pub rate_limiting_enabled: bool,
    /// Session encryption
    pub session_encryption: bool,
    /// Audit logging
    pub audit_logging: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            require_auth: false,
            allowed_origins: vec!["*".to_string()],
            api_key_validation: false,
            rate_limiting_enabled: true,
            session_encryption: false,
            audit_logging: true,
        }
    }
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
    /// Keep-alive interval in seconds
    pub keep_alive_interval: u64,
    /// Maximum connection retries
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay: u64,
    /// Whether to use connection pooling
    pub connection_pooling: bool,
    /// Pool size
    pub pool_size: u32,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            keep_alive_interval: 60,
            max_retries: 3,
            retry_delay: 1,
            connection_pooling: true,
            pool_size: 10,
        }
    }
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// Whether to show progress bars
    pub show_progress_bars: bool,
    /// Whether to show thought visualization
    pub show_thought_visualization: bool,
    /// Whether to show session statistics
    pub show_session_stats: bool,
    /// UI theme
    pub theme: String,
    /// Whether to enable color output
    pub color_output: bool,
    /// Whether to show timestamps
    pub show_timestamps: bool,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            show_progress_bars: true,
            show_thought_visualization: true,
            show_session_stats: true,
            theme: "default".to_string(),
            color_output: true,
            show_timestamps: true,
        }
    }
}

/// Configuration manager
pub struct ConfigManager {
    /// Server configuration
    server_config: Option<ServerConfig>,
    /// Client configuration
    client_config: Option<ClientConfig>,
    /// Configuration file path
    config_path: Option<String>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            server_config: None,
            client_config: None,
            config_path: None,
        }
    }

    /// Load configuration from file
    pub fn load_from_file<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            self.load_from_toml(&content)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            self.load_from_json(&content)?;
        } else {
            return Err("Unsupported configuration file format".into());
        }

        self.config_path = Some(path.to_string_lossy().to_string());
        Ok(())
    }

    /// Load configuration from TOML string
    pub fn load_from_toml(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config: toml::Value = toml::from_str(content)?;

        if let Some(server) = config.get("server") {
            self.server_config = Some(server.clone().try_into()?);
        }

        if let Some(client) = config.get("client") {
            self.client_config = Some(client.clone().try_into()?);
        }

        Ok(())
    }

    /// Load configuration from JSON string
    pub fn load_from_json(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config: serde_json::Value = serde_json::from_str(content)?;

        if let Some(server) = config.get("server") {
            self.server_config = Some(serde_json::from_value(server.clone())?);
        }

        if let Some(client) = config.get("client") {
            self.client_config = Some(serde_json::from_value(client.clone())?);
        }

        Ok(())
    }

    /// Load configuration from environment variables
    pub fn load_from_env(&mut self) {
        // Server configuration from environment
        if let Ok(name) = std::env::var("SEQUENTIAL_THINKING_SERVER_NAME") {
            self.server_config
                .get_or_insert_with(ServerConfig::default)
                .name = name;
        }

        if let Ok(transport) = std::env::var("SEQUENTIAL_THINKING_TRANSPORT") {
            self.server_config
                .get_or_insert_with(ServerConfig::default)
                .transport = transport;
        }

        if let Ok(port) = std::env::var("SEQUENTIAL_THINKING_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                self.server_config
                    .get_or_insert_with(ServerConfig::default)
                    .port = port_num;
            }
        }

        // Client configuration from environment
        if let Ok(server_url) = std::env::var("SEQUENTIAL_THINKING_SERVER_URL") {
            self.client_config
                .get_or_insert_with(ClientConfig::default)
                .server_url = server_url;
        }

        if let Ok(timeout) = std::env::var("SEQUENTIAL_THINKING_TIMEOUT") {
            if let Ok(timeout_num) = timeout.parse::<u64>() {
                self.client_config
                    .get_or_insert_with(ClientConfig::default)
                    .timeout_seconds = timeout_num;
            }
        }
    }

    /// Get server configuration
    pub fn get_server_config(&self) -> ServerConfig {
        self.server_config.clone().unwrap_or_default()
    }

    /// Get client configuration
    pub fn get_client_config(&self) -> ClientConfig {
        self.client_config.clone().unwrap_or_default()
    }

    /// Set server configuration
    pub fn set_server_config(&mut self, config: ServerConfig) {
        self.server_config = Some(config);
    }

    /// Set client configuration
    pub fn set_client_config(&mut self, config: ClientConfig) {
        self.client_config = Some(config);
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let config = serde_json::json!({
            "server": self.server_config,
            "client": self.client_config
        });

        let content = serde_json::to_string_pretty(&config)?;
        std::fs::write(path, content)?;

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate server configuration
        if let Some(ref server_config) = self.server_config {
            if server_config.name.is_empty() {
                errors.push("Server name cannot be empty".to_string());
            }

            if server_config.port == 0 {
                errors.push("Server port must be greater than 0".to_string());
            }

            if server_config.thinking.max_thoughts_per_session == 0 {
                errors.push("Max thoughts per session must be greater than 0".to_string());
            }
        }

        // Validate client configuration
        if let Some(ref client_config) = self.client_config {
            if client_config.server_url.is_empty() {
                errors.push("Server URL cannot be empty".to_string());
            }

            if client_config.timeout_seconds == 0 {
                errors.push("Timeout must be greater than 0".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration loading utilities
pub mod utils {
    use super::*;

    /// Load configuration from default locations
    pub fn load_default_config() -> Result<ConfigManager, Box<dyn std::error::Error>> {
        let mut manager = ConfigManager::new();

        // Try to load from default config file
        let default_paths = [
            "./config.toml",
            "./config.json",
            "./sequential-thinking.toml",
            "./sequential-thinking.json",
        ];

        for path in &default_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(()) = manager.load_from_file(path) {
                    break;
                }
            }
        }

        // Load from environment variables
        manager.load_from_env();

        // Validate configuration
        let _ = manager.validate();

        Ok(manager)
    }

    /// Create a default configuration file
    pub fn create_default_config<P: AsRef<Path>>(
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = ConfigManager::new();
        manager.set_server_config(ServerConfig::default());
        manager.set_client_config(ClientConfig::default());
        manager.save_to_file(path)?;
        Ok(())
    }

    /// Merge configurations
    pub fn merge_configs(base: &mut ServerConfig, override_config: &ServerConfig) {
        if !override_config.name.is_empty() {
            base.name = override_config.name.clone();
        }
        if !override_config.version.is_empty() {
            base.version = override_config.version.clone();
        }
        if !override_config.transport.is_empty() {
            base.transport = override_config.transport.clone();
        }
        if override_config.port != 0 {
            base.port = override_config.port;
        }
        // Merge other fields as needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.name, "ultrafast-sequential-thinking");
        assert_eq!(config.transport, "stdio");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.server_url, "stdio://");
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.retry_attempts, 3);
    }

    #[test]
    fn test_thinking_config_default() {
        let config = ThinkingConfig::default();
        assert_eq!(config.max_thoughts_per_session, 100);
        assert_eq!(config.max_branches_per_session, 10);
        assert!(config.enable_analytics);
    }

    #[test]
    fn test_config_manager() {
        let mut manager = ConfigManager::new();
        let server_config = ServerConfig {
            name: String::new(),
            ..Default::default()
        };
        manager.set_server_config(server_config);

        let loaded_config = manager.get_server_config();
        assert_eq!(loaded_config.name, "ultrafast-sequential-thinking");
    }

    #[test]
    fn test_config_validation() {
        let mut manager = ConfigManager::new();
        let server_config = ServerConfig {
            name: String::new(),
            ..Default::default()
        };
        manager.set_server_config(server_config);

        let result = manager.validate();
        assert!(result.is_err());
    }
}

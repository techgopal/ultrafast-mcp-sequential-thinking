//! # UltraFast MCP Sequential Thinking
//!
//! High-performance, Rust-based Model Context Protocol (MCP) server and client implementation
//! for sequential thinking, built on the UltraFast MCP framework.
//!
//! This library provides a complete implementation of the sequential thinking concept,
//! offering significant performance advantages over the official TypeScript version
//! while maintaining full compatibility with the MCP 2025-06-18 specification.
//!
//! ## Features
//!
//! - **Dynamic Problem Breakdown**: Break complex problems into manageable steps
//! - **Reflective Thinking**: Revise and refine thoughts as understanding deepens
//! - **Branching Logic**: Explore alternative paths of reasoning
//! - **Adaptive Planning**: Adjust the total number of thoughts dynamically
//! - **Solution Verification**: Generate and verify solution hypotheses
//! - **Context Preservation**: Maintain thinking context across multiple steps
//! - **High Performance**: 10-100x faster than TypeScript implementation
//! - **Type Safety**: Compile-time guarantees for protocol compliance
//!
//! ## Quick Start
//!
//! This crate provides a high-performance, type-safe Rust implementation of the Model Context Protocol (MCP) for sequential thinking. It enables you to break down complex problems into steps, track progress, and manage sessions programmatically.
//!
//! ### Example: Start a Session, Add a Thought, and Complete
//!
//! ```rust,no_run
//! use ultrafast_mcp_sequential_thinking::{SequentialThinkingClient, ThoughtData};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create a client connected to the server
//!     let client = SequentialThinkingClient::new("http://localhost:8080").await?;
//!
//!     // Start a new thinking session
//!     let session = client.start_session("Solve a complex problem".to_string()).await?;
//!
//!     // Add a thought to the session
//!     client.add_thought(&session.session_id, ThoughtData {
//!         thought: "First, I need to understand the problem scope".to_string(),
//!         thought_number: 1,
//!         total_thoughts: 5,
//!         next_thought_needed: true,
//!         ..Default::default()
//!     }).await?;
//!
//!     // Mark the session as complete
//!     client.complete_session(&session.session_id).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! See the crate documentation and module docs for more details and advanced usage.

pub mod analytics;
pub mod config;
pub mod export;
pub mod session;
pub mod thinking;

// Re-export main types for convenience
pub use session::{SessionManager, SessionMetadata, ThinkingSession};
pub use thinking::{ThinkingEngine, ThoughtData, ThoughtProcessor};

// Re-export client and server types
pub use crate::thinking::client::SequentialThinkingClient;
pub use crate::thinking::server::SequentialThinkingServer;

// Re-export error types
pub use crate::thinking::error::{SequentialThinkingError, SequentialThinkingResult};

// Re-export configuration types
pub use crate::config::{ClientConfig, ServerConfig, ThinkingConfig};

// Re-export analytics types
pub use crate::analytics::{AnalyticsEngine, SessionAnalytics};
pub use crate::thinking::ThinkingStats;

// Re-export export types
pub use crate::export::{ExportEngine, ExportFormat, ExportOptions};

/// Result type for sequential thinking operations
pub type Result<T> = std::result::Result<T, SequentialThinkingError>;

/// Default configuration for the sequential thinking server
pub fn default_server_config() -> ServerConfig {
    ServerConfig {
        name: "ultrafast-sequential-thinking".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        transport: "stdio".to_string(),
        port: 8080,
        thinking: ThinkingConfig::default(),
        export: config::ExportConfig::default(),
        analytics: config::AnalyticsConfig::default(),
        logging: config::LoggingConfig::default(),
        security: config::SecurityConfig::default(),
    }
}

/// Default configuration for the sequential thinking client
pub fn default_client_config() -> ClientConfig {
    ClientConfig {
        server_url: "stdio://".to_string(),
        timeout_seconds: 30,
        retry_attempts: 3,
        thinking: thinking::client::ClientThinkingConfig::default(),
        connection: config::ConnectionConfig::default(),
        ui: config::UIConfig::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_configs() {
        let server_config = default_server_config();
        assert_eq!(server_config.name, "ultrafast-sequential-thinking");
        assert_eq!(server_config.transport, "stdio");

        let client_config = default_client_config();
        assert_eq!(client_config.server_url, "stdio://");
        assert_eq!(client_config.timeout_seconds, 30);
    }
}

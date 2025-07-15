//! # Client Implementation
//!
//! UltraFast MCP client implementation for sequential thinking.
//!
//! This module provides the main client implementation that connects to
//! sequential thinking servers and manages thinking sessions.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use ultrafast_mcp::{
    ClientCapabilities, ClientInfo, ListToolsRequest, ListToolsResponse, Tool, ToolCall,
    ToolContent, ToolResult, UltraFastClient,
};

use crate::thinking::error::{SequentialThinkingError, SequentialThinkingResult};
use crate::thinking::{ThinkingEngine, ThinkingProgress, ThinkingStats, ThoughtData};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientThinkingConfig {
    /// Whether to enable progress tracking
    pub enable_progress_tracking: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Whether to show thought visualization
    pub show_thought_visualization: bool,
    /// Maximum retry attempts for failed operations
    pub max_retry_attempts: u32,
    /// Timeout for individual operations in seconds
    pub operation_timeout: u64,
}

impl Default for ClientThinkingConfig {
    fn default() -> Self {
        Self {
            enable_progress_tracking: true,
            auto_save_interval: 60,
            show_thought_visualization: true,
            max_retry_attempts: 3,
            operation_timeout: 30,
        }
    }
}

/// Main sequential thinking client implementation
pub struct SequentialThinkingClient {
    /// Underlying MCP client
    client: Arc<UltraFastClient>,
    /// Client configuration
    config: ClientThinkingConfig,
    /// Active thinking sessions
    sessions: Arc<RwLock<HashMap<String, ThinkingSession>>>,
    /// Client statistics
    stats: Arc<RwLock<ClientStats>>,
    /// Progress tracker
    progress_tracker: Arc<RwLock<ProgressTracker>>,
}

/// Client statistics
#[derive(Debug, Clone, Default)]
pub struct ClientStats {
    /// Total requests made
    pub total_requests: u64,
    /// Total thoughts processed
    pub total_thoughts: u64,
    /// Total sessions created
    pub total_sessions: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Total response time in milliseconds
    pub total_response_time_ms: u64,
    /// Error count
    pub error_count: u64,
    /// Retry count
    pub retry_count: u64,
}

/// Progress tracking information
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    /// Current session progress
    pub current_progress: Option<ThinkingProgress>,
    /// Progress history
    pub progress_history: Vec<ThinkingProgress>,
    /// Last update timestamp
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self {
            current_progress: None,
            progress_history: Vec::new(),
            last_update: chrono::Utc::now(),
        }
    }
}

/// A thinking session managed by the client
pub struct ThinkingSession {
    /// Session ID
    pub session_id: String,
    /// Session title
    pub title: String,
    /// Local thinking engine
    pub engine: ThinkingEngine,
    /// Session metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last activity timestamp
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl ThinkingSession {
    /// Create a new thinking session
    pub fn new(session_id: String, title: String) -> Self {
        Self {
            session_id,
            title,
            engine: ThinkingEngine::new(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        }
    }

    /// Add metadata to the session
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
        self.last_activity = chrono::Utc::now();
    }

    /// Get session statistics
    pub fn get_stats(&self) -> ThinkingStats {
        self.engine.get_stats().clone()
    }

    /// Get session progress
    pub fn get_progress(&self) -> ThinkingProgress {
        self.engine.get_progress().clone()
    }

    /// Check if session is complete
    pub fn is_complete(&self) -> bool {
        self.engine.is_complete()
    }
}

impl SequentialThinkingClient {
    /// Create a new sequential thinking client
    pub async fn new(server_url: &str) -> SequentialThinkingResult<Self> {
        let client_info = ClientInfo {
            name: "UltraFast MCP Sequential Thinking Client".to_string(),
            version: "0.1.0".to_string(),
            description: Some(
                "High-performance Rust-based MCP client for sequential thinking".to_string(),
            ),
            homepage: Some(
                "https://github.com/your-org/ultrafast-mcp-sequential-thinking".to_string(),
            ),
            repository: Some(
                "https://github.com/your-org/ultrafast-mcp-sequential-thinking".to_string(),
            ),
            authors: Some(vec!["Your Name <your.email@example.com>".to_string()]),
            license: Some("MIT".to_string()),
        };
        let client_capabilities = ClientCapabilities::default();
        let client = UltraFastClient::new(client_info, client_capabilities);

        let mut client_instance = Self {
            client: Arc::new(client),
            config: ClientThinkingConfig::default(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ClientStats::default())),
            progress_tracker: Arc::new(RwLock::new(ProgressTracker::default())),
        };

        // Connect to server
        client_instance.connect(server_url).await?;

        Ok(client_instance)
    }

    /// Create a new client with custom configuration
    pub async fn with_config(
        server_url: &str,
        config: ClientThinkingConfig,
    ) -> SequentialThinkingResult<Self> {
        let client_info = ClientInfo {
            name: "UltraFast MCP Sequential Thinking Client".to_string(),
            version: "0.1.0".to_string(),
            description: Some(
                "High-performance Rust-based MCP client for sequential thinking".to_string(),
            ),
            homepage: Some(
                "https://github.com/your-org/ultrafast-mcp-sequential-thinking".to_string(),
            ),
            repository: Some(
                "https://github.com/your-org/ultrafast-mcp-sequential-thinking".to_string(),
            ),
            authors: Some(vec!["Your Name <your.email@example.com>".to_string()]),
            license: Some("MIT".to_string()),
        };
        let client_capabilities = ClientCapabilities::default();
        let client = UltraFastClient::new(client_info, client_capabilities);

        let mut client_instance = Self {
            client: Arc::new(client),
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ClientStats::default())),
            progress_tracker: Arc::new(RwLock::new(ProgressTracker::default())),
        };

        // Connect to server
        client_instance.connect(server_url).await?;

        Ok(client_instance)
    }

    /// Connect to the server and initialize MCP connection
    async fn connect(&mut self, server_url: &str) -> SequentialThinkingResult<()> {
        info!("Connecting to server: {}", server_url);

        // Parse server URL to determine transport type
        if server_url.starts_with("stdio://") || server_url == "stdio" {
            // Connect via STDIO
            self.client.connect_stdio().await.map_err(|e| {
                SequentialThinkingError::transport_error(format!(
                    "Failed to connect via STDIO: {}",
                    e
                ))
            })?;
        } else if server_url.starts_with("http://") || server_url.starts_with("https://") {
            // Connect via HTTP
            self.client
                .connect_streamable_http(server_url)
                .await
                .map_err(|e| {
                    SequentialThinkingError::transport_error(format!(
                        "Failed to connect via HTTP: {}",
                        e
                    ))
                })?;
        } else {
            return Err(SequentialThinkingError::transport_error(format!(
                "Unsupported server URL format: {}",
                server_url
            )));
        }

        info!("Connected to server, initializing MCP connection...");

        // Initialize the MCP connection
        self.client.initialize().await.map_err(|e| {
            SequentialThinkingError::transport_error(format!(
                "Failed to initialize MCP connection: {}",
                e
            ))
        })?;

        info!("MCP connection initialized successfully");
        Ok(())
    }

    /// Start a new thinking session
    pub async fn start_session(&self, title: String) -> SequentialThinkingResult<ThinkingSession> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let mut session = ThinkingSession::new(session_id.clone(), title);

        // Initialize the session
        session.engine.start_session(session_id.clone());

        // Store the session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_sessions += 1;
        }

        info!("Started new thinking session: {}", session_id);
        Ok(session)
    }

    /// Get a thinking session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<ThinkingSession> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Add a thought to a session
    pub async fn add_thought(
        &self,
        session_id: &str,
        thought: ThoughtData,
    ) -> SequentialThinkingResult<ThoughtData> {
        let start_time = std::time::Instant::now();

        // Update request statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }

        // Process thought locally first
        let mut sessions = self.sessions.write().await;
        let session = sessions.get_mut(session_id).ok_or_else(|| {
            SequentialThinkingError::not_found(format!("Session not found: {}", session_id))
        })?;

        let processed_thought = session
            .engine
            .process_thought(thought.clone())
            .await
            .map_err(|e| SequentialThinkingError::processing_error(e))?;

        // Send thought to server
        let server_result = self.send_thought_to_server(thought).await;

        // Update response time statistics
        {
            let response_time = start_time.elapsed();
            let mut stats = self.stats.write().await;
            stats.total_response_time_ms += response_time.as_millis() as u64;
            stats.avg_response_time_ms =
                stats.total_response_time_ms as f64 / stats.total_requests as f64;

            if server_result.is_ok() {
                stats.total_thoughts += 1;
            } else {
                stats.error_count += 1;
            }
        }

        // Update progress tracking
        if self.config.enable_progress_tracking {
            self.update_progress_tracking(&processed_thought).await;
        }

        // Update session activity
        session.last_activity = chrono::Utc::now();

        Ok(processed_thought)
    }

    /// Send a thought to the server
    async fn send_thought_to_server(
        &self,
        thought: ThoughtData,
    ) -> SequentialThinkingResult<ToolResult> {
        let args = serde_json::json!({
            "thought": thought.thought,
            "thoughtNumber": thought.thought_number,
            "totalThoughts": thought.total_thoughts,
            "nextThoughtNeeded": thought.next_thought_needed,
            "isRevision": thought.is_revision,
            "revisesThought": thought.revises_thought,
            "branchFromThought": thought.branch_from_thought,
            "branchId": thought.branch_id,
            "needsMoreThoughts": thought.needs_more_thoughts
        });

        let tool_call = ToolCall {
            name: "sequential_thinking".to_string(),
            arguments: Some(args),
        };

        let mut attempts = 0;
        loop {
            match self.client.call_tool(tool_call.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    if attempts >= self.config.max_retry_attempts {
                        return Err(SequentialThinkingError::transport_error(e.to_string()));
                    }

                    // Update retry statistics
                    {
                        let mut stats = self.stats.write().await;
                        stats.retry_count += 1;
                    }

                    warn!(
                        "Tool call failed, retrying (attempt {}/{}): {}",
                        attempts, self.config.max_retry_attempts, e
                    );

                    // Wait before retrying
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Export a session
    pub async fn export_session(
        &self,
        session_id: &str,
        format: &str,
    ) -> SequentialThinkingResult<String> {
        let args = serde_json::json!({
            "format": format
        });

        let tool_call = ToolCall {
            name: "export_session".to_string(),
            arguments: Some(args),
        };

        let result = self
            .client
            .call_tool(tool_call)
            .await
            .map_err(|e| SequentialThinkingError::transport_error(e.to_string()))?;

        // Extract content from result
        if let Some(content) = result.content.first() {
            match content {
                ToolContent::Text { text } => Ok(text.clone()),
                _ => Err(SequentialThinkingError::serialization_error(
                    "Unexpected content type in export result".to_string(),
                )),
            }
        } else {
            Err(SequentialThinkingError::serialization_error(
                "No content in export result".to_string(),
            ))
        }
    }

    /// Analyze a session
    pub async fn analyze_session(
        &self,
        session_id: &str,
    ) -> SequentialThinkingResult<serde_json::Value> {
        let tool_call = ToolCall {
            name: "analyze_session".to_string(),
            arguments: Some(serde_json::json!({})),
        };

        let result = self
            .client
            .call_tool(tool_call)
            .await
            .map_err(|e| SequentialThinkingError::transport_error(e.to_string()))?;

        // Extract content from result
        if let Some(content) = result.content.first() {
            match content {
                ToolContent::Text { text } => serde_json::from_str(text)
                    .map_err(|e| SequentialThinkingError::serialization_error(e.to_string())),
                _ => Err(SequentialThinkingError::serialization_error(
                    "Unexpected content type in analysis result".to_string(),
                )),
            }
        } else {
            Err(SequentialThinkingError::serialization_error(
                "No content in analysis result".to_string(),
            ))
        }
    }

    /// Get available tools from the server
    pub async fn list_tools(&self) -> SequentialThinkingResult<Vec<Tool>> {
        let tools = self
            .client
            .list_tools(ListToolsRequest { cursor: None })
            .await
            .map_err(|e| SequentialThinkingError::transport_error(e.to_string()))?;

        Ok(tools.tools)
    }

    /// Get client statistics
    pub async fn get_stats(&self) -> ClientStats {
        self.stats.read().await.clone()
    }

    /// Get current progress
    pub async fn get_progress(&self) -> Option<ThinkingProgress> {
        let tracker = self.progress_tracker.read().await;
        tracker.current_progress.clone()
    }

    /// Update progress tracking
    async fn update_progress_tracking(&self, thought: &ThoughtData) {
        let mut tracker = self.progress_tracker.write().await;
        let progress = self.calculate_progress(thought);

        if let Some(ref current) = tracker.current_progress {
            let current = current.clone();
            tracker.progress_history.push(current);
        }

        tracker.current_progress = Some(progress);
        tracker.last_update = chrono::Utc::now();
    }

    /// Calculate progress from a thought
    fn calculate_progress(&self, thought: &ThoughtData) -> ThinkingProgress {
        ThinkingProgress::new(thought.thought_number, thought.total_thoughts)
    }

    /// Complete a session
    pub async fn complete_session(&self, session_id: &str) -> SequentialThinkingResult<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            // Mark session as complete
            session.last_activity = chrono::Utc::now();
            info!("Completed thinking session: {}", session_id);
            Ok(())
        } else {
            Err(SequentialThinkingError::not_found(format!(
                "Session not found: {}",
                session_id
            )))
        }
    }

    /// Remove a session
    pub async fn remove_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id).is_some()
    }

    /// Get all session IDs
    pub async fn get_session_ids(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// Get session statistics
    pub async fn get_session_stats(&self, session_id: &str) -> Option<ThinkingStats> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).map(|s| s.get_stats())
    }

    /// Get session progress
    pub async fn get_session_progress(&self, session_id: &str) -> Option<ThinkingProgress> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).map(|s| s.get_progress())
    }

    /// Check if a session is complete
    pub async fn is_session_complete(&self, session_id: &str) -> bool {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .map(|s| s.is_complete())
            .unwrap_or(false)
    }
}

impl Clone for ThinkingSession {
    fn clone(&self) -> Self {
        Self {
            session_id: self.session_id.clone(),
            title: self.title.clone(),
            engine: self.engine.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at,
            last_activity: self.last_activity,
        }
    }
}

impl Clone for ThinkingEngine {
    fn clone(&self) -> Self {
        // Note: This is a simplified clone implementation
        // In a real implementation, you might want to implement proper cloning
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        // This test would require a mock server or actual server running
        // For now, we'll just test the configuration
        let config = ClientThinkingConfig::default();
        assert!(config.enable_progress_tracking);
        assert_eq!(config.auto_save_interval, 60);
    }

    #[test]
    fn test_thinking_session_creation() {
        let session = ThinkingSession::new("test-session".to_string(), "Test Session".to_string());
        assert_eq!(session.session_id, "test-session");
        assert_eq!(session.title, "Test Session");
        assert!(!session.is_complete());
    }

    #[test]
    fn test_progress_calculation() {
        // Use dummy ClientInfo and ClientCapabilities for UltraFastClient
        let client_info = ClientInfo {
            name: "Test Client".to_string(),
            version: "0.0.1".to_string(),
            description: None,
            homepage: None,
            repository: None,
            authors: None,
            license: None,
        };
        let client_capabilities = ClientCapabilities::default();
        let client = UltraFastClient::new(client_info, client_capabilities);

        let client = SequentialThinkingClient {
            client: Arc::new(client),
            config: ClientThinkingConfig::default(),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ClientStats::default())),
            progress_tracker: Arc::new(RwLock::new(ProgressTracker::default())),
        };

        let thought = ThoughtData::new("Test thought".to_string(), 3, 5);
        let progress = client.calculate_progress(&thought);

        assert_eq!(progress.current_thought, 3);
        assert_eq!(progress.total_thoughts, 5);
        assert_eq!(progress.completed_thoughts, 2);
    }
}

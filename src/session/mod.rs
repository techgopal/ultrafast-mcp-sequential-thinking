//! # Session Management Module
//!
//! Session management functionality for the sequential thinking system.
//!
//! This module provides session creation, management, and persistence
//! capabilities for thinking sessions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::thinking::{ThinkingEngine, ThinkingProgress, ThinkingStats, ThoughtData};

/// Session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Session title
    pub title: String,
    /// Session description
    pub description: Option<String>,
    /// Session tags
    pub tags: Vec<String>,
    /// Session priority
    pub priority: SessionPriority,
    /// Session status
    pub status: SessionStatus,
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub last_modified: chrono::DateTime<chrono::Utc>,
    /// Expires at timestamp
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Custom metadata
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Session priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
    Expired,
}

impl Default for SessionMetadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            tags: Vec::new(),
            priority: SessionPriority::Normal,
            status: SessionStatus::Active,
            created_at: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
            expires_at: None,
            custom_data: HashMap::new(),
        }
    }
}

/// A thinking session
#[derive(Debug, Clone)]
pub struct ThinkingSession {
    /// Session ID
    pub session_id: String,
    /// Session metadata
    pub metadata: SessionMetadata,
    /// Thinking engine
    pub engine: ThinkingEngine,
    #[allow(dead_code)]
    lock: Arc<RwLock<()>>,
}

impl ThinkingSession {
    /// Create a new thinking session
    pub fn new(session_id: String, title: String) -> Self {
        let metadata = SessionMetadata { title, ..Default::default() };

        Self {
            session_id,
            metadata,
            engine: ThinkingEngine::new(),
            lock: Arc::new(RwLock::new(())),
        }
    }

    /// Create a new thinking session with metadata
    pub fn with_metadata(session_id: String, metadata: SessionMetadata) -> Self {
        Self {
            session_id,
            metadata,
            engine: ThinkingEngine::new(),
            lock: Arc::new(RwLock::new(())),
        }
    }

    /// Get session ID
    pub fn id(&self) -> &str {
        &self.session_id
    }

    /// Get session title
    pub fn title(&self) -> &str {
        &self.metadata.title
    }

    /// Get session status
    pub fn status(&self) -> &SessionStatus {
        &self.metadata.status
    }

    /// Set session status
    pub fn set_status(&mut self, status: SessionStatus) {
        self.metadata.status = status;
        self.metadata.last_modified = chrono::Utc::now();
    }

    /// Get session priority
    pub fn priority(&self) -> &SessionPriority {
        &self.metadata.priority
    }

    /// Set session priority
    pub fn set_priority(&mut self, priority: SessionPriority) {
        self.metadata.priority = priority;
        self.metadata.last_modified = chrono::Utc::now();
    }

    /// Add a tag to the session
    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
            self.metadata.last_modified = chrono::Utc::now();
        }
    }

    /// Remove a tag from the session
    pub fn remove_tag(&mut self, tag: &str) {
        self.metadata.tags.retain(|t| t != tag);
        self.metadata.last_modified = chrono::Utc::now();
    }

    /// Set custom metadata
    pub fn set_custom_data(&mut self, key: String, value: serde_json::Value) {
        self.metadata.custom_data.insert(key, value);
        self.metadata.last_modified = chrono::Utc::now();
    }

    /// Get custom metadata
    pub fn get_custom_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.custom_data.get(key)
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.metadata.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Check if session is active
    pub fn is_active(&self) -> bool {
        self.metadata.status == SessionStatus::Active && !self.is_expired()
    }

    /// Get session progress
    pub fn get_progress(&self) -> ThinkingProgress {
        self.engine.get_progress().clone()
    }

    /// Get session statistics
    pub fn get_stats(&self) -> ThinkingStats {
        self.engine.get_stats().clone()
    }

    /// Get all thoughts in the session
    pub fn get_thoughts(&self) -> Vec<ThoughtData> {
        self.engine.get_thoughts().to_vec()
    }

    /// Get session age
    pub fn age(&self) -> chrono::Duration {
        chrono::Utc::now() - self.metadata.created_at
    }

    /// Get session duration
    pub fn duration(&self) -> chrono::Duration {
        self.metadata.last_modified - self.metadata.created_at
    }
}

/// Session manager for handling multiple sessions
pub struct SessionManager {
    /// Active sessions
    sessions: Arc<RwLock<HashMap<String, ThinkingSession>>>,
    /// Session configuration
    config: SessionManagerConfig,
    /// Statistics
    stats: Arc<RwLock<SessionManagerStats>>,
}

/// Session manager configuration
#[derive(Debug, Clone)]
pub struct SessionManagerConfig {
    /// Maximum number of active sessions
    pub max_sessions: usize,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Whether to auto-cleanup expired sessions
    pub auto_cleanup: bool,
    /// Cleanup interval in seconds
    pub cleanup_interval: u64,
    /// Whether to persist sessions
    pub persist_sessions: bool,
    /// Persistence directory
    pub persistence_dir: String,
}

impl Default for SessionManagerConfig {
    fn default() -> Self {
        Self {
            max_sessions: 100,
            session_timeout: 3600,
            auto_cleanup: true,
            cleanup_interval: 300,
            persist_sessions: false,
            persistence_dir: "./sessions".to_string(),
        }
    }
}

/// Session manager statistics
#[derive(Debug, Clone, Default)]
pub struct SessionManagerStats {
    /// Total sessions created
    pub total_sessions_created: u64,
    /// Total sessions completed
    pub total_sessions_completed: u64,
    /// Total sessions cancelled
    pub total_sessions_cancelled: u64,
    /// Total sessions expired
    pub total_sessions_expired: u64,
    /// Current active sessions
    pub active_sessions: usize,
    /// Average session duration in seconds
    pub avg_session_duration: f64,
    /// Total session time in seconds
    pub total_session_time: u64,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: SessionManagerConfig::default(),
            stats: Arc::new(RwLock::new(SessionManagerStats::default())),
        }
    }

    /// Create a new session manager with configuration
    pub fn with_config(config: SessionManagerConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(SessionManagerStats::default())),
        }
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        title: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = Uuid::new_v4().to_string();

        // Check if we've reached the maximum number of sessions
        {
            let sessions = self.sessions.read().await;
            if sessions.len() >= self.config.max_sessions {
                return Err("Maximum number of sessions reached".into());
            }
        }

        let session = ThinkingSession::new(session_id.clone(), title);

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_sessions_created += 1;
            stats.active_sessions += 1;
        }

        Ok(session_id)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<ThinkingSession> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Update a session
    pub async fn update_session(&self, session_id: &str, session: ThinkingSession) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.to_string(), session).is_some()
    }

    /// Remove a session
    pub async fn remove_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        if sessions.remove(session_id).is_some() {
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.active_sessions = stats.active_sessions.saturating_sub(1);
            true
        } else {
            false
        }
    }

    /// List all session IDs
    pub async fn list_session_ids(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// List active sessions
    pub async fn list_active_sessions(&self) -> Vec<ThinkingSession> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|session| session.is_active())
            .cloned()
            .collect()
    }

    /// Get session statistics
    pub async fn get_stats(&self) -> SessionManagerStats {
        self.stats.read().await.clone()
    }

    /// Cleanup expired sessions
    pub async fn cleanup_expired_sessions(&self) -> usize {
        let mut sessions = self.sessions.write().await;
        let mut expired_count = 0;

        let expired_sessions: Vec<String> = sessions
            .iter()
            .filter(|(_, session)| session.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        for session_id in expired_sessions {
            if let Some(session) = sessions.remove(&session_id) {
                // Update statistics based on session status
                let mut stats = self.stats.write().await;
                match session.status() {
                    SessionStatus::Completed => stats.total_sessions_completed += 1,
                    SessionStatus::Cancelled => stats.total_sessions_cancelled += 1,
                    _ => stats.total_sessions_expired += 1,
                }
                stats.active_sessions = stats.active_sessions.saturating_sub(1);
                expired_count += 1;
            }
        }

        expired_count
    }

    /// Start auto-cleanup task
    pub async fn start_auto_cleanup(&self) {
        let sessions = Arc::clone(&self.sessions);
        let config = self.config.clone();
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(config.cleanup_interval));

            loop {
                interval.tick().await;

                let mut sessions_guard = sessions.write().await;
                let mut expired_count = 0;

                let expired_sessions: Vec<String> = sessions_guard
                    .iter()
                    .filter(|(_, session)| session.is_expired())
                    .map(|(id, _)| id.clone())
                    .collect();

                for session_id in expired_sessions {
                    if let Some(session) = sessions_guard.remove(&session_id) {
                        // Update statistics
                        let mut stats_guard = stats.write().await;
                        match session.status() {
                            SessionStatus::Completed => stats_guard.total_sessions_completed += 1,
                            SessionStatus::Cancelled => stats_guard.total_sessions_cancelled += 1,
                            _ => stats_guard.total_sessions_expired += 1,
                        }
                        stats_guard.active_sessions = stats_guard.active_sessions.saturating_sub(1);
                        expired_count += 1;
                    }
                }

                if expired_count > 0 {
                    tracing::info!("Cleaned up {} expired sessions", expired_count);
                }
            }
        });
    }

    /// Persist sessions to disk
    pub async fn persist_sessions(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.persist_sessions {
            return Ok(());
        }

        let sessions = self.sessions.read().await;
        let sessions_data: HashMap<String, serde_json::Value> = sessions
            .iter()
            .map(|(id, session)| {
                let session_data = serde_json::json!({
                    "metadata": session.metadata,
                    "thoughts": session.get_thoughts(),
                    "stats": session.get_stats()
                });
                (id.clone(), session_data)
            })
            .collect();

        let content = serde_json::to_string_pretty(&sessions_data)?;

        // Ensure directory exists
        std::fs::create_dir_all(&self.config.persistence_dir)?;

        let file_path = format!("{}/sessions.json", self.config.persistence_dir);
        std::fs::write(file_path, content)?;

        Ok(())
    }

    /// Load sessions from disk
    pub async fn load_sessions(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.persist_sessions {
            return Ok(());
        }

        let file_path = format!("{}/sessions.json", self.config.persistence_dir);
        if !std::path::Path::new(&file_path).exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(file_path)?;
        let sessions_data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

        let mut sessions = self.sessions.write().await;
        for (id, session_data) in sessions_data {
            // Reconstruct session from persisted data
            // This is a simplified implementation
            let metadata: SessionMetadata = serde_json::from_value(
                session_data
                    .get("metadata")
                    .unwrap_or(&serde_json::Value::Null)
                    .clone(),
            )?;

            let session = ThinkingSession::with_metadata(id.clone(), metadata);
            sessions.insert(id, session);
        }

        Ok(())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = ThinkingSession::new("test-session".to_string(), "Test Session".to_string());
        assert_eq!(session.id(), "test-session");
        assert_eq!(session.title(), "Test Session");
        assert_eq!(session.status(), &SessionStatus::Active);
        assert_eq!(session.priority(), &SessionPriority::Normal);
    }

    #[test]
    fn test_session_metadata() {
        let mut session =
            ThinkingSession::new("test-session".to_string(), "Test Session".to_string());

        session.set_priority(SessionPriority::High);
        assert_eq!(session.priority(), &SessionPriority::High);

        session.add_tag("important".to_string());
        assert!(session.metadata.tags.contains(&"important".to_string()));

        session.set_custom_data("key".to_string(), serde_json::json!("value"));
        assert_eq!(
            session.get_custom_data("key"),
            Some(&serde_json::json!("value"))
        );
    }

    #[tokio::test]
    async fn test_session_manager() {
        let manager = SessionManager::new();

        let session_id = manager
            .create_session("Test Session".to_string())
            .await
            .unwrap();
        assert!(!session_id.is_empty());

        let session = manager.get_session(&session_id).await;
        assert!(session.is_some());

        let session_ids = manager.list_session_ids().await;
        assert_eq!(session_ids.len(), 1);
        assert!(session_ids.contains(&session_id));
    }

    #[tokio::test]
    async fn test_session_cleanup() {
        let manager = SessionManager::new();

        // Create a session
        let session_id = manager
            .create_session("Test Session".to_string())
            .await
            .unwrap();

        // Mark session as expired
        if let Some(mut session) = manager.get_session(&session_id).await {
            session.metadata.expires_at = Some(chrono::Utc::now() - chrono::Duration::hours(1));
            manager.update_session(&session_id, session).await;
        }

        // Cleanup expired sessions
        let expired_count = manager.cleanup_expired_sessions().await;
        assert_eq!(expired_count, 1);

        // Verify session is removed
        let session = manager.get_session(&session_id).await;
        assert!(session.is_none());
    }
}

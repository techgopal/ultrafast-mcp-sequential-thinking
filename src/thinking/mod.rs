//! # Thinking Module
//!
//! Core thinking functionality for the sequential thinking MCP server and client.
//!
//! This module provides the fundamental types and logic for handling sequential
//! thinking processes, including thought data structures, processing logic,
//! and the main thinking engine.

pub mod client;
pub mod error;
pub mod server;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core data structure for a single thought in the sequential thinking process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThoughtData {
    /// The actual thought content
    pub thought: String,
    /// Current thought number in the sequence
    pub thought_number: u32,
    /// Estimated total number of thoughts needed
    pub total_thoughts: u32,
    /// Whether another thought step is needed
    pub next_thought_needed: bool,
    /// Whether this thought revises previous thinking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    /// Which thought is being reconsidered (if this is a revision)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revises_thought: Option<u32>,
    /// Branching point thought number (if this is a branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_thought: Option<u32>,
    /// Branch identifier (if this is a branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Whether more thoughts are needed (if reaching end but realizing more needed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_more_thoughts: Option<bool>,
    /// Timestamp when this thought was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Metadata associated with this thought
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for ThoughtData {
    fn default() -> Self {
        Self {
            thought: String::new(),
            thought_number: 1,
            total_thoughts: 1,
            next_thought_needed: true,
            is_revision: None,
            revises_thought: None,
            branch_from_thought: None,
            branch_id: None,
            needs_more_thoughts: None,
            timestamp: Some(chrono::Utc::now()),
            metadata: None,
        }
    }
}

impl ThoughtData {
    /// Create a new thought with basic information
    pub fn new(thought: String, thought_number: u32, total_thoughts: u32) -> Self {
        Self {
            thought,
            thought_number,
            total_thoughts,
            next_thought_needed: true,
            is_revision: None,
            revises_thought: None,
            branch_from_thought: None,
            branch_id: None,
            needs_more_thoughts: None,
            timestamp: Some(chrono::Utc::now()),
            metadata: None,
        }
    }

    /// Create a revision thought
    pub fn revision(thought: String, thought_number: u32, revises_thought: u32) -> Self {
        Self {
            thought,
            thought_number,
            total_thoughts: thought_number,
            next_thought_needed: true,
            is_revision: Some(true),
            revises_thought: Some(revises_thought),
            branch_from_thought: None,
            branch_id: None,
            needs_more_thoughts: None,
            timestamp: Some(chrono::Utc::now()),
            metadata: None,
        }
    }

    /// Create a branch thought
    pub fn branch(
        thought: String,
        thought_number: u32,
        branch_from_thought: u32,
        branch_id: String,
    ) -> Self {
        Self {
            thought,
            thought_number,
            total_thoughts: thought_number,
            next_thought_needed: true,
            is_revision: None,
            revises_thought: None,
            branch_from_thought: Some(branch_from_thought),
            branch_id: Some(branch_id),
            needs_more_thoughts: None,
            timestamp: Some(chrono::Utc::now()),
            metadata: None,
        }
    }

    /// Check if this thought is a revision
    pub fn is_revision(&self) -> bool {
        self.is_revision.unwrap_or(false)
    }

    /// Check if this thought is a branch
    pub fn is_branch(&self) -> bool {
        self.branch_from_thought.is_some()
    }

    /// Get the branch ID if this is a branch
    pub fn get_branch_id(&self) -> Option<&str> {
        self.branch_id.as_deref()
    }

    /// Get the thought being revised if this is a revision
    pub fn get_revised_thought(&self) -> Option<u32> {
        self.revises_thought
    }

    /// Add metadata to this thought
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        if let Some(ref mut metadata) = self.metadata {
            metadata.insert(key, value);
        }
        self
    }

    /// Validate the thought data
    pub fn validate(&self) -> Result<(), String> {
        if self.thought.is_empty() {
            return Err("Thought content cannot be empty".to_string());
        }
        if self.thought_number == 0 {
            return Err("Thought number must be greater than 0".to_string());
        }
        if self.total_thoughts == 0 {
            return Err("Total thoughts must be greater than 0".to_string());
        }
        if self.thought_number > self.total_thoughts {
            // This is actually allowed for dynamic adjustment
        }
        if self.is_revision() && self.revises_thought.is_none() {
            return Err(
                "Revision thoughts must specify which thought is being revised".to_string(),
            );
        }
        if self.is_branch() && self.branch_id.is_none() {
            return Err("Branch thoughts must have a branch ID".to_string());
        }
        Ok(())
    }
}

/// A collection of thoughts that form a branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtBranch {
    /// Branch identifier
    pub branch_id: String,
    /// Parent thought number
    pub parent_thought: u32,
    /// Thoughts in this branch
    pub thoughts: Vec<ThoughtData>,
    /// Branch metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// When the branch was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ThoughtBranch {
    /// Create a new branch
    pub fn new(branch_id: String, parent_thought: u32) -> Self {
        Self {
            branch_id,
            parent_thought,
            thoughts: Vec::new(),
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    /// Add a thought to this branch
    pub fn add_thought(&mut self, thought: ThoughtData) {
        self.thoughts.push(thought);
    }

    /// Get the number of thoughts in this branch
    pub fn thought_count(&self) -> usize {
        self.thoughts.len()
    }

    /// Get the latest thought in this branch
    pub fn latest_thought(&self) -> Option<&ThoughtData> {
        self.thoughts.last()
    }
}

/// Progress information for a thinking session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingProgress {
    /// Current thought number
    pub current_thought: u32,
    /// Total estimated thoughts
    pub total_thoughts: u32,
    /// Number of completed thoughts
    pub completed_thoughts: u32,
    /// Number of active branches
    pub active_branches: usize,
    /// Whether more thoughts are needed
    pub needs_more_thoughts: bool,
    /// Progress percentage (0.0 to 1.0)
    pub progress_percentage: f64,
    /// Estimated time remaining (if available)
    pub estimated_time_remaining: Option<std::time::Duration>,
}

impl ThinkingProgress {
    /// Create new progress information
    pub fn new(current_thought: u32, total_thoughts: u32) -> Self {
        let completed_thoughts = current_thought.saturating_sub(1);
        let progress_percentage = if total_thoughts > 0 {
            completed_thoughts as f64 / total_thoughts as f64
        } else {
            0.0
        };

        Self {
            current_thought,
            total_thoughts,
            completed_thoughts,
            active_branches: 0,
            needs_more_thoughts: true,
            progress_percentage,
            estimated_time_remaining: None,
        }
    }

    /// Update progress with new thought information
    pub fn update(&mut self, thought: &ThoughtData) {
        self.current_thought = thought.thought_number;
        self.total_thoughts = thought.total_thoughts;
        self.completed_thoughts = thought.thought_number.saturating_sub(1);
        self.needs_more_thoughts = thought.next_thought_needed;

        self.progress_percentage = if self.total_thoughts > 0 {
            self.completed_thoughts as f64 / self.total_thoughts as f64
        } else {
            0.0
        };
    }

    /// Check if the thinking process is complete
    pub fn is_complete(&self) -> bool {
        !self.needs_more_thoughts && self.completed_thoughts >= self.total_thoughts
    }
}

/// Trait for processing thoughts
#[async_trait::async_trait]
pub trait ThoughtProcessor: Send + Sync {
    /// Process a single thought
    async fn process_thought(&self, thought: ThoughtData) -> Result<ThoughtData, String>;

    /// Validate a thought before processing
    async fn validate_thought(&self, thought: &ThoughtData) -> Result<(), String>;

    /// Get processing statistics
    async fn get_stats(&self) -> Result<ThinkingStats, String>;
}

/// Statistics about thinking processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingStats {
    /// Total thoughts processed
    pub total_thoughts: u64,
    /// Total revisions made
    pub total_revisions: u64,
    /// Total branches created
    pub total_branches: u64,
    /// Average processing time per thought
    pub avg_processing_time_ms: f64,
    /// Total processing time
    pub total_processing_time_ms: u64,
}

impl Default for ThinkingStats {
    fn default() -> Self {
        Self {
            total_thoughts: 0,
            total_revisions: 0,
            total_branches: 0,
            avg_processing_time_ms: 0.0,
            total_processing_time_ms: 0,
        }
    }
}

/// Main thinking engine that coordinates the thinking process
#[derive(Debug)]
pub struct ThinkingEngine {
    /// Unique identifier for this engine instance
    id: Uuid,
    /// Current thinking session
    session_id: Option<String>,
    /// Thoughts in the current session
    thoughts: Vec<ThoughtData>,
    /// Active branches
    branches: HashMap<String, ThoughtBranch>,
    /// Thinking progress
    progress: ThinkingProgress,
    /// Processing statistics
    stats: ThinkingStats,
    /// Whether thought logging is disabled
    disable_logging: bool,
}

impl ThinkingEngine {
    /// Create a new thinking engine
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id: None,
            thoughts: Vec::new(),
            branches: HashMap::new(),
            progress: ThinkingProgress::new(1, 1),
            stats: ThinkingStats::default(),
            disable_logging: false,
        }
    }

    /// Create a new thinking engine with logging configuration
    pub fn with_logging(disable_logging: bool) -> Self {
        Self {
            disable_logging,
            ..Self::new()
        }
    }

    /// Start a new thinking session
    pub fn start_session(&mut self, session_id: String) {
        self.session_id = Some(session_id);
        self.thoughts.clear();
        self.branches.clear();
        self.progress = ThinkingProgress::new(1, 1);
        self.stats = ThinkingStats::default();
    }

    /// Process a thought and add it to the session
    pub async fn process_thought(&mut self, thought: ThoughtData) -> Result<ThoughtData, String> {
        let start_time = std::time::Instant::now();

        // Validate the thought
        thought.validate()?;

        // Adjust total thoughts if needed
        let mut processed_thought = thought.clone();
        if processed_thought.thought_number > processed_thought.total_thoughts {
            processed_thought.total_thoughts = processed_thought.thought_number;
        }

        // Add to main thoughts
        self.thoughts.push(processed_thought.clone());

        // Handle branching
        if let (Some(branch_from), Some(branch_id)) = (
            processed_thought.branch_from_thought,
            &processed_thought.branch_id,
        ) {
            let branch = self
                .branches
                .entry(branch_id.clone())
                .or_insert_with(|| ThoughtBranch::new(branch_id.clone(), branch_from));
            branch.add_thought(processed_thought.clone());
        }

        // Update progress
        self.progress.update(&processed_thought);

        // Update statistics
        let processing_time = start_time.elapsed();
        self.stats.total_thoughts += 1;
        self.stats.total_processing_time_ms += processing_time.as_millis() as u64;
        self.stats.avg_processing_time_ms =
            self.stats.total_processing_time_ms as f64 / self.stats.total_thoughts as f64;

        if processed_thought.is_revision() {
            self.stats.total_revisions += 1;
        }
        if processed_thought.is_branch() {
            self.stats.total_branches += 1;
        }

        // Log the thought if logging is enabled
        if !self.disable_logging {
            self.log_thought(&processed_thought);
        }

        Ok(processed_thought)
    }

    /// Get the current thinking progress
    pub fn get_progress(&self) -> &ThinkingProgress {
        &self.progress
    }

    /// Get all thoughts in the current session
    pub fn get_thoughts(&self) -> &[ThoughtData] {
        &self.thoughts
    }

    /// Get all branches in the current session
    pub fn get_branches(&self) -> &HashMap<String, ThoughtBranch> {
        &self.branches
    }

    /// Get thinking statistics
    pub fn get_stats(&self) -> &ThinkingStats {
        &self.stats
    }

    /// Check if the thinking session is complete
    pub fn is_complete(&self) -> bool {
        self.progress.is_complete()
    }

    /// Get the session ID
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Get the engine ID
    pub fn engine_id(&self) -> Uuid {
        self.id
    }

    /// Log a thought to stderr (for compatibility with official implementation)
    fn log_thought(&self, thought: &ThoughtData) {
        let prefix = if thought.is_revision() {
            "🔄 Revision"
        } else if thought.is_branch() {
            "🌿 Branch"
        } else {
            "💭 Thought"
        };

        let context = if thought.is_revision() {
            format!(
                " (revising thought {})",
                thought.revises_thought.unwrap_or(0)
            )
        } else if thought.is_branch() {
            format!(
                " (from thought {}, ID: {})",
                thought.branch_from_thought.unwrap_or(0),
                thought.branch_id.as_deref().unwrap_or("unknown")
            )
        } else {
            String::new()
        };

        let header = format!(
            "{} {}/{}",
            prefix, thought.thought_number, thought.total_thoughts
        );
        let border_length = std::cmp::max(header.len() + context.len(), thought.thought.len()) + 4;
        let border = "─".repeat(border_length);

        eprintln!(
            "\n┌{}┐\n│ {}{} │\n├{}┤\n│ {} │\n└{}┘",
            border, header, context, border, thought.thought, border
        );
    }
}

impl Default for ThinkingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thought_data_creation() {
        let thought = ThoughtData::new("Test thought".to_string(), 1, 5);
        assert_eq!(thought.thought, "Test thought");
        assert_eq!(thought.thought_number, 1);
        assert_eq!(thought.total_thoughts, 5);
        assert!(thought.next_thought_needed);
        assert!(!thought.is_revision());
        assert!(!thought.is_branch());
    }

    #[test]
    fn test_revision_thought() {
        let thought = ThoughtData::revision("Revised thought".to_string(), 3, 1);
        assert!(thought.is_revision());
        assert_eq!(thought.get_revised_thought(), Some(1));
        assert_eq!(thought.revises_thought, Some(1));
    }

    #[test]
    fn test_branch_thought() {
        let thought =
            ThoughtData::branch("Branch thought".to_string(), 4, 2, "branch-1".to_string());
        assert!(thought.is_branch());
        assert_eq!(thought.get_branch_id(), Some("branch-1"));
        assert_eq!(thought.branch_from_thought, Some(2));
    }

    #[test]
    fn test_thought_validation() {
        let valid_thought = ThoughtData::new("Valid thought".to_string(), 1, 5);
        assert!(valid_thought.validate().is_ok());

        let invalid_thought = ThoughtData {
            thought: String::new(),
            thought_number: 1,
            total_thoughts: 5,
            next_thought_needed: true,
            ..Default::default()
        };
        assert!(invalid_thought.validate().is_err());
    }

    #[tokio::test]
    async fn test_thinking_engine() {
        let mut engine = ThinkingEngine::new();
        engine.start_session("test-session".to_string());

        let thought = ThoughtData::new("First thought".to_string(), 1, 3);
        let processed = engine.process_thought(thought).await.unwrap();

        assert_eq!(processed.thought, "First thought");
        assert_eq!(engine.get_thoughts().len(), 1);
        assert!(!engine.is_complete());
    }

    #[test]
    fn test_thinking_progress() {
        let mut progress = ThinkingProgress::new(1, 5);
        assert_eq!(progress.current_thought, 1);
        assert_eq!(progress.total_thoughts, 5);
        assert_eq!(progress.progress_percentage, 0.0);

        let thought = ThoughtData::new("Test".to_string(), 3, 5);
        progress.update(&thought);
        assert_eq!(progress.current_thought, 3);
        assert_eq!(progress.completed_thoughts, 2);
        assert_eq!(progress.progress_percentage, 0.4);
    }
}

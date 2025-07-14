//! # Server Implementation
//!
//! UltraFast MCP server implementation for sequential thinking.
//!
//! This module provides the main server implementation that handles
//! sequential thinking requests through the MCP protocol.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use ultrafast_mcp::{
    UltraFastServer, ToolHandler, ToolCall, ToolResult, ToolContent, Tool, ListToolsRequest, ListToolsResponse,
    ServerInfo, ServerCapabilities, ToolsCapability, MCPError, MCPResult,
};

use crate::thinking::{ThoughtData, ThinkingEngine, ThinkingStats};
use crate::thinking::error::{SequentialThinkingError, SequentialThinkingResult};

#[derive(Debug, Clone)]
pub struct SequentialThinkingServer {
    /// Server information
    info: ServerInfo,
    /// Server capabilities
    capabilities: ServerCapabilities,
    /// Thinking engine
    engine: Arc<RwLock<ThinkingEngine>>,
    /// Session management
    sessions: Arc<RwLock<HashMap<String, ThinkingEngine>>>,
    /// Server statistics
    stats: Arc<RwLock<ServerStats>>,
}

/// Server statistics
#[derive(Debug, Clone, Default)]
pub struct ServerStats {
    /// Total requests processed
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
}

impl SequentialThinkingServer {
    /// Create a new sequential thinking server
    pub fn new() -> Self {
        let disable_logging = std::env::var("DISABLE_THOUGHT_LOGGING")
            .unwrap_or_default()
            .to_lowercase()
            == "true";

        Self {
            info: ServerInfo {
                name: "ultrafast-sequential-thinking".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: Some(
                    "High-performance Rust-based MCP server for sequential thinking".to_string(),
                ),
                homepage: Some(
                    "https://github.com/your-org/ultrafast-mcp-sequential-thinking".to_string(),
                ),
                repository: Some(
                    "https://github.com/your-org/ultrafast-mcp-sequential-thinking".to_string(),
                ),
                authors: Some(vec!["Your Name <your.email@example.com>".to_string()]),
                license: Some("MIT".to_string()),
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                resources: None,
                prompts: None,
                logging: None,
                completion: None,
            },
            engine: Arc::new(RwLock::new(ThinkingEngine::with_logging(disable_logging))),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ServerStats::default())),
        }
    }

    /// Create a new server with custom configuration
    pub fn with_config(
        info: ServerInfo,
        capabilities: ServerCapabilities,
        disable_logging: bool,
    ) -> Self {
        Self {
            info,
            capabilities,
            engine: Arc::new(RwLock::new(ThinkingEngine::with_logging(disable_logging))),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ServerStats::default())),
        }
    }

    /// Get server information
    pub fn info(&self) -> &ServerInfo {
        &self.info
    }

    /// Get server capabilities
    pub fn capabilities(&self) -> &ServerCapabilities {
        &self.capabilities
    }

    /// Get server statistics
    pub async fn get_stats(&self) -> ServerStats {
        self.stats.read().await.clone()
    }

    /// Create an UltraFast MCP server instance
    pub fn create_mcp_server(self) -> UltraFastServer {
        let info = self.info.clone();
        let capabilities = self.capabilities.clone();
        let tool_handler = Arc::new(SequentialThinkingToolHandler {
            server: Arc::new(self),
        });

        UltraFastServer::new(info, capabilities)
            .with_tool_handler(tool_handler)
    }

    /// Process a thought using the main engine
    pub async fn process_thought(&self, thought: ThoughtData) -> SequentialThinkingResult<ThoughtData> {
        let start_time = std::time::Instant::now();
        
        // Update request statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }

        // Process the thought
        let result = {
            let mut engine = self.engine.write().await;
            engine.process_thought(thought).await
        };

        // Update response time statistics
        {
            let response_time = start_time.elapsed();
            let mut stats = self.stats.write().await;
            stats.total_response_time_ms += response_time.as_millis() as u64;
            stats.avg_response_time_ms = 
                stats.total_response_time_ms as f64 / stats.total_requests as f64;
            
            if result.is_ok() {
                stats.total_thoughts += 1;
            } else {
                stats.error_count += 1;
            }
        }

        result.map_err(|e| SequentialThinkingError::processing_error(e))
    }

    /// Create a new thinking session
    pub async fn create_session(&self, session_id: String) -> SequentialThinkingResult<()> {
        let mut sessions = self.sessions.write().await;
        let engine = ThinkingEngine::new();
        sessions.insert(session_id.clone(), engine);
        
        {
            let mut stats = self.stats.write().await;
            stats.total_sessions += 1;
        }
        
        info!("Created new thinking session: {}", session_id);
        Ok(())
    }

    /// Get a thinking session
    pub async fn get_session(&self, session_id: &str) -> Option<ThinkingEngine> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Remove a thinking session
    pub async fn remove_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id).is_some()
    }

    /// Get all active session IDs
    pub async fn get_session_ids(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }
}

impl Default for SequentialThinkingServer {
    fn default() -> Self {
        Self::new()
    }
}

/// Tool handler for the sequential thinking server
struct SequentialThinkingToolHandler {
    server: Arc<SequentialThinkingServer>,
}

#[async_trait::async_trait]
impl ToolHandler for SequentialThinkingToolHandler {
    async fn handle_tool_call(&self, call: ToolCall) -> MCPResult<ToolResult> {
        match call.name.as_str() {
            "sequential_thinking" => self.handle_sequential_thinking(call).await,
            "export_session" => self.handle_export_session(call).await,
            "analyze_session" => self.handle_analyze_session(call).await,
            "merge_sessions" => self.handle_merge_sessions(call).await,
            _ => Err(MCPError::method_not_found(format!("Unknown tool: {}", call.name))),
        }
    }

    async fn list_tools(&self, _request: ListToolsRequest) -> MCPResult<ListToolsResponse> {
        let tools = vec![
            create_sequential_thinking_tool(),
            create_export_session_tool(),
            create_analyze_session_tool(),
            create_merge_sessions_tool(),
        ];

        Ok(ListToolsResponse { tools, next_cursor: None })
    }
}

impl SequentialThinkingToolHandler {
    /// Handle the main sequential thinking tool
    async fn handle_sequential_thinking(&self, call: ToolCall) -> MCPResult<ToolResult> {
        let start_time = std::time::Instant::now();
        
        // Extract and validate arguments
        let args = call.arguments.ok_or_else(|| {
            MCPError::invalid_params("Missing arguments for sequential_thinking".to_string())
        })?;

        let thought_data = self.extract_thought_data(&args)?;
        
        // Process the thought
        let processed_thought = self.server.process_thought(thought_data).await
            .map_err(|e| MCPError::internal_error(e.to_string()))?;

        // Get current progress and statistics
        let engine = self.server.engine.read().await;
        let progress = engine.get_progress();
        let stats = engine.get_stats();
        let branches = engine.get_branches();

        // Create response content
        let response_data = serde_json::json!({
            "thoughtNumber": processed_thought.thought_number,
            "totalThoughts": processed_thought.total_thoughts,
            "nextThoughtNeeded": processed_thought.next_thought_needed,
            "branches": branches.keys().collect::<Vec<_>>(),
            "thoughtHistoryLength": engine.get_thoughts().len(),
            "progress": {
                "currentThought": progress.current_thought,
                "totalThoughts": progress.total_thoughts,
                "completedThoughts": progress.completed_thoughts,
                "progressPercentage": progress.progress_percentage,
                "isComplete": progress.is_complete()
            },
            "stats": {
                "totalThoughts": stats.total_thoughts,
                "totalRevisions": stats.total_revisions,
                "totalBranches": stats.total_branches,
                "avgProcessingTimeMs": stats.avg_processing_time_ms
            },
            "processingTimeMs": start_time.elapsed().as_millis()
        });

        Ok(ToolResult {
            content: vec![ToolContent::text(serde_json::to_string_pretty(&response_data).unwrap())],
            is_error: Some(false),
        })
    }

    /// Handle session export
    async fn handle_export_session(&self, call: ToolCall) -> MCPResult<ToolResult> {
        let args = call.arguments.ok_or_else(|| {
            MCPError::invalid_params("Missing arguments for export_session".to_string())
        })?;

        let format = args.get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("json");

        let engine = self.server.engine.read().await;
        let thoughts = engine.get_thoughts();
        let branches = engine.get_branches();
        let stats = engine.get_stats();

        let export_data = serde_json::json!({
            "session": {
                "sessionId": engine.session_id(),
                "thoughts": thoughts,
                "branches": branches,
                "stats": stats,
                "exportedAt": chrono::Utc::now()
            },
            "format": format
        });

        let content = match format {
            "json" => serde_json::to_string_pretty(&export_data).unwrap(),
            "markdown" => self.export_to_markdown(&export_data),
            _ => return Err(MCPError::invalid_params(format!("Unsupported format: {}", format))),
        };

        Ok(ToolResult {
            content: vec![ToolContent::text(content)],
            is_error: Some(false),
        })
    }

    /// Handle session analysis
    async fn handle_analyze_session(&self, _call: ToolCall) -> MCPResult<ToolResult> {
        let engine = self.server.engine.read().await;
        let thoughts = engine.get_thoughts();
        let branches = engine.get_branches();
        let stats = engine.get_stats();

        // Perform analysis
        let analysis = self.analyze_thinking_session(thoughts, branches, stats);

        Ok(ToolResult {
            content: vec![ToolContent::text(serde_json::to_string_pretty(&analysis).unwrap())],
            is_error: Some(false),
        })
    }

    /// Handle session merging
    async fn handle_merge_sessions(&self, call: ToolCall) -> MCPResult<ToolResult> {
        let args = call.arguments.ok_or_else(|| {
            MCPError::invalid_params("Missing arguments for merge_sessions".to_string())
        })?;

        let session_ids = args.get("sessionIds")
            .and_then(|v| v.as_array())
            .ok_or_else(|| MCPError::invalid_params("Missing sessionIds array".to_string()))?;

        let mut merged_thoughts = Vec::new();
        let mut merged_stats = ThinkingStats::default();

        for session_id in session_ids {
            if let Some(session_id_str) = session_id.as_str() {
                if let Some(session) = self.server.get_session(session_id_str).await {
                    merged_thoughts.extend(session.get_thoughts().to_vec());
                    let session_stats = session.get_stats();
                    merged_stats.total_thoughts += session_stats.total_thoughts;
                    merged_stats.total_revisions += session_stats.total_revisions;
                    merged_stats.total_branches += session_stats.total_branches;
                }
            }
        }

        let merge_result = serde_json::json!({
            "mergedThoughts": merged_thoughts.len(),
            "mergedStats": merged_stats,
            "sessionIds": session_ids
        });

        Ok(ToolResult {
            content: vec![ToolContent::text(serde_json::to_string_pretty(&merge_result).unwrap())],
            is_error: Some(false),
        })
    }

    /// Extract thought data from tool call arguments
    fn extract_thought_data(&self, args: &serde_json::Value) -> MCPResult<ThoughtData> {
        let thought = args.get("thought")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MCPError::invalid_params("Missing 'thought' field".to_string()))?
            .to_string();

        let thought_number = args.get("thoughtNumber")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| MCPError::invalid_params("Missing 'thoughtNumber' field".to_string()))?
            as u32;

        let total_thoughts = args.get("totalThoughts")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| MCPError::invalid_params("Missing 'totalThoughts' field".to_string()))?
            as u32;

        let next_thought_needed = args.get("nextThoughtNeeded")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let is_revision = args.get("isRevision").and_then(|v| v.as_bool());
        let revises_thought = args.get("revisesThought").and_then(|v| v.as_u64()).map(|v| v as u32);
        let branch_from_thought = args.get("branchFromThought").and_then(|v| v.as_u64()).map(|v| v as u32);
        let branch_id = args.get("branchId").and_then(|v| v.as_str()).map(|s| s.to_string());
        let needs_more_thoughts = args.get("needsMoreThoughts").and_then(|v| v.as_bool());

        Ok(ThoughtData {
            thought,
            thought_number,
            total_thoughts,
            next_thought_needed,
            is_revision,
            revises_thought,
            branch_from_thought,
            branch_id,
            needs_more_thoughts,
            timestamp: Some(chrono::Utc::now()),
            metadata: None,
        })
    }

    /// Export session data to Markdown format
    fn export_to_markdown(&self, data: &serde_json::Value) -> String {
        let session = &data["session"];
        let thoughts = &session["thoughts"];
        
        let mut markdown = String::new();
        markdown.push_str("# Sequential Thinking Session\n\n");
        
        if let Some(session_id) = session["sessionId"].as_str() {
            markdown.push_str(&format!("**Session ID:** {}\n\n", session_id));
        }
        
        markdown.push_str("## Thoughts\n\n");
        
        if let Some(thoughts_array) = thoughts.as_array() {
            for (i, thought) in thoughts_array.iter().enumerate() {
                let thought_number = thought["thoughtNumber"].as_u64().unwrap_or(0);
                let total_thoughts = thought["totalThoughts"].as_u64().unwrap_or(0);
                let thought_content = thought["thought"].as_str().unwrap_or("");
                
                markdown.push_str(&format!("### Thought {}/{}\n\n", thought_number, total_thoughts));
                markdown.push_str(&format!("{}\n\n", thought_content));
                
                if thought["isRevision"].as_bool().unwrap_or(false) {
                    markdown.push_str("*This thought revises a previous thought*\n\n");
                }
                
                if thought["branchFromThought"].is_number() {
                    markdown.push_str("*This thought is a branch*\n\n");
                }
            }
        }
        
        markdown.push_str("## Statistics\n\n");
        if let Some(stats) = session.get("stats") {
            markdown.push_str(&format!("- Total Thoughts: {}\n", stats["totalThoughts"]));
            markdown.push_str(&format!("- Total Revisions: {}\n", stats["totalRevisions"]));
            markdown.push_str(&format!("- Total Branches: {}\n", stats["totalBranches"]));
            markdown.push_str(&format!("- Average Processing Time: {:.2}ms\n", stats["avgProcessingTimeMs"]));
        }
        
        markdown
    }

    /// Analyze thinking session
    fn analyze_thinking_session(
        &self,
        thoughts: &[ThoughtData],
        branches: &std::collections::HashMap<String, crate::thinking::ThoughtBranch>,
        stats: &ThinkingStats,
    ) -> serde_json::Value {
        let total_thoughts = thoughts.len();
        let revisions = thoughts.iter().filter(|t| t.is_revision()).count();
        let branch_thoughts = thoughts.iter().filter(|t| t.is_branch()).count();
        
        let avg_thought_length = if total_thoughts > 0 {
            thoughts.iter().map(|t| t.thought.len()).sum::<usize>() as f64 / total_thoughts as f64
        } else {
            0.0
        };

        serde_json::json!({
            "analysis": {
                "totalThoughts": total_thoughts,
                "revisions": revisions,
                "branchThoughts": branch_thoughts,
                "activeBranches": branches.len(),
                "avgThoughtLength": avg_thought_length,
                "revisionRate": if total_thoughts > 0 { revisions as f64 / total_thoughts as f64 } else { 0.0 },
                "branchRate": if total_thoughts > 0 { branch_thoughts as f64 / total_thoughts as f64 } else { 0.0 },
                "processingStats": stats
            }
        })
    }
}

/// Create the main sequential thinking tool definition
fn create_sequential_thinking_tool() -> Tool {
    Tool {
        name: "sequential_thinking".to_string(),
        description: "A detailed tool for dynamic and reflective problem-solving through thoughts.
This tool helps analyze problems through a flexible thinking process that can adapt and evolve.
Each thought can build on, question, or revise previous insights as understanding deepens.

When to use this tool:
- Breaking down complex problems into steps
- Planning and design with room for revision
- Analysis that might need course correction
- Problems where the full scope might not be clear initially
- Problems that require a multi-step solution
- Tasks that need to maintain context over multiple steps
- Situations where irrelevant information needs to be filtered out

Key features:
- You can adjust total_thoughts up or down as you progress
- You can question or revise previous thoughts
- You can add more thoughts even after reaching what seemed like the end
- You can express uncertainty and explore alternative approaches
- Not every thought needs to build linearly - you can branch or backtrack
- Generates a solution hypothesis
- Verifies the hypothesis based on the Chain of Thought steps
- Repeats the process until satisfied
- Provides a correct answer".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "thought": {
                    "type": "string",
                    "description": "Your current thinking step"
                },
                "nextThoughtNeeded": {
                    "type": "boolean",
                    "description": "Whether another thought step is needed"
                },
                "thoughtNumber": {
                    "type": "integer",
                    "description": "Current thought number",
                    "minimum": 1
                },
                "totalThoughts": {
                    "type": "integer",
                    "description": "Estimated total thoughts needed",
                    "minimum": 1
                },
                "isRevision": {
                    "type": "boolean",
                    "description": "Whether this revises previous thinking"
                },
                "revisesThought": {
                    "type": "integer",
                    "description": "Which thought is being reconsidered",
                    "minimum": 1
                },
                "branchFromThought": {
                    "type": "integer",
                    "description": "Branching point thought number",
                    "minimum": 1
                },
                "branchId": {
                    "type": "string",
                    "description": "Branch identifier"
                },
                "needsMoreThoughts": {
                    "type": "boolean",
                    "description": "If more thoughts are needed"
                }
            },
            "required": ["thought", "nextThoughtNeeded", "thoughtNumber", "totalThoughts"]
        }),
        annotations: None,
        output_schema: None,
    }
}

/// Create the export session tool definition
fn create_export_session_tool() -> Tool {
    Tool {
        name: "export_session".to_string(),
        description: "Export the current thinking session in various formats".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "format": {
                    "type": "string",
                    "enum": ["json", "markdown"],
                    "description": "Export format",
                    "default": "json"
                }
            }
        }),
        annotations: None,
        output_schema: None,
    }
}

/// Create the analyze session tool definition
fn create_analyze_session_tool() -> Tool {
    Tool {
        name: "analyze_session".to_string(),
        description: "Analyze the current thinking session and provide insights".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {}
        }),
        annotations: None,
        output_schema: None,
    }
}

/// Create the merge sessions tool definition
fn create_merge_sessions_tool() -> Tool {
    Tool {
        name: "merge_sessions".to_string(),
        description: "Merge multiple thinking sessions into one".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "sessionIds": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Array of session IDs to merge"
                }
            },
            "required": ["sessionIds"]
        }),
        annotations: None,
        output_schema: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = SequentialThinkingServer::new();
        assert_eq!(server.info().name, "ultrafast-sequential-thinking");
        assert!(server.capabilities().tools.is_some());
    }

    #[test]
    fn test_tool_definitions() {
        let sequential_tool = create_sequential_thinking_tool();
        assert_eq!(sequential_tool.name, "sequential_thinking");
        let export_tool = create_export_session_tool();
        assert_eq!(export_tool.name, "export_session");
    }

    #[tokio::test]
    async fn test_thought_processing() {
        let server = SequentialThinkingServer::new();
        let thought = ThoughtData::new("Test thought".to_string(), 1, 3);
        
        let result = server.process_thought(thought).await;
        assert!(result.is_ok());
        
        let stats = server.get_stats().await;
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.total_thoughts, 1);
    }
} 
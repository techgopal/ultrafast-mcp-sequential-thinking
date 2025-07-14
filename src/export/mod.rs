//! # Export Module
//!
//! Export functionality for the sequential thinking system.
//!
//! This module provides comprehensive export capabilities for thinking
//! sessions in various formats including JSON, Markdown, and PDF.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

use crate::thinking::{ThoughtData, ThinkingStats, ThinkingProgress};
use crate::session::SessionMetadata;

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
    /// Whether to include analytics
    pub include_analytics: bool,
    /// Export compression
    pub compression: bool,
    /// Export encryption
    pub encryption: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            formats: vec!["json".to_string(), "markdown".to_string(), "pdf".to_string()],
            auto_export: false,
            export_directory: "./exports".to_string(),
            filename_template: "session_{session_id}_{timestamp}".to_string(),
            include_metadata: true,
            include_statistics: true,
            include_analytics: false,
            compression: false,
            encryption: false,
        }
    }
}

/// Export format enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Markdown,
    Pdf,
    Html,
    Csv,
    Yaml,
    Toml,
}

impl ExportFormat {
    /// Get file extension for the format
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json => "json",
            ExportFormat::Markdown => "md",
            ExportFormat::Pdf => "pdf",
            ExportFormat::Html => "html",
            ExportFormat::Csv => "csv",
            ExportFormat::Yaml => "yml",
            ExportFormat::Toml => "toml",
        }
    }

    /// Get MIME type for the format
    pub fn mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Json => "application/json",
            ExportFormat::Markdown => "text/markdown",
            ExportFormat::Pdf => "application/pdf",
            ExportFormat::Html => "text/html",
            ExportFormat::Csv => "text/csv",
            ExportFormat::Yaml => "application/x-yaml",
            ExportFormat::Toml => "application/toml",
        }
    }
}

/// Export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Export format
    pub format: ExportFormat,
    /// Whether to include metadata
    pub include_metadata: bool,
    /// Whether to include statistics
    pub include_statistics: bool,
    /// Whether to include analytics
    pub include_analytics: bool,
    /// Whether to include progress information
    pub include_progress: bool,
    /// Whether to include branches
    pub include_branches: bool,
    /// Whether to include timestamps
    pub include_timestamps: bool,
    /// Whether to pretty print
    pub pretty_print: bool,
    /// Custom styling for HTML/PDF
    pub custom_styling: Option<String>,
    /// Export template
    pub template: Option<String>,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json,
            include_metadata: true,
            include_statistics: true,
            include_analytics: false,
            include_progress: true,
            include_branches: true,
            include_timestamps: true,
            pretty_print: true,
            custom_styling: None,
            template: None,
        }
    }
}

/// Export data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    /// Session information
    pub session: SessionExportData,
    /// Export metadata
    pub export_metadata: ExportMetadata,
    /// Custom data
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Session export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionExportData {
    /// Session ID
    pub session_id: String,
    /// Session metadata
    pub metadata: Option<SessionMetadata>,
    /// Thoughts
    pub thoughts: Vec<ThoughtData>,
    /// Statistics
    pub statistics: Option<ThinkingStats>,
    /// Progress
    pub progress: Option<ThinkingProgress>,
    /// Branches
    pub branches: HashMap<String, Vec<ThoughtData>>,
    /// Analytics
    pub analytics: Option<serde_json::Value>,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
    /// Export format
    pub format: String,
    /// Export version
    pub version: String,
    /// Export tool
    pub tool: String,
    /// Export options
    pub options: ExportOptions,
}

/// Export engine for handling session exports
pub struct ExportEngine {
    /// Configuration
    config: ExportConfig,
    /// Export templates
    templates: HashMap<String, String>,
    /// Export history
    export_history: Vec<ExportRecord>,
}

/// Export record for tracking export history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRecord {
    /// Session ID
    pub session_id: String,
    /// Export format
    pub format: ExportFormat,
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
    /// Export file path
    pub file_path: Option<PathBuf>,
    /// Export size in bytes
    pub file_size: Option<u64>,
    /// Export success
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
}

impl ExportEngine {
    /// Create a new export engine
    pub fn new() -> Self {
        Self {
            config: ExportConfig::default(),
            templates: HashMap::new(),
            export_history: Vec::new(),
        }
    }

    /// Create a new export engine with configuration
    pub fn with_config(config: ExportConfig) -> Self {
        Self {
            config,
            templates: HashMap::new(),
            export_history: Vec::new(),
        }
    }

    /// Export a session
    pub async fn export_session(
        &mut self,
        session_id: &str,
        session_metadata: Option<&SessionMetadata>,
        thoughts: &[ThoughtData],
        stats: Option<&ThinkingStats>,
        progress: Option<&ThinkingProgress>,
        branches: Option<&HashMap<String, Vec<ThoughtData>>>,
        analytics: Option<&serde_json::Value>,
        options: ExportOptions,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        
        // Prepare export data
        let export_data = self.prepare_export_data(
            session_id,
            session_metadata,
            thoughts,
            stats,
            progress,
            branches,
            analytics,
            &options,
        )?;
        
        // Generate filename
        let filename = self.generate_filename(session_id, &options.format)?;
        let file_path = PathBuf::from(&self.config.export_directory).join(&filename);
        
        // Ensure export directory exists
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Export based on format
        let content = match options.format {
            ExportFormat::Json => self.export_to_json(&export_data, &options)?,
            ExportFormat::Markdown => self.export_to_markdown(&export_data, &options)?,
            ExportFormat::Html => self.export_to_html(&export_data, &options)?,
            ExportFormat::Csv => self.export_to_csv(&export_data, &options)?,
            ExportFormat::Yaml => self.export_to_yaml(&export_data, &options)?,
            ExportFormat::Toml => self.export_to_toml(&export_data, &options)?,
            ExportFormat::Pdf => self.export_to_pdf(&export_data, &options)?,
        };
        
        // Write to file
        std::fs::write(&file_path, content)?;
        
        // Record export
        let file_size = std::fs::metadata(&file_path).ok().map(|m| m.len());
        let export_record = ExportRecord {
            session_id: session_id.to_string(),
            format: options.format,
            exported_at: Utc::now(),
            file_path: Some(file_path.clone()),
            file_size,
            success: true,
            error_message: None,
        };
        self.export_history.push(export_record);
        
        let duration = start_time.elapsed();
        tracing::info!(
            "Exported session {} to {} in {:?}",
            session_id,
            file_path.display(),
            duration
        );
        
        Ok(file_path)
    }

    /// Prepare export data
    fn prepare_export_data(
        &self,
        session_id: &str,
        session_metadata: Option<&SessionMetadata>,
        thoughts: &[ThoughtData],
        stats: Option<&ThinkingStats>,
        progress: Option<&ThinkingProgress>,
        branches: Option<&HashMap<String, Vec<ThoughtData>>>,
        analytics: Option<&serde_json::Value>,
        options: &ExportOptions,
    ) -> Result<ExportData, Box<dyn std::error::Error>> {
        let session_data = SessionExportData {
            session_id: session_id.to_string(),
            metadata: if options.include_metadata {
                session_metadata.cloned()
            } else {
                None
            },
            thoughts: thoughts.to_vec(),
            statistics: if options.include_statistics {
                stats.cloned()
            } else {
                None
            },
            progress: if options.include_progress {
                progress.cloned()
            } else {
                None
            },
            branches: if options.include_branches {
                branches.cloned().unwrap_or_default()
            } else {
                HashMap::new()
            },
            analytics: if options.include_analytics {
                analytics.cloned()
            } else {
                None
            },
        };
        
        let export_metadata = ExportMetadata {
            exported_at: Utc::now(),
            format: options.format.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tool: "ultrafast-mcp-sequential-thinking".to_string(),
            options: options.clone(),
        };
        
        Ok(ExportData {
            session: session_data,
            export_metadata,
            custom_data: HashMap::new(),
        })
    }

    /// Generate filename
    fn generate_filename(&self, session_id: &str, format: &ExportFormat) -> Result<String, Box<dyn std::error::Error>> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let extension = format.extension();
        
        let filename = self.config.filename_template
            .replace("{session_id}", session_id)
            .replace("{timestamp}", &timestamp.to_string())
            .replace("{date}", &Utc::now().format("%Y%m%d").to_string())
            .replace("{time}", &Utc::now().format("%H%M%S").to_string());
        
        Ok(format!("{}.{}", filename, extension))
    }

    /// Export to JSON format
    fn export_to_json(&self, data: &ExportData, options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        if options.pretty_print {
            Ok(serde_json::to_string_pretty(data)?)
        } else {
            Ok(serde_json::to_string(data)?)
        }
    }

    /// Export to Markdown format
    fn export_to_markdown(&self, data: &ExportData, _options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        let mut markdown = String::new();
        
        // Header
        markdown.push_str("# Sequential Thinking Session\n\n");
        
        // Session information
        markdown.push_str(&format!("**Session ID:** {}\n\n", data.session.session_id));
        
        if let Some(ref metadata) = data.session.metadata {
            markdown.push_str(&format!("**Title:** {}\n", metadata.title));
            if let Some(ref description) = metadata.description {
                markdown.push_str(&format!("**Description:** {}\n", description));
            }
            markdown.push_str(&format!("**Status:** {:?}\n", metadata.status));
            markdown.push_str(&format!("**Priority:** {:?}\n", metadata.priority));
            markdown.push_str(&format!("**Created:** {}\n", metadata.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
            markdown.push_str(&format!("**Modified:** {}\n", metadata.last_modified.format("%Y-%m-%d %H:%M:%S UTC")));
            markdown.push_str("\n");
        }
        
        // Statistics
        if let Some(ref stats) = data.session.statistics {
            markdown.push_str("## Statistics\n\n");
            markdown.push_str(&format!("- **Total Thoughts:** {}\n", stats.total_thoughts));
            markdown.push_str(&format!("- **Total Revisions:** {}\n", stats.total_revisions));
            markdown.push_str(&format!("- **Total Branches:** {}\n", stats.total_branches));
            markdown.push_str(&format!("- **Average Processing Time:** {:.2}ms\n", stats.avg_processing_time_ms));
            markdown.push_str(&format!("- **Total Processing Time:** {}ms\n", stats.total_processing_time_ms));
            markdown.push_str("\n");
        }
        
        // Progress
        if let Some(ref progress) = data.session.progress {
            markdown.push_str("## Progress\n\n");
            markdown.push_str(&format!("- **Current Thought:** {}/{}\n", progress.current_thought, progress.total_thoughts));
            markdown.push_str(&format!("- **Completed Thoughts:** {}\n", progress.completed_thoughts));
            markdown.push_str(&format!("- **Progress:** {:.1}%\n", progress.progress_percentage * 100.0));
            markdown.push_str(&format!("- **Status:** {}\n", if progress.is_complete() { "Complete" } else { "In Progress" }));
            markdown.push_str("\n");
        }
        
        // Thoughts
        markdown.push_str("## Thoughts\n\n");
        for (i, thought) in data.session.thoughts.iter().enumerate() {
            let thought_number = i + 1;
            let prefix = if thought.is_revision() {
                "🔄 Revision"
            } else if thought.is_branch() {
                "🌿 Branch"
            } else {
                "💭 Thought"
            };
            
            markdown.push_str(&format!("### {} {}/{}\n\n", prefix, thought.thought_number, thought.total_thoughts));
            
            if let Some(timestamp) = thought.timestamp {
                markdown.push_str(&format!("*{}*\n\n", timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
            }
            
            markdown.push_str(&format!("{}\n\n", thought.thought));
            
            if thought.is_revision() {
                if let Some(revises_thought) = thought.revises_thought {
                    markdown.push_str(&format!("*Revises thought {}*\n\n", revises_thought));
                }
            }
            
            if thought.is_branch() {
                if let Some(branch_id) = &thought.branch_id {
                    markdown.push_str(&format!("*Branch ID: {}*\n\n", branch_id));
                }
            }
        }
        
        // Branches
        if !data.session.branches.is_empty() {
            markdown.push_str("## Branches\n\n");
            for (branch_id, branch_thoughts) in &data.session.branches {
                markdown.push_str(&format!("### Branch: {}\n\n", branch_id));
                for thought in branch_thoughts {
                    markdown.push_str(&format!("- {}\n", thought.thought));
                }
                markdown.push_str("\n");
            }
        }
        
        // Analytics
        if let Some(ref analytics) = data.session.analytics {
            markdown.push_str("## Analytics\n\n");
            markdown.push_str("```json\n");
            markdown.push_str(&serde_json::to_string_pretty(analytics)?);
            markdown.push_str("\n```\n\n");
        }
        
        // Footer
        markdown.push_str("---\n\n");
        markdown.push_str(&format!("*Exported on {} using UltraFast MCP Sequential Thinking*\n", 
                                  data.export_metadata.exported_at.format("%Y-%m-%d %H:%M:%S UTC")));
        
        Ok(markdown)
    }

    /// Export to HTML format
    fn export_to_html(&self, data: &ExportData, options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        let mut html = String::new();
        
        // HTML header
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("<title>Sequential Thinking Session</title>\n");
        
        // CSS styling
        html.push_str("<style>\n");
        html.push_str(include_str!("../templates/export.css"));
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        // Content
        html.push_str("<div class=\"container\">\n");
        html.push_str("<h1>Sequential Thinking Session</h1>\n");
        
        // Session information
        html.push_str(&format!("<div class=\"session-info\">\n"));
        html.push_str(&format!("<p><strong>Session ID:</strong> {}</p>\n", data.session.session_id));
        
        if let Some(ref metadata) = data.session.metadata {
            html.push_str(&format!("<p><strong>Title:</strong> {}</p>\n", metadata.title));
            if let Some(ref description) = metadata.description {
                html.push_str(&format!("<p><strong>Description:</strong> {}</p>\n", description));
            }
            html.push_str(&format!("<p><strong>Status:</strong> {:?}</p>\n", metadata.status));
            html.push_str(&format!("<p><strong>Priority:</strong> {:?}</p>\n", metadata.priority));
        }
        html.push_str("</div>\n");
        
        // Thoughts
        html.push_str("<h2>Thoughts</h2>\n");
        html.push_str("<div class=\"thoughts\">\n");
        
        for (i, thought) in data.session.thoughts.iter().enumerate() {
            let thought_number = i + 1;
            let css_class = if thought.is_revision() {
                "thought revision"
            } else if thought.is_branch() {
                "thought branch"
            } else {
                "thought"
            };
            
            html.push_str(&format!("<div class=\"{}\">\n", css_class));
            html.push_str(&format!("<h3>Thought {}/{}</h3>\n", thought.thought_number, thought.total_thoughts));
            
            if let Some(timestamp) = thought.timestamp {
                html.push_str(&format!("<p class=\"timestamp\">{}</p>\n", timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
            }
            
            html.push_str(&format!("<p class=\"content\">{}</p>\n", thought.thought));
            
            if thought.is_revision() {
                if let Some(revises_thought) = thought.revises_thought {
                    html.push_str(&format!("<p class=\"revision-note\">Revises thought {}</p>\n", revises_thought));
                }
            }
            
            if thought.is_branch() {
                if let Some(branch_id) = &thought.branch_id {
                    html.push_str(&format!("<p class=\"branch-note\">Branch ID: {}</p>\n", branch_id));
                }
            }
            
            html.push_str("</div>\n");
        }
        
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        
        // Footer
        html.push_str("<footer>\n");
        html.push_str(&format!("<p>Exported on {} using UltraFast MCP Sequential Thinking</p>\n", 
                              data.export_metadata.exported_at.format("%Y-%m-%d %H:%M:%S UTC")));
        html.push_str("</footer>\n");
        
        html.push_str("</body>\n</html>");
        
        Ok(html)
    }

    /// Export to CSV format
    fn export_to_csv(&self, data: &ExportData, _options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        let mut csv = String::new();
        
        // Header
        csv.push_str("Thought Number,Total Thoughts,Content,Is Revision,Revises Thought,Is Branch,Branch ID,Timestamp\n");
        
        // Data rows
        for thought in &data.session.thoughts {
            let thought_number = thought.thought_number;
            let total_thoughts = thought.total_thoughts;
            let content = thought.thought.replace("\"", "\"\""); // Escape quotes
            let is_revision = if thought.is_revision() { "true" } else { "false" };
            let revises_thought = thought.revises_thought.map(|t| t.to_string()).unwrap_or_default();
            let is_branch = if thought.is_branch() { "true" } else { "false" };
            let branch_id = thought.branch_id.as_deref().unwrap_or("");
            let timestamp = thought.timestamp.map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string()).unwrap_or_default();
            
            csv.push_str(&format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                                 thought_number, total_thoughts, content, is_revision, 
                                 revises_thought, is_branch, branch_id, timestamp));
        }
        
        Ok(csv)
    }

    /// Export to YAML format
    fn export_to_yaml(&self, data: &ExportData, _options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_yaml::to_string(data)?)
    }

    /// Export to TOML format
    fn export_to_toml(&self, data: &ExportData, _options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        Ok(toml::to_string(data)?)
    }

    /// Export to PDF format
    fn export_to_pdf(&self, data: &ExportData, _options: &ExportOptions) -> Result<String, Box<dyn std::error::Error>> {
        // For now, we'll return an HTML representation that can be converted to PDF
        // In a real implementation, you would use a PDF library like `printpdf` or `wkhtmltopdf`
        self.export_to_html(data, _options)
    }

    /// Get export history
    pub fn get_export_history(&self) -> &[ExportRecord] {
        &self.export_history
    }

    /// Clear export history
    pub fn clear_export_history(&mut self) {
        self.export_history.clear();
    }

    /// Add export template
    pub fn add_template(&mut self, name: String, template: String) {
        self.templates.insert(name, template);
    }

    /// Get export template
    pub fn get_template(&self, name: &str) -> Option<&String> {
        self.templates.get(name)
    }
}

impl Default for ExportEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Display for ExportFormat
impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::Json => write!(f, "JSON"),
            ExportFormat::Markdown => write!(f, "Markdown"),
            ExportFormat::Pdf => write!(f, "PDF"),
            ExportFormat::Html => write!(f, "HTML"),
            ExportFormat::Csv => write!(f, "CSV"),
            ExportFormat::Yaml => write!(f, "YAML"),
            ExportFormat::Toml => write!(f, "TOML"),
        }
    }
}

// Implement FromStr for ExportFormat
impl std::str::FromStr for ExportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "markdown" | "md" => Ok(ExportFormat::Markdown),
            "pdf" => Ok(ExportFormat::Pdf),
            "html" => Ok(ExportFormat::Html),
            "csv" => Ok(ExportFormat::Csv),
            "yaml" | "yml" => Ok(ExportFormat::Yaml),
            "toml" => Ok(ExportFormat::Toml),
            _ => Err(format!("Unknown export format: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thinking::ThoughtData;

    #[test]
    fn test_export_engine_creation() {
        let engine = ExportEngine::new();
        assert_eq!(engine.config.formats.len(), 3);
        assert!(!engine.config.auto_export);
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Json.extension(), "json");
        assert_eq!(ExportFormat::Markdown.extension(), "md");
        assert_eq!(ExportFormat::Pdf.extension(), "pdf");
    }

    #[test]
    fn test_export_format_from_str() {
        assert_eq!("json".parse::<ExportFormat>().unwrap(), ExportFormat::Json);
        assert_eq!("markdown".parse::<ExportFormat>().unwrap(), ExportFormat::Markdown);
        assert_eq!("md".parse::<ExportFormat>().unwrap(), ExportFormat::Markdown);
        assert!("unknown".parse::<ExportFormat>().is_err());
    }

    #[test]
    fn test_filename_generation() {
        let engine = ExportEngine::new();
        let filename = engine.generate_filename("test-session", &ExportFormat::Json).unwrap();
        assert!(filename.contains("test-session"));
        assert!(filename.ends_with(".json"));
    }

    #[test]
    fn test_markdown_export() {
        let engine = ExportEngine::new();
        let thoughts = vec![
            ThoughtData::new("First thought".to_string(), 1, 3),
            ThoughtData::new("Second thought".to_string(), 2, 3),
        ];
        
        let export_data = ExportData {
            session: SessionExportData {
                session_id: "test-session".to_string(),
                metadata: None,
                thoughts,
                statistics: None,
                progress: None,
                branches: HashMap::new(),
                analytics: None,
            },
            export_metadata: ExportMetadata {
                exported_at: Utc::now(),
                format: "markdown".to_string(),
                version: "1.0.0".to_string(),
                tool: "test".to_string(),
                options: ExportOptions::default(),
            },
            custom_data: HashMap::new(),
        };
        
        let options = ExportOptions::default();
        let markdown = engine.export_to_markdown(&export_data, &options).unwrap();
        
        assert!(markdown.contains("Sequential Thinking Session"));
        assert!(markdown.contains("test-session"));
        assert!(markdown.contains("First thought"));
        assert!(markdown.contains("Second thought"));
    }
} 
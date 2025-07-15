//! # Error Handling Module
//!
//! Comprehensive error handling for the sequential thinking functionality.
//!
//! This module provides error types and handling for all aspects of the
//! sequential thinking process, including validation errors, processing
//! errors, and system errors.

use thiserror::Error;

/// Main error type for sequential thinking operations
#[derive(Error, Debug)]
pub enum SequentialThinkingError {
    /// Invalid thought data
    #[error("Invalid thought data: {message}")]
    InvalidThoughtData { message: String },

    /// Thought processing error
    #[error("Thought processing error: {message}")]
    ProcessingError { message: String },

    /// Session management error
    #[error("Session error: {message}")]
    SessionError { message: String },

    /// Branch management error
    #[error("Branch error: {message}")]
    BranchError { message: String },

    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError { message: String },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    /// Serialization/deserialization error
    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    /// Network/transport error
    #[error("Transport error: {message}")]
    TransportError { message: String },

    /// Internal system error
    #[error("Internal error: {message}")]
    InternalError { message: String },

    /// Resource not found
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// Permission denied
    #[error("Permission denied: {reason}")]
    PermissionDenied { reason: String },

    /// Rate limiting error
    #[error("Rate limit exceeded: {limit}")]
    RateLimitExceeded { limit: String },

    /// Timeout error
    #[error("Operation timed out after {duration:?}")]
    Timeout { duration: std::time::Duration },

    /// Cancellation error
    #[error("Operation was cancelled: {reason}")]
    Cancelled { reason: String },

    /// Wrapped error from underlying dependencies
    #[error("Wrapped error: {source}")]
    Wrapped {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl SequentialThinkingError {
    /// Create an invalid thought data error
    pub fn invalid_thought_data(message: impl Into<String>) -> Self {
        Self::InvalidThoughtData {
            message: message.into(),
        }
    }

    /// Create a processing error
    pub fn processing_error(message: impl Into<String>) -> Self {
        Self::ProcessingError {
            message: message.into(),
        }
    }

    /// Create a session error
    pub fn session_error(message: impl Into<String>) -> Self {
        Self::SessionError {
            message: message.into(),
        }
    }

    /// Create a branch error
    pub fn branch_error(message: impl Into<String>) -> Self {
        Self::BranchError {
            message: message.into(),
        }
    }

    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    /// Create a configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }

    /// Create a serialization error
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::SerializationError {
            message: message.into(),
        }
    }

    /// Create a transport error
    pub fn transport_error(message: impl Into<String>) -> Self {
        Self::TransportError {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// Create a permission denied error
    pub fn permission_denied(reason: impl Into<String>) -> Self {
        Self::PermissionDenied {
            reason: reason.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit_exceeded(limit: impl Into<String>) -> Self {
        Self::RateLimitExceeded {
            limit: limit.into(),
        }
    }

    /// Create a timeout error
    pub fn timeout(duration: std::time::Duration) -> Self {
        Self::Timeout { duration }
    }

    /// Create a cancellation error
    pub fn cancelled(reason: impl Into<String>) -> Self {
        Self::Cancelled {
            reason: reason.into(),
        }
    }

    /// Check if this is a retryable error
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::TransportError { .. } | Self::Timeout { .. } | Self::RateLimitExceeded { .. }
        )
    }

    /// Check if this is a client error (not retryable)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidThoughtData { .. }
                | Self::ValidationError { .. }
                | Self::ConfigError { .. }
                | Self::NotFound { .. }
                | Self::PermissionDenied { .. }
        )
    }

    /// Check if this is a server error (potentially retryable)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::ProcessingError { .. }
                | Self::SessionError { .. }
                | Self::BranchError { .. }
                | Self::InternalError { .. }
                | Self::SerializationError { .. }
        )
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Self::InvalidThoughtData { message } => {
                format!("Invalid thought data: {}", message)
            }
            Self::ProcessingError { message } => {
                format!("Failed to process thought: {}", message)
            }
            Self::SessionError { message } => {
                format!("Session error: {}", message)
            }
            Self::BranchError { message } => {
                format!("Branch error: {}", message)
            }
            Self::ValidationError { message } => {
                format!("Validation failed: {}", message)
            }
            Self::ConfigError { message } => {
                format!("Configuration error: {}", message)
            }
            Self::SerializationError { message } => {
                format!("Data format error: {}", message)
            }
            Self::TransportError { message } => {
                format!("Connection error: {}", message)
            }
            Self::InternalError { message } => {
                format!("System error: {}", message)
            }
            Self::NotFound { resource } => {
                format!("Resource not found: {}", resource)
            }
            Self::PermissionDenied { reason } => {
                format!("Access denied: {}", reason)
            }
            Self::RateLimitExceeded { limit } => {
                format!("Too many requests: {}", limit)
            }
            Self::Timeout { duration } => {
                format!("Operation timed out after {:?}", duration)
            }
            Self::Cancelled { reason } => {
                format!("Operation cancelled: {}", reason)
            }
            Self::Wrapped { source } => {
                format!("Error: {}", source)
            }
        }
    }

    /// Get error code for API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::InvalidThoughtData { .. } => "INVALID_THOUGHT_DATA",
            Self::ProcessingError { .. } => "PROCESSING_ERROR",
            Self::SessionError { .. } => "SESSION_ERROR",
            Self::BranchError { .. } => "BRANCH_ERROR",
            Self::ValidationError { .. } => "VALIDATION_ERROR",
            Self::ConfigError { .. } => "CONFIG_ERROR",
            Self::SerializationError { .. } => "SERIALIZATION_ERROR",
            Self::TransportError { .. } => "TRANSPORT_ERROR",
            Self::InternalError { .. } => "INTERNAL_ERROR",
            Self::NotFound { .. } => "NOT_FOUND",
            Self::PermissionDenied { .. } => "PERMISSION_DENIED",
            Self::RateLimitExceeded { .. } => "RATE_LIMIT_EXCEEDED",
            Self::Timeout { .. } => "TIMEOUT",
            Self::Cancelled { .. } => "CANCELLED",
            Self::Wrapped { .. } => "WRAPPED_ERROR",
        }
    }
}

/// Result type for sequential thinking operations
pub type SequentialThinkingResult<T> = Result<T, SequentialThinkingError>;

/// Error context for adding additional information to errors
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Operation being performed
    pub operation: String,
    /// Additional context information
    pub context: std::collections::HashMap<String, String>,
    /// Timestamp when error occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            context: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Add context information
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Add multiple context items
    pub fn with_contexts(mut self, contexts: Vec<(String, String)>) -> Self {
        for (key, value) in contexts {
            self.context.insert(key, value);
        }
        self
    }
}

/// Error handling utilities
pub mod utils {
    use super::*;

    /// Convert a string error to a SequentialThinkingError
    pub fn from_string_error(error: String) -> SequentialThinkingError {
        SequentialThinkingError::InternalError { message: error }
    }

    /// Convert a generic error to a SequentialThinkingError
    pub fn from_generic_error<E: std::error::Error + Send + Sync + 'static>(
        error: E,
    ) -> SequentialThinkingError {
        SequentialThinkingError::Wrapped {
            source: Box::new(error),
        }
    }

    /// Create a timeout error with a specific duration
    pub fn timeout_error(duration: std::time::Duration) -> SequentialThinkingError {
        SequentialThinkingError::Timeout { duration }
    }

    /// Create a validation error for a specific field
    pub fn field_validation_error(field: &str, message: &str) -> SequentialThinkingError {
        SequentialThinkingError::ValidationError {
            message: format!("Field '{}': {}", field, message),
        }
    }

    /// Create a required field error
    pub fn required_field_error(field: &str) -> SequentialThinkingError {
        field_validation_error(field, "Field is required")
    }

    /// Create an invalid format error
    pub fn invalid_format_error(field: &str, expected: &str) -> SequentialThinkingError {
        field_validation_error(field, &format!("Expected format: {}", expected))
    }
}

// Implement From for common error types
impl From<std::io::Error> for SequentialThinkingError {
    fn from(err: std::io::Error) -> Self {
        Self::TransportError {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for SequentialThinkingError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError {
            message: err.to_string(),
        }
    }
}

impl From<uuid::Error> for SequentialThinkingError {
    fn from(err: uuid::Error) -> Self {
        Self::ValidationError {
            message: err.to_string(),
        }
    }
}

impl From<chrono::ParseError> for SequentialThinkingError {
    fn from(err: chrono::ParseError) -> Self {
        Self::ValidationError {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = SequentialThinkingError::invalid_thought_data("Empty thought");
        assert!(matches!(error, SequentialThinkingError::InvalidThoughtData { .. }));
        assert!(error.is_client_error());
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_codes() {
        let error = SequentialThinkingError::processing_error("Test");
        assert_eq!(error.error_code(), "PROCESSING_ERROR");
    }

    #[test]
    fn test_user_message() {
        let error = SequentialThinkingError::validation_error("Invalid input");
        let message = error.user_message();
        assert!(message.contains("Validation failed"));
        assert!(message.contains("Invalid input"));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("test_operation")
            .with_context("user_id", "123")
            .with_context("session_id", "abc");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.context.get("user_id"), Some(&"123".to_string()));
        assert_eq!(context.context.get("session_id"), Some(&"abc".to_string()));
    }

    #[test]
    fn test_from_implementations() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let mcp_error: SequentialThinkingError = io_error.into();
        assert!(matches!(mcp_error, SequentialThinkingError::TransportError { .. }));

        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let mcp_error: SequentialThinkingError = json_error.into();
        assert!(matches!(mcp_error, SequentialThinkingError::SerializationError { .. }));
    }
} 
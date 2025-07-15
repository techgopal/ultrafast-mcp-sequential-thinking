//! # Sequential Thinking Server Binary
//!
//! Main binary for the UltraFast MCP Sequential Thinking server.
//!
//! This binary provides a command-line interface for running the
//! sequential thinking server with various configuration options.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use ultrafast_mcp::{ServerCapabilities, ServerInfo, ToolsCapability};
use ultrafast_mcp_sequential_thinking::{
    default_server_config, SequentialThinkingServer, ServerConfig,
};

/// Command-line arguments for the sequential thinking server
#[derive(Parser)]
#[command(
    name = "sequential-thinking-server",
    about = "UltraFast MCP Sequential Thinking Server",
    version = env!("CARGO_PKG_VERSION"),
    long_about = "High-performance Rust-based MCP server for sequential thinking"
)]
struct Args {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Transport type (stdio, http)
    #[arg(short, long, default_value = "stdio")]
    transport: String,

    /// Port for HTTP transport
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Server name
    #[arg(long)]
    name: Option<String>,

    /// Disable thought logging
    #[arg(long)]
    disable_logging: bool,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Log file path
    #[arg(long)]
    log_file: Option<PathBuf>,

    /// Enable analytics
    #[arg(long)]
    enable_analytics: bool,

    /// Analytics endpoint
    #[arg(long)]
    analytics_endpoint: Option<String>,

    /// Maximum thoughts per session
    #[arg(long)]
    max_thoughts: Option<u32>,

    /// Maximum branches per session
    #[arg(long)]
    max_branches: Option<u32>,

    /// Session timeout in seconds
    #[arg(long)]
    session_timeout: Option<u64>,

    /// Rate limiting enabled
    #[arg(long)]
    rate_limiting: bool,

    /// Requests per minute for rate limiting
    #[arg(long)]
    requests_per_minute: Option<u32>,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    /// Validate configuration
    Validate {
        /// Configuration file to validate
        config: PathBuf,
    },
    /// Generate default configuration
    Generate {
        /// Output file path
        output: PathBuf,
    },
    /// Show server information
    Info,
    /// Run health check
    Health,
}

/// Main server configuration
struct ServerApp {
    /// Server configuration
    config: ServerConfig,
    /// Server instance
    server: SequentialThinkingServer,
}

impl ServerApp {
    /// Create a new server application
    fn new(args: &Args) -> Result<Self, Box<dyn std::error::Error>> {
        // Load configuration
        let mut config = if let Some(config_path) = &args.config {
            Self::load_config_from_file(config_path)?
        } else {
            default_server_config()
        };

        // Override configuration with command-line arguments
        Self::override_config(&mut config, args);

        // Create server
        let server = SequentialThinkingServer::with_config(
            ServerInfo {
                name: config.name.clone(),
                version: config.version.clone(),
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
            ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                ..Default::default()
            },
            args.disable_logging,
        );

        Ok(Self { config, server })
    }

    /// Load configuration from file
    fn load_config_from_file(path: &PathBuf) -> Result<ServerConfig, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;

        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let config: toml::Value = toml::from_str(&content)?;
            if let Some(server) = config.get("server") {
                Ok(server.clone().try_into()?)
            } else {
                Ok(ServerConfig::default())
            }
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let config: serde_json::Value = serde_json::from_str(&content)?;
            if let Some(server) = config.get("server") {
                Ok(serde_json::from_value(server.clone())?)
            } else {
                Ok(ServerConfig::default())
            }
        } else {
            Err("Unsupported configuration file format".into())
        }
    }

    /// Override configuration with command-line arguments
    fn override_config(config: &mut ServerConfig, args: &Args) {
        if !args.transport.is_empty() {
            config.transport = args.transport.clone();
        }

        if args.port != 0 {
            config.port = args.port;
        }

        if let Some(ref name) = args.name {
            config.name = name.clone();
        }

        if args.enable_analytics {
            config.analytics.enabled = true;
        }

        if let Some(ref endpoint) = args.analytics_endpoint {
            config.analytics.endpoint = endpoint.clone();
        }

        if let Some(max_thoughts) = args.max_thoughts {
            config.thinking.max_thoughts_per_session = max_thoughts;
        }

        if let Some(max_branches) = args.max_branches {
            config.thinking.max_branches_per_session = max_branches;
        }

        if let Some(timeout) = args.session_timeout {
            config.thinking.session_timeout_seconds = timeout;
        }

        if args.rate_limiting {
            config.security.rate_limiting_enabled = true;
        }

        if let Some(requests_per_minute) = args.requests_per_minute {
            config.thinking.rate_limiting.requests_per_minute = requests_per_minute;
        }
    }

    /// Initialize logging
    fn init_logging(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        // Set up logging
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&args.log_level));

        let mut builder = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_ansi(atty::is(atty::Stream::Stderr));

        // TODO: Add file logging support later
        // if let Some(log_file) = &args.log_file {
        //     let file_appender = tracing_appender::rolling::never(
        //         log_file.parent().unwrap_or_else(|| std::path::Path::new(".")),
        //         log_file.file_name().unwrap(),
        //     );
        //     builder = builder.with_writer(file_appender);
        // }

        builder.init();
        Ok(())
    }

    /// Run the server
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting UltraFast MCP Sequential Thinking Server");
        info!("Server: {} v{}", self.config.name, self.config.version);
        info!("Transport: {}", self.config.transport);

        if self.config.transport == "http" {
            info!("Port: {}", self.config.port);
        }

        info!(
            "Max thoughts per session: {}",
            self.config.thinking.max_thoughts_per_session
        );
        info!(
            "Max branches per session: {}",
            self.config.thinking.max_branches_per_session
        );
        info!(
            "Session timeout: {} seconds",
            self.config.thinking.session_timeout_seconds
        );
        info!("Analytics enabled: {}", self.config.analytics.enabled);
        info!(
            "Rate limiting enabled: {}",
            self.config.security.rate_limiting_enabled
        );

        // Create MCP server
        let mcp_server = self.server.clone().create_mcp_server();

        // Run server based on transport
        match self.config.transport.as_str() {
            "stdio" => {
                info!("Running server with STDIO transport");
                mcp_server.run_stdio().await?;
            }
            "http" => {
                info!(
                    "Running server with HTTP transport on port {}",
                    self.config.port
                );
                mcp_server
                    .run_streamable_http("0.0.0.0", self.config.port)
                    .await?;
            }
            _ => {
                return Err(format!("Unsupported transport: {}", self.config.transport).into());
            }
        }

        Ok(())
    }

    /// Validate configuration
    fn validate_config(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.config.name.is_empty() {
            errors.push("Server name cannot be empty".to_string());
        }

        if self.config.port == 0 {
            errors.push("Server port must be greater than 0".to_string());
        }

        if self.config.thinking.max_thoughts_per_session == 0 {
            errors.push("Max thoughts per session must be greater than 0".to_string());
        }

        if self.config.thinking.max_branches_per_session == 0 {
            errors.push("Max branches per session must be greater than 0".to_string());
        }

        if self.config.thinking.session_timeout_seconds == 0 {
            errors.push("Session timeout must be greater than 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Generate default configuration
    fn generate_config(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let config = default_server_config();
        let config_json = serde_json::to_string_pretty(&config)?;

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(output_path, config_json)?;
        info!(
            "Generated default configuration at: {}",
            output_path.display()
        );

        Ok(())
    }

    /// Show server information
    fn show_info(&self) {
        println!("UltraFast MCP Sequential Thinking Server");
        println!("========================================");
        println!("Name: {}", self.config.name);
        println!("Version: {}", self.config.version);
        println!("Transport: {}", self.config.transport);
        println!("Port: {}", self.config.port);
        println!();
        println!("Configuration:");
        println!(
            "  Max thoughts per session: {}",
            self.config.thinking.max_thoughts_per_session
        );
        println!(
            "  Max branches per session: {}",
            self.config.thinking.max_branches_per_session
        );
        println!(
            "  Session timeout: {} seconds",
            self.config.thinking.session_timeout_seconds
        );
        println!("  Analytics enabled: {}", self.config.analytics.enabled);
        println!(
            "  Rate limiting enabled: {}",
            self.config.security.rate_limiting_enabled
        );
        println!(
            "  Thought logging enabled: {}",
            !self.config.thinking.enable_thought_logging
        );
    }

    /// Run health check
    async fn health_check(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stats = self.server.get_stats().await;

        println!("Health Check Results");
        println!("===================");
        println!("Status: OK");
        println!("Total requests: {}", stats.total_requests);
        println!("Total thoughts: {}", stats.total_thoughts);
        println!("Total sessions: {}", stats.total_sessions);
        println!("Error count: {}", stats.error_count);
        println!("Average response time: {:.2}ms", stats.avg_response_time_ms);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Handle subcommands first
    if let Some(ref command) = args.command {
        match command {
            Commands::Validate { config } => match ServerApp::load_config_from_file(&config) {
                Ok(mut server_config) => {
                    let server = SequentialThinkingServer::new();
                    let app = ServerApp {
                        config: server_config,
                        server,
                    };

                    match app.validate_config() {
                        Ok(()) => {
                            println!("Configuration is valid");
                            Ok(())
                        }
                        Err(errors) => {
                            println!("Configuration validation failed:");
                            for error in errors {
                                println!("  - {}", error);
                            }
                            Err("Configuration validation failed".into())
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to load configuration: {}", e);
                    Err(e)
                }
            },
            Commands::Generate { output } => ServerApp::generate_config(&output),
            Commands::Info => {
                let app = ServerApp::new(&args)?;
                app.show_info();
                Ok(())
            }
            Commands::Health => {
                let app = ServerApp::new(&args)?;
                app.health_check().await
            }
        }
    } else {
        // Initialize logging
        ServerApp::init_logging(&args)?;

        // Create and run server
        let app = ServerApp::new(&args)?;

        // Validate configuration
        if let Err(errors) = app.validate_config() {
            error!("Configuration validation failed:");
            for error in errors {
                error!("  - {}", error);
            }
            return Err("Configuration validation failed".into());
        }

        // Run the server
        app.run().await
    }
}

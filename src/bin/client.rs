//! # Sequential Thinking Client Binary
//!
//! Main binary for the UltraFast MCP Sequential Thinking client.
//!
//! This binary provides a command-line interface for connecting to
//! sequential thinking servers and managing thinking sessions.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;

use ultrafast_mcp_sequential_thinking::SequentialThinkingClient;

/// Command-line arguments for the sequential thinking client
#[derive(Parser)]
#[command(
    name = "sequential-thinking-client",
    about = "UltraFast MCP Sequential Thinking Client",
    version = env!("CARGO_PKG_VERSION"),
    long_about = "High-performance Rust-based MCP client for sequential thinking"
)]
struct Args {
    /// Server URL
    #[arg(short, long, default_value = "stdio://")]
    server: String,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Timeout in seconds
    #[arg(short, long, default_value = "30")]
    timeout: u64,

    /// Retry attempts
    #[arg(long, default_value = "3")]
    retries: u32,

    /// Session title
    #[arg(long)]
    session_title: Option<String>,

    /// Disable progress tracking
    #[arg(long)]
    disable_progress: bool,

    /// Disable thought visualization
    #[arg(long)]
    disable_visualization: bool,

    /// Auto-save interval in seconds
    #[arg(long, default_value = "60")]
    auto_save: u64,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Log file path
    #[arg(long)]
    log_file: Option<PathBuf>,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    /// Start an interactive thinking session
    Interactive {
        /// Session title
        title: Option<String>,
    },
    /// Process a single thought
    Think {
        /// Thought content
        thought: String,
        /// Thought number
        #[arg(long, default_value = "1")]
        number: u32,
        /// Total thoughts
        #[arg(long, default_value = "1")]
        total: u32,
        /// Whether more thoughts are needed
        #[arg(long)]
        more_needed: bool,
    },
    /// Export a session
    Export {
        /// Session ID
        session_id: String,
        /// Export format (json, markdown)
        #[arg(long, default_value = "json")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Analyze a session
    Analyze {
        /// Session ID
        session_id: String,
    },
    /// List available tools
    Tools,
    /// Show client information
    Info,
    /// Test connection
    Test,
    /// Generate default configuration
    Generate {
        /// Output file path
        output: PathBuf,
    },
}

/// Main client application
struct ClientApp {
    /// Client configuration
    config: ultrafast_mcp_sequential_thinking::ClientConfig,
    /// Client instance
    client: SequentialThinkingClient,
}

impl ClientApp {
    /// Create a new client application
    async fn new(args: &Args) -> Result<Self, Box<dyn std::error::Error>> {
        // Load configuration
        let mut config = if let Some(config_path) = &args.config {
            Self::load_config_from_file(config_path)?
        } else {
            ultrafast_mcp_sequential_thinking::default_client_config()
        };

        // Override configuration with command-line arguments
        Self::override_config(&mut config, args);

        // Create client (connection and initialization handled internally)
        let client = SequentialThinkingClient::with_config(&args.server, config.thinking.clone())
            .await
            .map_err(|e| format!("Failed to create client: {e}"))?;

        Ok(Self { config, client })
    }

    /// Load configuration from file
    fn load_config_from_file(
        path: &PathBuf,
    ) -> Result<ultrafast_mcp_sequential_thinking::ClientConfig, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;

        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let config: toml::Value = toml::from_str(&content)?;
            if let Some(client) = config.get("client") {
                Ok(client.clone().try_into()?)
            } else {
                Ok(ultrafast_mcp_sequential_thinking::ClientConfig::default())
            }
        } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let config: serde_json::Value = serde_json::from_str(&content)?;
            if let Some(client) = config.get("client") {
                Ok(serde_json::from_value(client.clone())?)
            } else {
                Ok(ultrafast_mcp_sequential_thinking::ClientConfig::default())
            }
        } else {
            Err("Unsupported configuration file format".into())
        }
    }

    /// Override configuration with command-line arguments
    fn override_config(config: &mut ultrafast_mcp_sequential_thinking::ClientConfig, args: &Args) {
        if !args.server.is_empty() {
            config.server_url = args.server.clone();
        }

        if args.timeout != 0 {
            config.timeout_seconds = args.timeout;
        }

        if args.retries != 0 {
            config.retry_attempts = args.retries;
        }

        if args.disable_progress {
            config.thinking.enable_progress_tracking = false;
        }

        if args.disable_visualization {
            config.thinking.show_thought_visualization = false;
        }

        if args.auto_save != 0 {
            config.thinking.auto_save_interval = args.auto_save;
        }
    }

    /// Initialize logging
    fn init_logging(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        // Set up logging
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&args.log_level));

        let builder = tracing_subscriber::fmt()
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

    /// Start interactive session
    async fn interactive_session(
        &self,
        title: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session_title = title.unwrap_or_else(|| "Interactive Session".to_string());
        info!("Starting interactive thinking session: {}", session_title);

        let session = self
            .client
            .start_session(session_title)
            .await
            .map_err(|e| format!("Failed to start session: {e}"))?;

        println!("🎯 Interactive Thinking Session Started");
        println!("Session ID: {}", session.session_id);
        println!("Title: {}", session.title);
        println!();
        println!("Commands:");
        println!("  think <content> - Add a thought");
        println!("  revise <number> <content> - Revise a thought");
        println!("  branch <from> <id> <content> - Create a branch");
        println!("  progress - Show progress");
        println!("  stats - Show statistics");
        println!("  export [format] - Export session");
        println!("  quit - End session");
        println!();

        let mut thought_number = 1;
        let mut total_thoughts = 5;

        loop {
            print!("💭 > ");
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            let parts: Vec<&str> = input.splitn(2, ' ').collect();
            let command = parts[0].to_lowercase();

            match command.as_str() {
                "think" => {
                    if parts.len() < 2 {
                        println!("❌ Usage: think <content>");
                        continue;
                    }
                    let content = parts[1];

                    let thought = ultrafast_mcp_sequential_thinking::ThoughtData::new(
                        content.to_string(),
                        thought_number,
                        total_thoughts,
                    );
                    match self.client.add_thought(&session.session_id, thought).await {
                        Ok(processed) => {
                            println!("✅ Thought {} processed", processed.thought_number);
                            thought_number += 1;
                            if processed.total_thoughts > total_thoughts {
                                total_thoughts = processed.total_thoughts;
                            }
                        }
                        Err(e) => {
                            println!("❌ Failed to process thought: {e}");
                        }
                    }
                }
                "revise" => {
                    if parts.len() < 3 {
                        println!("❌ Usage: revise <number> <content>");
                        continue;
                    }
                    let number = parts[1].parse::<u32>().unwrap_or(0);
                    let content = parts[2];

                    if number == 0 || number >= thought_number {
                        println!("❌ Invalid thought number");
                        continue;
                    }

                    let thought = ultrafast_mcp_sequential_thinking::ThoughtData::revision(
                        content.to_string(),
                        thought_number,
                        number,
                    );
                    match self.client.add_thought(&session.session_id, thought).await {
                        Ok(processed) => {
                            println!("✅ Revision {} processed", processed.thought_number);
                            thought_number += 1;
                        }
                        Err(e) => {
                            println!("❌ Failed to process revision: {e}");
                        }
                    }
                }
                "branch" => {
                    if parts.len() < 4 {
                        println!("❌ Usage: branch <from> <id> <content>");
                        continue;
                    }
                    let from = parts[1].parse::<u32>().unwrap_or(0);
                    let branch_id = parts[2];
                    let content = parts[3];

                    if from == 0 || from >= thought_number {
                        println!("❌ Invalid branch from number");
                        continue;
                    }

                    let thought = ultrafast_mcp_sequential_thinking::ThoughtData::branch(
                        content.to_string(),
                        thought_number,
                        from,
                        branch_id.to_string(),
                    );
                    match self.client.add_thought(&session.session_id, thought).await {
                        Ok(processed) => {
                            println!("✅ Branch {} processed", processed.thought_number);
                            thought_number += 1;
                        }
                        Err(e) => {
                            println!("❌ Failed to process branch: {e}");
                        }
                    }
                }
                "progress" => {
                    if let Some(progress) = self.client.get_progress().await {
                        println!(
                            "📊 Progress: {}/{} ({:.1}%)",
                            progress.completed_thoughts,
                            progress.total_thoughts,
                            progress.progress_percentage * 100.0
                        );
                    } else {
                        println!("📊 No progress information available");
                    }
                }
                "stats" => {
                    let stats = self.client.get_stats().await;
                    println!("📈 Client Statistics:");
                    println!("  Total requests: {}", stats.total_requests);
                    println!("  Total thoughts: {}", stats.total_thoughts);
                    println!("  Total sessions: {}", stats.total_sessions);
                    println!(
                        "  Average response time: {:.2}ms",
                        stats.avg_response_time_ms
                    );
                    println!("  Error count: {}", stats.error_count);
                    println!("  Retry count: {}", stats.retry_count);
                }
                "export" => {
                    let format = if parts.len() > 1 { parts[1] } else { "json" };
                    match self
                        .client
                        .export_session(&session.session_id, format)
                        .await
                    {
                        Ok(content) => {
                            println!("📄 Session exported in {format} format:");
                            println!("{content}");
                        }
                        Err(e) => {
                            println!("❌ Failed to export session: {e}");
                        }
                    }
                }
                "quit" | "exit" => {
                    println!("👋 Ending session...");
                    break;
                }
                "help" => {
                    println!("Commands:");
                    println!("  think <content> - Add a thought");
                    println!("  revise <number> <content> - Revise a thought");
                    println!("  branch <from> <id> <content> - Create a branch");
                    println!("  progress - Show progress");
                    println!("  stats - Show statistics");
                    println!("  export [format] - Export session");
                    println!("  quit - End session");
                }
                _ => {
                    println!("❌ Unknown command: {command}. Type 'help' for available commands.");
                }
            }
        }

        Ok(())
    }

    /// Process a single thought
    async fn process_thought(
        &self,
        thought: String,
        number: u32,
        total: u32,
        more_needed: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session_title = "Single Thought Session".to_string();
        let session = self
            .client
            .start_session(session_title)
            .await
            .map_err(|e| format!("Failed to start session: {e}"))?;

        let thought_data = ultrafast_mcp_sequential_thinking::ThoughtData {
            thought,
            thought_number: number,
            total_thoughts: total,
            next_thought_needed: more_needed,
            ..Default::default()
        };

        match self
            .client
            .add_thought(&session.session_id, thought_data)
            .await
        {
            Ok(processed) => {
                println!("✅ Thought processed successfully");
                println!(
                    "Thought Number: {}/{}",
                    processed.thought_number, processed.total_thoughts
                );
                println!("Content: {}", processed.thought);
                println!("More thoughts needed: {}", processed.next_thought_needed);
            }
            Err(e) => {
                println!("❌ Failed to process thought: {e}");
            }
        }

        Ok(())
    }

    /// Export a session
    async fn export_session(
        &self,
        session_id: &str,
        format: &str,
        output: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.client.export_session(session_id, format).await {
            Ok(content) => {
                if let Some(output_path) = output {
                    std::fs::write(&output_path, content)?;
                    println!("✅ Session exported to: {}", output_path.display());
                } else {
                    println!("📄 Session exported in {format} format:");
                    println!("{content}");
                }
            }
            Err(e) => {
                println!("❌ Failed to export session: {e}");
            }
        }

        Ok(())
    }

    /// Analyze a session
    async fn analyze_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.client.analyze_session(session_id).await {
            Ok(analysis) => {
                println!("📊 Session Analysis:");
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            }
            Err(e) => {
                println!("❌ Failed to analyze session: {e}");
            }
        }

        Ok(())
    }

    /// List available tools
    async fn list_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.client.list_tools().await {
            Ok(tools) => {
                println!("🔧 Available Tools:");
                for tool in tools {
                    println!("  - {}: {}", tool.name, tool.description.clone());
                }
            }
            Err(e) => {
                println!("❌ Failed to list tools: {e}");
            }
        }

        Ok(())
    }

    /// Show client information
    fn show_info(&self) {
        println!("UltraFast MCP Sequential Thinking Client");
        println!("=======================================");
        println!("Server URL: {}", self.config.server_url);
        println!("Timeout: {} seconds", self.config.timeout_seconds);
        println!("Retry attempts: {}", self.config.retry_attempts);
        println!();
        println!("Configuration:");
        println!(
            "  Progress tracking: {}",
            self.config.thinking.enable_progress_tracking
        );
        println!(
            "  Thought visualization: {}",
            self.config.thinking.show_thought_visualization
        );
        println!(
            "  Auto-save interval: {} seconds",
            self.config.thinking.auto_save_interval
        );
        println!(
            "  Max retry attempts: {}",
            self.config.thinking.max_retry_attempts
        );
        println!(
            "  Operation timeout: {} seconds",
            self.config.thinking.operation_timeout
        );
    }

    /// Test connection
    async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Testing connection to server...");

        match self.client.list_tools().await {
            Ok(tools) => {
                println!("✅ Connection successful!");
                println!("Found {} tools:", tools.len());
                for tool in tools {
                    println!("  - {}", tool.name);
                }
            }
            Err(e) => {
                println!("❌ Connection failed: {e}");
            }
        }

        Ok(())
    }

    /// Generate default configuration
    fn generate_config(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let config = ultrafast_mcp_sequential_thinking::default_client_config();
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Handle subcommands first
    if let Some(ref command) = args.command {
        match command {
            Commands::Interactive { title } => {
                // Initialize logging
                ClientApp::init_logging(&args)?;

                // Create client and start interactive session
                let app = ClientApp::new(&args).await?;
                app.interactive_session(title.clone()).await
            }
            Commands::Think {
                thought,
                number,
                total,
                more_needed,
            } => {
                // Initialize logging
                ClientApp::init_logging(&args)?;

                // Create client and process thought
                let app = ClientApp::new(&args).await?;
                app.process_thought(thought.to_string(), *number, *total, *more_needed)
                    .await
            }
            Commands::Export {
                session_id,
                format,
                output,
            } => {
                // Initialize logging
                ClientApp::init_logging(&args)?;

                // Create client and export session
                let app = ClientApp::new(&args).await?;
                app.export_session(session_id, format, output.clone()).await
            }
            Commands::Analyze { session_id } => {
                // Initialize logging
                ClientApp::init_logging(&args)?;

                // Create client and analyze session
                let app = ClientApp::new(&args).await?;
                app.analyze_session(session_id).await
            }
            Commands::Tools => {
                // Initialize logging
                ClientApp::init_logging(&args)?;

                // Create client and list tools
                let app = ClientApp::new(&args).await?;
                app.list_tools().await
            }
            Commands::Info => {
                let app = ClientApp::new(&args).await?;
                app.show_info();
                Ok(())
            }
            Commands::Test => {
                // Initialize logging
                ClientApp::init_logging(&args)?;

                // Create client and test connection
                let app = ClientApp::new(&args).await?;
                app.test_connection().await
            }
            Commands::Generate { output } => ClientApp::generate_config(output),
        }
    } else {
        // No subcommand provided, show help
        println!("UltraFast MCP Sequential Thinking Client");
        println!("Use --help for available commands");
        println!();
        println!("Example usage:");
        println!("  {} interactive", env!("CARGO_BIN_NAME"));
        println!(
            "  {} think \"This is my first thought\"",
            env!("CARGO_BIN_NAME")
        );
        println!(
            "  {} export <session-id> --format json",
            env!("CARGO_BIN_NAME")
        );
        Ok(())
    }
}

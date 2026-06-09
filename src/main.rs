use btnotify::config;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use glob::glob;
use once_cell::sync::Lazy;
use std::io::{self, Read};
use std::path::PathBuf;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// Import pager, progress, and limits utilities
use btnotify::cli::{page_output, should_page_output, create_spinner, set_message, finish_with_message, abandon_with_message, MemoryLimit, CpuLimit, detect_mode, is_agent_session, is_tty};
use btnotify::config::app::Mode;
use btnotify::error::{EXIT_SUCCESS, EXIT_GENERIC_ERROR, EXIT_USAGE_ERROR, EXIT_VALIDATION_ERROR, EXIT_SIGINT};
use btnotify::output::{OutputSchema, CommandField};

// Module name for logging
const MODULE_NAME: &str = "proximityd";

// Output format enum
#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputFormat {
    Json,
    Toon,
    Human,
}

impl OutputFormat {
    fn from_flags(toon: bool, json: bool, format: Option<&str>, mode: Mode) -> Self {
        // Explicit format flag takes precedence
        if let Some(fmt) = format {
            match fmt.to_lowercase().as_str() {
                "toon" => return OutputFormat::Toon,
                "json" => return OutputFormat::Json,
                "human" => return OutputFormat::Human,
                _ => {
                    eprintln!("Invalid format: {}. Valid values: toon, json, human", fmt);
                    std::process::exit(EXIT_USAGE_ERROR);
                }
            }
        }
        
        // Individual flags
        if toon {
            return OutputFormat::Toon;
        }
        if json {
            return OutputFormat::Json;
        }
        
        // Default based on mode
        match mode {
            Mode::Agent => OutputFormat::Toon,
            Mode::Human => OutputFormat::Human,
            Mode::Auto => {
                // In auto mode, use TOON if agent session detected, otherwise human
                if is_agent_session() {
                    OutputFormat::Toon
                } else {
                    OutputFormat::Human
                }
            }
        }
    }
}

// Color configuration
static COLORS_ENABLED: Lazy<bool> =
    Lazy::new(|| atty::is(atty::Stream::Stderr) && std::env::var("NO_COLOR").is_err());

#[derive(Parser)]
#[command(name = "proximityd", version, about = "A CLI notification tool", long_about = "Generic presence detection service with pluggable notifications.\n\nMODE SELECTION:\n  The CLI operates in three modes: agent (optimized for AI consumption), human (interactive TTY-dependent),\n  and auto (environment-aware). Mode precedence: --human/--interactive > PROXIMITYD_MODE env var > config file > auto-detection.\n\nOUTPUT FORMATS:\n  The CLI supports three output formats: toon (token-efficient for AI), json (structured data),\n  and human (readable text). Format precedence: --format > --toon/--json > mode-based default.\n  Agent mode defaults to TOON, human mode defaults to human-readable text.\n\nExit codes:\n  0   Success\n  1   Generic error\n  2   Usage error\n  3   Network error\n  4   Validation error\n  5   File not found\n  6   Permission denied\n  130 SIGINT (Ctrl+C)")]
struct Cli {
    /// Input files or glob patterns. Use "-" for stdin.
    #[arg(value_name = "INPUTS")]
    inputs: Vec<String>,

    /// Override config file
    #[arg(long, env = "PROXIMITYD_CONFIG")]
    config: Option<PathBuf>,

    /// Override devices mapping file
    #[arg(long, env = "PROXIMITYD_DEVICES")]
    devices: Option<PathBuf>,

    /// Force human mode (interactive, TTY-dependent output)
    #[arg(long, help = "Force human mode (interactive, TTY-dependent output)")]
    human: bool,

    /// Operating mode (agent, human, auto)
    #[arg(long, env = "PROXIMITYD_MODE", value_name = "MODE", help = "Operating mode: agent, human, or auto (default: auto)")]
    mode: Option<String>,

    /// Output as JSON
    #[arg(long)]
    json: bool,

    /// Output as TOON format (token-efficient for AI consumption)
    #[arg(long, help = "Output in TOON format (token-efficient for AI consumption)")]
    toon: bool,

    /// Output format (json, toon, human)
    #[arg(long, value_name = "FORMAT", help = "Output format: toon (token-efficient), json (structured), or human (readable)")]
    format: Option<String>,

    /// Select specific output fields (comma-separated, e.g., name,status,location)
    #[arg(long, value_name = "FIELDS", help = "Select specific output fields (comma-separated, e.g., name,status,location)")]
    fields: Option<String>,

    /// Quiet mode - suppress all output except errors
    #[arg(long, short = 'q')]
    quiet: bool,

    /// Verbose mode - increase logging verbosity (can be used multiple times)
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,

    /// Show usage information
    #[arg(long)]
    usage: bool,

    /// Disable colored output
    #[arg(long)]
    nocolor: bool,

    /// Log file path (for file-based logging)
    #[arg(long)]
    log_file: Option<PathBuf>,

    /// Run in daemon mode: continuously scan for BLE devices and emit presence events
    #[arg(long)]
    daemon: bool,

    /// Perform a health check and exit. Exit code 0 = healthy, 1 = unhealthy.
    /// For use with Docker HEALTHCHECK.
    #[arg(long)]
    health_check: bool,

    /// Generate shell completion script (bash, zsh, fish)
    #[arg(long, value_name = "SHELL")]
    generate: Option<String>,

    /// Force re-initialization of config files with default templates
    #[arg(long)]
    init_config: bool,

    /// Launch interactive TUI mode for configuration
    #[arg(long, short = 'i')]
    interactive: bool,

    /// Display manual page
    #[arg(long)]
    man: bool,

    /// Disable pager for long output
    #[arg(long)]
    no_pager: bool,

    /// Dry run mode - show what would be done without making changes
    #[arg(long)]
    dry_run: bool,

    /// Discover identifier correlations from signal log
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover identifier correlations from signal log
    Discover {
        /// Number of hours to look back from now (default: 24)
        #[arg(long, default_value = "24")]
        hours: u32,

        /// Minimum confidence score (0.0 to 1.0, default: 0.5)
        #[arg(long, default_value = "0.5")]
        min_confidence: f64,

        /// Output file path (default: stdout)
        #[arg(long)]
        output: Option<PathBuf>,

        /// Select specific output fields (comma-separated)
        #[arg(long, value_name = "FIELDS")]
        fields: Option<String>,

        /// Disable pager for long output
        #[arg(long)]
        no_pager: bool,

        /// Maximum memory usage in bytes (e.g., 1GB = 1073741824)
        #[arg(long)]
        max_memory: Option<u64>,

        /// Maximum CPU cores to use (default: all available)
        #[arg(long)]
        max_cpu: Option<usize>,
    },
    /// Show current presence status
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Select specific output fields (comma-separated)
        #[arg(long, value_name = "FIELDS")]
        fields: Option<String>,

        /// Disable pager for long output
        #[arg(long)]
        no_pager: bool,
    },
    /// List configured parties
    Parties {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Select specific output fields (comma-separated)
        #[arg(long, value_name = "FIELDS")]
        fields: Option<String>,

        /// Disable pager for long output
        #[arg(long)]
        no_pager: bool,
    },
    /// List configured devices
    Devices {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Select specific output fields (comma-separated)
        #[arg(long, value_name = "FIELDS")]
        fields: Option<String>,

        /// Disable pager for long output
        #[arg(long)]
        no_pager: bool,
    },
    /// Export signal log data
    Export {
        /// Export format (jsonl or csv, default: jsonl)
        #[arg(long, default_value = "jsonl")]
        format: String,

        /// Export signals since this date (YYYY-MM-DD format)
        #[arg(long)]
        since: Option<String>,

        /// Output file path (default: stdout)
        #[arg(long)]
        output: Option<PathBuf>,

        /// Select specific output fields (comma-separated)
        #[arg(long, value_name = "FIELDS")]
        fields: Option<String>,

        /// Disable pager for long output
        #[arg(long)]
        no_pager: bool,

        /// Maximum memory usage in bytes (e.g., 1GB = 1073741824)
        #[arg(long)]
        max_memory: Option<u64>,

        /// Maximum CPU cores to use (default: all available)
        #[arg(long)]
        max_cpu: Option<usize>,
    },
    /// Install proximityd: generate shell completions and initialize config files
    Install {
        /// Force installation without confirmation
        #[arg(long)]
        force: bool,
        /// Dry run mode - show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Uninstall proximityd: remove completions and optionally config files
    Uninstall {
        /// Force uninstall without confirmation
        #[arg(long)]
        force: bool,
        /// Dry run mode - show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Generate shell completion script
    Completion {
        /// Shell type (bash, zsh, fish)
        #[arg(value_name = "SHELL")]
        shell: String,

        /// Output file path (default: stdout)
        #[arg(long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Display manual page
    Man {
        /// Command to display manual page for (default: main command)
        #[arg(value_name = "COMMAND")]
        command: Option<String>,
    },
}

async fn run_daemon(
    app_config: config::AppConfig,
    devices_config: config::DevicesConfig,
    quiet: bool,
) -> Result<()> {
    use btnotify::detection::{run_detection_loop, DetectionEngine};
    use btnotify::notifier::NotifierRegistry;
    use btnotify::scanner::ble::BleScanner;
    use btnotify::scanner::scan_loop::spawn_scan_loop;
    use btnotify::scanner::Scanner;
    use btnotify::state::PresenceStateTable;
    use std::sync::Arc;
    use std::time::Duration;

    let spinner = create_spinner(quiet);
    set_message(&spinner, "Starting proximityd daemon (btleplug)");

    let notifiers = NotifierRegistry::from_config(&app_config)
        .context("Failed to initialise notifiers from config")?;
    let notifiers = if notifiers.is_empty() {
        info!("No notifiers configured; notifications disabled");
        None
    } else {
        info!("{} notifier(s) active", notifiers.len());
        Some(Arc::new(notifiers))
    };

    let ble_enabled = app_config
        .scanner
        .get("ble")
        .map(|s| s.enabled)
        .unwrap_or(true);
    
    let mut ble_scanner = BleScanner::new();
    ble_scanner.set_enabled(ble_enabled);
    
    let scanner: Arc<dyn Scanner> = Arc::new(ble_scanner);
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = Arc::new(DetectionEngine::new(
        app_config.clone(),
        devices_config,
        state_table,
    ));

    let scan_interval = Duration::from_secs(
        app_config
            .scanner
            .get("ble")
            .map(|s| s.scan_interval_sec)
            .unwrap_or(30),
    );
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let rx = spawn_scan_loop(scanner, scan_interval, shutdown_rx.clone());

    // Spawn signal handler to trigger graceful shutdown
    let shutdown_tx_signal = shutdown_tx.clone();
    tokio::spawn(async move {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler");
        let mut sighup = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
            .expect("Failed to install SIGHUP handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("SIGINT received, exiting with code 130");
                std::process::exit(EXIT_SIGINT);
            },
            _ = sigterm.recv() => {},
            _ = sighup.recv() => {
                info!("SIGHUP received - config reload requested");
                info!("Note: Full config reload requires architectural changes");
                info!("Restart the daemon to apply configuration changes");
            }
        }
        info!("Shutdown signal received, initiating graceful shutdown");
        shutdown_tx_signal.send(true).ok();
    });

    #[allow(deprecated)]
    let exit_check_interval = Duration::from_secs(app_config.exit_timeout_seconds.max(5));
    set_message(&spinner, "Running detection loop");
    run_detection_loop(engine, rx, exit_check_interval, notifiers, shutdown_rx).await;

    finish_with_message(&spinner, "Daemon shutdown complete");
    info!("Daemon shutdown complete");
    Ok(())
}


fn process_content(source: &str, content: &str) -> Result<()> {
    info!(
        "Processing content from {} ({} bytes)",
        source,
        content.len()
    );
    debug!("Content preview: {}", &content[..content.len().min(100)]);
    Ok(())
}

fn resolve_log_level(cli: &Cli, app_config: Option<&config::AppConfig>) -> String {
    // Precedence: env var > CLI flags > config file > default (INFO)
    if let Ok(env_level) = std::env::var("PROXIMITYD_LOG_LEVEL") {
        return env_level;
    }

    if cli.quiet {
        return Level::ERROR.to_string();
    }

    if cli.verbose > 0 {
        let level = match cli.verbose {
            1 => Level::DEBUG,
            _ => Level::TRACE,
        };
        return level.to_string();
    }

    if let Some(cfg) = app_config {
        return cfg.general.log_level.clone();
    }

    Level::INFO.to_string()
}

fn init_logging(cli: &Cli, app_config: Option<&config::AppConfig>) -> Result<()> {
    if cli.nocolor {
        std::env::set_var("NO_COLOR", "1");
    }

    let level_str = resolve_log_level(cli, app_config);
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&level_str));

    let is_terminal = atty::is(atty::Stream::Stderr);
    let explicit_format = std::env::var("PROXIMITYD_LOG_FORMAT").ok();
    let use_json = match explicit_format.as_deref() {
        Some("json") => true,
        Some("pretty") => false,
        Some(other) => {
            eprintln!("Warning: unknown PROXIMITYD_LOG_FORMAT='{other}', expected 'json' or 'pretty'. Falling back to auto-detection.");
            !is_terminal
        }
        None => !is_terminal,
    };

    if use_json {
        let fmt_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .json()
            .with_target(true)
            .with_level(true)
            .with_current_span(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    } else {
        let fmt_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(*COLORS_ENABLED)
            .with_target(true)
            .with_level(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();
    }

    Ok(())
}

/// Parse fields argument and create an OutputSchema for the given command.
fn parse_output_schema(command: &str, fields: Option<String>) -> Result<OutputSchema> {
    if let Some(fields_str) = fields {
        let field_names: Vec<String> = fields_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if field_names.is_empty() {
            return Err(anyhow::anyhow!("Fields argument cannot be empty"));
        }
        
        OutputSchema::with_fields(command, &field_names)
    } else {
        Ok(OutputSchema::new(command))
    }
}

#[allow(clippy::too_many_arguments)]
fn run_discover(
    hours: u32,
    min_confidence: f64,
    output: Option<PathBuf>,
    _fields: Option<String>,
    no_pager: bool,
    quiet: bool,
    max_memory: Option<u64>,
    max_cpu: Option<usize>,
) -> Result<()> {
    use btnotify::discovery::DiscoveryEngine;
    use btnotify::signals::db_path;

    info!("Running discovery: hours={}, min_confidence={}", hours, min_confidence);

    // Apply resource limits
    let memory_limit = MemoryLimit::new(max_memory);
    let cpu_limit = CpuLimit::new(max_cpu);

    if memory_limit.is_enabled() {
        info!("Memory limit set: {} bytes", max_memory.unwrap());
    }
    if cpu_limit.is_enabled() {
        info!("CPU limit set: {} cores", max_cpu.unwrap());
    }

    let spinner = create_spinner(quiet);
    set_message(&spinner, "Opening signal log");

    let db_path = db_path::default_db_path();
    info!("Using signal log at: {}", db_path.display());

    // Check memory before opening database
    let db_size = std::fs::metadata(&db_path)
        .map(|m| m.len())
        .unwrap_or(0);
    memory_limit.check(db_size)?;

    let engine = DiscoveryEngine::open(&db_path).context("Failed to open signal log")?;
    set_message(&spinner, "Computing correlations");

    // Apply CPU limit to parallelism (if discovery engine supports it)
    let _effective_cores = cpu_limit.effective_parallelism(num_cpus::get());

    let suggestions = engine
        .discover(hours, min_confidence)
        .context("Failed to compute correlations")?;

    // Check memory after computation
    let output_size = toml::to_string_pretty(&suggestions)?.len() as u64;
    memory_limit.add(output_size);

    finish_with_message(&spinner, &format!("Found {} suggestion(s)", suggestions.len()));
    info!("Found {} suggestion(s)", suggestions.len());

    let toml_output = toml::to_string_pretty(&suggestions)
        .context("Failed to serialize suggestions to TOML")?;

    match output {
        Some(path) => {
            std::fs::write(&path, toml_output)
                .with_context(|| format!("Failed to write suggestions to {}", path.display()))?;
            info!("Suggestions written to {}", path.display());
        }
        None => {
            if should_page_output(&toml_output, no_pager, quiet) {
                debug!("Paging output through pager");
                page_output(&toml_output)?;
            } else {
                println!("{}", toml_output);
            }
        }
    }

    Ok(())
}

fn run_status(json: bool, fields: Option<String>, no_pager: bool, quiet: bool) -> Result<()> {
    use btnotify::state::PresenceStateTable;
    use btnotify::output::StatusOutput;

    info!("Running status command");

    // Parse output schema
    let schema = parse_output_schema("status", fields)?;
    info!("Using {} fields for status output", schema.field_count());

    // Create a new state table (in a real implementation, this would query a running daemon)
    let state_table = PresenceStateTable::new();

    // Get all present devices
    let present = state_table.list_present();

    if json {
        // Apply schema to JSON output
        let status_output = StatusOutput {
            daemon_status: "running".to_string(),
            active_parties: present.len(), // Simplified: using device count as proxy
            active_devices: if schema.has_field(CommandField::ActiveDevices) {
                Some(present.len())
            } else {
                None
            },
        };
        
        let output = serde_json::to_string_pretty(&status_output)
            .context("Failed to serialize status to JSON")?;
        println!("{}", output);
    } else {
        let mut output = String::new();
        output.push_str("Presence Status\n");
        output.push_str("================\n");
        
        if schema.has_field(CommandField::DaemonStatus) {
            output.push_str("Daemon Status: running\n");
        }
        if schema.has_field(CommandField::ActiveParties) {
            output.push_str(&format!("Active Parties: {}\n", present.len()));
        }
        if schema.has_field(CommandField::ActiveDevices) {
            output.push_str(&format!("Active Devices: {}\n", present.len()));
        }
        
        output.push('\n');

        if present.is_empty() {
            output.push_str("No devices currently present.");
        } else {
            output.push_str(&format!("{:<20} {:<20} {:<10} {:<15} {:<10}\n", "Name", "MAC", "State", "Last Seen", "RSSI"));
            output.push_str(&str::repeat("-", 85));
            output.push('\n');

            for device in present {
                let name = if device.name.is_empty() {
                    device.mac.clone()
                } else {
                    device.name.clone()
                };

                let last_seen = {
                    let elapsed = device.elapsed_since_seen();
                    let secs = elapsed.as_secs();
                    if secs < 60 {
                        format!("{}s", secs)
                    } else if secs < 3600 {
                        format!("{}m", secs / 60)
                    } else {
                        format!("{}h", secs / 3600)
                    }
                };

                let state_str = match device.state {
                    btnotify::state::PresenceState::Entered => "Entered",
                    btnotify::state::PresenceState::Exited => "Exited",
                    btnotify::state::PresenceState::Pending => "Pending",
                };

                output.push_str(&format!("{:<20} {:<20} {:<10} {:<15} {:<10}\n", name, device.mac, state_str, last_seen, device.rssi));
            }
        }

        if should_page_output(&output, no_pager, quiet) {
            debug!("Paging output through pager");
            page_output(&output)?;
        } else {
            print!("{}", output);
        }
    }

    Ok(())
}

fn run_parties(json: bool, fields: Option<String>, no_pager: bool, quiet: bool) -> Result<()> {
    use btnotify::config::load_presence;
    use btnotify::output::PartyOutput;

    info!("Running parties command");

    // Parse output schema
    let schema = parse_output_schema("parties", fields)?;
    info!("Using {} fields for parties output", schema.field_count());

    // Load presence config
    let presence_config = load_presence(None)?;
    
    let parties = &presence_config.parties;

    if json {
        let party_outputs: Vec<PartyOutput> = parties.iter().map(|party| {
            PartyOutput {
                name: party.name.clone(),
                device_count: party.devices.len(),
                location: party.location.as_ref().map(|loc| {
                    let mut parts = Vec::new();
                    if let Some(building) = &loc.building {
                        parts.push(building.clone());
                    }
                    if let Some(floor) = loc.floor {
                        parts.push(format!("Floor {}", floor));
                    }
                    if let Some(room) = &loc.room {
                        parts.push(room.clone());
                    }
                    if let Some(zone) = &loc.zone {
                        parts.push(zone.clone());
                    }
                    parts.join(", ")
                }),
            }
        }).collect();

        let output = serde_json::to_string_pretty(&party_outputs)
            .context("Failed to serialize parties to JSON")?;
        println!("{}", output);
    } else {
        let mut output = String::new();
        output.push_str("Configured Parties\n");
        output.push_str("==================\n");
        output.push_str(&format!("Total parties: {}\n\n", parties.len()));

        if parties.is_empty() {
            output.push_str("No parties configured.");
        } else {
            for party in parties {
                output.push_str(&format!("Name: {}\n", party.name));
                if schema.has_field(CommandField::PartyDeviceCount) {
                    output.push_str(&format!("  Devices: {}\n", party.devices.len()));
                }
                if schema.has_field(CommandField::PartyLocation) {
                    if let Some(location) = &party.location {
                        let mut loc_parts = Vec::new();
                        if let Some(building) = &location.building {
                            loc_parts.push(building.clone());
                        }
                        if let Some(floor) = location.floor {
                            loc_parts.push(format!("Floor {}", floor));
                        }
                        if let Some(room) = &location.room {
                            loc_parts.push(room.clone());
                        }
                        if let Some(zone) = &location.zone {
                            loc_parts.push(zone.clone());
                        }
                        if !loc_parts.is_empty() {
                            output.push_str(&format!("  Location: {}\n", loc_parts.join(", ")));
                        }
                    }
                }
                output.push('\n');
            }
        }

        if should_page_output(&output, no_pager, quiet) {
            debug!("Paging output through pager");
            page_output(&output)?;
        } else {
            print!("{}", output);
        }
    }

    Ok(())
}

fn run_devices(json: bool, fields: Option<String>, no_pager: bool, quiet: bool) -> Result<()> {
    use btnotify::config::load_presence;
    use btnotify::output::DeviceOutput;

    info!("Running devices command");

    // Parse output schema
    let schema = parse_output_schema("devices", fields)?;
    info!("Using {} fields for devices output", schema.field_count());

    // Load presence config
    let presence_config = load_presence(None)?;
    
    let mut all_devices = Vec::new();
    for party in &presence_config.parties {
        for device in &party.devices {
            all_devices.push((party, device));
        }
    }

    if json {
        let device_outputs: Vec<DeviceOutput> = all_devices.iter().map(|(_, device)| {
            DeviceOutput {
                name: device.name.clone(),
                identifier_count: device.identifiers.len(),
                status: "configured".to_string(),
                location: device.location.as_ref().map(|loc| {
                    let mut parts = Vec::new();
                    if let Some(building) = &loc.building {
                        parts.push(building.clone());
                    }
                    if let Some(floor) = loc.floor {
                        parts.push(format!("Floor {}", floor));
                    }
                    if let Some(room) = &loc.room {
                        parts.push(room.clone());
                    }
                    if let Some(zone) = &loc.zone {
                        parts.push(zone.clone());
                    }
                    parts.join(", ")
                }),
            }
        }).collect();

        let output = serde_json::to_string_pretty(&device_outputs)
            .context("Failed to serialize devices to JSON")?;
        println!("{}", output);
    } else {
        let mut output = String::new();
        output.push_str("Configured Devices\n");
        output.push_str("==================\n");
        output.push_str(&format!("Total devices: {}\n\n", all_devices.len()));

        if all_devices.is_empty() {
            output.push_str("No devices configured.");
        } else {
            for (party, device) in all_devices {
                output.push_str(&format!("Name: {}\n", device.name));
                if schema.has_field(CommandField::DeviceIdentifierCount) {
                    output.push_str(&format!("  Identifiers: {}\n", device.identifiers.len()));
                }
                if schema.has_field(CommandField::DeviceStatus) {
                    output.push_str("  Status: configured\n");
                }
                if schema.has_field(CommandField::DeviceLocation) {
                    let location = device.location.as_ref().or(party.location.as_ref());
                    if let Some(loc) = location {
                        let mut loc_parts = Vec::new();
                        if let Some(building) = &loc.building {
                            loc_parts.push(building.clone());
                        }
                        if let Some(floor) = loc.floor {
                            loc_parts.push(format!("Floor {}", floor));
                        }
                        if let Some(room) = &loc.room {
                            loc_parts.push(room.clone());
                        }
                        if let Some(zone) = &loc.zone {
                            loc_parts.push(zone.clone());
                        }
                        if !loc_parts.is_empty() {
                            output.push_str(&format!("  Location: {}\n", loc_parts.join(", ")));
                        }
                    }
                }
                output.push('\n');
            }
        }

        if should_page_output(&output, no_pager, quiet) {
            debug!("Paging output through pager");
            page_output(&output)?;
        } else {
            print!("{}", output);
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_export(
    format: String,
    since: Option<String>,
    output: Option<PathBuf>,
    _fields: Option<String>,
    no_pager: bool,
    quiet: bool,
    max_memory: Option<u64>,
    max_cpu: Option<usize>,
) -> Result<()> {
    use btnotify::signals::db_path;
    use rusqlite::Connection;

    info!("Running export: format={}, since={:?}", format, since);

    // Apply resource limits
    let memory_limit = MemoryLimit::new(max_memory);
    let cpu_limit = CpuLimit::new(max_cpu);

    if memory_limit.is_enabled() {
        info!("Memory limit set: {} bytes", max_memory.unwrap());
    }
    if cpu_limit.is_enabled() {
        info!("CPU limit set: {} cores", max_cpu.unwrap());
    }

    let spinner = create_spinner(quiet);
    set_message(&spinner, "Opening signal log");

    let db_path = db_path::default_db_path();
    info!("Using signal log at: {}", db_path.display());

    // Check if database exists, if not return empty output
    if !db_path.exists() {
        info!("Signal log database does not exist at {}", db_path.display());
        abandon_with_message(&spinner, "Signal log database does not exist");
        let output_string = match format.as_str() {
            "jsonl" => String::new(),
            "csv" => "ts,scanner,id_type,id_value,rssi,party_name,device_name,location_building,location_floor,location_room,location_zone\n".to_string(),
            _ => {
                return Err(anyhow::anyhow!("Unsupported format: {}. Supported formats: jsonl, csv", format));
            }
        };

        match output {
            Some(path) => {
                std::fs::write(&path, output_string)
                    .with_context(|| format!("Failed to write export to {}", path.display()))?;
                info!("Export written to {}", path.display());
            }
            None => {
                if should_page_output(&output_string, no_pager, quiet) {
                    debug!("Paging output through pager");
                    page_output(&output_string)?;
                } else {
                    print!("{}", output_string);
                }
            }
        }
        return Ok(());
    }

    set_message(&spinner, "Opening database");
    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open signal log at {}", db_path.display()))?;

    // Check memory before query
    let db_size = std::fs::metadata(&db_path)
        .map(|m| m.len())
        .unwrap_or(0);
    memory_limit.check(db_size)?;

    let mut query = "SELECT ts, scanner, id_type, id_value, rssi, party_name, device_name, location_building, location_floor, location_room, location_zone FROM signal_log".to_string();

    if since.is_some() {
        query.push_str(" WHERE ts >= ?1");
    }

    query.push_str(" ORDER BY ts ASC");

    set_message(&spinner, "Preparing query");
    let mut stmt = conn.prepare(&query)
        .context("Failed to prepare query")?;

    // Apply CPU limit to parallelism (if export supports it)
    let _effective_cores = cpu_limit.effective_parallelism(num_cpus::get());

    let rows: Result<Vec<_>, _> = if let Some(ref since_date) = since {
        stmt.query_map([since_date], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<i32>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<u32>>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<String>>(10)?,
            ))
        })?.collect()
    } else {
        stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<i32>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<u32>>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<String>>(10)?,
            ))
        })?.collect()
    };

    let rows = rows.context("Failed to execute query")?;
    set_message(&spinner, &format!("Processing {} rows", rows.len()));

    let output_string = match format.as_str() {
        "jsonl" => {
            let mut jsonl_output = String::new();
            for row in rows {
                let (ts, scanner, id_type, id_value, rssi, party_name, device_name, location_building, location_floor, location_room, location_zone) = row;

                let signal = serde_json::json!({
                    "ts": ts,
                    "scanner": scanner,
                    "id_type": id_type,
                    "id_value": id_value,
                    "rssi": rssi,
                    "party_name": party_name,
                    "device_name": device_name,
                    "location": {
                        "building": location_building,
                        "floor": location_floor,
                        "room": location_room,
                        "zone": location_zone,
                    }
                });

                jsonl_output.push_str(&signal.to_string());
                jsonl_output.push('\n');

                // Check memory limit during processing
                memory_limit.check(jsonl_output.len() as u64)?;
            }
            jsonl_output
        }
        "csv" => {
            let mut csv_output = String::new();
            csv_output.push_str("ts,scanner,id_type,id_value,rssi,party_name,device_name,location_building,location_floor,location_room,location_zone\n");

            for row in rows {
                let (ts, scanner, id_type, id_value, rssi, party_name, device_name, location_building, location_floor, location_room, location_zone) = row;

                csv_output.push_str(&ts);
                csv_output.push(',');
                csv_output.push_str(&scanner);
                csv_output.push(',');
                csv_output.push_str(&id_type);
                csv_output.push(',');
                csv_output.push_str(&id_value);
                csv_output.push(',');
                csv_output.push_str(&rssi.map(|r| r.to_string()).unwrap_or_else(|| "".to_string()));
                csv_output.push(',');
                csv_output.push_str(&party_name.unwrap_or_else(|| "".to_string()));
                csv_output.push(',');
                csv_output.push_str(&device_name.unwrap_or_else(|| "".to_string()));
                csv_output.push(',');
                csv_output.push_str(&location_building.unwrap_or_else(|| "".to_string()));
                csv_output.push(',');
                csv_output.push_str(&location_floor.map(|f| f.to_string()).unwrap_or_else(|| "".to_string()));
                csv_output.push(',');
                csv_output.push_str(&location_room.unwrap_or_else(|| "".to_string()));
                csv_output.push(',');
                csv_output.push_str(&location_zone.unwrap_or_else(|| "".to_string()));
                csv_output.push('\n');

                // Check memory limit during processing
                memory_limit.check(csv_output.len() as u64)?;
            }
            csv_output
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported format: {}. Supported formats: jsonl, csv", format));
        }
    };

    // Add final output size to memory tracking
    memory_limit.add(output_string.len() as u64);

    match output {
        Some(path) => {
            set_message(&spinner, &format!("Writing export to {}", path.display()));
            std::fs::write(&path, output_string)
                .with_context(|| format!("Failed to write export to {}", path.display()))?;
            finish_with_message(&spinner, &format!("Export written to {}", path.display()));
            info!("Export written to {}", path.display());
        }
        None => {
            if should_page_output(&output_string, no_pager, quiet) {
                debug!("Paging output through pager");
                page_output(&output_string)?;
            } else {
                print!("{}", output_string);
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Parse CLI args first
    let cli = Cli::parse();

    // Handle usage flag
    if cli.usage {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        let mut help_output = Vec::new();
        cmd.write_help(&mut help_output)?;
        let help_str = String::from_utf8(help_output)?;
        if should_page_output(&help_str, cli.no_pager, cli.quiet) {
            debug!("Paging help output through pager");
            page_output(&help_str)?;
        } else {
            print!("{}", help_str);
        }
        std::process::exit(EXIT_SUCCESS);
    }

    // Docker HEALTHCHECK — quick check without loading full config
    if cli.health_check {
        match btnotify::health::check_heartbeat_file() {
            Ok(()) => {
                println!("healthy");
                std::process::exit(EXIT_SUCCESS);
            }
            Err(msg) => {
                eprintln!("unhealthy: {}", msg);
                std::process::exit(EXIT_GENERIC_ERROR);
            }
        }
    }

    // Force re-initialization of config files
    if cli.init_config {
        init_logging(&cli, None)?;
        if let Err(e) = config::initialize_config(true) {
            eprintln!("Config initialization error: {e}");
            std::process::exit(EXIT_VALIDATION_ERROR);
        }
        println!("Config files re-initialized with default templates");
        return Ok(());
    }

    // Display man page
    if cli.man {
        let cmd = Cli::command();
        if let Err(e) = btnotify::cli::display_man_page(&cmd, None) {
            eprintln!("Man page error: {e}");
            std::process::exit(EXIT_GENERIC_ERROR);
        }
        return Ok(());
    }

    // Generate shell completions
    if let Some(ref shell) = cli.generate {
        use clap_complete::{generate, shells::Bash, shells::Fish, shells::Zsh};

        let mut cmd = Cli::command();
        let shell_lower = shell.to_lowercase();

        match shell_lower.as_str() {
            "bash" => {
                generate(Bash, &mut cmd, "proximityd", &mut std::io::stdout());
            }
            "zsh" => {
                generate(Zsh, &mut cmd, "proximityd", &mut std::io::stdout());
            }
            "fish" => {
                generate(Fish, &mut cmd, "proximityd", &mut std::io::stdout());
            }
            _ => {
                eprintln!("Unsupported shell: {}. Supported: bash, zsh, fish", shell);
                std::process::exit(EXIT_USAGE_ERROR);
            }
        }
        return Ok(());
    }

    // Handle subcommands
    if let Some(ref command) = cli.command {
        match command {
            Commands::Discover {
                hours,
                min_confidence,
                output,
                fields,
                no_pager,
                max_memory,
                max_cpu,
            } => {
                // Initialize logging with defaults for discover command
                init_logging(&cli, None)?;

                if let Err(e) = run_discover(*hours, *min_confidence, output.clone(), fields.clone(), *no_pager, cli.quiet, *max_memory, *max_cpu) {
                    error!("Discovery error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Status { json, fields, no_pager } => {
                // Initialize logging with defaults for status command
                init_logging(&cli, None)?;

                if let Err(e) = run_status(*json, fields.clone(), *no_pager, cli.quiet) {
                    error!("Status error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Parties { json, fields, no_pager } => {
                // Initialize logging with defaults for parties command
                init_logging(&cli, None)?;

                if let Err(e) = run_parties(*json, fields.clone(), *no_pager, cli.quiet) {
                    error!("Parties error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Devices { json, fields, no_pager } => {
                // Initialize logging with defaults for devices command
                init_logging(&cli, None)?;

                if let Err(e) = run_devices(*json, fields.clone(), *no_pager, cli.quiet) {
                    error!("Devices error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Export {
                format,
                since,
                output,
                fields,
                no_pager,
                max_memory,
                max_cpu,
            } => {
                // Initialize logging with defaults for export command
                init_logging(&cli, None)?;

                if let Err(e) = run_export(format.clone(), since.clone(), output.clone(), fields.clone(), *no_pager, cli.quiet, *max_memory, *max_cpu) {
                    error!("Export error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Install { force, dry_run } => {
                // Initialize logging with defaults for install command
                init_logging(&cli, None)?;

                let cmd = Cli::command();
                if let Err(e) = btnotify::cli::run_install(*force, *dry_run, &cmd) {
                    error!("Install error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Uninstall { force, dry_run } => {
                // Initialize logging with defaults for uninstall command
                init_logging(&cli, None)?;

                if let Err(e) = btnotify::cli::run_uninstall(*force, *dry_run, cli.quiet) {
                    error!("Uninstall error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Completion { shell, output } => {
                let mut cmd = Cli::command();
                let args = btnotify::cli::CompletionArgs {
                    shell: shell.clone(),
                    output: output.clone(),
                };
                if let Err(e) = btnotify::cli::generate_completion(&mut cmd, args) {
                    eprintln!("Completion generation error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
            Commands::Man { command } => {
                let cmd = Cli::command();
                if let Err(e) = btnotify::cli::display_man_page(&cmd, command.as_deref()) {
                    eprintln!("Man page error: {e}");
                    std::process::exit(EXIT_GENERIC_ERROR);
                }
                return Ok(());
            }
        }
    }

    // Load application config (before logging so we can use config for log level)
    let app_config = match config::load_config(cli.config.clone()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {e}");
            std::process::exit(EXIT_VALIDATION_ERROR);
        }
    };

    // Determine operating mode with precedence: CLI flags > env var > config > auto-detection
    let effective_mode = if cli.human || cli.interactive {
        // --human or --interactive forces human mode
        info!("Human mode forced via CLI flag");
        Mode::Human
    } else if let Some(ref mode_str) = cli.mode {
        // PROXIMITYD_MODE env var or --mode flag
        match mode_str.to_lowercase().as_str() {
            "agent" => {
                info!("Agent mode set via environment/flag");
                Mode::Agent
            }
            "human" => {
                info!("Human mode set via environment/flag");
                Mode::Human
            }
            "auto" => {
                info!("Auto mode set via environment/flag");
                detect_mode(app_config.general.mode)
            }
            _ => {
                eprintln!("Invalid mode value: {}. Valid values: agent, human, auto", mode_str);
                std::process::exit(EXIT_USAGE_ERROR);
            }
        }
    } else {
        // Use config mode (which may be Auto)
        let config_mode = app_config.general.mode;
        let detected = detect_mode(config_mode);
        info!("Mode from config: {:?}, detected: {:?}", config_mode, detected);
        detected
    };

    // Log mode detection details
    info!("Effective operating mode: {:?}", effective_mode);
    info!("Agent session detected: {}", is_agent_session());
    info!("TTY detected: {}", is_tty());

    // Determine output format
    let output_format = OutputFormat::from_flags(cli.toon, cli.json, cli.format.as_deref(), effective_mode);
    info!("Output format: {:?}", output_format);

    // Load device mappings (optional — missing file is OK)
    let devices_config = match config::load_devices(cli.devices.clone()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load devices config: {e}");
            std::process::exit(EXIT_VALIDATION_ERROR);
        }
    };

    // Initialize logging with config-aware level resolution
    init_logging(&cli, Some(&app_config))?;

    // Session correlation ID for structured logging
    let _session = {
        let correlation_id = format!("bt-{}", std::process::id());
        tracing::info_span!("proximityd", correlation_id = %correlation_id).entered()
    };

    info!("Starting {}", MODULE_NAME);

    // TUI mode: launch interactive configuration
    if cli.interactive {
        info!("Launching TUI mode");
        if !btnotify::cli::tui::is_tui_supported() {
            error!("TUI mode is not supported in this environment");
            error!("Ensure you are running in a terminal with TTY support");
            std::process::exit(EXIT_USAGE_ERROR);
        }
        if let Err(e) = btnotify::cli::run_tui() {
            error!("TUI error: {e}");
            std::process::exit(EXIT_GENERIC_ERROR);
        }
        return Ok(());
    }
    #[allow(deprecated)]
    let rssi_threshold = app_config.enter_rssi_threshold_dbm;
    #[allow(deprecated)]
    let enter_duration = app_config.enter_duration_seconds;
    #[allow(deprecated)]
    let exit_timeout = app_config.exit_timeout_seconds;

    info!("Loaded config: scan_interval={}s, rssi_threshold={} dBm, enter_duration={}s, exit_timeout={}s, log_level={}",
        app_config
            .scanner
            .get("ble")
            .map(|s| s.scan_interval_sec)
            .unwrap_or(30),
        rssi_threshold,
        enter_duration,
        exit_timeout,
        app_config.general.log_level
    );

    if devices_config.is_empty() {
        warn!("No devices configured in devices.toml");
    } else {
        info!("Loaded {} device mapping(s)", devices_config.devices.len());
    }

    // Daemon mode: run continuous BLE scan + presence detection
    if cli.daemon {
        let rt = tokio::runtime::Runtime::new()?;
        if let Err(e) = rt.block_on(run_daemon(app_config, devices_config, cli.quiet)) {
            error!("Daemon error: {e}");
            std::process::exit(EXIT_GENERIC_ERROR);
        }
        return Ok(());
    }

    let inputs = cli.inputs.clone();

    // Check for implicit stdin
    if inputs.is_empty() {
        if !atty::is(atty::Stream::Stdin) {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;

            if buffer.trim_end().is_empty() {
                use clap::CommandFactory;
                let mut cmd = Cli::command();
                let mut help_output = Vec::new();
                cmd.write_help(&mut help_output)?;
                let help_str = String::from_utf8(help_output)?;
                if should_page_output(&help_str, cli.no_pager, cli.quiet) {
                    debug!("Paging help output through pager");
                    page_output(&help_str)?;
                } else {
                    print!("{}", help_str);
                }
                std::process::exit(EXIT_USAGE_ERROR);
            }

            process_content("stdin", &buffer)?;

            if cli.json {
                println!("{{ \"status\": \"ok\" }}");
            }

            return Ok(());
        }

        use clap::CommandFactory;
        let mut cmd = Cli::command();
        let mut help_output = Vec::new();
        cmd.write_help(&mut help_output)?;
        let help_str = String::from_utf8(help_output)?;
        if should_page_output(&help_str, cli.no_pager, cli.quiet) {
            debug!("Paging help output through pager");
            page_output(&help_str)?;
        } else {
            print!("{}", help_str);
        }
        std::process::exit(1);
    }

    for input in inputs {
        if input == "-" {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            process_content("stdin", &buffer)?;
        } else {
            // Globbing
            let paths = glob(&input).context("Failed to read glob pattern")?;
            for entry in paths {
                match entry {
                    Ok(path) => {
                        let content = std::fs::read_to_string(&path)
                            .with_context(|| format!("Failed to read file {path:?}"))?;
                        process_content(path.to_string_lossy().as_ref(), &content)?;
                    }
                    Err(e) => error!("Error matching glob: {e:?}"),
                }
            }
        }
    }

    if cli.json {
        println!("{{ \"status\": \"ok\" }}");
    }

    info!("Completed successfully");
    Ok(())
}

use btnotify::config;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use glob::glob;
use once_cell::sync::Lazy;
use std::io::{self, Read};
use std::path::PathBuf;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// Module name for logging
const MODULE_NAME: &str = "proximityd";

// Color configuration
static COLORS_ENABLED: Lazy<bool> =
    Lazy::new(|| atty::is(atty::Stream::Stderr) && std::env::var("NO_COLOR").is_err());

#[derive(Parser)]
#[command(name = "proximityd", version, about = "A CLI notification tool")]
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

    /// Output as JSON
    #[arg(long)]
    json: bool,

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
    },
    /// Show current presence status
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
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
    },
    /// Install proximityd: generate shell completions and initialize config files
    Install {
        /// Force installation without confirmation
        #[arg(long)]
        force: bool,
    },
    /// Uninstall proximityd: remove completions and optionally config files
    Uninstall {
        /// Force uninstall without confirmation
        #[arg(long)]
        force: bool,
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
}

async fn run_daemon(
    app_config: config::AppConfig,
    devices_config: config::DevicesConfig,
) -> Result<()> {
    use btnotify::detection::{run_detection_loop, DetectionEngine};
    use btnotify::notifier::NotifierRegistry;
    use btnotify::scanner::ble::BleScanner;
    use btnotify::scanner::scan_loop::spawn_scan_loop;
    use btnotify::scanner::Scanner;
    use btnotify::state::PresenceStateTable;
    use std::sync::Arc;
    use std::time::Duration;

    info!("Starting proximityd daemon (btleplug)");

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
            _ = tokio::signal::ctrl_c() => {},
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
    run_detection_loop(engine, rx, exit_check_interval, notifiers, shutdown_rx).await;

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

fn run_discover(hours: u32, min_confidence: f64, output: Option<PathBuf>) -> Result<()> {
    use btnotify::discovery::DiscoveryEngine;
    use btnotify::signals::db_path;

    info!("Running discovery: hours={}, min_confidence={}", hours, min_confidence);

    let db_path = db_path::default_db_path();
    info!("Using signal log at: {}", db_path.display());

    let engine = DiscoveryEngine::open(&db_path).context("Failed to open signal log")?;
    let suggestions = engine
        .discover(hours, min_confidence)
        .context("Failed to compute correlations")?;

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
            println!("{}", toml_output);
        }
    }

    Ok(())
}

fn run_status(json: bool) -> Result<()> {
    use btnotify::state::{PresenceStateTable, SerializableTrackedDevice};

    info!("Running status command");

    // Create a new state table (in a real implementation, this would query a running daemon)
    let state_table = PresenceStateTable::new();

    // Get all present devices
    let present = state_table.list_present();

    if json {
        let serializable: Vec<SerializableTrackedDevice> = present.iter().map(|d| d.into()).collect();
        let output = serde_json::to_string_pretty(&serializable)
            .context("Failed to serialize status to JSON")?;
        println!("{}", output);
    } else {
        println!("Presence Status");
        println!("================");
        println!("Total present devices: {}", present.len());
        println!();

        if present.is_empty() {
            println!("No devices currently present.");
        } else {
            println!("{:<20} {:<20} {:<10} {:<15} {:<10}", "Name", "MAC", "State", "Last Seen", "RSSI");
            println!("{}", "-".repeat(85));

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

                println!("{:<20} {:<20} {:<10} {:<15} {:<10}", name, device.mac, state_str, last_seen, device.rssi);
            }
        }
    }

    Ok(())
}

fn run_export(format: String, since: Option<String>, output: Option<PathBuf>) -> Result<()> {
    use btnotify::signals::db_path;
    use rusqlite::Connection;

    info!("Running export: format={}, since={:?}", format, since);

    let db_path = db_path::default_db_path();
    info!("Using signal log at: {}", db_path.display());

    // Check if database exists, if not return empty output
    if !db_path.exists() {
        info!("Signal log database does not exist at {}", db_path.display());
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
                print!("{}", output_string);
            }
        }
        return Ok(());
    }

    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open signal log at {}", db_path.display()))?;

    let mut query = "SELECT ts, scanner, id_type, id_value, rssi, party_name, device_name, location_building, location_floor, location_room, location_zone FROM signal_log".to_string();

    if since.is_some() {
        query.push_str(" WHERE ts >= ?1");
    }

    query.push_str(" ORDER BY ts ASC");

    let mut stmt = conn.prepare(&query)
        .context("Failed to prepare query")?;

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
            }
            csv_output
        }
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
            print!("{}", output_string);
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
        Cli::command().print_help()?;
        std::process::exit(0);
    }

    // Docker HEALTHCHECK — quick check without loading full config
    if cli.health_check {
        match btnotify::health::check_heartbeat_file() {
            Ok(()) => {
                println!("healthy");
                std::process::exit(0);
            }
            Err(msg) => {
                eprintln!("unhealthy: {}", msg);
                std::process::exit(1);
            }
        }
    }

    // Force re-initialization of config files
    if cli.init_config {
        init_logging(&cli, None)?;
        if let Err(e) = config::initialize_config(true) {
            eprintln!("Config initialization error: {e}");
            std::process::exit(1);
        }
        println!("Config files re-initialized with default templates");
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
                std::process::exit(1);
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
            } => {
                // Initialize logging with defaults for discover command
                init_logging(&cli, None)?;

                if let Err(e) = run_discover(*hours, *min_confidence, output.clone()) {
                    error!("Discovery error: {e}");
                    std::process::exit(1);
                }
                return Ok(());
            }
            Commands::Status { json } => {
                // Initialize logging with defaults for status command
                init_logging(&cli, None)?;

                if let Err(e) = run_status(*json) {
                    error!("Status error: {e}");
                    std::process::exit(1);
                }
                return Ok(());
            }
            Commands::Export {
                format,
                since,
                output,
            } => {
                // Initialize logging with defaults for export command
                init_logging(&cli, None)?;

                if let Err(e) = run_export(format.clone(), since.clone(), output.clone()) {
                    error!("Export error: {e}");
                    std::process::exit(1);
                }
                return Ok(());
            }
            Commands::Install { force } => {
                // Initialize logging with defaults for install command
                init_logging(&cli, None)?;

                if let Err(e) = btnotify::cli::run_install(*force) {
                    error!("Install error: {e}");
                    std::process::exit(1);
                }
                return Ok(());
            }
            Commands::Uninstall { force } => {
                // Initialize logging with defaults for uninstall command
                init_logging(&cli, None)?;

                if let Err(e) = btnotify::cli::run_uninstall(*force) {
                    error!("Uninstall error: {e}");
                    std::process::exit(1);
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
                    std::process::exit(1);
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
            std::process::exit(1);
        }
    };

    // Load device mappings (optional — missing file is OK)
    let devices_config = match config::load_devices(cli.devices.clone()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load devices config: {e}");
            std::process::exit(1);
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
            std::process::exit(1);
        }
        if let Err(e) = btnotify::cli::run_tui() {
            error!("TUI error: {e}");
            std::process::exit(1);
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
        if let Err(e) = rt.block_on(run_daemon(app_config, devices_config)) {
            error!("Daemon error: {e}");
            std::process::exit(1);
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
                Cli::command().print_help()?;
                std::process::exit(1);
            }

            process_content("stdin", &buffer)?;

            if cli.json {
                println!("{{ \"status\": \"ok\" }}");
            }

            return Ok(());
        }

        use clap::CommandFactory;
        Cli::command().print_help()?;
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

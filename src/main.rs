use btnotify::config;

use anyhow::{Context, Result};
use clap::Parser;
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
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
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

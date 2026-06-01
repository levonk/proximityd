use btnotify::config;

use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::io::{self, Read};
use glob::glob;
use tracing::{Level, debug, info, warn, error};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use once_cell::sync::Lazy;

// Module name for logging
const MODULE_NAME: &str = "btnotify";

// Color configuration
static COLORS_ENABLED: Lazy<bool> = Lazy::new(|| {
    atty::is(atty::Stream::Stderr) && std::env::var("NO_COLOR").is_err()
});

#[derive(Parser)]
#[command(name = "btnotify", version, about = "A CLI notification tool")]
struct Cli {
    /// Input files or glob patterns. Use "-" for stdin.
    #[arg(value_name = "INPUTS")]
    inputs: Vec<String>,

    /// Override config file
    #[arg(long, env = "BTNOTIFY_CONFIG")]
    config: Option<PathBuf>,

    /// Override devices mapping file
    #[arg(long, env = "BTNOTIFY_DEVICES")]
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
}

#[cfg(target_os = "linux")]
async fn run_daemon(app_config: config::AppConfig, devices_config: config::DevicesConfig) -> Result<()> {
    use std::sync::Arc;
    use std::time::Duration;
    use btnotify::bluetooth::{BlueZAdapter, spawn_scan_loop};
    use btnotify::detection::{DetectionEngine, run_detection_loop};
    use btnotify::notifier::NotifierRegistry;
    use btnotify::state::PresenceStateTable;

    info!("Starting btnotify daemon (BlueZ)");

    let notifiers = NotifierRegistry::from_config(&app_config)
        .context("Failed to initialise notifiers from config")?;
    let notifiers = if notifiers.is_empty() {
        info!("No notifiers configured; notifications disabled");
        None
    } else {
        info!("{} notifier(s) active", notifiers.len());
        Some(Arc::new(notifiers))
    };

    let adapter = Arc::new(BlueZAdapter::new().await?);
    let state_table = Arc::new(PresenceStateTable::new());
    let engine = Arc::new(DetectionEngine::new(
        app_config.clone(),
        devices_config,
        state_table,
    ));

    let scan_interval = Duration::from_secs(app_config.scan_interval_seconds);
    let rx = spawn_scan_loop(adapter, scan_interval);

    let exit_check_interval = Duration::from_secs(app_config.exit_timeout_seconds.max(5));
    run_detection_loop(engine, rx, exit_check_interval, notifiers).await;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn run_daemon(_app_config: config::AppConfig, _devices_config: config::DevicesConfig) -> Result<()> {
    anyhow::bail!("Daemon mode requires a BLE adapter; only Linux (BlueZ) is currently supported");
}

fn process_content(source: &str, content: &str) -> Result<()> {
    info!("Processing content from {} ({} bytes)", source, content.len());
    debug!("Content preview: {}", &content[..content.len().min(100)]);
    Ok(())
}

fn init_logging(cli: &Cli) -> Result<()> {
    let log_level = match cli.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let effective_level = if cli.quiet {
        Level::ERROR
    } else {
        log_level
    };

    if cli.nocolor {
        std::env::set_var("NO_COLOR", "1");
    }

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(effective_level.to_string()));

    let fmt_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(*COLORS_ENABLED)
        .with_target(true)
        .with_level(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

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

    // Initialize logging
    init_logging(&cli)?;

    // Handle Ctrl+C gracefully
    ctrlc::set_handler(move || {
        error!("Received Ctrl+C, exiting...");
        std::process::exit(130);
    })?;

    info!("Starting {}", MODULE_NAME);

    // Load application config
    let app_config = match config::load_config(cli.config.clone()) {
        Ok(cfg) => {
            info!("Loaded config: scan_interval={}s, rssi_threshold={} dBm, enter_duration={}s, exit_timeout={}s",
                cfg.scan_interval_seconds,
                cfg.enter_rssi_threshold_dbm,
                cfg.enter_duration_seconds,
                cfg.exit_timeout_seconds
            );
            cfg
        }
        Err(e) => {
            error!("Failed to load config: {e}");
            std::process::exit(1);
        }
    };

    // Load device mappings (optional — missing file is OK)
    let devices_config = match config::load_devices(cli.devices.clone()) {
        Ok(cfg) => {
            if cfg.is_empty() {
                warn!("No devices configured in devices.toml");
            } else {
                info!("Loaded {} device mapping(s)", cfg.devices.len());
            }
            cfg
        }
        Err(e) => {
            error!("Failed to load devices config: {e}");
            std::process::exit(1);
        }
    };

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
                    },
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

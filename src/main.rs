mod config;

use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::io::{self, Read};
use directories::ProjectDirs;
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

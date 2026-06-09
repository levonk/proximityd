//! CLI commands for session hooks

use crate::hooks::{find_proximityd_binary, generate_session_context, register_claude_hooks, register_codex_hooks, SessionContext};
use crate::output::toon::ToonEncoder;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json;

/// Session hook commands
#[derive(Parser, Debug, Clone)]
pub struct HooksArgs {
    #[command(subcommand)]
    pub command: HooksCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum HooksCommand {
    /// Output session context in compact format
    SessionContext {
        /// Output format (toon, json)
        #[arg(long, default_value = "toon")]
        format: String,
        /// Compact output for token budget (minimal fields)
        #[arg(long)]
        compact: bool,
    },
    /// Install agent hooks for ambient context injection
    InstallAgentHooks {
        /// Install hooks for Claude Code
        #[arg(long)]
        claude: bool,
        /// Install hooks for Codex
        #[arg(long)]
        codex: bool,
        /// Install hooks for all supported platforms
        #[arg(long)]
        all: bool,
    },
}

/// Run hooks command
pub fn run_hooks(args: HooksArgs) -> Result<()> {
    match args.command {
        HooksCommand::SessionContext { format, compact } => run_session_context(format, compact),
        HooksCommand::InstallAgentHooks { claude, codex, all } => {
            run_install_agent_hooks(claude, codex, all)
        }
    }
}

/// Run session-context command
fn run_session_context(format: String, compact: bool) -> Result<()> {
    let cwd = std::env::current_dir().context("Failed to get current directory")?;
    let context = generate_session_context(&cwd)?;

    // Apply compact mode if requested (token-budget-aware)
    let output_context = if compact {
        apply_compact_mode(context)
    } else {
        context
    };

    match format.as_str() {
        "toon" => {
            let json_value = serde_json::to_value(&output_context)?;
            let encoder = ToonEncoder::new();
            let toon_output = encoder.encode(&json_value)?;
            println!("{}", toon_output);
        }
        "json" => {
            println!("{}", serde_json::to_string_pretty(&output_context)?);
        }
        _ => {
            anyhow::bail!("Unsupported format: {}. Use 'toon' or 'json'", format);
        }
    }

    Ok(())
}

/// Apply compact mode to reduce token consumption
fn apply_compact_mode(mut context: SessionContext) -> SessionContext {
    use crate::hooks::ConfigSummary;
    
    // In compact mode, only include essential fields
    context.config = ConfigSummary {
        scan_interval: None,
        presence_threshold: None,
        device_count: context.config.device_count,
    };
    
    // Simplify git info
    if let Some(ref mut git) = context.git {
        git.remote = None; // Remove remote URL to save tokens
    }
    
    context
}

/// Run install-agent-hooks command
fn run_install_agent_hooks(claude: bool, codex: bool, all: bool) -> Result<()> {
    let binary_path = find_proximityd_binary()?;
    let mut installed = Vec::new();

    if all || claude {
        register_claude_hooks(&binary_path)
            .context("Failed to register Claude Code hooks")?;
        installed.push("Claude Code");
    }

    if all || codex {
        register_codex_hooks(&binary_path)
            .context("Failed to register Codex hooks")?;
        installed.push("Codex");
    }

    if installed.is_empty() {
        println!("No hooks installed. Specify --claude, --codex, or --all");
    } else {
        println!("Successfully installed hooks for: {}", installed.join(", "));
        println!("Binary path: {}", binary_path.display());
    }

    Ok(())
}

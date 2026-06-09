//! CLI commands for skill generation and management

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::skill::{
    generate_skill, CliMetadata, CommandMetadata, SkillGenerationOptions, SkillFormat,
    check_skill_staleness, write_skill_file,
};

/// Generate agent skill from CLI metadata
#[derive(Parser, Debug)]
pub struct GenerateSkillCommand {
    /// Output file path (default: stdout)
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Include live state in skill (default: false)
    #[arg(long)]
    pub include_live_state: bool,

    /// Use interactive command examples (default: false)
    #[arg(long)]
    pub interactive: bool,

    /// Output format (markdown, json)
    #[arg(long, default_value = "markdown")]
    pub format: String,
}

/// Check if skill file is stale (outdated)
#[derive(Parser, Debug)]
pub struct CheckSkillCommand {
    /// Skill file path to check
    #[arg(value_name = "SKILL_FILE")]
    pub skill_file: PathBuf,
}

/// Run generate-skill command
pub fn run_generate_skill(cmd: &GenerateSkillCommand) -> Result<()> {
    // Create CLI metadata from current binary
    let cli_metadata = create_cli_metadata()?;

    // Parse format
    let format = match cmd.format.to_lowercase().as_str() {
        "markdown" => SkillFormat::Markdown,
        "json" => SkillFormat::Json,
        _ => anyhow::bail!("Invalid format: {}. Use 'markdown' or 'json'", cmd.format),
    };

    // Create generation options
    let options = SkillGenerationOptions {
        include_live_state: cmd.include_live_state,
        non_interactive: !cmd.interactive,
        format,
    };

    // Generate skill
    let skill = generate_skill(&cli_metadata, None, &options)?;

    // Output skill
    if let Some(output_path) = &cmd.output {
        write_skill_file(&skill, output_path)?;
        println!("Skill written to: {}", output_path.display());
    } else {
        println!("{}", skill.content);
    }

    Ok(())
}

/// Run check-skill command
pub fn run_check_skill(cmd: &CheckSkillCommand) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    let is_stale = check_skill_staleness(&cmd.skill_file, current_version)?;

    if is_stale {
        eprintln!("Skill file is stale (outdated). Run 'proximityd skill generate' to update.");
        std::process::exit(1);
    } else {
        println!("Skill file is up to date.");
        Ok(())
    }
}

/// Create CLI metadata from current binary
fn create_cli_metadata() -> Result<CliMetadata> {
    Ok(CliMetadata {
        name: "proximityd".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Generic presence detection service with pluggable notifications".to_string(),
        repository: Some("https://github.com/levonk/proximityd".to_string()),
        commands: vec![
            CommandMetadata {
                name: "discover".to_string(),
                description: "Discover identifier correlations from signal log".to_string(),
                usage: "proximityd discover --hours 24".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "status".to_string(),
                description: "Show current presence status".to_string(),
                usage: "proximityd status".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "parties".to_string(),
                description: "List configured parties (includes device count aggregates)".to_string(),
                usage: "proximityd parties".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "devices".to_string(),
                description: "List configured devices (includes identifier count aggregates)".to_string(),
                usage: "proximityd devices".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "export".to_string(),
                description: "Export signal log data".to_string(),
                usage: "proximityd export --format jsonl".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "install".to_string(),
                description: "Install proximityd: generate shell completions and initialize config files".to_string(),
                usage: "proximityd install".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "uninstall".to_string(),
                description: "Uninstall proximityd: remove completions and optionally config files".to_string(),
                usage: "proximityd uninstall".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "completion".to_string(),
                description: "Generate shell completion script".to_string(),
                usage: "proximityd completion bash".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "man".to_string(),
                description: "Display manual page".to_string(),
                usage: "proximityd man".to_string(),
                subcommands: vec![],
            },
            CommandMetadata {
                name: "hooks".to_string(),
                description: "Session hook commands for ambient context injection".to_string(),
                usage: "proximityd hooks session-context".to_string(),
                subcommands: vec![
                    CommandMetadata {
                        name: "session-context".to_string(),
                        description: "Output session context in TOON format".to_string(),
                        usage: "proximityd hooks session-context".to_string(),
                        subcommands: vec![],
                    },
                    CommandMetadata {
                        name: "install-agent-hooks".to_string(),
                        description: "Install hooks for Claude Code or Codex".to_string(),
                        usage: "proximityd hooks install-agent-hooks --platform claude".to_string(),
                        subcommands: vec![],
                    },
                ],
            },
            CommandMetadata {
                name: "skill".to_string(),
                description: "Generate agent skill for AI integration".to_string(),
                usage: "proximityd skill generate".to_string(),
                subcommands: vec![
                    CommandMetadata {
                        name: "generate".to_string(),
                        description: "Generate agent skill from CLI metadata".to_string(),
                        usage: "proximityd skill generate".to_string(),
                        subcommands: vec![],
                    },
                    CommandMetadata {
                        name: "check".to_string(),
                        description: "Check if skill file is stale (outdated)".to_string(),
                        usage: "proximityd skill check SKILL.md".to_string(),
                        subcommands: vec![],
                    },
                ],
            },
        ],
    })
}

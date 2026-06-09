//! Agent skill generation for AXI compliance
//!
//! This module provides functionality to generate installable agent skills
//! from CLI metadata and session context. Skills provide an alternative to
//! hooks for ambient context injection with AI agents.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Agent skill metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Skill name
    pub name: String,
    /// Skill description
    pub description: String,
    /// Skill version
    pub version: String,
    /// Triggers that activate this skill
    pub triggers: Vec<String>,
    /// Skill author
    pub author: String,
    /// Skill homepage
    pub homepage: Option<String>,
}

/// Generated skill content
#[derive(Debug, Clone)]
pub struct GeneratedSkill {
    /// Skill metadata
    pub metadata: SkillMetadata,
    /// Skill content in markdown format
    pub content: String,
}

/// Skill generation options
#[derive(Debug, Clone)]
pub struct SkillGenerationOptions {
    /// Include live state (default: false for static skills)
    pub include_live_state: bool,
    /// Use non-interactive command examples
    pub non_interactive: bool,
    /// Output format (markdown, json)
    pub format: SkillFormat,
}

/// Skill output format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkillFormat {
    Markdown,
    Json,
}

impl Default for SkillGenerationOptions {
    fn default() -> Self {
        Self {
            include_live_state: false,
            non_interactive: true,
            format: SkillFormat::Markdown,
        }
    }
}

/// Generate agent skill from CLI metadata and session context
pub fn generate_skill(
    cli_metadata: &CliMetadata,
    session_context: Option<&SessionContext>,
    options: &SkillGenerationOptions,
) -> Result<GeneratedSkill> {
    let metadata = create_skill_metadata(cli_metadata)?;
    let content = generate_skill_content(&metadata, cli_metadata, session_context, options)?;

    Ok(GeneratedSkill {
        metadata,
        content,
    })
}

/// CLI metadata for skill generation
#[derive(Debug, Clone)]
pub struct CliMetadata {
    /// CLI name
    pub name: String,
    /// CLI version
    pub version: String,
    /// CLI description
    pub description: String,
    /// Available commands
    pub commands: Vec<CommandMetadata>,
    /// Repository URL
    pub repository: Option<String>,
}

/// Command metadata
#[derive(Debug, Clone)]
pub struct CommandMetadata {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Command usage example
    pub usage: String,
    /// Subcommands (if any)
    pub subcommands: Vec<CommandMetadata>,
}

/// Session context reference (simplified from hooks module)
#[derive(Debug, Clone)]
pub struct SessionContext {
    /// Current working directory
    pub cwd: String,
    /// Git repository information (if in a git repo)
    pub git: Option<GitInfo>,
}

/// Git repository information
#[derive(Debug, Clone)]
pub struct GitInfo {
    /// Repository root path
    pub root: String,
    /// Current branch
    pub branch: Option<String>,
    /// Current commit hash
    pub commit: Option<String>,
    /// Remote URL
    pub remote: Option<String>,
}

/// Create skill metadata from CLI metadata
fn create_skill_metadata(cli_metadata: &CliMetadata) -> Result<SkillMetadata> {
    Ok(SkillMetadata {
        name: cli_metadata.name.clone(),
        description: format!(
            "{} - Agent skill for {}",
            cli_metadata.name, cli_metadata.description
        ),
        version: cli_metadata.version.clone(),
        triggers: vec![
            format!("use {}", cli_metadata.name),
            format!("{} help", cli_metadata.name),
            format!("{} --help", cli_metadata.name),
        ],
        author: "levonk".to_string(),
        homepage: cli_metadata.repository.clone(),
    })
}

/// Generate skill content in markdown format
fn generate_skill_content(
    metadata: &SkillMetadata,
    cli_metadata: &CliMetadata,
    session_context: Option<&SessionContext>,
    options: &SkillGenerationOptions,
) -> Result<String> {
    let mut content = String::new();

    // Add frontmatter
    content.push_str("---");
    content.push('\n');
    content.push_str(&format!("name: \"{}\"\n", metadata.name));
    content.push_str(&format!("description: \"{}\"\n", metadata.description));
    content.push_str(&format!("version: \"{}\"\n", metadata.version));
    content.push_str("triggers:");
    content.push('\n');
    for trigger in &metadata.triggers {
        content.push_str(&format!("  - \"{}\"\n", trigger));
    }
    if let Some(homepage) = &metadata.homepage {
        content.push_str(&format!("homepage: \"{}\"\n", homepage));
    }
    content.push_str("---");
    content.push('\n');
    content.push('\n');

    // Add skill description
    content.push_str(&format!("# {}\n\n", metadata.name));
    content.push_str(&format!("{}\n\n", metadata.description));

    // Add overview section
    content.push_str("## Overview");
    content.push('\n');
    content.push('\n');
    content.push_str(&format!(
        "{} is a CLI tool for {}. This skill provides ambient context and command examples for AI agents.\n\n",
        cli_metadata.name, cli_metadata.description
    ));

    // Add session context if available and not stripping live state
    if let Some(ctx) = session_context {
        if options.include_live_state {
            content.push_str("## Current Session Context");
            content.push('\n');
            content.push('\n');
            content.push_str(&format!("**Working Directory:** `{}`\n\n", ctx.cwd));
            if let Some(git) = &ctx.git {
                content.push_str("**Git Repository:**");
                content.push('\n');
                content.push_str(&format!("- Root: `{}`\n", git.root));
                if let Some(branch) = &git.branch {
                    content.push_str(&format!("- Branch: `{}`\n", branch));
                }
                if let Some(commit) = &git.commit {
                    content.push_str(&format!("- Commit: `{}`\n", commit));
                }
                if let Some(remote) = &git.remote {
                    content.push_str(&format!("- Remote: `{}`\n", remote));
                }
                content.push('\n');
            }
        }
    }

    // Add commands section
    content.push_str("## Available Commands");
    content.push('\n');
    content.push('\n');
    for cmd in &cli_metadata.commands {
        content.push_str(&format!("### {}\n\n", cmd.name));
        content.push_str(&format!("{}\n\n", cmd.description));
        
        // Add usage example (non-interactive if requested)
        let usage = if options.non_interactive {
            rewrite_to_non_interactive(&cmd.usage)
        } else {
            cmd.usage.clone()
        };
        content.push_str(&format!("**Usage:**\n```\n{}\n```\n\n", usage));

        // Add subcommands if any
        if !cmd.subcommands.is_empty() {
            content.push_str("**Subcommands:**");
            content.push('\n');
            content.push('\n');
            for subcmd in &cmd.subcommands {
                content.push_str(&format!("- `{}`: {}\n", subcmd.name, subcmd.description));
            }
            content.push('\n');
        }
    }

    // Add examples section
    content.push_str("## Examples");
    content.push('\n');
    content.push('\n');
    content.push_str("### Common Workflows");
    content.push('\n');
    content.push('\n');
    content.push_str(&format!("```bash\n# Get help\n{} --help\n\n", cli_metadata.name));
    content.push_str(&format!("# Check status\n{} status\n\n", cli_metadata.name));
    content.push_str(&format!("# List devices\n{} devices\n\n", cli_metadata.name));

    Ok(content)
}

/// Rewrite command examples to non-interactive form
fn rewrite_to_non_interactive(usage: &str) -> String {
    // Remove interactive flags like --interactive, -i
    let usage = usage.replace("--interactive", "").replace("-i", "");
    
    // Remove TUI-related flags
    let usage = usage.replace("--tui", "");
    
    // Clean up extra spaces
    usage.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Check if a skill file is stale (outdated compared to current CLI)
pub fn check_skill_staleness(skill_path: &Path, current_version: &str) -> Result<bool> {
    if !skill_path.exists() {
        return Ok(true); // File doesn't exist, needs generation
    }

    let content = fs::read_to_string(skill_path)
        .context("Failed to read skill file")?;

    // Extract version from skill file
    let skill_version = extract_version_from_skill(&content)?;

    Ok(skill_version != current_version)
}

/// Extract version from skill content
fn extract_version_from_skill(content: &str) -> Result<String> {
    // Look for version in frontmatter
    for line in content.lines() {
        if line.starts_with("version:") {
            let version = line
                .strip_prefix("version:")
                .unwrap()
                .trim()
                .trim_matches('"')
                .to_string();
            return Ok(version);
        }
    }

    anyhow::bail!("Version not found in skill file")
}

/// Write generated skill to file
pub fn write_skill_file(skill: &GeneratedSkill, output_path: &Path) -> Result<()> {
    fs::write(output_path, &skill.content)
        .context("Failed to write skill file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_to_non_interactive() {
        let input = "proximityd --interactive config";
        let output = rewrite_to_non_interactive(input);
        assert!(!output.contains("--interactive"));
        assert!(!output.contains("-i"));
    }

    #[test]
    fn test_extract_version_from_skill() {
        let content = r#"---
name: "proximityd"
version: "1.0.0"
---
"#;
        let version = extract_version_from_skill(content).unwrap();
        assert_eq!(version, "1.0.0");
    }

    #[test]
    fn test_check_skill_staleness_missing_file() {
        let result = check_skill_staleness(Path::new("/nonexistent/file.md"), "1.0.0");
        assert!(result.unwrap()); // Should return true (stale) for missing file
    }
}

//! Session hook infrastructure for ambient context injection
//!
//! This module provides the infrastructure for registering and managing
//! session hooks that enable ambient context injection for AI agents.
//! Supports Claude Code, Codex, and future agent platforms.

use anyhow::{Context, Result};
use directories;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Session context output in TOON format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    /// Current working directory
    pub cwd: String,
    /// Git repository information (if in a git repo)
    pub git: Option<GitInfo>,
    /// Proximityd configuration summary
    pub config: ConfigSummary,
    /// Active devices/presence state
    pub presence: PresenceSummary,
    /// Session metadata
    pub metadata: SessionMetadata,
}

/// Git repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Configuration summary for session context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSummary {
    /// Scan interval in seconds
    pub scan_interval: Option<u64>,
    /// Presence threshold in seconds
    pub presence_threshold: Option<u64>,
    /// Number of configured devices
    pub device_count: usize,
}

/// Presence state summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceSummary {
    /// Number of currently present devices
    pub present_count: usize,
    /// Number of configured devices
    pub total_count: usize,
}

/// Session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Session start time
    pub start_time: String,
    /// Proximityd version
    pub version: String,
    /// Operating system
    pub os: String,
    /// Cached session ID for context enrichment
    pub session_id: Option<String>,
}

/// Hook configuration for different agent platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    /// Claude Code hooks
    pub claude: Option<ClaudeHookConfig>,
    /// Codex hooks
    pub codex: Option<CodexHookConfig>,
}

/// Claude Code hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeHookConfig {
    /// Path to Claude Code settings file
    pub settings_path: PathBuf,
    /// Hook command to register
    pub hook_command: String,
}

/// Codex hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexHookConfig {
    /// Path to Codex hooks file
    pub hooks_path: PathBuf,
    /// Hook command to register
    pub hook_command: String,
}

/// Generate session context for the current working directory
pub fn generate_session_context(cwd: &Path) -> Result<SessionContext> {
    let cwd_str = cwd
        .canonicalize()
        .context("Failed to canonicalize CWD")?
        .to_string_lossy()
        .to_string();

    let git_info = detect_git_info(cwd);
    let config_summary = get_config_summary()?;
    let presence_summary = get_presence_summary()?;
    let metadata = get_session_metadata();

    Ok(SessionContext {
        cwd: cwd_str,
        git: git_info,
        config: config_summary,
        presence: presence_summary,
        metadata,
    })
}

/// Detect git repository information
fn detect_git_info(cwd: &Path) -> Option<GitInfo> {
    // Try to find .git directory
    let mut current = cwd;
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            // Found git repository
            let root = current.to_string_lossy().to_string();
            
            // Try to get branch and commit info
            let branch = get_git_branch(current);
            let commit = get_git_commit(current);
            let remote = get_git_remote(current);
            
            return Some(GitInfo {
                root,
                branch,
                commit,
                remote,
            });
        }
        
        // Move to parent directory
        match current.parent() {
            Some(parent) => current = parent,
            None => return None, // Reached root without finding .git
        }
    }
}

/// Get current git branch
fn get_git_branch(repo_root: &Path) -> Option<String> {
    let head_path = repo_root.join(".git").join("HEAD");
    if let Ok(content) = fs::read_to_string(&head_path) {
        // HEAD format: "ref: refs/heads/branch-name" or direct commit hash
        if content.starts_with("ref: refs/heads/") {
            Some(content.trim().strip_prefix("ref: refs/heads/")?.to_string())
        } else {
            // Detached HEAD - return commit hash
            Some(content.trim().to_string())
        }
    } else {
        None
    }
}

/// Get current git commit hash
fn get_git_commit(repo_root: &Path) -> Option<String> {
    let head_path = repo_root.join(".git").join("HEAD");
    if let Ok(content) = fs::read_to_string(&head_path) {
        if content.starts_with("ref: ") {
            // Follow the reference
            let ref_path = content.trim().strip_prefix("ref: ")?;
            let full_ref_path = repo_root.join(".git").join(ref_path);
            if let Ok(commit) = fs::read_to_string(&full_ref_path) {
                return Some(commit.trim().to_string());
            }
        } else {
            // Direct commit hash
            return Some(content.trim().to_string());
        }
    }
    None
}

/// Get git remote URL
fn get_git_remote(repo_root: &Path) -> Option<String> {
    let config_path = repo_root.join(".git").join("config");
    if let Ok(content) = fs::read_to_string(&config_path) {
        // Parse git config to find remote URL
        for line in content.lines() {
            if line.trim().starts_with("url = ") {
                return Some(line.trim().strip_prefix("url = ")?.to_string());
            }
        }
    }
    None
}

/// Get configuration summary
fn get_config_summary() -> Result<ConfigSummary> {
    // Try to load config from default location
    let proj_dirs = directories::ProjectDirs::from("com.github", "levonk", "proximityd")
        .context("Failed to determine config directory")?;
    
    let config_dir = proj_dirs.config_dir();
    let config_file = config_dir.join("config.toml");
    let presence_file = config_dir.join("presence.toml");
    
    let scan_interval = if config_file.exists() {
        // Parse config to get scan interval
        // For now, return None - will implement proper parsing
        None
    } else {
        None
    };
    
    let presence_threshold = None; // Will implement proper parsing
    
    let device_count = if presence_file.exists() {
        // Count devices in presence.toml
        // For now, return 0 - will implement proper parsing
        0
    } else {
        0
    };
    
    Ok(ConfigSummary {
        scan_interval,
        presence_threshold,
        device_count,
    })
}

/// Get presence summary
fn get_presence_summary() -> Result<PresenceSummary> {
    // For now, return empty summary
    // Will implement actual presence state detection
    Ok(PresenceSummary {
        present_count: 0,
        total_count: 0,
    })
}

/// Get session metadata
fn get_session_metadata() -> SessionMetadata {
    SessionMetadata {
        start_time: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        session_id: Some(uuid::Uuid::new_v4().to_string()),
    }
}

/// Find the proximityd binary path
pub fn find_proximityd_binary() -> Result<PathBuf> {
    // First, try to find via PATH
    if let Ok(path) = which::which("proximityd") {
        return Ok(path);
    }
    
    // Fallback to current executable
    std::env::current_exe().context("Failed to get current executable path")
}

/// Register Claude Code hooks
pub fn register_claude_hooks(binary_path: &Path) -> Result<()> {
    let base_dirs = directories::BaseDirs::new()
        .context("Failed to get base directories")?;
    
    let claude_dir = base_dirs.home_dir().join(".claude");
    
    fs::create_dir_all(&claude_dir)
        .context("Failed to create Claude directory")?;
    
    let settings_path = claude_dir.join("settings.json");
    
    // Read existing settings or create new
    let mut settings: serde_json::Value = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)
            .context("Failed to read Claude settings")?;
        serde_json::from_str(&content)
            .context("Failed to parse Claude settings")?
    } else {
        serde_json::json!({})
    };
    
    // Add session-start hook
    let hook_command = format!(
        "{} session-context --format toon",
        binary_path.display()
    );
    
    // Ensure hooks object exists
    if !settings.as_object().map(|o| o.contains_key("hooks")).unwrap_or(false) {
        settings["hooks"] = serde_json::json!({});
    }
    
    // Add session-start hook
    settings["hooks"]["session-start"] = serde_json::json!({
        "command": hook_command
    });
    
    // Write updated settings
    fs::write(&settings_path, serde_json::to_string_pretty(&settings)?)
        .context("Failed to write Claude settings")?;
    
    Ok(())
}

/// Register Codex hooks
pub fn register_codex_hooks(binary_path: &Path) -> Result<()> {
    let base_dirs = directories::BaseDirs::new()
        .context("Failed to get base directories")?;
    
    let codex_dir = base_dirs.home_dir().join(".codex");
    
    fs::create_dir_all(&codex_dir)
        .context("Failed to create Codex directory")?;
    
    let hooks_path = codex_dir.join("hooks.json");
    
    // Read existing hooks or create new
    let mut hooks: serde_json::Value = if hooks_path.exists() {
        let content = fs::read_to_string(&hooks_path)
            .context("Failed to read Codex hooks")?;
        serde_json::from_str(&content)
            .context("Failed to parse Codex hooks")?
    } else {
        serde_json::json!({})
    };
    
    // Add session-start hook
    let hook_command = format!(
        "{} session-context --format toon",
        binary_path.display()
    );
    
    hooks["session-start"] = serde_json::json!({
        "command": hook_command
    });
    
    // Write updated hooks
    fs::write(&hooks_path, serde_json::to_string_pretty(&hooks)?)
        .context("Failed to write Codex hooks")?;
    
    Ok(())
}

/// Register session-end hook
pub fn register_session_end_hook(_binary_path: &Path) -> Result<()> {
    // Session-end hooks are platform-specific
    // For now, this is a placeholder for future implementation
    // The hook would be registered alongside session-start hooks
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_generate_session_context() {
        let cwd = env::current_dir().unwrap();
        let context = generate_session_context(&cwd).unwrap();
        
        assert!(!context.cwd.is_empty());
        assert_eq!(context.metadata.os, std::env::consts::OS);
    }
    
    #[test]
    fn test_detect_git_info() {
        let cwd = env::current_dir().unwrap();
        let git_info = detect_git_info(&cwd);
        
        // May or may not be in a git repo
        if let Some(info) = git_info {
            assert!(!info.root.is_empty());
        }
    }
    
    #[test]
    fn test_session_metadata() {
        let metadata = get_session_metadata();
        
        assert!(!metadata.start_time.is_empty());
        assert!(!metadata.version.is_empty());
        assert!(!metadata.os.is_empty());
    }
}

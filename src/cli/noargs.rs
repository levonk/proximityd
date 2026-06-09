//! No-args content-first behavior
//!
//! This module implements content-first no-args behavior that shows
//! the most relevant live state instead of the usage manual.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// No-args context for content display
#[derive(Debug, Clone)]
pub struct NoArgsContext {
    /// Current working directory
    pub cwd: String,
    /// Is in config directory
    pub is_config_dir: bool,
    /// Is daemon running
    pub is_daemon_running: bool,
    /// Config directory path
    pub config_dir: Option<String>,
}

/// No-args content summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoArgsSummary {
    /// Context type
    pub context: String,
    /// Summary message
    pub summary: String,
    /// Help suggestions
    pub suggestions: Vec<String>,
    /// Daemon status (if applicable)
    pub daemon_status: Option<String>,
    /// Parties count (if in config dir)
    pub parties_count: Option<usize>,
}

/// Detect no-args context
pub fn detect_noargs_context() -> Result<NoArgsContext> {
    let cwd = std::env::current_dir()
        .context("Failed to get current directory")?
        .to_string_lossy()
        .to_string();

    // Check if in config directory
    let is_config_dir = is_in_config_directory(&cwd);

    // Check if daemon is running
    let is_daemon_running = check_daemon_running();

    // Get config directory
    let config_dir = get_config_directory();

    Ok(NoArgsContext {
        cwd,
        is_config_dir,
        is_daemon_running,
        config_dir,
    })
}

/// Check if current directory is a config directory
fn is_in_config_directory(cwd: &str) -> bool {
    // Check if we're in a directory that contains config files
    let config_files = ["config.toml", "presence.toml", "devices.toml"];
    
    for config_file in &config_files {
        let path = std::path::Path::new(cwd).join(config_file);
        if path.exists() {
            return true;
        }
    }
    
    false
}

/// Check if daemon is running
fn check_daemon_running() -> bool {
    // Try to detect if proximityd daemon is running
    // This is a simple check - in production, you might use PID files or process detection
    #[cfg(target_os = "linux")]
    {
        // On Linux, check for proximityd process
        if let Ok(output) = std::process::Command::new("pgrep")
            .arg("proximityd")
            .arg("--daemon")
            .output()
        {
            if !output.stdout.is_empty() {
                return true;
            }
        }
    }
    
    false
}

/// Get config directory
fn get_config_directory() -> Option<String> {
    directories::ProjectDirs::from("com.github", "levonk", "proximityd")
        .map(|proj_dirs| proj_dirs.config_dir().to_string_lossy().to_string())
}

/// Generate no-args summary based on context
pub fn generate_noargs_summary(context: &NoArgsContext) -> NoArgsSummary {
    let mut suggestions = vec![
        "proximityd --help".to_string(),
        "proximityd status".to_string(),
    ];

    let (context_type, summary, daemon_status, parties_count) = if context.is_config_dir {
        suggestions.push("proximityd parties".to_string());
        suggestions.push("proximityd devices".to_string());
        
        let parties_count = count_parties_in_config(&context.cwd);
        
        (
            "config_directory".to_string(),
            format!("In config directory with {} parties", parties_count),
            None,
            Some(parties_count),
        )
    } else if context.is_daemon_running {
        suggestions.push("proximityd --daemon --status".to_string());
        
        (
            "daemon_running".to_string(),
            "Daemon is running and monitoring devices".to_string(),
            Some("active".to_string()),
            None,
        )
    } else {
        suggestions.push("proximityd install".to_string());
        suggestions.push("proximityd --daemon".to_string());
        
        (
            "default".to_string(),
            "Proximityd presence detection service".to_string(),
            Some("stopped".to_string()),
            None,
        )
    };

    NoArgsSummary {
        context: context_type,
        summary,
        suggestions,
        daemon_status,
        parties_count,
    }
}

/// Count parties in config directory
fn count_parties_in_config(cwd: &str) -> usize {
    // Try to parse presence.toml to count parties
    let presence_path = std::path::Path::new(cwd).join("presence.toml");
    if presence_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&presence_path) {
            // Simple count of [[parties]] sections
            content.matches("[[parties]]").count()
        } else {
            0
        }
    } else {
        0
    }
}

/// Format no-args summary for human mode
pub fn format_noargs_human(summary: &NoArgsSummary) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("{}\n", summary.summary));
    output.push('\n');
    
    if let Some(ref status) = summary.daemon_status {
        output.push_str(&format!("Daemon Status: {}\n", status));
        output.push('\n');
    }
    
    if let Some(count) = summary.parties_count {
        output.push_str(&format!("Configured Parties: {}\n", count));
        output.push('\n');
    }
    
    output.push_str("Common Commands:\n");
    for suggestion in &summary.suggestions {
        output.push_str(&format!("  {}\n", suggestion));
    }
    
    output.push('\n');
    output.push_str("Use --help for detailed usage information\n");
    
    output
}

/// Format no-args summary for agent mode (TOON format)
pub fn format_noargs_toon(summary: &NoArgsSummary) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("context:{}\n", summary.context));
    output.push_str(&format!("summary:{}\n", summary.summary));
    
    if let Some(ref status) = summary.daemon_status {
        output.push_str(&format!("daemon_status:{}\n", status));
    }
    
    if let Some(count) = summary.parties_count {
        output.push_str(&format!("parties_count:{}\n", count));
    }
    
    output.push_str("suggestions:");
    for suggestion in &summary.suggestions {
        output.push_str(&format!(" {}", suggestion));
    }
    output.push('\n');
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_noargs_context() {
        let context = detect_noargs_context().unwrap();
        assert!(!context.cwd.is_empty());
    }

    #[test]
    fn test_generate_noargs_summary_default() {
        let context = NoArgsContext {
            cwd: "/test/path".to_string(),
            is_config_dir: false,
            is_daemon_running: false,
            config_dir: None,
        };
        
        let summary = generate_noargs_summary(&context);
        assert_eq!(summary.context, "default");
        assert_eq!(summary.daemon_status, Some("stopped".to_string()));
        assert!(summary.suggestions.contains(&"proximityd --help".to_string()));
    }

    #[test]
    fn test_generate_noargs_summary_config_dir() {
        let context = NoArgsContext {
            cwd: "/test/path".to_string(),
            is_config_dir: true,
            is_daemon_running: false,
            config_dir: None,
        };
        
        let summary = generate_noargs_summary(&context);
        assert_eq!(summary.context, "config_directory");
        assert!(summary.suggestions.contains(&"proximityd parties".to_string()));
        assert!(summary.parties_count.is_some());
    }

    #[test]
    fn test_generate_noargs_summary_daemon_running() {
        let context = NoArgsContext {
            cwd: "/test/path".to_string(),
            is_config_dir: false,
            is_daemon_running: true,
            config_dir: None,
        };
        
        let summary = generate_noargs_summary(&context);
        assert_eq!(summary.context, "daemon_running");
        assert_eq!(summary.daemon_status, Some("active".to_string()));
    }

    #[test]
    fn test_format_noargs_human() {
        let summary = NoArgsSummary {
            context: "default".to_string(),
            summary: "Test summary".to_string(),
            suggestions: vec!["cmd1".to_string(), "cmd2".to_string()],
            daemon_status: Some("stopped".to_string()),
            parties_count: None,
        };
        
        let formatted = format_noargs_human(&summary);
        assert!(formatted.contains("Test summary"));
        assert!(formatted.contains("Daemon Status: stopped"));
        assert!(formatted.contains("cmd1"));
        assert!(formatted.contains("--help"));
    }

    #[test]
    fn test_format_noargs_toon() {
        let summary = NoArgsSummary {
            context: "default".to_string(),
            summary: "Test summary".to_string(),
            suggestions: vec!["cmd1".to_string(), "cmd2".to_string()],
            daemon_status: Some("stopped".to_string()),
            parties_count: None,
        };
        
        let formatted = format_noargs_toon(&summary);
        assert!(formatted.contains("context:default"));
        assert!(formatted.contains("summary:Test summary"));
        assert!(formatted.contains("daemon_status:stopped"));
        assert!(formatted.contains("suggestions:"));
    }
}

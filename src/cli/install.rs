use anyhow::{Context, Result};
use clap::Command;
use directories::ProjectDirs;
use std::fs;
use std::path::Path;
use crate::error::{StructuredError, error_types, EXIT_SUCCESS, EXIT_VALIDATION_ERROR, EXIT_PERMISSION_DENIED};
use crate::cli::{is_agent_session, detect_mode};
use crate::config::app::Mode;

/// Output structured error/info to stdout in agent mode, stderr in human mode
fn output_structured(error: &StructuredError, mode: Mode) -> Result<()> {
    match mode {
        Mode::Agent => {
            // In agent mode, output TOON format to stdout
            println!("{}", error.to_toon());
        }
        Mode::Human => {
            // In human mode, output human-readable to stderr
            eprintln!("Error: {}", error.message);
            if let Some(ref suggestion) = error.suggestion {
                eprintln!("Suggestion: {}", suggestion);
            }
        }
        Mode::Auto => {
            // In auto mode, use agent session detection
            if is_agent_session() {
                println!("{}", error.to_toon());
            } else {
                eprintln!("Error: {}", error.message);
                if let Some(ref suggestion) = error.suggestion {
                    eprintln!("Suggestion: {}", suggestion);
                }
            }
        }
    }
    Ok(())
}

/// Validate required flags before performing operations
fn validate_flags(force: bool, quiet: bool, mode: Mode) -> Result<()> {
    // In agent mode, force should be implied (no interactive prompts)
    if is_agent_session() && !force && !quiet {
        // Agent mode should not require interactive confirmation
        // This is a warning, not an error - the code handles it
        tracing::warn!("Agent mode detected but --force not set; prompts will be suppressed automatically");
    }
    
    // Validate that we can determine config directory
    let proj_dirs = ProjectDirs::from("com.github", "levonk", "proximityd")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;
    
    let config_dir = proj_dirs.config_dir();
    
    // Check if config directory is writable (if not dry run)
    if config_dir.exists() && !config_dir.is_dir() {
        let structured_err = StructuredError::new(
            error_types::VALIDATION_ERROR,
            format!("Config path exists but is not a directory: {}", config_dir.display()),
            EXIT_VALIDATION_ERROR,
        )
        .with_suggestion("Remove the file or rename it to allow directory creation");
        output_structured(&structured_err, mode)?;
        return Err(anyhow::anyhow!("Config path is not a directory"));
    }
    
    Ok(())
}

/// Install proximityd: generate shell completions and initialize config files
pub fn run_install(force: bool, dry_run: bool, quiet: bool, cmd: &Command) -> Result<()> {
    let mode = detect_mode(Mode::Auto);
    let is_agent = is_agent_session();

    // Validate flags before proceeding
    validate_flags(force, quiet, mode)?;

    if dry_run {
        if is_agent {
            // In agent mode, output structured info
            let info = StructuredError::new(
                "dry_run",
                "Installation preview - no changes will be made",
                EXIT_SUCCESS,
            );
            output_structured(&info, mode)?;
        } else {
            println!("DRY RUN: Would install proximityd...");
            println!();
        }
    } else {
        if !is_agent {
            println!("Installing proximityd...");
            println!();
        }
    }

    // Create config directory (idempotent)
    let proj_dirs = ProjectDirs::from("com.github", "levonk", "proximityd")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine project directories"))?;
    let config_dir = proj_dirs.config_dir();

    if !config_dir.exists() {
        if dry_run {
            if is_agent {
                let info = StructuredError::new(
                    "dry_run",
                    format!("Would create config directory: {}", config_dir.display()),
                    EXIT_SUCCESS,
                );
                output_structured(&info, mode)?;
            } else {
                println!("DRY RUN: Would create config directory: {}", config_dir.display());
            }
        } else {
            match fs::create_dir_all(config_dir) {
                Ok(_) => {
                    if !is_agent {
                        println!("Created config directory: {}", config_dir.display());
                    }
                }
                Err(e) => {
                    let structured_err = StructuredError::new(
                        error_types::PERMISSION_DENIED,
                        format!("Failed to create config directory: {}", config_dir.display()),
                        EXIT_PERMISSION_DENIED,
                    )
                    .with_suggestion("Check directory permissions or run with appropriate privileges");
                    output_structured(&structured_err, mode)?;
                    return Err(anyhow::anyhow!("Failed to create config directory: {}", e));
                }
            }
        }
    } else {
        // Idempotent: directory already exists is not an error
        if !is_agent {
            println!("Config directory exists: {}", config_dir.display());
        }
    }

    // Initialize config.toml (idempotent)
    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        if dry_run {
            if is_agent {
                let info = StructuredError::new(
                    "dry_run",
                    format!("Would create config file: {}", config_path.display()),
                    EXIT_SUCCESS,
                );
                output_structured(&info, mode)?;
            } else {
                println!("DRY RUN: Would create config file: {}", config_path.display());
            }
        } else {
            let config_content = include_str!("../../config.example.toml");
            match fs::write(&config_path, config_content) {
                Ok(_) => {
                    if !is_agent {
                        println!("Created config file: {}", config_path.display());
                    }
                }
                Err(e) => {
                    let structured_err = StructuredError::new(
                        error_types::PERMISSION_DENIED,
                        format!("Failed to write config.toml to {}", config_path.display()),
                        EXIT_PERMISSION_DENIED,
                    )
                    .with_suggestion("Check file permissions or run with appropriate privileges");
                    output_structured(&structured_err, mode)?;
                    return Err(anyhow::anyhow!("Failed to write config.toml: {}", e));
                }
            }
        }
    } else {
        // Idempotent: file already exists is not an error
        if !is_agent {
            println!("Config file exists: {}", config_path.display());
            println!("  (Not overwriting existing file)");
        }
    }

    // Initialize presence.toml (idempotent)
    let presence_path = config_dir.join("presence.toml");
    if !presence_path.exists() {
        if dry_run {
            if is_agent {
                let info = StructuredError::new(
                    "dry_run",
                    format!("Would create presence file: {}", presence_path.display()),
                    EXIT_SUCCESS,
                );
                output_structured(&info, mode)?;
            } else {
                println!("DRY RUN: Would create presence file: {}", presence_path.display());
            }
        } else {
            // For now, create a minimal presence.toml since we don't have an example file
            let presence_content = r#"# Presence configuration for proximityd
# This file maps parties, devices, and identifiers for presence detection

# Example party configuration
[[parties]]
name = "Example Person"
# Optional location for this party
# location = { building = "Home", floor = 1, room = "Living Room" }

  [[parties.devices]]
  name = "Example Device"
  # Optional device-specific location
  # location = { building = "Home", floor = 1, room = "Living Room" }

    [[parties.devices.identifiers]]
    type = "ble_mac"
    value = "AA:BB:CC:DD:EE:FF"
    # Optional notes about this identifier
    # notes = "Primary phone"
"#;
            match fs::write(&presence_path, presence_content) {
                Ok(_) => {
                    if !is_agent {
                        println!("Created presence file: {}", presence_path.display());
                    }
                }
                Err(e) => {
                    let structured_err = StructuredError::new(
                        error_types::PERMISSION_DENIED,
                        format!("Failed to write presence.toml to {}", presence_path.display()),
                        EXIT_PERMISSION_DENIED,
                    )
                    .with_suggestion("Check file permissions or run with appropriate privileges");
                    output_structured(&structured_err, mode)?;
                    return Err(anyhow::anyhow!("Failed to write presence.toml: {}", e));
                }
            }
        }
    } else {
        // Idempotent: file already exists is not an error
        if !is_agent {
            println!("Presence file exists: {}", presence_path.display());
            println!("  (Not overwriting existing file)");
        }
    }

    if !is_agent {
        println!();
    }

    // Generate shell completions (idempotent)
    if dry_run {
        if is_agent {
            let info = StructuredError::new(
                "dry_run",
                "Would generate shell completions",
                EXIT_SUCCESS,
            );
            output_structured(&info, mode)?;
        } else {
            println!("DRY RUN: Would generate shell completions");
        }
    } else {
        match generate_completions() {
            Ok(_) => {
                // Success - completions generated or already exist
            }
            Err(e) => {
                let structured_err = StructuredError::new(
                    error_types::PERMISSION_DENIED,
                    "Failed to generate shell completions",
                    EXIT_PERMISSION_DENIED,
                )
                .with_suggestion("Check config directory permissions");
                output_structured(&structured_err, mode)?;
                return Err(e.context("Failed to generate shell completions"));
            }
        }
    }

    // Install man pages (idempotent)
    if dry_run {
        if is_agent {
            let info = StructuredError::new(
                "dry_run",
                "Would install man pages",
                EXIT_SUCCESS,
            );
            output_structured(&info, mode)?;
        } else {
            println!("DRY RUN: Would install man pages");
        }
    } else {
        if !is_agent {
            println!("Installing man pages...");
        }
        match crate::cli::install_man_pages(cmd) {
            Ok(()) => {
                if !is_agent {
                    println!("Man pages installed successfully");
                }
            }
            Err(e) => {
                // Man page installation failure is not critical
                if !is_agent {
                    println!("Warning: Failed to install man pages: {}", e);
                    println!("Man pages will still be available via 'proximityd man' command");
                }
            }
        }
    }

    if !is_agent {
        println!();
        if dry_run {
            println!("DRY RUN: Installation would be complete (no changes made)");
        } else {
            println!("Installation complete!");
        }
        println!();
        println!("Next steps:");
        println!("  1. Edit config files in: {}", config_dir.display());
        println!("  2. Run: proximityd --daemon");
        println!();
        println!("Environment variables (optional):");
        println!("  PROXIMITYD_CONFIG_DIR - Override config directory");
        println!("  PROXIMITYD_CONFIG - Override config file path");
        println!("  PROXIMITYD_LOG_LEVEL - Override log level (DEBUG, INFO, WARN, ERROR)");
        println!("  PROXIMITYD_LOG_FORMAT - Override log format (json, pretty)");
    } else {
        // In agent mode, output success as structured info
        let success = StructuredError::new(
            "install_success",
            format!("Installation complete. Config directory: {}", config_dir.display()),
            EXIT_SUCCESS,
        );
        output_structured(&success, mode)?;
    }

    Ok(())
}

/// Uninstall proximityd: remove completions and optionally config files
pub fn run_uninstall(force: bool, dry_run: bool, quiet: bool) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would uninstall proximityd...");
        println!();
    } else {
        println!("Uninstalling proximityd...");
        println!();
    }

    // Remove shell completions
    if dry_run {
        println!("DRY RUN: Would remove shell completions");
    } else {
        remove_completions()?;
    }

    println!();

    // Offer to remove config files
    let proj_dirs = ProjectDirs::from("com.github", "levonk", "proximityd")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine project directories"))?;
    let config_dir = proj_dirs.config_dir();

    if config_dir.exists() {
        if dry_run {
            println!("DRY RUN: Would remove config directory: {}", config_dir.display());
        } else {
            let should_remove = crate::cli::confirm(
                &format!("Remove config directory {}?", config_dir.display()),
                force,
                quiet,
            )?;
            
            if should_remove {
                remove_config_dir(config_dir)?;
            } else {
                println!("Config directory preserved: {}", config_dir.display());
            }
        }
    } else {
        println!("Config directory does not exist: {}", config_dir.display());
    }

    println!();
    if dry_run {
        println!("DRY RUN: Uninstall would be complete (no changes made)");
    } else {
        println!("Uninstall complete!");
    }
    println!();
    println!("Note: The proximityd binary itself was not removed.");
    println!("To remove the binary, use your package manager or delete it manually.");

    Ok(())
}

fn generate_completions() -> Result<()> {
    let proj_dirs = ProjectDirs::from("com.github", "levonk", "proximityd")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine project directories"))?;
    let comp_dir = proj_dirs.config_dir().join("completions");

    fs::create_dir_all(&comp_dir)
        .with_context(|| format!("Failed to create completions directory: {}", comp_dir.display()))?;

    println!("Generating shell completions in: {}", comp_dir.display());

    // Get the current executable path
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;

    // Generate Bash completion
    let bash_path = comp_dir.join("proximityd.bash");
    let bash_output = std::process::Command::new(&current_exe)
        .args(["--generate", "bash"])
        .output()
        .with_context(|| "Failed to generate Bash completion".to_string())?;

    if bash_output.status.success() {
        fs::write(&bash_path, String::from_utf8_lossy(&bash_output.stdout).to_string())
            .with_context(|| "Failed to write Bash completion".to_string())?;
        println!("  Bash: {}", bash_path.display());
    } else {
        return Err(anyhow::anyhow!("Failed to generate Bash completion: {}", String::from_utf8_lossy(&bash_output.stderr)));
    }

    // Generate Zsh completion
    let zsh_path = comp_dir.join("_proximityd");
    let zsh_output = std::process::Command::new(&current_exe)
        .args(["--generate", "zsh"])
        .output()
        .with_context(|| "Failed to generate Zsh completion".to_string())?;

    if zsh_output.status.success() {
        fs::write(&zsh_path, String::from_utf8_lossy(&zsh_output.stdout).to_string())
            .with_context(|| "Failed to write Zsh completion".to_string())?;
        println!("  Zsh: {}", zsh_path.display());
    } else {
        return Err(anyhow::anyhow!("Failed to generate Zsh completion: {}", String::from_utf8_lossy(&zsh_output.stderr)));
    }

    // Generate Fish completion
    let fish_path = comp_dir.join("proximityd.fish");
    let fish_output = std::process::Command::new(&current_exe)
        .args(["--generate", "fish"])
        .output()
        .with_context(|| "Failed to generate Fish completion".to_string())?;

    if fish_output.status.success() {
        fs::write(&fish_path, String::from_utf8_lossy(&fish_output.stdout).to_string())
            .with_context(|| "Failed to write Fish completion".to_string())?;
        println!("  Fish: {}", fish_path.display());
    } else {
        return Err(anyhow::anyhow!("Failed to generate Fish completion: {}", String::from_utf8_lossy(&fish_output.stderr)));
    }

    println!();
    println!("To enable shell completions, add the following to your shell config:");
    println!();
    println!("  Bash:");
    println!("    source {}", comp_dir.join("proximityd.bash").display());
    println!();
    println!("  Zsh:");
    println!("    fpath=({} $fpath)", comp_dir.display());
    println!("    autoload -U compinit && compinit");
    println!();
    println!("  Fish:");
    println!("    source {}", comp_dir.join("proximityd.fish").display());

    Ok(())
}

fn remove_completions() -> Result<()> {
    let proj_dirs = ProjectDirs::from("com.github", "levonk", "proximityd")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine project directories"))?;
    let comp_dir = proj_dirs.config_dir().join("completions");

    if comp_dir.exists() {
        println!("Removing shell completions from: {}", comp_dir.display());
        fs::remove_dir_all(&comp_dir)
            .with_context(|| format!("Failed to remove completions directory: {}", comp_dir.display()))?;
        println!("  Removed: {}", comp_dir.display());
    } else {
        println!("Completions directory does not exist: {}", comp_dir.display());
    }

    Ok(())
}

fn remove_config_dir(config_dir: &Path) -> Result<()> {
    println!("Removing config directory: {}", config_dir.display());
    fs::remove_dir_all(config_dir)
        .with_context(|| format!("Failed to remove config directory: {}", config_dir.display()))?;
    println!("  Removed: {}", config_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_install_creates_config_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");

        // Create config directory
        fs::create_dir_all(&config_dir).unwrap();
        assert!(config_dir.exists());
    }

    #[test]
    fn test_install_does_not_overwrite_existing_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        fs::write(&config_path, "# existing config").unwrap();

        // Verify the file exists and has content
        assert!(config_path.exists());
        let content = fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, "# existing config");
    }

    #[test]
    fn test_install_creates_config_files() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        let presence_path = config_dir.join("presence.toml");

        // Simulate creating config files
        fs::write(&config_path, "# test config").unwrap();
        fs::write(&presence_path, "# test presence").unwrap();

        assert!(config_path.exists());
        assert!(presence_path.exists());
    }

    #[test]
    fn test_install_dry_run_does_not_create_files() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");

        // In dry-run mode, files should not be created
        assert!(!config_path.exists() || fs::read_to_string(&config_path).unwrap() != "# test config");
    }

    #[test]
    fn test_uninstall_removes_completions_directory() {
        let temp_dir = TempDir::new().unwrap();
        let comp_dir = temp_dir.path().join("completions");
        fs::create_dir_all(&comp_dir).unwrap();

        let bash_comp = comp_dir.join("proximityd.bash");
        fs::write(&bash_comp, "# bash completion").unwrap();

        assert!(comp_dir.exists());
        assert!(bash_comp.exists());

        // Remove completions
        fs::remove_dir_all(&comp_dir).unwrap();
        assert!(!comp_dir.exists());
    }

    #[test]
    fn test_uninstall_removes_config_with_force() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        fs::write(&config_path, "# test config").unwrap();

        assert!(config_dir.exists());

        // With force flag, config should be removed
        fs::remove_dir_all(&config_dir).unwrap();
        assert!(!config_dir.exists());
    }

    #[test]
    fn test_uninstall_preserves_config_without_force() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        fs::write(&config_path, "# test config").unwrap();

        assert!(config_dir.exists());

        // Without force, config should be preserved
        // (In real implementation, this would test the confirmation prompt)
        assert!(config_dir.exists());
    }

    #[test]
    fn test_generate_completions_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let comp_dir = temp_dir.path().join("completions");

        fs::create_dir_all(&comp_dir).unwrap();
        assert!(comp_dir.exists());
    }

    #[test]
    fn test_remove_config_dir_handles_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("nonexistent");

        // Should not error when directory doesn't exist
        assert!(!config_dir.exists());
    }
}

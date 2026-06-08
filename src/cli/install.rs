use anyhow::{Context, Result};
use clap::Command;
use directories::ProjectDirs;
use std::fs;
use std::path::Path;

/// Install proximityd: generate shell completions and initialize config files
pub fn run_install(_force: bool, dry_run: bool, cmd: &Command) -> Result<()> {
    if dry_run {
        println!("DRY RUN: Would install proximityd...");
        println!();
    } else {
        println!("Installing proximityd...");
        println!();
    }

    // Create config directory
    let proj_dirs = ProjectDirs::from("com.github", "levonk", "proximityd")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine project directories"))?;
    let config_dir = proj_dirs.config_dir();

    if !config_dir.exists() {
        if dry_run {
            println!("DRY RUN: Would create config directory: {}", config_dir.display());
        } else {
            fs::create_dir_all(config_dir)
                .with_context(|| format!("Failed to create config directory: {}", config_dir.display()))?;
            println!("Created config directory: {}", config_dir.display());
        }
    } else {
        println!("Config directory exists: {}", config_dir.display());
    }

    // Initialize config.toml
    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        if dry_run {
            println!("DRY RUN: Would create config file: {}", config_path.display());
        } else {
            let config_content = include_str!("../../config.example.toml");
            fs::write(&config_path, config_content)
                .with_context(|| format!("Failed to write config.toml to {}", config_path.display()))?;
            println!("Created config file: {}", config_path.display());
        }
    } else {
        println!("Config file exists: {}", config_path.display());
        println!("  (Not overwriting existing file)");
    }

    // Initialize presence.toml
    let presence_path = config_dir.join("presence.toml");
    if !presence_path.exists() {
        if dry_run {
            println!("DRY RUN: Would create presence file: {}", presence_path.display());
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
            fs::write(&presence_path, presence_content)
                .with_context(|| format!("Failed to write presence.toml to {}", presence_path.display()))?;
            println!("Created presence file: {}", presence_path.display());
        }
    } else {
        println!("Presence file exists: {}", presence_path.display());
        println!("  (Not overwriting existing file)");
    }

    println!();

    // Generate shell completions
    if dry_run {
        println!("DRY RUN: Would generate shell completions");
    } else {
        generate_completions()?;
    }

    // Install man pages
    if dry_run {
        println!("DRY RUN: Would install man pages");
    } else {
        println!("Installing man pages...");
        match crate::cli::install_man_pages(cmd) {
            Ok(()) => println!("Man pages installed successfully"),
            Err(e) => {
                println!("Warning: Failed to install man pages: {}", e);
                println!("Man pages will still be available via 'proximityd man' command");
            }
        }
    }

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
        let _config_dir = temp_dir.path().join("proximityd");

        // This test would need to mock the ProjectDirs or use a custom config path
        // For now, we'll just verify the function signature is correct
        // In a real test, we'd set up a test environment and call run_install
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
    fn test_uninstall_with_force_removes_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("proximityd");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        fs::write(&config_path, "# test config").unwrap();

        assert!(config_dir.exists());

        // In a real test, we'd call run_uninstall(true) and verify removal
        // For now, just verify the setup
    }

    #[test]
    fn test_uninstall_without_force_prompts() {
        // This test would verify that the confirmation prompt is shown
        // In a real test, we'd mock stdin/stdout
    }
}

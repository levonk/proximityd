use anyhow::{Context, Result};
use clap::Command;
use clap_mangen::Man;
use std::fs::{self, File};
use std::path::Path;

/// Generate man pages for all commands
pub fn generate_man_pages(cmd: &Command, output_dir: &Path) -> Result<()> {
    // Generate main man page
    generate_single_man_page(cmd, output_dir, "proximityd.1")?;

    // Generate subcommand man pages
    for subcommand in cmd.get_subcommands() {
        let name = subcommand.get_name();
        let filename = format!("proximityd-{}.1", name);
        generate_single_man_page(subcommand, output_dir, &filename)?;
    }

    Ok(())
}

/// Generate a single man page for a command
fn generate_single_man_page(cmd: &Command, output_dir: &Path, filename: &str) -> Result<()> {
    let output_path = output_dir.join(filename);
    let mut file = File::create(&output_path)
        .with_context(|| format!("Failed to create man page file: {}", output_path.display()))?;

    let man = Man::new(cmd.clone());
    man.render(&mut file)
        .with_context(|| format!("Failed to render man page to: {}", output_path.display()))?;

    Ok(())
}

/// Generate and display man page to stdout
pub fn display_man_page(cmd: &Command, command_name: Option<&str>) -> Result<()> {
    let target_cmd = if let Some(name) = command_name {
        // Find the subcommand
        cmd.get_subcommands()
            .find(|sub| sub.get_name() == name)
            .with_context(|| format!("Unknown subcommand: {}", name))?
    } else {
        cmd
    };

    let man = Man::new(target_cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)
        .context("Failed to render man page")?;

    print!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}

/// Install man pages to system directory
pub fn install_man_pages(cmd: &Command) -> Result<()> {
    let man_dir = Path::new("/usr/local/share/man/man1");

    // Check if directory is writable
    if !man_dir.exists() {
        fs::create_dir_all(man_dir)
            .with_context(|| format!("Failed to create man directory: {}", man_dir.display()))?;
    }

    if let Err(e) = fs::metadata(man_dir) {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            eprintln!("Warning: Cannot install man pages to {}", man_dir.display());
            eprintln!("Permission denied. Try running with sudo.");
            return Ok(());
        }
    }

    // Generate man pages to a temporary directory first
    let temp_dir = std::env::temp_dir().join("proximityd-man-pages");
    fs::create_dir_all(&temp_dir)
        .context("Failed to create temporary directory for man pages")?;

    generate_man_pages(cmd, &temp_dir)
        .context("Failed to generate man pages")?;

    // Copy generated man pages to system directory
    for entry in fs::read_dir(&temp_dir)
        .context("Failed to read temporary man pages directory")?
    {
        let entry = entry.context("Failed to read directory entry")?;
        let src_path = entry.path();
        let filename = entry.file_name();
        let dst_path = man_dir.join(&filename);

        fs::copy(&src_path, &dst_path)
            .with_context(|| format!("Failed to copy man page to: {}", dst_path.display()))?;
    }

    // Clean up temporary directory
    fs::remove_dir_all(&temp_dir)
        .context("Failed to clean up temporary directory")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_man_pages() {
        let temp_dir = std::env::temp_dir().join("proximityd-man-test");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a simple command for testing
        let cmd = clap::Command::new("proximityd")
            .about("Test command")
            .subcommand(clap::Command::new("status").about("Show status"));

        let result = generate_man_pages(&cmd, &temp_dir);

        assert!(result.is_ok());

        // Check that main man page was created
        let main_man = temp_dir.join("proximityd.1");
        assert!(main_man.exists());

        // Check that subcommand man pages were created
        let status_man = temp_dir.join("proximityd-status.1");
        assert!(status_man.exists());

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_display_man_page() {
        let cmd = clap::Command::new("proximityd").about("Test command");
        let result = display_man_page(&cmd, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_man_page_subcommand() {
        let cmd = clap::Command::new("proximityd")
            .about("Test command")
            .subcommand(clap::Command::new("status").about("Show status"));
        let result = display_man_page(&cmd, Some("status"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_man_page_invalid_subcommand() {
        let cmd = clap::Command::new("proximityd").about("Test command");
        let result = display_man_page(&cmd, Some("invalid"));
        assert!(result.is_err());
    }
}

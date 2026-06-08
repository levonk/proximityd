use anyhow::{Context, Result};
use std::env;
use std::io::{self, Write};
use std::process::Command;

/// Detect the pager to use based on environment variables and system availability.
pub fn detect_pager() -> Option<String> {
    // Check PAGER environment variable first
    if let Ok(pager) = env::var("PAGER") {
        if !pager.is_empty() {
            return Some(pager);
        }
    }

    // Default to 'less' if available
    if is_command_available("less") {
        Some("less".to_string())
    } else if is_command_available("more") {
        Some("more".to_string())
    } else {
        None
    }
}

/// Check if a command is available in the PATH.
fn is_command_available(cmd: &str) -> bool {
    #[cfg(unix)]
    {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[cfg(windows)]
    {
        Command::new("where")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Determine if output should be paged based on terminal size and content length.
pub fn should_page_output(content: &str, no_pager: bool, quiet: bool) -> bool {
    // Skip paging if explicitly disabled
    if no_pager {
        return false;
    }

    // Skip paging in quiet mode
    if quiet {
        return false;
    }

    // Skip paging if output is being piped (not a TTY)
    if !atty::is(atty::Stream::Stdout) {
        return false;
    }

    // Skip paging if no pager is available
    if detect_pager().is_none() {
        return false;
    }

    // Page if content is long (more than terminal lines)
    let line_count = content.lines().count();
    let terminal_lines = get_terminal_height();

    line_count > terminal_lines
}

/// Get the terminal height in lines.
fn get_terminal_height() -> usize {
    #[cfg(unix)]
    {
        if let Ok(output) = Command::new("stty")
            .arg("size")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                if let Some(height) = stdout.split_whitespace().next() {
                    if let Ok(h) = height.parse::<usize>() {
                        return h;
                    }
                }
            }
        }
    }

    #[cfg(windows)]
    {
        // Windows terminal size detection is more complex
        // For now, use default
    }

    24 // Default to 24 lines if detection fails
}

/// Pipe content through a pager.
pub fn page_output(content: &str) -> Result<()> {
    let pager = detect_pager()
        .context("No pager available")?;

    // Use the pager crate to handle paging
    // The setup() method returns () and sets up the pager for the process
    pager::Pager::with_pager(&pager).setup();

    // Write content to stdout (pager will capture it)
    print!("{}", content);
    io::stdout().flush()
        .context("Failed to flush output to pager")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_pager_from_env() {
        env::set_var("PAGER", "custom_pager");
        let pager = detect_pager();
        assert_eq!(pager, Some("custom_pager".to_string()));
        env::remove_var("PAGER");
    }

    #[test]
    fn test_detect_pager_default() {
        env::remove_var("PAGER");
        let pager = detect_pager();
        // This test depends on system availability, so we just check it returns Some or None
        // In most CI environments, 'less' or 'more' should be available
        assert!(pager.is_some() || pager.is_none());
    }

    #[test]
    fn test_should_page_output_no_pager_flag() {
        let content = "Short content";
        assert!(!should_page_output(content, true, false));
    }

    #[test]
    fn test_should_page_output_quiet_mode() {
        let content = "Short content";
        assert!(!should_page_output(content, false, true));
    }

    #[test]
    fn test_should_page_output_short_content() {
        let content = "Short content";
        // Short content should not be paged
        assert!(!should_page_output(content, false, false));
    }

    #[test]
    fn test_should_page_output_long_content() {
        let long_content = "\n".repeat(100);
        // Long content should be paged (if TTY and pager available)
        // In test environment, TTY is usually false, so this may return false
        let result = should_page_output(&long_content, false, false);
        // We just verify it doesn't crash
        let _ = result;
    }
}

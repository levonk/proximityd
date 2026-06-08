use std::path::Path;

/// Terminal size information
#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self {
            width: 80,  // Default to 80 columns
            height: 24, // Default to 24 rows
        }
    }
}

/// Detect terminal size
/// Returns terminal width and height, or defaults if detection fails
pub fn detect_terminal_size() -> TerminalSize {
    match crossterm::terminal::size() {
        Ok((width, height)) => {
            tracing::debug!("Detected terminal size: {}x{}", width, height);
            TerminalSize { width, height }
        }
        Err(e) => {
            tracing::debug!("Failed to detect terminal size: {}, using defaults", e);
            TerminalSize::default()
        }
    }
}

/// Wrap text to fit within terminal width
pub fn wrap_text(text: &str, width: usize) -> String {
    textwrap::fill(text, width)
}

/// Truncate text with ellipsis if it exceeds max width
pub fn truncate_text(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        text.to_string()
    } else if max_width <= 3 {
        // Not enough space for ellipsis
        text[..max_width].to_string()
    } else {
        format!("{}...", &text[..max_width - 3])
    }
}

/// Format a table row with column widths
pub fn format_table_row(columns: &[&str], widths: &[usize]) -> String {
    let mut result = String::new();
    for (i, (col, &width)) in columns.iter().zip(widths.iter()).enumerate() {
        if i > 0 {
            result.push(' ');
        }
        if col.len() > width {
            result.push_str(&truncate_text(col, width));
        } else {
            result.push_str(col);
            // Pad with spaces
            for _ in 0..(width - col.len()) {
                result.push(' ');
            }
        }
    }
    result
}

/// Check if terminal resize handling is supported
/// Returns true if we're in a TTY and can detect resize events
pub fn is_resize_supported() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Get current terminal size (cached)
/// This is a convenience wrapper around detect_terminal_size
pub fn get_terminal_size() -> TerminalSize {
    detect_terminal_size()
}

/// Handle terminal resize event
/// This is a stub for future async signal handling
/// Currently just logs the resize event
pub fn handle_resize() {
    let size = detect_terminal_size();
    tracing::debug!("Terminal resized to {}x{}", size.width, size.height);
}

/// Format a file reference in VSCode-compatible format
/// Format: file:///absolute/path/to/file:line:column
pub fn format_file_reference(path: &Path, line: Option<u32>, column: Option<u32>) -> String {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .ok()
            .and_then(|cwd| cwd.join(path).canonicalize().ok())
            .unwrap_or_else(|| path.to_path_buf())
    };

    let mut result = format!("file://{}", absolute.display());

    if let Some(line) = line {
        result.push(':');
        result.push_str(&line.to_string());

        if let Some(col) = column {
            result.push(':');
            result.push_str(&col.to_string());
        }
    }

    result
}

/// Format a file reference in simple format
/// Format: file:line:column (relative path if possible)
pub fn format_file_reference_simple(path: &Path, line: Option<u32>, column: Option<u32>) -> String {
    let display_path = if path.is_absolute() {
        // Try to make it relative to current directory
        std::env::current_dir()
            .ok()
            .and_then(|cwd| path.strip_prefix(&cwd).ok())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| path.to_path_buf())
    } else {
        path.to_path_buf()
    };

    let mut result = display_path.display().to_string();

    if let Some(line) = line {
        result.push(':');
        result.push_str(&line.to_string());

        if let Some(col) = column {
            result.push(':');
            result.push_str(&col.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_reference_absolute() {
        let path = Path::new("/tmp/test.rs");
        let result = format_file_reference(path, Some(10), Some(5));
        assert_eq!(result, "file:///tmp/test.rs:10:5");
    }

    #[test]
    fn test_format_file_reference_line_only() {
        let path = Path::new("/tmp/test.rs");
        let result = format_file_reference(path, Some(10), None);
        assert_eq!(result, "file:///tmp/test.rs:10");
    }

    #[test]
    fn test_format_file_reference_no_line() {
        let path = Path::new("/tmp/test.rs");
        let result = format_file_reference(path, None, None);
        assert_eq!(result, "file:///tmp/test.rs");
    }

    #[test]
    fn test_format_file_reference_simple() {
        let path = Path::new("src/main.rs");
        let result = format_file_reference_simple(path, Some(42), Some(3));
        assert!(result.contains("src/main.rs:42:3"));
    }

    #[test]
    fn test_format_file_reference_simple_absolute() {
        let path = Path::new("/tmp/test.rs");
        let result = format_file_reference_simple(path, Some(10), None);
        // Should try to make relative if possible, otherwise absolute
        assert!(result.contains("test.rs:10"));
    }

    #[test]
    fn test_detect_terminal_size() {
        let size = detect_terminal_size();
        // Should return a valid size (either detected or default)
        assert!(size.width > 0);
        assert!(size.height > 0);
    }

    #[test]
    fn test_terminal_size_default() {
        let size = TerminalSize::default();
        assert_eq!(size.width, 80);
        assert_eq!(size.height, 24);
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a long line that should be wrapped";
        let wrapped = wrap_text(text, 20);
        assert!(wrapped.contains('\n'));
    }

    #[test]
    fn test_truncate_text() {
        let text = "This is a long string";
        let truncated = truncate_text(text, 10);
        assert_eq!(truncated.len(), 10);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_truncate_text_short() {
        let text = "Short";
        let truncated = truncate_text(text, 20);
        assert_eq!(truncated, "Short");
    }

    #[test]
    fn test_format_table_row() {
        let columns = vec!["Name", "Value"];
        let widths = vec![10, 15];
        let row = format_table_row(&columns, &widths);
        assert!(row.starts_with("Name"));
        assert!(row.contains("Value"));
    }

    #[test]
    fn test_is_resize_supported() {
        // Just check that the function runs without panicking
        let _supported = is_resize_supported();
    }

    #[test]
    fn test_get_terminal_size() {
        let size = get_terminal_size();
        assert!(size.width > 0);
        assert!(size.height > 0);
    }

    #[test]
    fn test_handle_resize() {
        // Just check that the function runs without panicking
        handle_resize();
    }
}

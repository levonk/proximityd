use anyhow::{Context, Result};
use glob::glob;
use std::path::PathBuf;
use tracing::debug;

/// Expand glob patterns to file paths
pub fn expand_glob(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in glob(pattern).context("Failed to read glob pattern")? {
        match entry {
            Ok(path) => {
                debug!("Glob matched: {}", path.display());
                paths.push(path);
            }
            Err(e) => {
                tracing::warn!("Glob error: {}", e);
            }
        }
    }

    Ok(paths)
}

/// Check if a pattern is a glob pattern (contains * or ?)
pub fn is_glob_pattern(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?')
}

/// Expand input arguments, handling both glob patterns and literal paths
pub fn expand_inputs(inputs: &[String]) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for input in inputs {
        if input == "-" {
            // stdin marker - return as special path
            paths.push(PathBuf::from("-"));
        } else if is_glob_pattern(input) {
            // Expand glob pattern
            let matched = expand_glob(input)?;
            if matched.is_empty() {
                tracing::warn!("Glob pattern matched no files: {}", input);
            }
            paths.extend(matched);
        } else {
            // Literal path
            paths.push(PathBuf::from(input));
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_glob_pattern() {
        assert!(is_glob_pattern("*.txt"));
        assert!(is_glob_pattern("test?"));
        assert!(is_glob_pattern("**/*.rs"));
        assert!(!is_glob_pattern("file.txt"));
        assert!(!is_glob_pattern("path/to/file"));
    }

    #[test]
    fn test_expand_inputs_literal() {
        let inputs = vec!["file1.txt".to_string(), "file2.txt".to_string()];
        let paths = expand_inputs(&inputs).unwrap();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], PathBuf::from("file1.txt"));
        assert_eq!(paths[1], PathBuf::from("file2.txt"));
    }

    #[test]
    fn test_expand_inputs_stdin() {
        let inputs = vec!["-".to_string()];
        let paths = expand_inputs(&inputs).unwrap();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], PathBuf::from("-"));
    }

    #[test]
    fn test_expand_inputs_glob() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        // Create test files
        fs::write(dir.join("test1.txt"), "content1").unwrap();
        fs::write(dir.join("test2.txt"), "content2").unwrap();
        fs::write(dir.join("other.rs"), "content3").unwrap();

        let pattern = dir.join("*.txt").to_string_lossy().to_string();
        let inputs = vec![pattern];
        let paths = expand_inputs(&inputs).unwrap();

        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|p| p.ends_with("test1.txt")));
        assert!(paths.iter().any(|p| p.ends_with("test2.txt")));
    }

    #[test]
    fn test_expand_inputs_mixed() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        // Create test files
        fs::write(dir.join("test.txt"), "content").unwrap();

        let pattern = dir.join("*.txt").to_string_lossy().to_string();
        let inputs = vec!["-".to_string(), pattern, "literal.txt".to_string()];
        let paths = expand_inputs(&inputs).unwrap();

        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], PathBuf::from("-"));
        assert!(paths[1].ends_with("test.txt"));
        assert_eq!(paths[2], PathBuf::from("literal.txt"));
    }
}

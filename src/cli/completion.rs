use anyhow::Result;
use clap::{Command, Parser};
use std::fs;
use std::path::PathBuf;

/// Generate shell completion scripts for bash, zsh, or fish
#[derive(Parser, Debug)]
pub struct CompletionArgs {
    /// Shell type (bash, zsh, fish)
    #[arg(value_name = "SHELL")]
    pub shell: String,

    /// Output file path (default: stdout)
    #[arg(long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

/// Generate shell completion for the specified shell
pub fn generate_completion(cmd: &mut Command, args: CompletionArgs) -> Result<()> {
    let shell = args.shell.to_lowercase();

    if let Some(output_path) = args.output {
        // Create parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        // Generate to buffer then write to file
        let mut buffer = Vec::new();
        match shell.as_str() {
            "bash" => {
                clap_complete::generate(clap_complete::Shell::Bash, cmd, "proximityd", &mut buffer);
            }
            "zsh" => {
                clap_complete::generate(clap_complete::Shell::Zsh, cmd, "proximityd", &mut buffer);
            }
            "fish" => {
                clap_complete::generate(clap_complete::Shell::Fish, cmd, "proximityd", &mut buffer);
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported shell: {}. Supported shells: bash, zsh, fish",
                    args.shell
                ))
            }
        }
        fs::write(&output_path, buffer)?;
        println!("Completion script written to: {}", output_path.display());
    } else {
        // Write to stdout
        match shell.as_str() {
            "bash" => {
                clap_complete::generate(clap_complete::Shell::Bash, cmd, "proximityd", &mut std::io::stdout());
            }
            "zsh" => {
                clap_complete::generate(clap_complete::Shell::Zsh, cmd, "proximityd", &mut std::io::stdout());
            }
            "fish" => {
                clap_complete::generate(clap_complete::Shell::Fish, cmd, "proximityd", &mut std::io::stdout());
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported shell: {}. Supported shells: bash, zsh, fish",
                    args.shell
                ))
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;
    use tempfile::TempDir;

    #[test]
    fn test_completion_args_parsing() {
        let args = CompletionArgs {
            shell: "bash".to_string(),
            output: None,
        };
        assert_eq!(args.shell, "bash");
        assert!(args.output.is_none());
    }

    #[test]
    fn test_completion_args_with_output() {
        let args = CompletionArgs {
            shell: "zsh".to_string(),
            output: Some(PathBuf::from("/tmp/completion.sh")),
        };
        assert_eq!(args.shell, "zsh");
        assert!(args.output.is_some());
    }

    #[test]
    fn test_completion_args_case_insensitive() {
        let args = CompletionArgs {
            shell: "BASH".to_string(),
            output: None,
        };
        assert_eq!(args.shell.to_lowercase(), "bash");
    }

    #[test]
    fn test_generate_completion_bash_to_stdout() {
        let mut cmd = Command::new("proximityd");
        let args = CompletionArgs {
            shell: "bash".to_string(),
            output: None,
        };
        // This should not error when writing to stdout
        let result = generate_completion(&mut cmd, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_completion_zsh_to_stdout() {
        let mut cmd = Command::new("proximityd");
        let args = CompletionArgs {
            shell: "zsh".to_string(),
            output: None,
        };
        let result = generate_completion(&mut cmd, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_completion_fish_to_stdout() {
        let mut cmd = Command::new("proximityd");
        let args = CompletionArgs {
            shell: "fish".to_string(),
            output: None,
        };
        let result = generate_completion(&mut cmd, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_completion_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("completion.sh");
        let mut cmd = Command::new("proximityd");
        let args = CompletionArgs {
            shell: "bash".to_string(),
            output: Some(output_path.clone()),
        };
        let result = generate_completion(&mut cmd, args);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_generate_completion_creates_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("subdir/completion.sh");
        let mut cmd = Command::new("proximityd");
        let args = CompletionArgs {
            shell: "bash".to_string(),
            output: Some(output_path.clone()),
        };
        let result = generate_completion(&mut cmd, args);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_generate_completion_unsupported_shell() {
        let mut cmd = Command::new("proximityd");
        let args = CompletionArgs {
            shell: "powershell".to_string(),
            output: None,
        };
        let result = generate_completion(&mut cmd, args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported shell"));
    }

    #[test]
    fn test_generate_completion_all_supported_shells() {
        let shells = vec!["bash", "zsh", "fish"];
        for shell in shells {
            let mut cmd = Command::new("proximityd");
            let args = CompletionArgs {
                shell: shell.to_string(),
                output: None,
            };
            let result = generate_completion(&mut cmd, args);
            assert!(result.is_ok(), "Shell {} should be supported", shell);
        }
    }
}

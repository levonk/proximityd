use anyhow::Result;
use std::io::{self, Write};

/// Prompt user for confirmation with a yes/no question.
/// Returns true if user confirms, false otherwise.
/// 
/// # Arguments
/// * `prompt` - The question to ask the user
/// * `force` - If true, skip the prompt and return true
/// * `quiet` - If true, skip the prompt and return false (auto-confirm No)
pub fn confirm(prompt: &str, force: bool, quiet: bool) -> Result<bool> {
    // If force is set, skip prompt and return true
    if force {
        return Ok(true);
    }
    
    // If quiet is set, skip prompt and return false (auto-confirm No)
    if quiet {
        return Ok(false);
    }
    
    // Otherwise, prompt the user
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_confirm_force_returns_true() {
        assert!(confirm("Test prompt?", true, false).unwrap());
    }
    
    #[test]
    fn test_confirm_quiet_returns_false() {
        assert!(!confirm("Test prompt?", false, true).unwrap());
    }
    
    #[test]
    fn test_confirm_force_takes_precedence_over_quiet() {
        assert!(confirm("Test prompt?", true, true).unwrap());
    }
}

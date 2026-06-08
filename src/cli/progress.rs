use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a progress bar for long-running operations
/// Returns None if quiet mode is enabled
pub fn create_progress_bar(total: u64, quiet: bool) -> Option<ProgressBar> {
    if quiet {
        return None;
    }

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Invalid progress template")
            .progress_chars("#>-")
    );
    Some(pb)
}

/// Create a spinner for indeterminate operations
/// Returns None if quiet mode is enabled
pub fn create_spinner(quiet: bool) -> Option<ProgressBar> {
    if quiet {
        return None;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid spinner template")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    Some(pb)
}

/// Set the message for a progress bar/spinner
pub fn set_message(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.set_message(msg.to_string());
    }
}

/// Increment a progress bar
pub fn inc(pb: &Option<ProgressBar>, delta: u64) {
    if let Some(pb) = pb {
        pb.inc(delta);
    }
}

/// Finish a progress bar
pub fn finish(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.finish();
    }
}

/// Finish a progress bar with a message
pub fn finish_with_message(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.finish_with_message(msg.to_string());
    }
}

/// Abandon a progress bar
pub fn abandon(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.abandon();
    }
}

/// Abandon a progress bar with a message
pub fn abandon_with_message(pb: &Option<ProgressBar>, msg: &str) {
    if let Some(pb) = pb {
        pb.abandon_with_message(msg.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_in_quiet_mode() {
        let pb = create_progress_bar(100, true);
        assert!(pb.is_none());
    }

    #[test]
    fn test_progress_bar_in_normal_mode() {
        let pb = create_progress_bar(100, false);
        assert!(pb.is_some());
    }

    #[test]
    fn test_spinner_in_quiet_mode() {
        let spinner = create_spinner(true);
        assert!(spinner.is_none());
    }

    #[test]
    fn test_spinner_in_normal_mode() {
        let spinner = create_spinner(false);
        assert!(spinner.is_some());
    }

    #[test]
    fn test_set_message_none() {
        let pb: Option<ProgressBar> = None;
        set_message(&pb, "test");
        // Should not panic
    }

    #[test]
    fn test_inc_none() {
        let pb: Option<ProgressBar> = None;
        inc(&pb, 1);
        // Should not panic
    }

    #[test]
    fn test_finish_none() {
        let pb: Option<ProgressBar> = None;
        finish(&pb);
        // Should not panic
    }
}

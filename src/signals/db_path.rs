use std::path::PathBuf;

/// Returns the default cross-platform path for the signal log database.
///
/// - Linux:   `~/.local/share/proximityd/signals.db`
/// - macOS:   `~/Library/Application Support/proximityd/signals.db`
/// - Windows: `%LOCALAPPDATA%\proximityd\signals.db`
pub fn default_db_path() -> PathBuf {
    let base = if cfg!(target_os = "windows") {
        std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default())
    } else if let Some(dirs) = directories::BaseDirs::new() {
        dirs.data_dir().to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default()
    };

    base.join("proximityd").join("signals.db")
}

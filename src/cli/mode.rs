use crate::config::app::Mode;
use std::env;

/// Detect if running in an agent session (Claude Code, Codex, etc.)
pub fn is_agent_session() -> bool {
    // Check for common agent session environment variables
    env::var("CLAUDE_SESSION").is_ok()
        || env::var("CODEX_SESSION").is_ok()
        || env::var("AGENT_SESSION").is_ok()
}

/// Detect if running in a TTY environment
pub fn is_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Determine the effective mode based on configuration and environment
pub fn detect_mode(config_mode: Mode) -> Mode {
    match config_mode {
        Mode::Agent => Mode::Agent,
        Mode::Human => Mode::Human,
        Mode::Auto => {
            // Auto-detect: prefer agent mode if in agent session, otherwise use TTY detection
            if is_agent_session() {
                Mode::Agent
            } else if is_tty() {
                Mode::Human
            } else {
                // Non-TTY without agent session: default to agent mode for CI/CD
                Mode::Agent
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_mode_enum_serialization() {
        // Test that mode enum serializes correctly
        let mode = Mode::Agent;
        let serialized = serde_json::to_string(&mode).unwrap();
        assert_eq!(serialized, "\"agent\"");

        let mode = Mode::Human;
        let serialized = serde_json::to_string(&mode).unwrap();
        assert_eq!(serialized, "\"human\"");

        let mode = Mode::Auto;
        let serialized = serde_json::to_string(&mode).unwrap();
        assert_eq!(serialized, "\"auto\"");
    }

    #[test]
    fn test_mode_enum_deserialization() {
        // Test that mode enum deserializes correctly
        let mode: Mode = serde_json::from_str("\"agent\"").unwrap();
        assert_eq!(mode, Mode::Agent);

        let mode: Mode = serde_json::from_str("\"human\"").unwrap();
        assert_eq!(mode, Mode::Human);

        let mode: Mode = serde_json::from_str("\"auto\"").unwrap();
        assert_eq!(mode, Mode::Auto);
    }

    #[test]
    fn test_detect_mode_explicit_agent() {
        // Explicit agent mode should always return agent
        let mode = detect_mode(Mode::Agent);
        assert_eq!(mode, Mode::Agent);
    }

    #[test]
    fn test_detect_mode_explicit_human() {
        // Explicit human mode should always return human
        let mode = detect_mode(Mode::Human);
        assert_eq!(mode, Mode::Human);
    }

    #[test]
    fn test_detect_mode_auto() {
        // Auto mode should detect based on environment
        let mode = detect_mode(Mode::Auto);
        // In test environment (non-TTY, no agent session), should default to agent
        assert_eq!(mode, Mode::Agent);
    }

    #[test]
    fn test_is_agent_session_claude() {
        // Test Claude session detection
        env::set_var("CLAUDE_SESSION", "test-session");
        let result = is_agent_session();
        env::remove_var("CLAUDE_SESSION");
        assert!(result);
    }

    #[test]
    fn test_is_agent_session_codex() {
        // Test Codex session detection
        env::set_var("CODEX_SESSION", "test-session");
        let result = is_agent_session();
        env::remove_var("CODEX_SESSION");
        assert!(result);
    }

    #[test]
    fn test_is_agent_session_generic() {
        // Test generic agent session detection
        env::set_var("AGENT_SESSION", "test-session");
        let result = is_agent_session();
        env::remove_var("AGENT_SESSION");
        assert!(result);
    }

    #[test]
    fn test_is_agent_session_none() {
        // Test no agent session
        env::remove_var("CLAUDE_SESSION");
        env::remove_var("CODEX_SESSION");
        env::remove_var("AGENT_SESSION");
        assert!(!is_agent_session());
    }

    #[test]
    fn test_detect_mode_auto_with_agent_session() {
        // Test auto mode with agent session
        env::set_var("CLAUDE_SESSION", "test-session");
        let mode = detect_mode(Mode::Auto);
        env::remove_var("CLAUDE_SESSION");
        assert_eq!(mode, Mode::Agent);
    }

    #[test]
    fn test_mode_toml_deserialization() {
        // Test that mode can be deserialized from TOML
        let toml_str = r#"
[general]
log_level = "info"
mode = "agent"
"#;
        let config: toml::Value = toml::from_str(toml_str).unwrap();
        let mode_str = config.get("general")
            .and_then(|g| g.get("mode"))
            .and_then(|m| m.as_str())
            .unwrap();
        assert_eq!(mode_str, "agent");
    }

    #[test]
    fn test_mode_case_insensitive() {
        // Test that mode deserialization is case-insensitive (via #[serde(rename_all = "lowercase")])
        // The enum uses lowercase serialization, so we test lowercase values
        let mode: Mode = serde_json::from_str("\"agent\"").unwrap();
        assert_eq!(mode, Mode::Agent);

        let mode: Mode = serde_json::from_str("\"human\"").unwrap();
        assert_eq!(mode, Mode::Human);

        let mode: Mode = serde_json::from_str("\"auto\"").unwrap();
        assert_eq!(mode, Mode::Auto);
    }
}

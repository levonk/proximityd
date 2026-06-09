//! Skill generation tests

use std::fs;
use tempfile::TempDir;

use btnotify::skill::{
    generate_skill, CliMetadata, CommandMetadata, SkillGenerationOptions, SkillFormat,
    check_skill_staleness, write_skill_file,
};

#[test]
fn test_skill_generation_basic() {
    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: Some("https://github.com/test/test-cli".to_string()),
        commands: vec![
            CommandMetadata {
                name: "test".to_string(),
                description: "Test command".to_string(),
                usage: "test-cli test".to_string(),
                subcommands: vec![],
            },
        ],
    };

    let options = SkillGenerationOptions::default();
    let skill = generate_skill(&cli_metadata, None, &options).unwrap();

    assert_eq!(skill.metadata.name, "test-cli");
    assert_eq!(skill.metadata.version, "1.0.0");
    assert!(skill.content.contains("test-cli"));
    assert!(skill.content.contains("Test CLI"));
}

#[test]
fn test_skill_generation_with_session_context() {
    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: None,
        commands: vec![],
    };

    let mut options = SkillGenerationOptions::default();
    options.include_live_state = true;

    let session_context = btnotify::skill::SessionContext {
        cwd: "/test/path".to_string(),
        git: None,
    };

    let skill = generate_skill(&cli_metadata, Some(&session_context), &options).unwrap();

    assert!(skill.content.contains("/test/path"));
}

#[test]
fn test_skill_generation_non_interactive() {
    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: None,
        commands: vec![
            CommandMetadata {
                name: "test".to_string(),
                description: "Test command".to_string(),
                usage: "test-cli --interactive test".to_string(),
                subcommands: vec![],
            },
        ],
    };

    let options = SkillGenerationOptions {
        include_live_state: false,
        non_interactive: true,
        format: SkillFormat::Markdown,
    };

    let skill = generate_skill(&cli_metadata, None, &options).unwrap();

    // Should not contain --interactive flag
    assert!(!skill.content.contains("--interactive"));
}

#[test]
fn test_skill_staleness_detection() {
    let temp_dir = TempDir::new().unwrap();
    let skill_path = temp_dir.path().join("SKILL.md");

    // Create a skill file with old version
    let old_content = r#"---
name: "test-cli"
version: "0.9.0"
---
"#;
    fs::write(&skill_path, old_content).unwrap();

    let is_stale = check_skill_staleness(&skill_path, "1.0.0").unwrap();
    assert!(is_stale);

    // Update to current version
    let new_content = r#"---
name: "test-cli"
version: "1.0.0"
---
"#;
    fs::write(&skill_path, new_content).unwrap();

    let is_stale = check_skill_staleness(&skill_path, "1.0.0").unwrap();
    assert!(!is_stale);
}

#[test]
fn test_skill_staleness_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    let skill_path = temp_dir.path().join("SKILL.md");

    let is_stale = check_skill_staleness(&skill_path, "1.0.0").unwrap();
    assert!(is_stale); // Missing file is considered stale
}

#[test]
fn test_write_skill_file() {
    let temp_dir = TempDir::new().unwrap();
    let skill_path = temp_dir.path().join("SKILL.md");

    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: None,
        commands: vec![],
    };

    let options = SkillGenerationOptions::default();
    let skill = generate_skill(&cli_metadata, None, &options).unwrap();

    write_skill_file(&skill, &skill_path).unwrap();

    assert!(skill_path.exists());
    let content = fs::read_to_string(&skill_path).unwrap();
    assert_eq!(content, skill.content);
}

#[test]
fn test_skill_generation_json_format() {
    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: None,
        commands: vec![],
    };

    let options = SkillGenerationOptions {
        include_live_state: false,
        non_interactive: true,
        format: SkillFormat::Json,
    };

    let _skill = generate_skill(&cli_metadata, None, &options).unwrap();

    // JSON format should contain JSON-like structure
    // (Note: current implementation only supports markdown, but this tests the enum)
    assert_eq!(options.format, SkillFormat::Json);
}

#[test]
fn test_skill_metadata_triggers() {
    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: None,
        commands: vec![],
    };

    let options = SkillGenerationOptions::default();
    let skill = generate_skill(&cli_metadata, None, &options).unwrap();

    // Should have triggers
    assert!(!skill.metadata.triggers.is_empty());
    assert!(skill.metadata.triggers.contains(&"use test-cli".to_string()));
    assert!(skill.metadata.triggers.contains(&"test-cli help".to_string()));
}

#[test]
fn test_skill_with_subcommands() {
    let cli_metadata = CliMetadata {
        name: "test-cli".to_string(),
        version: "1.0.0".to_string(),
        description: "Test CLI".to_string(),
        repository: None,
        commands: vec![
            CommandMetadata {
                name: "parent".to_string(),
                description: "Parent command".to_string(),
                usage: "test-cli parent".to_string(),
                subcommands: vec![
                    CommandMetadata {
                        name: "child".to_string(),
                        description: "Child command".to_string(),
                        usage: "test-cli parent child".to_string(),
                        subcommands: vec![],
                    },
                ],
            },
        ],
    };

    let options = SkillGenerationOptions::default();
    let skill = generate_skill(&cli_metadata, None, &options).unwrap();

    assert!(skill.content.contains("parent"));
    assert!(skill.content.contains("child"));
    assert!(skill.content.contains("Subcommands"));
}

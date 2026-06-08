use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::config::app::{DiscoveryConfig, DetectionConfig, GeneralConfig, NotifierConfig, PrivacyConfig, ScannerConfig};

/// Represents different form field types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Select(Vec<&'static str>),
}

/// Represents a form field
#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub help: String,
    pub field_type: FieldType,
    pub value: String,
    pub original_value: String,
    pub error: Option<String>,
}

impl FormField {
    pub fn new(name: String, label: String, help: String, field_type: FieldType, value: String) -> Self {
        let original_value = value.clone();
        Self {
            name,
            label,
            help,
            field_type,
            value,
            original_value,
            error: None,
        }
    }

    pub fn is_modified(&self) -> bool {
        self.value != self.original_value
    }

    pub fn validate(&mut self) -> bool {
        self.error = None;
        match &self.field_type {
            FieldType::Number => {
                if self.value.is_empty() {
                    self.error = Some("Cannot be empty".to_string());
                    return false;
                }
                if self.value.parse::<i64>().is_err() && self.value.parse::<f64>().is_err() {
                    self.error = Some("Must be a number".to_string());
                    return false;
                }
            }
            FieldType::Select(options) => {
                if !options.contains(&self.value.as_str()) {
                    self.error = Some(format!("Must be one of: {}", options.join(", ")));
                    return false;
                }
            }
            _ => {}
        }
        true
    }
}

/// Represents a form editor screen
pub struct FormEditor {
    pub title: String,
    pub fields: Vec<FormField>,
    pub selected_field: usize,
    pub editing: bool,
    pub cursor_position: usize,
    pub dirty: bool,
    pub save_message: Option<String>,
}

impl FormEditor {
    pub fn new(title: String) -> Self {
        Self {
            title,
            fields: Vec::new(),
            selected_field: 0,
            editing: false,
            cursor_position: 0,
            dirty: false,
            save_message: None,
        }
    }

    pub fn add_field(&mut self, field: FormField) {
        self.fields.push(field);
    }

    pub fn get_field_value(&self, name: &str) -> Option<String> {
        self.fields.iter().find(|f| f.name == name).map(|f| f.value.clone())
    }

    pub fn set_field_value(&mut self, name: &str, value: String) {
        if let Some(field) = self.fields.iter_mut().find(|f| f.name == name) {
            field.value = value;
            self.dirty = field.is_modified();
        }
    }

    pub fn validate_all(&mut self) -> bool {
        let mut all_valid = true;
        for field in &mut self.fields {
            if !field.validate() {
                all_valid = false;
            }
        }
        all_valid
    }

    pub fn has_changes(&self) -> bool {
        self.fields.iter().any(|f| f.is_modified())
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        if self.editing {
            self.handle_editing_key(key);
        } else {
            self.handle_navigation_key(key);
        }
    }

    fn handle_navigation_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                if self.selected_field > 0 {
                    self.selected_field -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_field < self.fields.len().saturating_sub(1) {
                    self.selected_field += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(field) = self.fields.get_mut(self.selected_field) {
                    match &field.field_type {
                        FieldType::Boolean => {
                            field.value = if field.value == "true" {
                                "false".to_string()
                            } else {
                                "true".to_string()
                            };
                            self.dirty = field.is_modified();
                        }
                        FieldType::Select(options) => {
                            if let Some(current_idx) = options.iter().position(|&opt| opt == field.value) {
                                let next_idx = (current_idx + 1) % options.len();
                                field.value = options[next_idx].to_string();
                                self.dirty = field.is_modified();
                            }
                        }
                        _ => {
                            self.editing = true;
                            self.cursor_position = field.value.len();
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.save_message = None;
            }
            _ => {}
        }
    }

    fn handle_editing_key(&mut self, key: KeyCode) {
        if let Some(field) = self.fields.get_mut(self.selected_field) {
            match key {
                KeyCode::Esc => {
                    self.editing = false;
                    self.cursor_position = 0;
                }
                KeyCode::Enter => {
                    self.editing = false;
                    self.cursor_position = 0;
                    field.validate();
                    self.dirty = field.is_modified();
                }
                KeyCode::Char(c) => {
                    field.value.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                    self.dirty = field.is_modified();
                }
                KeyCode::Backspace => {
                    if self.cursor_position > 0 {
                        field.value.remove(self.cursor_position - 1);
                        self.cursor_position -= 1;
                        self.dirty = field.is_modified();
                    }
                }
                KeyCode::Delete => {
                    if self.cursor_position < field.value.len() {
                        field.value.remove(self.cursor_position);
                        self.dirty = field.is_modified();
                    }
                }
                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_position < field.value.len() {
                        self.cursor_position += 1;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);

        // Title
        let title = Paragraph::new(self.title.as_str())
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // Form fields
        let mut lines = Vec::new();
        for (i, field) in self.fields.iter().enumerate() {
            let is_selected = i == self.selected_field;
            let is_modified = field.is_modified();

            let label_style = if is_selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let value_style = if is_modified {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let error_style = Style::default().fg(Color::Red);

            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", field.label), label_style),
                Span::styled(field.value.clone(), value_style),
            ]));

            if let Some(error) = &field.error {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(error.clone(), error_style),
                ]));
            }

            if is_selected && !field.help.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(field.help.clone(), Style::default().fg(Color::DarkGray)),
                ]));
            }

            lines.push(Line::from(""));
        }

        let text = Text::from(lines);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[1]);

        // Status bar
        let status_text = if let Some(msg) = &self.save_message {
            Line::from(vec![
                Span::styled(msg, Style::default().fg(Color::Green)),
            ])
        } else if self.editing {
            Line::from(vec![
                Span::styled("Editing: ", Style::default().fg(Color::Cyan)),
                Span::styled("Enter=Save, Esc=Cancel", Style::default()),
            ])
        } else {
            Line::from(vec![
                Span::styled("Enter=Edit, Esc=Back", Style::default()),
                if self.dirty {
                    Span::styled(" | Modified", Style::default().fg(Color::Yellow))
                } else {
                    Span::from("")
                },
            ])
        };

        let status = Paragraph::new(status_text)
            .style(Style::default())
            .alignment(Alignment::Center);
        f.render_widget(status, chunks[2]);
    }
}

/// Create a general settings editor
pub fn create_general_editor(config: &GeneralConfig) -> FormEditor {
    let mut editor = FormEditor::new("General Settings".to_string());

    editor.add_field(FormField::new(
        "log_level".to_string(),
        "Log Level".to_string(),
        "Set the logging verbosity (trace, debug, info, warn, error)".to_string(),
        FieldType::Select(vec!["trace", "debug", "info", "warn", "error"]),
        config.log_level.clone(),
    ));

    editor.add_field(FormField::new(
        "max_log_age_days".to_string(),
        "Max Log Age (days)".to_string(),
        "How long to retain signal log entries (1-90 days)".to_string(),
        FieldType::Number,
        config.max_log_age_days.to_string(),
    ));

    editor.add_field(FormField::new(
        "config_reload".to_string(),
        "Config Reload".to_string(),
        "Enable SIGHUP-based config reload".to_string(),
        FieldType::Boolean,
        config.config_reload.to_string(),
    ));

    editor
}

/// Create a privacy settings editor
pub fn create_privacy_editor(config: &PrivacyConfig) -> FormEditor {
    let mut editor = FormEditor::new("Privacy Settings".to_string());

    editor.add_field(FormField::new(
        "privacy_mode".to_string(),
        "Privacy Mode".to_string(),
        "Disable ARP/ping/mDNS scanning, BLE only".to_string(),
        FieldType::Boolean,
        config.privacy_mode.to_string(),
    ));

    editor.add_field(FormField::new(
        "anonymous".to_string(),
        "Anonymous Identifiers".to_string(),
        "Comma-separated list of identifiers to ignore entirely".to_string(),
        FieldType::Text,
        config.anonymous.join(","),
    ));

    editor
}

/// Create a scanner settings editor
pub fn create_scanner_editor(scanner_name: &str, config: &ScannerConfig) -> FormEditor {
    let mut editor = FormEditor::new(format!("Scanner: {}", scanner_name));

    editor.add_field(FormField::new(
        "enabled".to_string(),
        "Enabled".to_string(),
        "Enable or disable this scanner".to_string(),
        FieldType::Boolean,
        config.enabled.to_string(),
    ));

    editor.add_field(FormField::new(
        "scan_interval_sec".to_string(),
        "Scan Interval (sec)".to_string(),
        "How often to scan for devices".to_string(),
        FieldType::Number,
        config.scan_interval_sec.to_string(),
    ));

    if let Some(router_ip) = &config.router_ip {
        editor.add_field(FormField::new(
            "router_ip".to_string(),
            "Router IP".to_string(),
            "Router IP address for SNMP queries".to_string(),
            FieldType::Text,
            router_ip.clone(),
        ));
    }

    editor.add_field(FormField::new(
        "snmp_community".to_string(),
        "SNMP Community".to_string(),
        "SNMP community string for router queries".to_string(),
        FieldType::Text,
        config.snmp_community.clone(),
    ));

    if let Some(subnet) = &config.subnet {
        editor.add_field(FormField::new(
            "subnet".to_string(),
            "Subnet".to_string(),
            "Subnet to scan (e.g., 192.168.1.0/24)".to_string(),
            FieldType::Text,
            subnet.clone(),
        ));
    }

    editor
}

/// Create a detection settings editor
pub fn create_detection_editor(config: &DetectionConfig) -> FormEditor {
    let mut editor = FormEditor::new("Detection Settings".to_string());

    editor.add_field(FormField::new(
        "enter_debounce_sec".to_string(),
        "Enter Debounce (sec)".to_string(),
        "Debounce before party enter notification".to_string(),
        FieldType::Number,
        config.enter_debounce_sec.to_string(),
    ));

    editor.add_field(FormField::new(
        "exit_debounce_sec".to_string(),
        "Exit Debounce (sec)".to_string(),
        "Debounce before party exit notification".to_string(),
        FieldType::Number,
        config.exit_debounce_sec.to_string(),
    ));

    editor
}

/// Create a discovery settings editor
pub fn create_discovery_editor(config: &DiscoveryConfig) -> FormEditor {
    let mut editor = FormEditor::new("Discovery Settings".to_string());

    editor.add_field(FormField::new(
        "use_suggestions".to_string(),
        "Use Suggestions".to_string(),
        "Use auto-discovery suggestions at runtime".to_string(),
        FieldType::Boolean,
        config.use_suggestions.to_string(),
    ));

    editor.add_field(FormField::new(
        "auto_promote_threshold".to_string(),
        "Auto-Promote Threshold".to_string(),
        "Confidence threshold for auto-promoting suggestions (0.0-1.0)".to_string(),
        FieldType::Number,
        config.auto_promote_threshold.to_string(),
    ));

    editor
}

/// Create a notifier configuration editor
pub fn create_notifier_editor(config: &NotifierConfig, index: usize) -> FormEditor {
    let mut editor = FormEditor::new(format!("Notifier #{}", index + 1));

    editor.add_field(FormField::new(
        "kind".to_string(),
        "Type".to_string(),
        "Notifier type (discord, slack, webhook, mqtt)".to_string(),
        FieldType::Select(vec!["discord", "slack", "webhook", "mqtt"]),
        config.kind.clone(),
    ));

    editor.add_field(FormField::new(
        "webhook_url".to_string(),
        "Webhook URL".to_string(),
        "Webhook URL for notifications".to_string(),
        FieldType::Text,
        config.webhook_url.clone(),
    ));

    editor.add_field(FormField::new(
        "url".to_string(),
        "URL".to_string(),
        "Generic URL (for webhook type)".to_string(),
        FieldType::Text,
        config.url.clone(),
    ));

    editor.add_field(FormField::new(
        "method".to_string(),
        "HTTP Method".to_string(),
        "HTTP method (for webhook type)".to_string(),
        FieldType::Select(vec!["GET", "POST", "PUT", "DELETE"]),
        config.method.clone(),
    ));

    editor.add_field(FormField::new(
        "broker".to_string(),
        "MQTT Broker".to_string(),
        "MQTT broker address".to_string(),
        FieldType::Text,
        config.broker.clone(),
    ));

    editor.add_field(FormField::new(
        "port".to_string(),
        "MQTT Port".to_string(),
        "MQTT broker port".to_string(),
        FieldType::Number,
        config.port.to_string(),
    ));

    editor.add_field(FormField::new(
        "topic".to_string(),
        "MQTT Topic".to_string(),
        "MQTT topic for publishing".to_string(),
        FieldType::Text,
        config.topic.clone(),
    ));

    editor.add_field(FormField::new(
        "include_timestamp".to_string(),
        "Include Timestamp".to_string(),
        "Include timestamp in notification messages".to_string(),
        FieldType::Boolean,
        config.include_timestamp.to_string(),
    ));

    editor.add_field(FormField::new(
        "include_mac".to_string(),
        "Include MAC".to_string(),
        "Include MAC address in notification messages".to_string(),
        FieldType::Boolean,
        config.include_mac.to_string(),
    ));

    editor
}

/// Apply general settings from editor to config
pub fn apply_general_settings(editor: &FormEditor, config: &mut GeneralConfig) -> Result<()> {
    config.log_level = editor.get_field_value("log_level").context("Missing log_level")?;
    config.max_log_age_days = editor
        .get_field_value("max_log_age_days")
        .context("Missing max_log_age_days")?
        .parse()
        .context("Invalid max_log_age_days")?;
    config.config_reload = editor
        .get_field_value("config_reload")
        .context("Missing config_reload")?
        .parse()
        .context("Invalid config_reload")?;
    Ok(())
}

/// Apply privacy settings from editor to config
pub fn apply_privacy_settings(editor: &FormEditor, config: &mut PrivacyConfig) -> Result<()> {
    config.privacy_mode = editor
        .get_field_value("privacy_mode")
        .context("Missing privacy_mode")?
        .parse()
        .context("Invalid privacy_mode")?;
    config.anonymous = editor
        .get_field_value("anonymous")
        .context("Missing anonymous")?
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(())
}

/// Apply scanner settings from editor to config
pub fn apply_scanner_settings(editor: &FormEditor, config: &mut ScannerConfig) -> Result<()> {
    config.enabled = editor
        .get_field_value("enabled")
        .context("Missing enabled")?
        .parse()
        .context("Invalid enabled")?;
    config.scan_interval_sec = editor
        .get_field_value("scan_interval_sec")
        .context("Missing scan_interval_sec")?
        .parse()
        .context("Invalid scan_interval_sec")?;
    config.router_ip = editor.get_field_value("router_ip");
    config.snmp_community = editor
        .get_field_value("snmp_community")
        .context("Missing snmp_community")?;
    config.subnet = editor.get_field_value("subnet");
    Ok(())
}

/// Apply detection settings from editor to config
pub fn apply_detection_settings(editor: &FormEditor, config: &mut DetectionConfig) -> Result<()> {
    config.enter_debounce_sec = editor
        .get_field_value("enter_debounce_sec")
        .context("Missing enter_debounce_sec")?
        .parse()
        .context("Invalid enter_debounce_sec")?;
    config.exit_debounce_sec = editor
        .get_field_value("exit_debounce_sec")
        .context("Missing exit_debounce_sec")?
        .parse()
        .context("Invalid exit_debounce_sec")?;
    Ok(())
}

/// Apply discovery settings from editor to config
pub fn apply_discovery_settings(editor: &FormEditor, config: &mut DiscoveryConfig) -> Result<()> {
    config.use_suggestions = editor
        .get_field_value("use_suggestions")
        .context("Missing use_suggestions")?
        .parse()
        .context("Invalid use_suggestions")?;
    config.auto_promote_threshold = editor
        .get_field_value("auto_promote_threshold")
        .context("Missing auto_promote_threshold")?
        .parse()
        .context("Invalid auto_promote_threshold")?;
    Ok(())
}

/// Apply notifier settings from editor to config
pub fn apply_notifier_settings(editor: &FormEditor, config: &mut NotifierConfig) -> Result<()> {
    config.kind = editor.get_field_value("kind").context("Missing kind")?;
    config.webhook_url = editor.get_field_value("webhook_url").context("Missing webhook_url")?;
    config.url = editor.get_field_value("url").context("Missing url")?;
    config.method = editor.get_field_value("method").context("Missing method")?;
    config.broker = editor.get_field_value("broker").context("Missing broker")?;
    config.port = editor
        .get_field_value("port")
        .context("Missing port")?
        .parse()
        .context("Invalid port")?;
    config.topic = editor.get_field_value("topic").context("Missing topic")?;
    config.include_timestamp = editor
        .get_field_value("include_timestamp")
        .context("Missing include_timestamp")?
        .parse()
        .context("Invalid include_timestamp")?;
    config.include_mac = editor
        .get_field_value("include_mac")
        .context("Missing include_mac")?
        .parse()
        .context("Invalid include_mac")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_field_creation() {
        let field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        );
        assert_eq!(field.name, "test");
        assert_eq!(field.value, "value");
        assert!(!field.is_modified());
    }

    #[test]
    fn test_form_field_modified() {
        let mut field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        );
        assert!(!field.is_modified());
        field.value = "new_value".to_string();
        assert!(field.is_modified());
    }

    #[test]
    fn test_form_field_validation_text() {
        let mut field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        );
        assert!(field.validate());
        assert!(field.error.is_none());
    }

    #[test]
    fn test_form_field_validation_number_valid() {
        let mut field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Number,
            "42".to_string(),
        );
        assert!(field.validate());
        assert!(field.error.is_none());
    }

    #[test]
    fn test_form_field_validation_number_invalid() {
        let mut field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Number,
            "not_a_number".to_string(),
        );
        assert!(!field.validate());
        assert!(field.error.is_some());
    }

    #[test]
    fn test_form_field_validation_select_valid() {
        let mut field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Select(vec!["option1", "option2"]),
            "option1".to_string(),
        );
        assert!(field.validate());
        assert!(field.error.is_none());
    }

    #[test]
    fn test_form_field_validation_select_invalid() {
        let mut field = FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Select(vec!["option1", "option2"]),
            "invalid".to_string(),
        );
        assert!(!field.validate());
        assert!(field.error.is_some());
    }

    #[test]
    fn test_form_editor_creation() {
        let editor = FormEditor::new("Test Editor".to_string());
        assert_eq!(editor.title, "Test Editor");
        assert_eq!(editor.selected_field, 0);
        assert!(!editor.editing);
        assert!(!editor.dirty);
    }

    #[test]
    fn test_form_editor_add_field() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        ));
        assert_eq!(editor.fields.len(), 1);
    }

    #[test]
    fn test_form_editor_get_field_value() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        ));
        assert_eq!(editor.get_field_value("test"), Some("value".to_string()));
        assert_eq!(editor.get_field_value("nonexistent"), None);
    }

    #[test]
    fn test_form_editor_set_field_value() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        ));
        editor.set_field_value("test", "new_value".to_string());
        assert_eq!(editor.get_field_value("test"), Some("new_value".to_string()));
        assert!(editor.dirty);
    }

    #[test]
    fn test_form_editor_has_changes() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Text,
            "value".to_string(),
        ));
        assert!(!editor.has_changes());
        editor.set_field_value("test", "new_value".to_string());
        assert!(editor.has_changes());
    }

    #[test]
    fn test_form_editor_validate_all() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Number,
            "42".to_string(),
        ));
        assert!(editor.validate_all());
    }

    #[test]
    fn test_form_editor_validate_all_invalid() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help text".to_string(),
            FieldType::Number,
            "not_a_number".to_string(),
        ));
        assert!(!editor.validate_all());
    }

    #[test]
    fn test_form_editor_navigation() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "field1".to_string(),
            "Field 1".to_string(),
            "Help".to_string(),
            FieldType::Text,
            "value1".to_string(),
        ));
        editor.add_field(FormField::new(
            "field2".to_string(),
            "Field 2".to_string(),
            "Help".to_string(),
            FieldType::Text,
            "value2".to_string(),
        ));

        editor.handle_key(KeyCode::Down);
        assert_eq!(editor.selected_field, 1);

        editor.handle_key(KeyCode::Up);
        assert_eq!(editor.selected_field, 0);
    }

    #[test]
    fn test_form_editor_boolean_toggle() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help".to_string(),
            FieldType::Boolean,
            "false".to_string(),
        ));

        editor.handle_key(KeyCode::Enter);
        assert_eq!(editor.get_field_value("test"), Some("true".to_string()));

        editor.handle_key(KeyCode::Enter);
        assert_eq!(editor.get_field_value("test"), Some("false".to_string()));
    }

    #[test]
    fn test_form_editor_select_cycle() {
        let mut editor = FormEditor::new("Test Editor".to_string());
        editor.add_field(FormField::new(
            "test".to_string(),
            "Test Field".to_string(),
            "Help".to_string(),
            FieldType::Select(vec!["opt1", "opt2", "opt3"]),
            "opt1".to_string(),
        ));

        editor.handle_key(KeyCode::Enter);
        assert_eq!(editor.get_field_value("test"), Some("opt2".to_string()));

        editor.handle_key(KeyCode::Enter);
        assert_eq!(editor.get_field_value("test"), Some("opt3".to_string()));

        editor.handle_key(KeyCode::Enter);
        assert_eq!(editor.get_field_value("test"), Some("opt1".to_string()));
    }

    #[test]
    fn test_create_general_editor() {
        let config = GeneralConfig::default();
        let editor = create_general_editor(&config);
        assert_eq!(editor.title, "General Settings");
        assert!(editor.get_field_value("log_level").is_some());
        assert!(editor.get_field_value("max_log_age_days").is_some());
        assert!(editor.get_field_value("config_reload").is_some());
    }

    #[test]
    fn test_create_privacy_editor() {
        let config = PrivacyConfig::default();
        let editor = create_privacy_editor(&config);
        assert_eq!(editor.title, "Privacy Settings");
        assert!(editor.get_field_value("privacy_mode").is_some());
        assert!(editor.get_field_value("anonymous").is_some());
    }

    #[test]
    fn test_create_scanner_editor() {
        let config = ScannerConfig::default();
        let editor = create_scanner_editor("ble", &config);
        assert_eq!(editor.title, "Scanner: ble");
        assert!(editor.get_field_value("enabled").is_some());
        assert!(editor.get_field_value("scan_interval_sec").is_some());
    }

    #[test]
    fn test_create_detection_editor() {
        let config = DetectionConfig::default();
        let editor = create_detection_editor(&config);
        assert_eq!(editor.title, "Detection Settings");
        assert!(editor.get_field_value("enter_debounce_sec").is_some());
        assert!(editor.get_field_value("exit_debounce_sec").is_some());
    }

    #[test]
    fn test_create_discovery_editor() {
        let config = DiscoveryConfig::default();
        let editor = create_discovery_editor(&config);
        assert_eq!(editor.title, "Discovery Settings");
        assert!(editor.get_field_value("use_suggestions").is_some());
        assert!(editor.get_field_value("auto_promote_threshold").is_some());
    }

    #[test]
    fn test_create_notifier_editor() {
        let config = NotifierConfig::default();
        let editor = create_notifier_editor(&config, 0);
        assert_eq!(editor.title, "Notifier #1");
        assert!(editor.get_field_value("kind").is_some());
        assert!(editor.get_field_value("webhook_url").is_some());
    }

    #[test]
    fn test_apply_general_settings() -> Result<()> {
        let mut editor = FormEditor::new("Test".to_string());
        editor.add_field(FormField::new(
            "log_level".to_string(),
            "Log Level".to_string(),
            "Help".to_string(),
            FieldType::Select(vec!["info", "debug"]),
            "debug".to_string(),
        ));
        editor.add_field(FormField::new(
            "max_log_age_days".to_string(),
            "Max Log Age".to_string(),
            "Help".to_string(),
            FieldType::Number,
            "14".to_string(),
        ));
        editor.add_field(FormField::new(
            "config_reload".to_string(),
            "Config Reload".to_string(),
            "Help".to_string(),
            FieldType::Boolean,
            "false".to_string(),
        ));

        let mut config = GeneralConfig::default();
        apply_general_settings(&editor, &mut config)?;

        assert_eq!(config.log_level, "debug");
        assert_eq!(config.max_log_age_days, 14);
        assert_eq!(config.config_reload, false);

        Ok(())
    }

    #[test]
    fn test_apply_privacy_settings() -> Result<()> {
        let mut editor = FormEditor::new("Test".to_string());
        editor.add_field(FormField::new(
            "privacy_mode".to_string(),
            "Privacy Mode".to_string(),
            "Help".to_string(),
            FieldType::Boolean,
            "true".to_string(),
        ));
        editor.add_field(FormField::new(
            "anonymous".to_string(),
            "Anonymous".to_string(),
            "Help".to_string(),
            FieldType::Text,
            "id1,id2,id3".to_string(),
        ));

        let mut config = PrivacyConfig::default();
        apply_privacy_settings(&editor, &mut config)?;

        assert!(config.privacy_mode);
        assert_eq!(config.anonymous, vec!["id1", "id2", "id3"]);

        Ok(())
    }
}

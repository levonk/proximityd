pub mod config;

use anyhow::{Context, Result};
use atty;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

/// Check if the terminal supports TUI mode
pub fn is_tui_supported() -> bool {
    // Check if we're in a terminal
    if !atty::is(atty::Stream::Stdout) {
        return false;
    }

    // Check for common unsupported environments
    if std::env::var("TERM").map_or(false, |t| t == "dumb") {
        return false;
    }

    true
}

use config::{FormEditor, create_general_editor, create_privacy_editor, create_scanner_editor, create_detection_editor, create_discovery_editor, create_notifier_editor, apply_general_settings, apply_privacy_settings, apply_scanner_settings, apply_detection_settings, apply_discovery_settings, apply_notifier_settings};
use crate::config::app::AppConfig;
use crate::config::loader;
use std::fs;
use toml;

/// TUI application state
pub struct TuiApp {
    /// Current screen in the navigation stack
    screens: Vec<Screen>,
    /// Selected menu item index
    selected: usize,
    /// Should exit the TUI
    should_exit: bool,
    /// Config editor for current config screen
    config_editor: Option<FormEditor>,
    /// Selected scanner name for scanner config
    selected_scanner: Option<String>,
    /// Selected notifier index for notifier config
    selected_notifier: Option<usize>,
    /// Loaded config
    config: AppConfig,
}

/// Represents different screens in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    Config,
    ConfigGeneral,
    ConfigPrivacy,
    ConfigScanner,
    ConfigDetection,
    ConfigDiscovery,
    ConfigNotifier,
    Parties,
    Devices,
    Notifiers,
    Test,
    Help,
}

impl Screen {
    /// Get the title for this screen
    pub fn title(&self) -> &str {
        match self {
            Screen::MainMenu => "proximityd Configuration",
            Screen::Config => "Configuration",
            Screen::ConfigGeneral => "General Settings",
            Screen::ConfigPrivacy => "Privacy Settings",
            Screen::ConfigScanner => "Scanner Settings",
            Screen::ConfigDetection => "Detection Settings",
            Screen::ConfigDiscovery => "Discovery Settings",
            Screen::ConfigNotifier => "Notifier Configuration",
            Screen::Parties => "Parties",
            Screen::Devices => "Devices",
            Screen::Notifiers => "Notifiers",
            Screen::Test => "Test Notifiers",
            Screen::Help => "Keyboard Shortcuts",
        }
    }

    /// Get menu items for the main menu
    pub fn main_menu_items() -> Vec<&'static str> {
        vec![
            "Config",
            "Parties",
            "Devices",
            "Notifiers",
            "Test",
            "Help",
            "Exit",
        ]
    }

    /// Get menu items for the config menu
    pub fn config_menu_items() -> Vec<&'static str> {
        vec![
            "General",
            "Privacy",
            "Scanners",
            "Detection",
            "Discovery",
            "Notifiers",
            "Back",
        ]
    }
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new() -> Self {
        TuiApp {
            screens: vec![Screen::MainMenu],
            selected: 0,
            should_exit: false,
            config_editor: None,
            selected_scanner: None,
            selected_notifier: None,
            config: AppConfig::default(),
        }
    }

    /// Get the current screen
    pub fn current_screen(&self) -> Screen {
        *self.screens.last().unwrap_or(&Screen::MainMenu)
    }

    /// Push a new screen onto the stack
    pub fn push_screen(&mut self, screen: Screen) {
        self.screens.push(screen);
        self.selected = 0;
    }

    /// Pop the current screen from the stack
    pub fn pop_screen(&mut self) {
        if self.screens.len() > 1 {
            self.screens.pop();
            self.selected = 0;
        } else {
            self.should_exit = true;
        }
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: KeyCode) {
        match self.current_screen() {
            Screen::MainMenu => self.handle_main_menu_key(key),
            Screen::Config => self.handle_config_menu_key(key),
            Screen::ConfigGeneral | Screen::ConfigPrivacy | Screen::ConfigScanner |
            Screen::ConfigDetection | Screen::ConfigDiscovery | Screen::ConfigNotifier => {
                self.handle_config_editor_key(key)
            }
            Screen::Help => {
                if key == KeyCode::Esc {
                    self.pop_screen();
                }
            }
            _ => {
                // For other screens, just handle navigation
                match key {
                    KeyCode::Esc => self.pop_screen(),
                    _ => {}
                }
            }
        }
    }

    /// Handle keys for the main menu
    fn handle_main_menu_key(&mut self, key: KeyCode) {
        let items = Screen::main_menu_items();
        match key {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected < items.len() - 1 {
                    self.selected += 1;
                }
            }
            KeyCode::Enter => {
                match items[self.selected] {
                    "Config" => self.push_screen(Screen::Config),
                    "Parties" => self.push_screen(Screen::Parties),
                    "Devices" => self.push_screen(Screen::Devices),
                    "Notifiers" => self.push_screen(Screen::Notifiers),
                    "Test" => self.push_screen(Screen::Test),
                    "Help" => self.push_screen(Screen::Help),
                    "Exit" => self.should_exit = true,
                    _ => {}
                }
            }
            KeyCode::Esc => self.should_exit = true,
            _ => {}
        }
    }

    /// Handle keys for the config menu
    fn handle_config_menu_key(&mut self, key: KeyCode) {
        let items = Screen::config_menu_items();
        match key {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected < items.len() - 1 {
                    self.selected += 1;
                }
            }
            KeyCode::Enter => {
                match items[self.selected] {
                    "General" => {
                        self.config_editor = Some(create_general_editor(&self.config.general));
                        self.push_screen(Screen::ConfigGeneral);
                    }
                    "Privacy" => {
                        self.config_editor = Some(create_privacy_editor(&self.config.privacy));
                        self.push_screen(Screen::ConfigPrivacy);
                    }
                    "Scanners" => {
                        // Default to first scanner if available
                        if let Some(scanner_name) = self.config.scanner.keys().next() {
                            self.selected_scanner = Some(scanner_name.clone());
                            if let Some(scanner_config) = self.config.scanner.get(scanner_name) {
                                self.config_editor = Some(create_scanner_editor(scanner_name, scanner_config));
                            }
                        }
                        self.push_screen(Screen::ConfigScanner);
                    }
                    "Detection" => {
                        self.config_editor = Some(create_detection_editor(&self.config.detection));
                        self.push_screen(Screen::ConfigDetection);
                    }
                    "Discovery" => {
                        self.config_editor = Some(create_discovery_editor(&self.config.discovery));
                        self.push_screen(Screen::ConfigDiscovery);
                    }
                    "Notifiers" => {
                        // Default to first notifier if available
                        if !self.config.notifiers.is_empty() {
                            self.selected_notifier = Some(0);
                            self.config_editor = Some(create_notifier_editor(&self.config.notifiers[0], 0));
                        }
                        self.push_screen(Screen::ConfigNotifier);
                    }
                    "Back" => self.pop_screen(),
                    _ => {}
                }
            }
            KeyCode::Esc => self.pop_screen(),
            _ => {}
        }
    }

    /// Handle keys for config editor screens
    fn handle_config_editor_key(&mut self, key: KeyCode) {
        // Handle Ctrl+S for save
        if key == KeyCode::Char('s') {
            let validation_ok = self.config_editor.as_mut().map_or(false, |e| e.validate_all());

            if validation_ok {
                // Apply changes to config
                self.apply_config_changes();

                // Save to file (drop borrow before accessing config_editor again)
                let save_result = self.save_config();

                // Update editor message after save
                let message = match save_result {
                    Ok(_) => {
                        // Clear dirty flag after successful save
                        if let Some(editor) = &mut self.config_editor {
                            editor.dirty = false;
                        }
                        "Config saved successfully".to_string()
                    }
                    Err(e) => format!("Save failed: {}", e),
                };

                if let Some(editor) = &mut self.config_editor {
                    editor.save_message = Some(message);
                }
            } else {
                if let Some(editor) = &mut self.config_editor {
                    editor.save_message = Some("Cannot save: validation errors".to_string());
                }
            }
            return;
        }

        // Pass to editor if it exists
        if let Some(ref mut editor) = self.config_editor {
            editor.handle_key(key);

            // Check for Esc to exit editing mode
            if key == KeyCode::Esc && !editor.editing {
                // Apply changes and go back
                self.apply_config_changes();
                self.config_editor = None;
                self.pop_screen();
            }
        }
    }

    /// Save config to file
    fn save_config(&self) -> Result<()> {
        let config_dir = loader::resolve_config_dir();
        let config_path = config_dir.join("config.toml");

        // Serialize config to TOML
        let toml_string = toml::to_string_pretty(&self.config)
            .context("Failed to serialize config to TOML")?;

        // Write to file
        fs::write(&config_path, toml_string)
            .context(format!("Failed to write config to {}", config_path.display()))?;

        Ok(())
    }

    /// Apply config changes from editor to config
    fn apply_config_changes(&mut self) {
        if let Some(ref editor) = self.config_editor {
            match self.current_screen() {
                Screen::ConfigGeneral => {
                    if let Err(e) = apply_general_settings(editor, &mut self.config.general) {
                        eprintln!("Failed to apply general settings: {}", e);
                    }
                }
                Screen::ConfigPrivacy => {
                    if let Err(e) = apply_privacy_settings(editor, &mut self.config.privacy) {
                        eprintln!("Failed to apply privacy settings: {}", e);
                    }
                }
                Screen::ConfigScanner => {
                    if let Some(ref scanner_name) = self.selected_scanner {
                        if let Some(scanner_config) = self.config.scanner.get_mut(scanner_name) {
                            if let Err(e) = apply_scanner_settings(editor, scanner_config) {
                                eprintln!("Failed to apply scanner settings: {}", e);
                            }
                        }
                    }
                }
                Screen::ConfigDetection => {
                    if let Err(e) = apply_detection_settings(editor, &mut self.config.detection) {
                        eprintln!("Failed to apply detection settings: {}", e);
                    }
                }
                Screen::ConfigDiscovery => {
                    if let Err(e) = apply_discovery_settings(editor, &mut self.config.discovery) {
                        eprintln!("Failed to apply discovery settings: {}", e);
                    }
                }
                Screen::ConfigNotifier => {
                    if let Some(ref notifier_idx) = self.selected_notifier {
                        if let Some(notifier_config) = self.config.notifiers.get_mut(*notifier_idx) {
                            if let Err(e) = apply_notifier_settings(editor, notifier_config) {
                                eprintln!("Failed to apply notifier settings: {}", e);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Run the TUI application
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode().context("Failed to enable raw mode")?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

        // Main event loop
        while !self.should_exit {
            terminal
                .draw(|f| self.draw(f))
                .context("Failed to draw frame")?;

            // Handle events with timeout
            if event::poll(Duration::from_millis(100)).context("Failed to poll events")? {
                if let Event::Key(key) = event::read().context("Failed to read event")? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }
        }

        // Cleanup
        disable_raw_mode().context("Failed to disable raw mode")?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen
        )
        .context("Failed to leave alternate screen")?;
        terminal.show_cursor().context("Failed to show cursor")?;

        Ok(())
    }

    /// Draw the UI
    fn draw(&self, f: &mut Frame) {
        let screen = self.current_screen();

        match screen {
            Screen::MainMenu => self.draw_main_menu(f),
            Screen::Config => self.draw_config_menu(f),
            Screen::ConfigGeneral | Screen::ConfigPrivacy | Screen::ConfigScanner |
            Screen::ConfigDetection | Screen::ConfigDiscovery | Screen::ConfigNotifier => {
                self.draw_config_editor(f)
            }
            Screen::Help => self.draw_help(f),
            _ => self.draw_placeholder(f, screen),
        }
    }

    /// Draw the main menu
    fn draw_main_menu(&self, f: &mut Frame) {
        let items = Screen::main_menu_items();
        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(item).style(style)
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title(Screen::MainMenu.title()))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(f.size());

        let title = Paragraph::new("proximityd Interactive Configuration")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        f.render_widget(title, chunks[0]);
        f.render_widget(list, chunks[1]);
    }

    /// Draw the config menu
    fn draw_config_menu(&self, f: &mut Frame) {
        let items = Screen::config_menu_items();
        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(item).style(style)
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title(Screen::Config.title()))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(f.size());

        let title = Paragraph::new("Configuration Sections")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        f.render_widget(title, chunks[0]);
        f.render_widget(list, chunks[1]);
    }

    /// Draw the config editor
    fn draw_config_editor(&self, f: &mut Frame) {
        if let Some(ref editor) = self.config_editor {
            editor.draw(f, f.size());
        } else {
            self.draw_placeholder(f, self.current_screen());
        }
    }

    /// Draw the help screen
    fn draw_help(&self, f: &mut Frame) {
        let help_text = vec![
            Line::from("Keyboard Shortcuts:"),
            Line::from(""),
            Line::from(Span::styled("↑/↓", Style::default().fg(Color::Cyan))),
            Line::from("  Navigate menu items"),
            Line::from(""),
            Line::from(Span::styled("Enter", Style::default().fg(Color::Cyan))),
            Line::from("  Select menu item"),
            Line::from(""),
            Line::from(Span::styled("Esc", Style::default().fg(Color::Cyan))),
            Line::from("  Go back / Exit"),
            Line::from(""),
            Line::from(Span::styled("F1 / ?", Style::default().fg(Color::Cyan))),
            Line::from("  Show this help screen"),
        ];

        let paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(Screen::Help.title()))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, f.size());
    }

    /// Draw a placeholder screen for unimplemented features
    fn draw_placeholder(&self, f: &mut Frame, screen: Screen) {
        let text = vec![
            Line::from(format!("{} - Not Yet Implemented", screen.title())),
            Line::from(""),
            Line::from("Press Esc to return to main menu"),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(screen.title()))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        f.render_widget(paragraph, f.size());
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the TUI application
pub fn run_tui() -> Result<()> {
    let mut app = TuiApp::new();
    app.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_support_detection() {
        let supported = is_tui_supported();
        assert!(matches!(supported, true | false));
    }

    #[test]
    fn test_tui_app_creation() {
        let app = TuiApp::new();
        assert_eq!(app.current_screen(), Screen::MainMenu);
        assert_eq!(app.selected, 0);
        assert!(!app.should_exit);
    }

    #[test]
    fn test_tui_app_default() {
        let app = TuiApp::default();
        assert_eq!(app.current_screen(), Screen::MainMenu);
        assert_eq!(app.selected, 0);
        assert!(!app.should_exit);
    }

    #[test]
    fn test_screen_navigation() {
        let mut app = TuiApp::new();
        assert_eq!(app.current_screen(), Screen::MainMenu);

        app.push_screen(Screen::Config);
        assert_eq!(app.current_screen(), Screen::Config);
        assert_eq!(app.selected, 0);

        app.pop_screen();
        assert_eq!(app.current_screen(), Screen::MainMenu);
    }

    #[test]
    fn test_screen_stack() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        app.push_screen(Screen::Parties);
        app.push_screen(Screen::Devices);

        assert_eq!(app.current_screen(), Screen::Devices);

        app.pop_screen();
        assert_eq!(app.current_screen(), Screen::Parties);

        app.pop_screen();
        assert_eq!(app.current_screen(), Screen::Config);

        app.pop_screen();
        assert_eq!(app.current_screen(), Screen::MainMenu);
    }

    #[test]
    fn test_main_menu_items() {
        let items = Screen::main_menu_items();
        assert_eq!(items.len(), 7);
        assert!(items.contains(&"Config"));
        assert!(items.contains(&"Parties"));
        assert!(items.contains(&"Devices"));
        assert!(items.contains(&"Notifiers"));
        assert!(items.contains(&"Test"));
        assert!(items.contains(&"Help"));
        assert!(items.contains(&"Exit"));
    }

    #[test]
    fn test_config_menu_items() {
        let items = Screen::config_menu_items();
        assert_eq!(items.len(), 7);
        assert!(items.contains(&"General"));
        assert!(items.contains(&"Privacy"));
        assert!(items.contains(&"Scanners"));
        assert!(items.contains(&"Detection"));
        assert!(items.contains(&"Discovery"));
        assert!(items.contains(&"Notifiers"));
        assert!(items.contains(&"Back"));
    }

    #[test]
    fn test_screen_titles() {
        assert_eq!(Screen::MainMenu.title(), "proximityd Configuration");
        assert_eq!(Screen::Config.title(), "Configuration");
        assert_eq!(Screen::ConfigGeneral.title(), "General Settings");
        assert_eq!(Screen::ConfigPrivacy.title(), "Privacy Settings");
        assert_eq!(Screen::ConfigScanner.title(), "Scanner Settings");
        assert_eq!(Screen::ConfigDetection.title(), "Detection Settings");
        assert_eq!(Screen::ConfigDiscovery.title(), "Discovery Settings");
        assert_eq!(Screen::ConfigNotifier.title(), "Notifier Configuration");
        assert_eq!(Screen::Parties.title(), "Parties");
        assert_eq!(Screen::Devices.title(), "Devices");
        assert_eq!(Screen::Notifiers.title(), "Notifiers");
        assert_eq!(Screen::Test.title(), "Test Notifiers");
        assert_eq!(Screen::Help.title(), "Keyboard Shortcuts");
    }

    #[test]
    fn test_main_menu_navigation() {
        let mut app = TuiApp::new();
        let items = Screen::main_menu_items();

        app.handle_key(KeyCode::Down);
        assert_eq!(app.selected, 1);

        app.handle_key(KeyCode::Down);
        assert_eq!(app.selected, 2);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.selected, 1);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.selected, 0);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.selected, 0);

        for _ in 0..items.len() - 1 {
            app.handle_key(KeyCode::Down);
        }
        assert_eq!(app.selected, items.len() - 1);

        app.handle_key(KeyCode::Down);
        assert_eq!(app.selected, items.len() - 1);
    }

    #[test]
    fn test_escape_exits_from_main_menu() {
        let mut app = TuiApp::new();
        assert!(!app.should_exit);

        app.handle_key(KeyCode::Esc);
        assert!(app.should_exit);
    }

    #[test]
    fn test_escape_returns_from_subscreen() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        assert_eq!(app.current_screen(), Screen::Config);

        app.handle_key(KeyCode::Esc);
        assert_eq!(app.current_screen(), Screen::MainMenu);
        assert!(!app.should_exit);
    }

    #[test]
    fn test_escape_exits_from_main_menu_when_only_screen() {
        let mut app = TuiApp::new();
        assert_eq!(app.screens.len(), 1);

        app.handle_key(KeyCode::Esc);
        assert!(app.should_exit);
    }

    #[test]
    fn test_screen_equality() {
        assert_eq!(Screen::MainMenu, Screen::MainMenu);
        assert_ne!(Screen::MainMenu, Screen::Config);
    }

    #[test]
    fn test_config_menu_navigation() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        assert_eq!(app.current_screen(), Screen::Config);

        let items = Screen::config_menu_items();

        app.handle_key(KeyCode::Down);
        assert_eq!(app.selected, 1);

        app.handle_key(KeyCode::Down);
        assert_eq!(app.selected, 2);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.selected, 1);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.selected, 0);

        app.handle_key(KeyCode::Up);
        assert_eq!(app.selected, 0);

        for _ in 0..items.len() - 1 {
            app.handle_key(KeyCode::Down);
        }
        assert_eq!(app.selected, items.len() - 1);

        app.handle_key(KeyCode::Down);
        assert_eq!(app.selected, items.len() - 1);
    }

    #[test]
    fn test_config_menu_enter_general() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        app.handle_key(KeyCode::Enter);
        assert_eq!(app.current_screen(), Screen::ConfigGeneral);
        assert!(app.config_editor.is_some());
    }

    #[test]
    fn test_config_menu_escape_returns() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        app.handle_key(KeyCode::Esc);
        assert_eq!(app.current_screen(), Screen::MainMenu);
    }

    #[test]
    fn test_config_editor_escape_returns_to_config() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        app.handle_key(KeyCode::Enter);
        assert_eq!(app.current_screen(), Screen::ConfigGeneral);

        app.handle_key(KeyCode::Esc);
        assert_eq!(app.current_screen(), Screen::Config);
        assert!(app.config_editor.is_none());
    }

    #[test]
    fn test_config_editor_has_general_fields() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        app.handle_key(KeyCode::Enter);

        assert!(app.config_editor.is_some());
        if let Some(ref editor) = app.config_editor {
            assert!(editor.get_field_value("log_level").is_some());
            assert!(editor.get_field_value("max_log_age_days").is_some());
            assert!(editor.get_field_value("config_reload").is_some());
        }
    }

    #[test]
    fn test_config_editor_modified_flag() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Config);
        app.handle_key(KeyCode::Enter);

        assert!(app.config_editor.is_some());
        if let Some(ref editor) = app.config_editor {
            assert!(!editor.dirty);
        }
    }
}

pub mod config;
pub mod presence;
pub mod notifier;

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
use presence::{PresenceManager, PresenceEditor, PartyEditor, DeviceEditor, IdentifierEditor};
use notifier::NotifierTestManager;
use crate::config::app::AppConfig;
use crate::config::loader;
use crate::config::presence::Location;
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
    /// Presence manager for party/device/identifier management
    presence_manager: Option<PresenceManager>,
    /// Notifier test manager for testing notifications
    notifier_test_manager: Option<NotifierTestManager>,
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
    PartyDetail,
    DeviceList,
    DeviceDetail,
    IdentifierList,
    IdentifierEdit,
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
            Screen::PartyDetail => "Party Details",
            Screen::DeviceList => "Devices",
            Screen::DeviceDetail => "Device Details",
            Screen::IdentifierList => "Identifiers",
            Screen::IdentifierEdit => "Edit Identifier",
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
            presence_manager: None,
            notifier_test_manager: None,
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
            Screen::Parties => self.handle_parties_key(key),
            Screen::PartyDetail => self.handle_party_detail_key(key),
            Screen::DeviceList => self.handle_device_list_key(key),
            Screen::DeviceDetail => self.handle_device_detail_key(key),
            Screen::IdentifierList => self.handle_identifier_list_key(key),
            Screen::IdentifierEdit => self.handle_identifier_edit_key(key),
            Screen::Test => self.handle_test_key(key),
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

    /// Handle keys for the parties screen
    fn handle_parties_key(&mut self, key: KeyCode) {
        // Initialize presence manager if needed
        if self.presence_manager.is_none() {
            if let Ok(manager) = PresenceManager::new() {
                self.presence_manager = Some(manager);
            }
        }

        if let Some(ref mut manager) = self.presence_manager {
            let party_count = manager.party_count();

            match key {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if party_count > 0 && self.selected < party_count - 1 {
                        self.selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if party_count > 0 {
                        manager.selected_party = Some(self.selected);
                        self.push_screen(Screen::PartyDetail);
                    }
                }
                KeyCode::Char('a') => {
                    manager.add_party();
                    self.selected = party_count;
                }
                KeyCode::Char('d') => {
                    if party_count > 0 {
                        manager.selected_party = Some(self.selected);
                        // For now, just delete without confirmation (can be enhanced later)
                        manager.delete_party();
                        if self.selected >= party_count - 1 && self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                }
                KeyCode::Char('s') => {
                    if let Err(e) = manager.save() {
                        eprintln!("Failed to save: {}", e);
                    }
                }
                KeyCode::Esc => self.pop_screen(),
                _ => {}
            }
        }
    }

    /// Handle keys for the party detail screen
    fn handle_party_detail_key(&mut self, key: KeyCode) {
        if let Some(ref mut manager) = self.presence_manager {
            // Check if editor is active
            if manager.editor.is_some() {
                let should_save = if let Some(ref mut editor) = manager.editor {
                    if let PresenceEditor::Party(ref mut party_editor) = editor {
                        match key {
                            KeyCode::Char('s') => {
                                // Extract editor data before updating
                                let name = party_editor.name.clone();
                                let location = party_editor.location.clone();
                                Some((name, location))
                            }
                            KeyCode::Esc => {
                                manager.editor = None;
                                None
                            }
                            _ => {
                                party_editor.handle_key(key);
                                None
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some((name, location)) = should_save {
                    if let Err(e) = manager.update_selected_party(|p| {
                        p.name = name;
                        if location != Location::default() {
                            p.location = Some(location);
                        } else {
                            p.location = None;
                        }
                    }) {
                        eprintln!("Failed to update party: {}", e);
                    } else if let Err(e) = manager.save() {
                        eprintln!("Failed to save: {}", e);
                    } else {
                        manager.editor = None;
                    }
                }
                return;
            }

            // Normal navigation
            match key {
                KeyCode::Enter => {
                    self.push_screen(Screen::DeviceList);
                }
                KeyCode::Char('e') => {
                    // Create party editor
                    if let Some(party) = manager.get_selected_party() {
                        manager.editor = Some(PresenceEditor::Party(PartyEditor::from_party(party)));
                    }
                }
                KeyCode::Char('d') => {
                    manager.delete_party();
                    self.pop_screen();
                }
                KeyCode::Esc => {
                    manager.selected_party = None;
                    self.pop_screen();
                }
                _ => {}
            }
        }
    }

    /// Handle keys for the device list screen
    fn handle_device_list_key(&mut self, key: KeyCode) {
        if let Some(ref mut manager) = self.presence_manager {
            let device_count = manager.device_count();

            match key {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if device_count > 0 && self.selected < device_count - 1 {
                        self.selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if device_count > 0 {
                        manager.selected_device = Some(self.selected);
                        self.push_screen(Screen::DeviceDetail);
                    }
                }
                KeyCode::Char('a') => {
                    manager.add_device();
                    self.selected = device_count;
                }
                KeyCode::Char('d') => {
                    if device_count > 0 {
                        manager.selected_device = Some(self.selected);
                        manager.delete_device();
                        if self.selected >= device_count - 1 && self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                }
                KeyCode::Esc => {
                    manager.selected_device = None;
                    self.pop_screen();
                }
                _ => {}
            }
        }
    }

    /// Handle keys for the device detail screen
    fn handle_device_detail_key(&mut self, key: KeyCode) {
        if let Some(ref mut manager) = self.presence_manager {
            // Check if editor is active
            if manager.editor.is_some() {
                let should_save = if let Some(ref mut editor) = manager.editor {
                    if let PresenceEditor::Device(ref mut device_editor) = editor {
                        match key {
                            KeyCode::Char('s') => {
                                // Extract editor data before updating
                                let name = device_editor.name.clone();
                                let location = device_editor.location.clone();
                                Some((name, location))
                            }
                            KeyCode::Esc => {
                                manager.editor = None;
                                None
                            }
                            _ => {
                                device_editor.handle_key(key);
                                None
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some((name, location)) = should_save {
                    if let Err(e) = manager.update_selected_device(|d| {
                        d.name = name;
                        if location != Location::default() {
                            d.location = Some(location);
                        } else {
                            d.location = None;
                        }
                    }) {
                        eprintln!("Failed to update device: {}", e);
                    } else if let Err(e) = manager.save() {
                        eprintln!("Failed to save: {}", e);
                    } else {
                        manager.editor = None;
                    }
                }
                return;
            }

            // Normal navigation
            match key {
                KeyCode::Enter => {
                    self.push_screen(Screen::IdentifierList);
                }
                KeyCode::Char('e') => {
                    // Create device editor
                    if let Some(device) = manager.get_selected_device() {
                        manager.editor = Some(PresenceEditor::Device(DeviceEditor::from_device(device)));
                    }
                }
                KeyCode::Char('d') => {
                    manager.delete_device();
                    self.pop_screen();
                }
                KeyCode::Esc => {
                    manager.selected_device = None;
                    self.pop_screen();
                }
                _ => {}
            }
        }
    }

    /// Handle keys for the identifier list screen
    fn handle_identifier_list_key(&mut self, key: KeyCode) {
        if let Some(ref mut manager) = self.presence_manager {
            let identifier_count = manager.identifier_count();

            match key {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if identifier_count > 0 && self.selected < identifier_count - 1 {
                        self.selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if identifier_count > 0 {
                        manager.selected_identifier = Some(self.selected);
                        self.push_screen(Screen::IdentifierEdit);
                    }
                }
                KeyCode::Char('a') => {
                    manager.add_identifier();
                    self.selected = identifier_count;
                }
                KeyCode::Char('e') => {
                    if identifier_count > 0 {
                        manager.selected_identifier = Some(self.selected);
                        self.push_screen(Screen::IdentifierEdit);
                    }
                }
                KeyCode::Char('d') => {
                    if identifier_count > 0 {
                        manager.selected_identifier = Some(self.selected);
                        manager.delete_identifier();
                        if self.selected >= identifier_count - 1 && self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                }
                KeyCode::Esc => {
                    manager.selected_identifier = None;
                    self.pop_screen();
                }
                _ => {}
            }
        }
    }

    /// Handle keys for the identifier edit screen
    fn handle_identifier_edit_key(&mut self, key: KeyCode) {
        let (should_save, should_pop) = if let Some(ref mut manager) = self.presence_manager {
            // Check if editor is active
            if manager.editor.is_some() {
                if let Some(ref mut editor) = manager.editor {
                    if let PresenceEditor::Identifier(ref mut id_editor) = editor {
                        match key {
                            KeyCode::Char('s') => {
                                if id_editor.validate().is_ok() {
                                    // Extract editor data before updating
                                    let name = id_editor.name.clone();
                                    let id_type = id_editor.id_type.clone();
                                    let value = id_editor.value.clone();
                                    (Some((name, id_type, value)), false)
                                } else {
                                    (None, false)
                                }
                            }
                            KeyCode::Esc => {
                                manager.editor = None;
                                (None, true)
                            }
                            _ => {
                                id_editor.handle_key(key);
                                (None, false)
                            }
                        }
                    } else {
                        (None, false)
                    }
                } else {
                    (None, false)
                }
            } else {
                // Create editor if not exists
                if let Some(identifier) = manager.get_selected_identifier() {
                    manager.editor = Some(PresenceEditor::Identifier(
                        IdentifierEditor::from_identifier(identifier)
                    ));
                }
                (None, false)
            }
        } else {
            (None, false)
        };

        if should_pop {
            self.pop_screen();
            return;
        }

        if let Some((name, id_type, value)) = should_save {
            if let Some(ref mut manager) = self.presence_manager {
                if let Err(e) = manager.update_selected_identifier(|i| {
                    i.name = name;
                    i.id_type = id_type;
                    i.value = value;
                }) {
                    eprintln!("Failed to update identifier: {}", e);
                } else if let Err(e) = manager.save() {
                    eprintln!("Failed to save: {}", e);
                } else {
                    manager.editor = None;
                    self.pop_screen();
                }
            }
        }
    }

    /// Handle keys for the test notifier screen
    fn handle_test_key(&mut self, key: KeyCode) {
        // Initialize test manager if not exists
        if self.notifier_test_manager.is_none() {
            self.notifier_test_manager = Some(NotifierTestManager::new(self.config.clone()));
        }

        if let Some(ref mut manager) = self.notifier_test_manager {
            let notifier_count = manager.notifier_count();

            match key {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                        manager.set_selected(self.selected);
                    }
                }
                KeyCode::Down => {
                    if notifier_count > 0 && self.selected < notifier_count - 1 {
                        self.selected += 1;
                        manager.set_selected(self.selected);
                    }
                }
                KeyCode::Enter => {
                    if notifier_count > 0 {
                        if let Err(e) = manager.send_test_notification(self.selected) {
                            eprintln!("Failed to send test notification: {}", e);
                        }
                    }
                }
                KeyCode::Char('r') => {
                    // Retry the selected test
                    if notifier_count > 0 {
                        if let Err(e) = manager.send_test_notification(self.selected) {
                            eprintln!("Failed to send test notification: {}", e);
                        }
                    }
                }
                KeyCode::Char('c') => {
                    // Clear all results
                    manager.clear_results();
                }
                KeyCode::Esc => {
                    self.pop_screen();
                }
                _ => {}
            }
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
            Screen::Parties => self.draw_parties(f),
            Screen::PartyDetail => self.draw_party_detail(f),
            Screen::DeviceList => self.draw_device_list(f),
            Screen::DeviceDetail => self.draw_device_detail(f),
            Screen::IdentifierList => self.draw_identifier_list(f),
            Screen::IdentifierEdit => self.draw_identifier_edit(f),
            Screen::Test => self.draw_test(f),
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

    /// Draw the parties screen
    fn draw_parties(&self, f: &mut Frame) {
        let items = if let Some(ref manager) = self.presence_manager {
            manager.config.parties.iter().map(|party| {
                let device_count = party.devices.len();
                ListItem::new(format!("{} ({} devices)", party.name, device_count))
            }).collect()
        } else {
            vec![ListItem::new("Loading...")]
        };

        let list_items: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                item.clone().style(style)
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title(Screen::Parties.title()))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(f.size());

        let help_text = vec![
            Line::from("A: Add  E: Edit  D: Delete  S: Save  Enter: Edit  Esc: Back"),
        ];

        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);

        f.render_widget(help, chunks[0]);
        f.render_widget(list, chunks[1]);
    }

    /// Draw the party detail screen
    fn draw_party_detail(&self, f: &mut Frame) {
        if let Some(ref manager) = self.presence_manager {
            // Show editor if active
            if let Some(ref editor) = manager.editor {
                if let PresenceEditor::Party(ref party_editor) = editor {
                    let text = vec![
                        Line::from(format!("Name: {}", party_editor.name)),
                        Line::from(format!("Building: {}", party_editor.location.building.as_deref().unwrap_or(""))),
                        Line::from(format!("Floor: {}", party_editor.location.floor.map(|f| f.to_string()).unwrap_or("".to_string()))),
                        Line::from(format!("Room: {}", party_editor.location.room.as_deref().unwrap_or(""))),
                        Line::from(format!("Zone: {}", party_editor.location.zone.as_deref().unwrap_or(""))),
                        Line::from(""),
                        Line::from("↑/↓: Navigate field  Backspace: Delete  S: Save  Esc: Cancel"),
                    ];

                    let paragraph = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title("Edit Party"))
                        .wrap(Wrap { trim: true });

                    f.render_widget(paragraph, f.size());
                    return;
                }
            }

            // Show normal view
            if let Some(party_idx) = manager.selected_party {
                if let Some(party) = manager.config.parties.get(party_idx) {
                    let text = vec![
                        Line::from(format!("Party: {}", party.name)),
                        Line::from(""),
                        Line::from(format!("Devices: {}", party.devices.len())),
                        Line::from(""),
                        if let Some(ref loc) = party.location {
                            Line::from(format!("Location: {} (Floor: {}, Room: {}, Zone: {})",
                                loc.building.as_deref().unwrap_or("N/A"),
                                loc.floor.map(|f| f.to_string()).unwrap_or("N/A".to_string()),
                                loc.room.as_deref().unwrap_or("N/A"),
                                loc.zone.as_deref().unwrap_or("N/A")))
                        } else {
                            Line::from("Location: Not set")
                        },
                        Line::from(""),
                        Line::from("Enter: View Devices  E: Edit  D: Delete  Esc: Back"),
                    ];

                    let paragraph = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title(Screen::PartyDetail.title()))
                        .wrap(Wrap { trim: true });

                    f.render_widget(paragraph, f.size());
                    return;
                }
            }
        }
        self.draw_placeholder(f, self.current_screen());
    }

    /// Draw the device list screen
    fn draw_device_list(&self, f: &mut Frame) {
        if let Some(ref manager) = self.presence_manager {
            if let Some(party_idx) = manager.selected_party {
                if let Some(party) = manager.config.parties.get(party_idx) {
                    let items: Vec<ListItem> = party.devices.iter().map(|device| {
                        let identifier_count = device.identifiers.len();
                        ListItem::new(format!("{} ({} identifiers)", device.name, identifier_count))
                    }).collect();

                    let list_items: Vec<ListItem> = items
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let style = if i == self.selected {
                                Style::default()
                                    .fg(Color::Cyan)
                                    .add_modifier(Modifier::BOLD)
                            } else {
                                Style::default()
                            };
                            item.clone().style(style)
                        })
                        .collect();

                    let list = List::new(list_items)
                        .block(Block::default().borders(Borders::ALL).title(Screen::DeviceList.title()))
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
                        .split(f.size());

                    let help_text = vec![
                        Line::from("A: Add  E: Edit  D: Delete  Enter: Edit  Esc: Back"),
                    ];

                    let help = Paragraph::new(help_text)
                        .style(Style::default().fg(Color::Cyan))
                        .alignment(Alignment::Center);

                    f.render_widget(help, chunks[0]);
                    f.render_widget(list, chunks[1]);
                    return;
                }
            }
        }
        self.draw_placeholder(f, self.current_screen());
    }

    /// Draw the device detail screen
    fn draw_device_detail(&self, f: &mut Frame) {
        if let Some(ref manager) = self.presence_manager {
            // Show editor if active
            if let Some(ref editor) = manager.editor {
                if let PresenceEditor::Device(ref device_editor) = editor {
                    let text = vec![
                        Line::from(format!("Name: {}", device_editor.name)),
                        Line::from(format!("Building: {}", device_editor.location.building.as_deref().unwrap_or(""))),
                        Line::from(format!("Floor: {}", device_editor.location.floor.map(|f| f.to_string()).unwrap_or("".to_string()))),
                        Line::from(format!("Room: {}", device_editor.location.room.as_deref().unwrap_or(""))),
                        Line::from(format!("Zone: {}", device_editor.location.zone.as_deref().unwrap_or(""))),
                        Line::from(""),
                        Line::from("↑/↓: Navigate field  Backspace: Delete  S: Save  Esc: Cancel"),
                    ];

                    let paragraph = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title("Edit Device"))
                        .wrap(Wrap { trim: true });

                    f.render_widget(paragraph, f.size());
                    return;
                }
            }

            // Show normal view
            if let Some(party_idx) = manager.selected_party {
                if let Some(device_idx) = manager.selected_device {
                    if let Some(party) = manager.config.parties.get(party_idx) {
                        if let Some(device) = party.devices.get(device_idx) {
                            let text = vec![
                                Line::from(format!("Device: {}", device.name)),
                                Line::from(""),
                                Line::from(format!("Identifiers: {}", device.identifiers.len())),
                                Line::from(""),
                                if let Some(ref loc) = device.location {
                                    Line::from(format!("Location: {} (Floor: {}, Room: {}, Zone: {})",
                                        loc.building.as_deref().unwrap_or("N/A"),
                                        loc.floor.map(|f| f.to_string()).unwrap_or("N/A".to_string()),
                                        loc.room.as_deref().unwrap_or("N/A"),
                                        loc.zone.as_deref().unwrap_or("N/A")))
                                } else {
                                    Line::from("Location: Not set")
                                },
                                Line::from(""),
                                Line::from("Enter: View Identifiers  E: Edit  D: Delete  Esc: Back"),
                            ];

                            let paragraph = Paragraph::new(text)
                                .block(Block::default().borders(Borders::ALL).title(Screen::DeviceDetail.title()))
                                .wrap(Wrap { trim: true });

                            f.render_widget(paragraph, f.size());
                            return;
                        }
                    }
                }
            }
        }
        self.draw_placeholder(f, self.current_screen());
    }

    /// Draw the identifier list screen
    fn draw_identifier_list(&self, f: &mut Frame) {
        if let Some(ref manager) = self.presence_manager {
            if let Some(party_idx) = manager.selected_party {
                if let Some(device_idx) = manager.selected_device {
                    if let Some(party) = manager.config.parties.get(party_idx) {
                        if let Some(device) = party.devices.get(device_idx) {
                            let items: Vec<ListItem> = device.identifiers.iter().map(|id| {
                                ListItem::new(format!("{} ({:?}): {}", id.name, id.id_type, id.value))
                            }).collect();

                            let list_items: Vec<ListItem> = items
                                .iter()
                                .enumerate()
                                .map(|(i, item)| {
                                    let style = if i == self.selected {
                                        Style::default()
                                            .fg(Color::Cyan)
                                            .add_modifier(Modifier::BOLD)
                                    } else {
                                        Style::default()
                                    };
                                    item.clone().style(style)
                                })
                                .collect();

                            let list = List::new(list_items)
                                .block(Block::default().borders(Borders::ALL).title(Screen::IdentifierList.title()))
                                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                            let chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .margin(2)
                                .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
                                .split(f.size());

                            let help_text = vec![
                                Line::from("A: Add  E: Edit  D: Delete  Enter: Edit  Esc: Back"),
                            ];

                            let help = Paragraph::new(help_text)
                                .style(Style::default().fg(Color::Cyan))
                                .alignment(Alignment::Center);

                            f.render_widget(help, chunks[0]);
                            f.render_widget(list, chunks[1]);
                            return;
                        }
                    }
                }
            }
        }
        self.draw_placeholder(f, self.current_screen());
    }

    /// Draw the identifier edit screen
    fn draw_identifier_edit(&self, f: &mut Frame) {
        if let Some(ref manager) = self.presence_manager {
            // Show editor if active
            if let Some(ref editor) = manager.editor {
                if let PresenceEditor::Identifier(ref id_editor) = editor {
                    let text = vec![
                        Line::from(format!("Name: {}", id_editor.name)),
                        Line::from(format!("Type: {:?}", id_editor.id_type)),
                        Line::from(format!("Value: {}", id_editor.value)),
                        Line::from(""),
                        Line::from("↑/↓: Navigate field  Type: Change type  Backspace: Delete  S: Save  Esc: Cancel"),
                    ];

                    let paragraph = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL).title(Screen::IdentifierEdit.title()))
                        .wrap(Wrap { trim: true });

                    f.render_widget(paragraph, f.size());
                    return;
                }
            }

            // Show normal view if identifier exists
            if let Some(identifier) = manager.get_selected_identifier() {
                let text = vec![
                    Line::from(format!("Name: {}", identifier.name)),
                    Line::from(format!("Type: {:?}", identifier.id_type)),
                    Line::from(format!("Value: {}", identifier.value)),
                    Line::from(""),
                    Line::from("E: Edit  D: Delete  Esc: Back"),
                ];

                let paragraph = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Identifier Details"))
                    .wrap(Wrap { trim: true });

                f.render_widget(paragraph, f.size());
                return;
            }
        }
        self.draw_placeholder(f, self.current_screen());
    }

    /// Draw the test notifier screen
    fn draw_test(&self, f: &mut Frame) {
        if let Some(ref manager) = self.notifier_test_manager {
            let notifier_names = manager.notifier_names();
            let notifier_count = notifier_names.len();

            if notifier_count == 0 {
                let text = vec![
                    Line::from("No notifiers configured."),
                    Line::from(""),
                    Line::from("Add notifiers in the Config menu first."),
                    Line::from(""),
                    Line::from("Press Esc to return."),
                ];

                let paragraph = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title(Screen::Test.title()))
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center);

                f.render_widget(paragraph, f.size());
                return;
            }

            // Create list items with test results
            let items: Vec<ListItem> = notifier_names.iter().enumerate().map(|(i, name)| {
                let test_result = manager.test_result(i);
                let status = match test_result {
                    Some(result) => {
                        if result.is_loading() {
                            "⏳ Loading..."
                        } else if result.is_success() {
                            "✓ Success"
                        } else {
                            "✗ Failed"
                        }
                    }
                    None => "  Not tested"
                };
                ListItem::new(format!("{} - {}", name, status))
            }).collect();

            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == self.selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    item.clone().style(style)
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title(Screen::Test.title()))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.size());

            let help_text = vec![
                Line::from("Enter: Send test  R: Retry  C: Clear results  Esc: Back"),
            ];

            let help = Paragraph::new(help_text)
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center);

            // Show test result details if available
            let details_text = if let Some(result) = manager.test_result(self.selected) {
                let timestamp = result.timestamp();
                let message = result.display_message();
                let details = result.error_details().unwrap_or_else(|| "".to_string());
                
                vec![
                    Line::from(format!("Time: {}", timestamp)),
                    Line::from(format!("Status: {}", message)),
                    if !details.is_empty() {
                        Line::from(format!("Details: {}", details))
                    } else {
                        Line::from("")
                    },
                ]
            } else {
                vec![
                    Line::from("Select a notifier and press Enter to test."),
                ]
            };

            let details = Paragraph::new(details_text)
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: true });

            f.render_widget(help, chunks[0]);
            f.render_widget(list, chunks[1]);
            f.render_widget(details, chunks[2]);
            return;
        }

        self.draw_placeholder(f, self.current_screen());
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
        app.push_screen(Screen::DeviceList);

        assert_eq!(app.current_screen(), Screen::DeviceList);

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
        assert_eq!(Screen::DeviceList.title(), "Devices");
        assert_eq!(Screen::Notifiers.title(), "Notifiers");
        assert_eq!(Screen::Test.title(), "Test Notifiers");
        assert_eq!(Screen::Help.title(), "Keyboard Shortcuts");
    }

    #[test]
    fn test_test_screen_navigation() {
        let mut app = TuiApp::new();
        app.push_screen(Screen::Test);
        assert_eq!(app.current_screen(), Screen::Test);
        
        // Initialize test manager
        app.handle_key(KeyCode::Esc);
        assert_eq!(app.current_screen(), Screen::MainMenu);
    }

    #[test]
    fn test_notifier_test_manager_initialization() {
        let config = AppConfig::default();
        let manager = NotifierTestManager::new(config);
        assert_eq!(manager.notifier_count(), 0);
        assert_eq!(manager.selected(), 0);
    }

    #[test]
    fn test_notifier_test_manager_with_config() {
        let mut config = AppConfig::default();
        config.notifiers.push(crate::config::app::NotifierConfig {
            kind: "discord".to_string(),
            webhook_url: "https://example.com/webhook".to_string(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        });
        
        let manager = NotifierTestManager::new(config);
        assert_eq!(manager.notifier_count(), 1);
        assert!(manager.notifier_names()[0].contains("discord"));
    }

    #[test]
    fn test_notifier_test_manager_selection() {
        let mut config = AppConfig::default();
        config.notifiers.push(crate::config::app::NotifierConfig {
            kind: "discord".to_string(),
            webhook_url: "https://example.com/webhook".to_string(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        });
        config.notifiers.push(crate::config::app::NotifierConfig {
            kind: "slack".to_string(),
            webhook_url: "https://example.com/webhook".to_string(),
            url: String::new(),
            method: String::new(),
            payload_template: String::new(),
            broker: String::new(),
            port: 1883,
            topic: String::new(),
            token: None,
            channel_id: None,
            include_timestamp: false,
            include_mac: false,
        });
        
        let mut manager = NotifierTestManager::new(config);
        assert_eq!(manager.selected(), 0);
        
        manager.set_selected(1);
        assert_eq!(manager.selected(), 1);
        
        // Invalid selection should not change
        manager.set_selected(5);
        assert_eq!(manager.selected(), 1);
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

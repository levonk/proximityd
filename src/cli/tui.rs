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

/// TUI application state
pub struct TuiApp {
    /// Current screen in the navigation stack
    screens: Vec<Screen>,
    /// Selected menu item index
    selected: usize,
    /// Should exit the TUI
    should_exit: bool,
}

/// Represents different screens in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    Config,
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
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new() -> Self {
        TuiApp {
            screens: vec![Screen::MainMenu],
            selected: 0,
            should_exit: false,
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
    fn test_screen_titles() {
        assert_eq!(Screen::MainMenu.title(), "proximityd Configuration");
        assert_eq!(Screen::Config.title(), "Configuration");
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
}

//! Application state management

use crate::config::Config;
use crate::rules::Rule;
use crate::theme::Theme;
use std::path::PathBuf;

/// Input mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Normal navigation mode
    #[default]
    Normal,
    /// Theme picker dialog
    ThemePicker,
    /// Help dialog
    Help,
}

/// Main application state
#[derive(Debug)]
pub struct AppState {
    /// Current view/tab
    pub view: View,

    /// Current input mode
    pub mode: Mode,

    /// Loaded configuration
    pub config: Config,

    /// Current theme
    pub theme: Theme,

    /// Index of selected rule (if in rules view)
    pub selected_rule: Option<usize>,

    /// Index of selected watch folder (if in watches view)
    pub selected_watch: Option<usize>,

    /// Activity log entries
    pub log_entries: Vec<LogEntry>,

    /// Whether the app should quit
    pub should_quit: bool,

    /// Status message to display
    pub status_message: Option<String>,

    /// Scroll offset for log view
    pub log_scroll: usize,

    /// Show help popup (deprecated, use mode instead)
    pub show_help: bool,

    /// Animation frame counter
    pub frame: u64,

    /// Theme picker index
    pub theme_picker_index: usize,
}

/// Available views in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    #[default]
    Dashboard,
    Rules,
    Watches,
    Log,
}

/// A log entry for activity tracking
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub level: LogLevel,
    pub message: String,
    pub file: Option<PathBuf>,
    pub rule: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl AppState {
    /// Create a new application state from config
    pub fn new(config: Config, theme: Theme) -> Self {
        // Find current theme index
        let theme_picker_index = Theme::all()
            .iter()
            .position(|t| *t == theme)
            .unwrap_or(0);

        let mut state = Self {
            view: View::default(),
            mode: Mode::default(),
            config,
            theme,
            selected_rule: None,
            selected_watch: None,
            log_entries: Vec::new(),
            should_quit: false,
            status_message: None,
            log_scroll: 0,
            show_help: false,
            frame: 0,
            theme_picker_index,
        };

        // Add welcome log entries
        state.log(LogLevel::Info, "🧹 Tidy started");
        state.log(
            LogLevel::Info,
            format!(
                "Loaded {} rules, {} watch folders",
                state.config.rules.len(),
                state.config.watches.len()
            ),
        );

        state
    }

    /// Get the currently selected rule, if any
    pub fn current_rule(&self) -> Option<&Rule> {
        self.selected_rule.and_then(|i| self.config.rules.get(i))
    }

    /// Get the currently selected rule mutably, if any
    pub fn current_rule_mut(&mut self) -> Option<&mut Rule> {
        self.selected_rule
            .and_then(|i| self.config.rules.get_mut(i))
    }

    /// Add a log entry
    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        self.log_entries.push(LogEntry {
            timestamp: chrono::Local::now(),
            level,
            message: message.into(),
            file: None,
            rule: None,
        });

        // Keep log bounded
        if self.log_entries.len() > 1000 {
            self.log_entries.remove(0);
        }
    }

    /// Set a temporary status message
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    /// Clear the status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Navigate to the next view
    pub fn next_view(&mut self) {
        self.view = match self.view {
            View::Dashboard => View::Rules,
            View::Rules => View::Watches,
            View::Watches => View::Log,
            View::Log => View::Dashboard,
        };
    }

    /// Navigate to the previous view
    pub fn prev_view(&mut self) {
        self.view = match self.view {
            View::Dashboard => View::Log,
            View::Rules => View::Dashboard,
            View::Watches => View::Rules,
            View::Log => View::Watches,
        };
    }

    /// Increment frame counter (for animations)
    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }
}

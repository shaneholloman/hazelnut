//! Event handling for the TUI

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::state::{AppState, View};

/// Handle a key event and update state
pub fn handle_key(state: &mut AppState, key: KeyEvent) {
    // Close help popup if open
    if state.show_help {
        if matches!(key.code, KeyCode::Esc | KeyCode::Char('?') | KeyCode::Enter) {
            state.show_help = false;
        }
        return;
    }

    // Global keybindings
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c'))
        | (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
            state.should_quit = true;
            return;
        }
        (_, KeyCode::Char('q')) if state.view == View::Dashboard => {
            state.should_quit = true;
            return;
        }
        (_, KeyCode::Char('?')) | (_, KeyCode::F(1)) => {
            state.show_help = true;
            return;
        }
        (_, KeyCode::Tab) => {
            state.next_view();
            return;
        }
        (KeyModifiers::SHIFT, KeyCode::BackTab) => {
            state.prev_view();
            return;
        }
        // Number keys for quick navigation
        (_, KeyCode::Char('1')) => {
            state.view = View::Dashboard;
            return;
        }
        (_, KeyCode::Char('2')) => {
            state.view = View::Rules;
            return;
        }
        (_, KeyCode::Char('3')) => {
            state.view = View::Watches;
            return;
        }
        (_, KeyCode::Char('4')) => {
            state.view = View::Log;
            return;
        }
        // Theme cycling
        (_, KeyCode::Char('t')) if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.theme = state.theme.next();
            state.set_status(format!("Theme: {}", state.theme.name()));
            return;
        }
        _ => {}
    }

    // View-specific keybindings
    match state.view {
        View::Dashboard => handle_dashboard_key(state, key),
        View::Rules => handle_rules_key(state, key),
        View::Watches => handle_watches_key(state, key),
        View::Log => handle_log_key(state, key),
    }
}

fn handle_dashboard_key(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Char('r') => state.view = View::Rules,
        KeyCode::Char('w') => state.view = View::Watches,
        KeyCode::Char('l') => state.view = View::Log,
        _ => {}
    }
}

fn handle_rules_key(state: &mut AppState, key: KeyEvent) {
    let len = state.config.rules.len();
    if len == 0 {
        if key.code == KeyCode::Char('n') {
            state.set_status("Rule creation not implemented yet");
        }
        return;
    }

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            state.selected_rule = Some(
                state
                    .selected_rule
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(0),
            );
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.selected_rule = Some(
                state
                    .selected_rule
                    .map(|i| (i + 1).min(len - 1))
                    .unwrap_or(0),
            );
        }
        KeyCode::Home | KeyCode::Char('g') => {
            state.selected_rule = Some(0);
        }
        KeyCode::End | KeyCode::Char('G') => {
            state.selected_rule = Some(len.saturating_sub(1));
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            // Toggle rule enabled status
            if let Some(rule) = state.current_rule_mut() {
                rule.enabled = !rule.enabled;
                let name = rule.name.clone();
                let status = if rule.enabled { "enabled" } else { "disabled" };
                state.set_status(format!("Rule '{}' {}", name, status));
            }
        }
        KeyCode::Char('e') => {
            state.set_status("Rule editor not implemented yet");
        }
        KeyCode::Char('d') | KeyCode::Delete => {
            state.set_status("Delete not implemented yet (use config file)");
        }
        KeyCode::Char('n') => {
            state.set_status("New rule not implemented yet (use config file)");
        }
        _ => {}
    }
}

fn handle_watches_key(state: &mut AppState, key: KeyEvent) {
    let len = state.config.watches.len();
    if len == 0 {
        if key.code == KeyCode::Char('a') {
            state.set_status("Add watch not implemented yet");
        }
        return;
    }

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            state.selected_watch = Some(
                state
                    .selected_watch
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(0),
            );
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.selected_watch = Some(
                state
                    .selected_watch
                    .map(|i| (i + 1).min(len - 1))
                    .unwrap_or(0),
            );
        }
        KeyCode::Home | KeyCode::Char('g') => {
            state.selected_watch = Some(0);
        }
        KeyCode::End | KeyCode::Char('G') => {
            state.selected_watch = Some(len.saturating_sub(1));
        }
        KeyCode::Char('a') => {
            state.set_status("Add watch not implemented yet");
        }
        KeyCode::Char('d') | KeyCode::Delete => {
            state.set_status("Remove watch not implemented yet");
        }
        KeyCode::Char('o') | KeyCode::Enter => {
            // Open folder in file manager
            if let Some(i) = state.selected_watch
                && let Some(watch) = state.config.watches.get(i)
            {
                let path = watch.path.display().to_string();
                state.set_status(format!("Would open: {}", path));
            }
        }
        _ => {}
    }
}

fn handle_log_key(state: &mut AppState, key: KeyEvent) {
    let len = state.log_entries.len();

    match key.code {
        KeyCode::Char('c') => {
            state.log_entries.clear();
            state.log_scroll = 0;
            state.set_status("Log cleared");
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.log_scroll = state.log_scroll.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if state.log_scroll < len.saturating_sub(1) {
                state.log_scroll += 1;
            }
        }
        KeyCode::PageUp => {
            state.log_scroll = state.log_scroll.saturating_sub(10);
        }
        KeyCode::PageDown => {
            state.log_scroll = (state.log_scroll + 10).min(len.saturating_sub(1));
        }
        KeyCode::Home | KeyCode::Char('g') => {
            state.log_scroll = 0;
        }
        KeyCode::End | KeyCode::Char('G') => {
            state.log_scroll = len.saturating_sub(1);
        }
        _ => {}
    }
}

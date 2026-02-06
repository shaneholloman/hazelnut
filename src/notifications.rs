//! Desktop notifications for error alerts
//!
//! Only notifies on errors to avoid being noisy.

use notify_rust::{Notification, Timeout};
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::warn;

/// Global flag to enable/disable notifications
static NOTIFICATIONS_ENABLED: AtomicBool = AtomicBool::new(false);

/// Initialize notifications with the enabled setting
pub fn init(enabled: bool) {
    NOTIFICATIONS_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Check if notifications are enabled
pub fn is_enabled() -> bool {
    NOTIFICATIONS_ENABLED.load(Ordering::SeqCst)
}

/// Notification severity level
#[derive(Debug, Clone, Copy)]
pub enum NotificationKind {
    /// Rule execution failed
    RuleError,
    /// Watch folder issue
    WatchError,
    /// Command execution failed
    CommandError,
}

impl NotificationKind {
    fn icon(&self) -> &'static str {
        match self {
            NotificationKind::RuleError => "dialog-error",
            NotificationKind::WatchError => "dialog-warning",
            NotificationKind::CommandError => "dialog-error",
        }
    }

    fn prefix(&self) -> &'static str {
        match self {
            NotificationKind::RuleError => "Rule Error",
            NotificationKind::WatchError => "Watch Error",
            NotificationKind::CommandError => "Command Error",
        }
    }
}

/// Send a notification if enabled
///
/// This is fire-and-forget - errors are logged but don't propagate.
pub fn notify(kind: NotificationKind, message: &str) {
    if !is_enabled() {
        return;
    }

    let result = Notification::new()
        .appname("Hazelnut")
        .summary(&format!("Hazelnut: {}", kind.prefix()))
        .body(message)
        .icon(kind.icon())
        .timeout(Timeout::Milliseconds(5000))
        .show();

    if let Err(e) = result {
        warn!("Failed to send notification: {}", e);
    }
}

/// Convenience function for rule errors
pub fn notify_rule_error(rule_name: &str, error: &str) {
    notify(
        NotificationKind::RuleError,
        &format!("Rule '{}' failed: {}", rule_name, error),
    );
}

/// Convenience function for watch errors
pub fn notify_watch_error(path: &str, error: &str) {
    notify(
        NotificationKind::WatchError,
        &format!("Watch '{}': {}", path, error),
    );
}

/// Convenience function for command errors
pub fn notify_command_error(command: &str, error: &str) {
    // Truncate command if too long
    let cmd_display = if command.len() > 50 {
        format!("{}...", &command[..47])
    } else {
        command.to_string()
    };
    notify(
        NotificationKind::CommandError,
        &format!("Command '{}' failed: {}", cmd_display, error),
    );
}

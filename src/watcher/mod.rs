//! File system watcher

mod handler;

pub use handler::EventHandler;

use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use tracing::{debug, error, info};

use crate::rules::RuleEngine;

/// File system watcher that monitors directories and applies rules
pub struct Watcher {
    watcher: RecommendedWatcher,
    engine: RuleEngine,
    rx: mpsc::Receiver<Result<notify::Event, notify::Error>>,
    event_handler: EventHandler,
}

impl Watcher {
    /// Create a new watcher with the given rule engine, polling interval, and debounce duration
    pub fn new(
        engine: RuleEngine,
        polling_interval_secs: u64,
        debounce_seconds: u64,
    ) -> Result<Self> {
        let (tx, rx) = mpsc::channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                if let Err(e) = tx.send(res) {
                    error!("Failed to send watch event: {}", e);
                }
            },
            Config::default().with_poll_interval(Duration::from_secs(polling_interval_secs)),
        )?;

        Ok(Self {
            watcher,
            engine,
            rx,
            event_handler: EventHandler::new(debounce_seconds),
        })
    }

    /// Start watching a directory
    pub fn watch(&mut self, path: &Path, recursive: bool) -> Result<()> {
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        self.watcher.watch(path, mode)?;
        info!("Watching: {} (recursive: {})", path.display(), recursive);

        Ok(())
    }

    /// Stop watching a directory
    pub fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        info!("Stopped watching: {}", path.display());
        Ok(())
    }

    /// Process pending events (non-blocking)
    pub fn poll(&self) -> Result<Vec<notify::Event>> {
        let mut events = Vec::new();

        while let Ok(result) = self.rx.try_recv() {
            match result {
                Ok(event) => events.push(event),
                Err(e) => error!("Watch error: {}", e),
            }
        }

        Ok(events)
    }

    /// Process events and apply rules (with debouncing)
    pub fn process_events(&mut self) -> Result<usize> {
        let mut processed = 0;

        for event in self.poll()? {
            debug!("Event: {:?}", event.kind);

            // Only process create and modify events
            match event.kind {
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Access(_) => {
                    // Use event handler to debounce
                    let paths_to_process = self.event_handler.should_process(&event);

                    for path in paths_to_process {
                        if path.is_file() && path.exists() {
                            info!("File event detected: {}", path.display());
                            match self.engine.process(&path) {
                                Ok(true) => processed += 1,
                                Ok(false) => {} // No matching rule
                                Err(e) => {
                                    error!("Rule processing failed for {}: {}", path.display(), e);
                                    // Find which rule matched (if any) for the notification
                                    let rule_name = self
                                        .engine
                                        .evaluate(&path)
                                        .ok()
                                        .flatten()
                                        .map(|_| self.find_matching_rule_name(&path))
                                        .unwrap_or_else(|| "unknown".to_string());
                                    crate::notifications::notify_rule_error(
                                        &rule_name,
                                        &e.to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
                _ => {
                    debug!("Ignoring event kind: {:?}", event.kind);
                }
            }
        }

        // Periodically clean up old entries
        self.event_handler.cleanup();

        Ok(processed)
    }

    /// Find the name of the first matching rule for a path
    fn find_matching_rule_name(&self, path: &std::path::Path) -> String {
        for rule in self.engine.rules() {
            if rule.enabled && rule.condition.matches(path).unwrap_or(false) {
                return rule.name.clone();
            }
        }
        "unknown".to_string()
    }

    /// Get the rule engine
    pub fn engine(&self) -> &RuleEngine {
        &self.engine
    }
}

//! Event handler for file system events

use indexmap::IndexMap;
use notify::Event;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Maximum number of entries in the debounce map before forcing a cleanup
const MAX_DEBOUNCE_ENTRIES: usize = 10_000;

/// Debounces file system events to avoid processing the same file multiple times
pub struct EventHandler {
    /// Recent events by path (IndexMap preserves insertion order for fair cleanup)
    recent: IndexMap<PathBuf, Instant>,

    /// Debounce duration
    debounce: Duration,
}

impl EventHandler {
    /// Create a new event handler with the given debounce duration
    pub fn new(debounce_seconds: u64) -> Self {
        Self {
            recent: IndexMap::new(),
            debounce: Duration::from_secs(debounce_seconds),
        }
    }

    /// Check if an event should be processed (returns true if not recently seen)
    pub fn should_process(&mut self, event: &Event) -> Vec<PathBuf> {
        let now = Instant::now();
        let mut paths_to_process = Vec::new();

        for path in &event.paths {
            let should_process = self
                .recent
                .get(path)
                .map(|&last| now.duration_since(last) > self.debounce)
                .unwrap_or(true);

            if should_process {
                self.recent.insert(path.clone(), now);
                paths_to_process.push(path.clone());
            }
        }

        // If the map has grown too large, force a cleanup
        if self.recent.len() > MAX_DEBOUNCE_ENTRIES {
            self.cleanup();
        }

        paths_to_process
    }

    /// Clean up old entries (call periodically)
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        let threshold = self.debounce * 10; // Keep entries for 10x debounce period

        self.recent
            .retain(|_, &mut last| now.duration_since(last) < threshold);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::EventKind;

    #[test]
    fn test_debounce() {
        let mut handler = EventHandler::new(1);

        let event = Event {
            kind: EventKind::Create(notify::event::CreateKind::File),
            paths: vec![PathBuf::from("/tmp/test.txt")],
            attrs: Default::default(),
        };

        // First event should be processed
        let paths = handler.should_process(&event);
        assert_eq!(paths.len(), 1);

        // Immediate second event should be debounced
        let paths = handler.should_process(&event);
        assert_eq!(paths.len(), 0);
    }
}

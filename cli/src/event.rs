use std::time::Duration;

use crossterm::event::{self, KeyEvent, Event};
use tokio::sync::mpsc;

/// Re-export the API result type from the api module.
pub use crate::api::ApiResult;

/// Events produced by the event stream and consumed by the main application loop.
#[derive(Debug)]
pub enum AppEvent {
    /// Keyboard input from crossterm.
    Input(KeyEvent),
    /// Terminal was resized to (columns, rows).
    Resize(u16, u16),
    /// Periodic tick for animations (~30fps).
    Tick,
    /// An API response arrived from a background task.
    ApiResponse(ApiResult),
}

/// Merges crossterm terminal events, a 33ms tick timer, and an mpsc receiver
/// for API responses into a single async event stream.
pub struct EventStream {
    /// Receiver for API responses sent from background tokio tasks.
    api_rx: mpsc::UnboundedReceiver<ApiResult>,
    /// Tick interval duration (~30fps).
    tick_rate: Duration,
}

impl EventStream {
    /// Create a new EventStream.
    ///
    /// `api_rx` receives API results from background tasks.
    pub fn new(api_rx: mpsc::UnboundedReceiver<ApiResult>) -> Self {
        Self {
            api_rx,
            tick_rate: Duration::from_millis(33),
        }
    }

    /// Returns the next event by selecting between:
    /// - Crossterm terminal events (keyboard, resize) with a 33ms poll timeout
    /// - API responses from the mpsc channel
    ///
    /// If no crossterm event arrives within the tick interval, a Tick event is produced.
    /// API responses are checked on every iteration so they are never starved.
    pub async fn next(&mut self) -> AppEvent {
        loop {
            // Check for pending API responses first (non-blocking).
            if let Ok(result) = self.api_rx.try_recv() {
                return AppEvent::ApiResponse(result);
            }

            // Poll crossterm for terminal events with the tick timeout.
            if event::poll(self.tick_rate).expect("failed to poll terminal events") {
                let ev = event::read().expect("failed to read terminal event");
                match ev {
                    Event::Key(key_event) => return AppEvent::Input(key_event),
                    Event::Resize(cols, rows) => return AppEvent::Resize(cols, rows),
                    // Ignore mouse and other events for now.
                    _ => continue,
                }
            } else {
                // No terminal event within tick_rate — emit a Tick.
                return AppEvent::Tick;
            }
        }
    }
}

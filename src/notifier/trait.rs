use anyhow::Result;

use crate::state::PresenceEvent;

/// Generic notifier trait for all notification backends.
///
/// Implementors must be `Send + Sync` so they can safely be held in a
/// registry and dispatched from the async scan loop.
pub trait Notifier: Send + Sync {
    /// Deliver a notification for the given presence event.
    fn notify(&self, event: &PresenceEvent) -> Result<()>;
}

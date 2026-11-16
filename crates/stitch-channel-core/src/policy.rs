//! Backpressure policies for bounded channels.

/// Strategy for handling backpressure when a channel reaches capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackpressurePolicy {
    /// Reject item immediately and return [`crate::error::ChannelError::Full`].
    #[default]
    Reject,
    /// Overwrite oldest unread items in the ring (Lossy / Telemetry / Game State mode).
    DropOldest,
    /// Spin/yield wait until capacity becomes available.
    Block,
}

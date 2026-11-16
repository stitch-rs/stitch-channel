//! Error types and result invariants for `stitch-channel`.

use core::fmt;

/// Primary error variants for channel operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelError<T> {
    /// Channel buffer is full and backpressure rejects additional items.
    Full(T),
    /// Channel buffer is currently empty.
    Empty,
    /// Channel has been disconnected or closed by peer.
    Disconnected,
    /// Consumer lagged behind in an overwrite ring buffer.
    Lagged {
        /// Number of unread items dropped/missed.
        missed: u64,
    },
}

impl<T> fmt::Display for ChannelError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full(_) => write!(f, "channel is full"),
            Self::Empty => write!(f, "channel is empty"),
            Self::Disconnected => write!(f, "channel is disconnected"),
            Self::Lagged { missed } => write!(f, "consumer lagged by {missed} items"),
        }
    }
}

#[cfg(feature = "std")]
impl<T: fmt::Debug> std::error::Error for ChannelError<T> {}

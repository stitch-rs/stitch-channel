//! Fluent builder for ergonomic channel creation.

use stitch_channel_core::topology::spsc::SpscBuffer;

/// Fluent builder for creating channels.
pub struct ChannelBuilder<const CAP: usize = 1024>;

impl<const CAP: usize> ChannelBuilder<CAP> {
    /// Specifies capacity for the channel (must be power of two).
    pub const fn with_capacity<const NEW_CAP: usize>() -> ChannelBuilder<NEW_CAP> {
        ChannelBuilder
    }

    /// Builds a static SPSC buffer.
    pub const fn build_spsc<T>() -> SpscBuffer<T, CAP> {
        SpscBuffer::new()
    }
}

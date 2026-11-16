#![no_std]
#![deny(missing_docs)]

//! `stitch-channel`: High-performance monomorphic channel facade, builders, and SMA integration.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod builder;
pub mod egress;
pub mod fanout;
pub mod ingress;

pub use builder::ChannelBuilder;
pub use egress::ChannelEgressTerminal;
pub use fanout::StaticFanout;
pub use ingress::ChannelIngressLayer;

// Re-exports from core
pub use stitch_channel_core::cursor::CachePaddedAtomicUsize;
pub use stitch_channel_core::error::ChannelError;
pub use stitch_channel_core::policy::BackpressurePolicy;
pub use stitch_channel_core::topology::broadcast::StaticMulticast;
pub use stitch_channel_core::topology::spsc::{Consumer, Producer, SpscBuffer};

/// Prelude containing commonly used types and traits.
pub mod prelude {
    pub use crate::builder::ChannelBuilder;
    pub use crate::egress::ChannelEgressTerminal;
    pub use crate::fanout::StaticFanout;
    pub use crate::ingress::ChannelIngressLayer;
    pub use stitch_channel_core::error::ChannelError;
    pub use stitch_channel_core::policy::BackpressurePolicy;
    pub use stitch_channel_core::topology::broadcast::StaticMulticast;
    pub use stitch_channel_core::topology::spsc::{Consumer, Producer, SpscBuffer};
}

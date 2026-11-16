//! Static monomorphic fan-out composition for SMA.

use stitch_channel_core::topology::broadcast::{BroadcastError, StaticMulticast};

/// Static fan-out pipeline layer dispatching items across compile-time tuple of subscribers.
pub struct StaticFanout<S> {
    subscribers: S,
}

impl<S> StaticFanout<S> {
    /// Creates a new static fan-out dispatcher wrapping a tuple of multicast subscribers.
    #[inline(always)]
    pub const fn new(subscribers: S) -> Self {
        Self { subscribers }
    }

    /// Access the underlying subscribers tuple.
    #[inline(always)]
    pub fn subscribers(&self) -> &S {
        &self.subscribers
    }

    /// Mutable access to the underlying subscribers tuple.
    #[inline(always)]
    pub fn subscribers_mut(&mut self) -> &mut S {
        &mut self.subscribers
    }
}

impl<T: Clone, S: StaticMulticast<T>> StaticMulticast<T> for StaticFanout<S> {
    #[inline(always)]
    fn broadcast(&mut self, item: &T) -> Result<(), BroadcastError> {
        self.subscribers.broadcast(item)
    }
}

//! Static monomorphic fan-out / multicast primitives.

use core::fmt;

/// Error emitted when broadcast dispatch fails across subscribers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BroadcastError {
    /// At least one subscriber failed to accept item.
    DeliveryFailed,
}

impl fmt::Display for BroadcastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "multicast broadcast delivery failed")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BroadcastError {}

/// Trait for static broadcast endpoints.
pub trait StaticMulticast<T> {
    /// Broadcast an item reference across receivers with zero dynamic dispatch.
    fn broadcast(&mut self, item: &T) -> Result<(), BroadcastError>;
}

macro_rules! impl_static_multicast_tuple {
    ($($idx:tt: $elem:ident),+) => {
        impl<T: Clone, $($elem: StaticMulticast<T>),+> StaticMulticast<T> for ($($elem,)+) {
            #[inline(always)]
            fn broadcast(&mut self, item: &T) -> Result<(), BroadcastError> {
                $(
                    self.$idx.broadcast(item)?;
                )+
                Ok(())
            }
        }
    };
}

impl_static_multicast_tuple!(0: A, 1: B);
impl_static_multicast_tuple!(0: A, 1: B, 2: C);
impl_static_multicast_tuple!(0: A, 1: B, 2: C, 3: D);
impl_static_multicast_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_static_multicast_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_static_multicast_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_static_multicast_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);

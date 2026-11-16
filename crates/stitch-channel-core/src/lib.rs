#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

//! `stitch-channel-core`: Pure `#![no_std]` zero-allocation, cacheline-aligned channel and ring primitives.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod cursor;
pub mod error;
pub mod policy;
pub mod topology;

pub use cursor::CachePaddedAtomicUsize;
pub use error::ChannelError;
pub use policy::BackpressurePolicy;
pub use topology::broadcast::StaticMulticast;
pub use topology::spsc::{Consumer, Producer, SpscBuffer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spsc_basic_send_recv() {
        let buffer: SpscBuffer<u64, 4> = SpscBuffer::new();
        let (mut tx, mut rx) = buffer.split();

        assert_eq!(rx.try_recv(), Err(ChannelError::Empty));
        assert!(tx.try_send(42).is_ok());
        assert!(tx.try_send(43).is_ok());
        assert!(tx.try_send(44).is_ok());
        assert!(tx.try_send(45).is_ok());
        assert_eq!(tx.try_send(46), Err(ChannelError::Full(46)));

        assert_eq!(rx.try_recv(), Ok(42));
        assert_eq!(rx.try_recv(), Ok(43));
        assert_eq!(rx.try_recv(), Ok(44));
        assert_eq!(rx.try_recv(), Ok(45));
        assert_eq!(rx.try_recv(), Err(ChannelError::Empty));
    }
}

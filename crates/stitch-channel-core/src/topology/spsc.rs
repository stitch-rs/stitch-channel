//! Single-Producer Single-Consumer (SPSC) cacheline-aligned lock-free channel.

use crate::cursor::CachePaddedAtomicUsize;
use crate::error::ChannelError;
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

/// Storage structure for SPSC channel, 64-byte aligned to prevent false sharing.
pub struct SpscBuffer<T, const CAP: usize> {
    head: CachePaddedAtomicUsize,
    tail: CachePaddedAtomicUsize,
    closed: AtomicBool,
    slots: [UnsafeCell<MaybeUninit<T>>; CAP],
}

// SAFETY: SPSC synchronization coordinates single producer and single consumer access across threads.
unsafe impl<T: Send, const CAP: usize> Sync for SpscBuffer<T, CAP> {}

impl<T, const CAP: usize> SpscBuffer<T, CAP> {
    /// Compile-time invariant asserting power of two capacity.
    const CHECK_POWER_OF_TWO: () = {
        assert!(CAP > 0, "Capacity must be greater than zero");
        assert!(CAP.is_power_of_two(), "Capacity must be a power of two");
    };

    /// Initializes a new static/stack SPSC buffer.
    pub const fn new() -> Self {
        #[allow(clippy::let_unit_value)]
        let _ = Self::CHECK_POWER_OF_TWO;

        // SAFETY: [UnsafeCell<MaybeUninit<T>>; CAP] consists entirely of MaybeUninit elements,
        // for which uninitialized memory is completely valid and sound.
        let slots =
            unsafe { MaybeUninit::<[UnsafeCell<MaybeUninit<T>>; CAP]>::uninit().assume_init() };

        Self {
            head: CachePaddedAtomicUsize::new(0),
            tail: CachePaddedAtomicUsize::new(0),
            closed: AtomicBool::new(false),
            slots,
        }
    }

    /// Splits buffer into borrowed producer and consumer handles.
    pub fn split(&self) -> (Producer<'_, T, CAP>, Consumer<'_, T, CAP>) {
        (
            Producer {
                buffer: self,
                cached_tail: 0,
            },
            Consumer {
                buffer: self,
                cached_head: 0,
            },
        )
    }
}

impl<T, const CAP: usize> Default for SpscBuffer<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

/// SPSC channel producer handle.
pub struct Producer<'a, T, const CAP: usize> {
    buffer: &'a SpscBuffer<T, CAP>,
    cached_tail: usize,
}

impl<'a, T, const CAP: usize> Producer<'a, T, CAP> {
    const MASK: usize = CAP - 1;

    /// Attempts to send an item into the channel without blocking.
    #[inline(always)]
    pub fn try_send(&mut self, item: T) -> Result<(), ChannelError<T>> {
        if self.buffer.closed.load(Ordering::Relaxed) {
            return Err(ChannelError::Disconnected);
        }

        let head = self.buffer.head.load(Ordering::Relaxed);
        if head.wrapping_sub(self.cached_tail) >= CAP {
            self.cached_tail = self.buffer.tail.load(Ordering::Acquire);
            if head.wrapping_sub(self.cached_tail) >= CAP {
                return Err(ChannelError::Full(item));
            }
        }

        let idx = head & Self::MASK;
        // SAFETY: Single producer owns head slot write.
        unsafe {
            let slot = self.buffer.slots.get_unchecked(idx).get();
            (*slot).write(item);
        }

        self.buffer
            .head
            .store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Closes the channel.
    #[inline(always)]
    pub fn close(&self) {
        self.buffer.closed.store(true, Ordering::Release);
    }
}

/// SPSC channel consumer handle.
pub struct Consumer<'a, T, const CAP: usize> {
    buffer: &'a SpscBuffer<T, CAP>,
    cached_head: usize,
}

impl<'a, T, const CAP: usize> Consumer<'a, T, CAP> {
    const MASK: usize = CAP - 1;

    /// Attempts to receive an item from the channel without blocking.
    #[inline(always)]
    pub fn try_recv(&mut self) -> Result<T, ChannelError<T>> {
        let tail = self.buffer.tail.load(Ordering::Relaxed);
        if tail == self.cached_head {
            self.cached_head = self.buffer.head.load(Ordering::Acquire);
            if tail == self.cached_head {
                if self.buffer.closed.load(Ordering::Acquire) {
                    return Err(ChannelError::Disconnected);
                }
                return Err(ChannelError::Empty);
            }
        }

        let idx = tail & Self::MASK;
        // SAFETY: Single consumer owns tail slot read.
        let val = unsafe {
            let slot = self.buffer.slots.get_unchecked(idx).get();
            (*slot).assume_init_read()
        };

        self.buffer
            .tail
            .store(tail.wrapping_add(1), Ordering::Release);
        Ok(val)
    }

    /// Checks if channel is closed and all items consumed.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let tail = self.buffer.tail.load(Ordering::Relaxed);
        let head = self.buffer.head.load(Ordering::Acquire);
        tail == head
    }
}

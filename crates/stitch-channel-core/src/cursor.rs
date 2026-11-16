//! Mechanical Sympathy cache-line isolated atomic cursors.

use core::sync::atomic::{AtomicUsize, Ordering};

/// 64-byte aligned atomic cursor eliminating false sharing between CPU cores.
#[repr(align(64))]
pub struct CachePaddedAtomicUsize {
    inner: AtomicUsize,
}

impl CachePaddedAtomicUsize {
    /// Creates a new cache-line isolated atomic counter.
    #[inline(always)]
    pub const fn new(val: usize) -> Self {
        Self {
            inner: AtomicUsize::new(val),
        }
    }

    /// Loads the cursor value with specified memory ordering.
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> usize {
        self.inner.load(order)
    }

    /// Stores the cursor value with specified memory ordering.
    #[inline(always)]
    pub fn store(&self, val: usize, order: Ordering) {
        self.inner.store(val, order)
    }

    /// Performs compare-exchange.
    #[inline(always)]
    pub fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<usize, usize> {
        self.inner.compare_exchange(current, new, success, failure)
    }
}

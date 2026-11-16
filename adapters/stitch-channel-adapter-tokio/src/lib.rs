//! Tokio asynchronous channel adapters for `stitch-channel`.

use stitch_channel_core::error::ChannelError;
use tokio::sync::mpsc;

/// Asynchronous adapter bridging a `tokio::sync::mpsc::Sender` to `stitch-channel`.
pub struct TokioMpscSender<T> {
    inner: mpsc::Sender<T>,
}

impl<T> TokioMpscSender<T> {
    /// Creates a new adapter wrapping a Tokio MPSC sender.
    pub fn new(inner: mpsc::Sender<T>) -> Self {
        Self { inner }
    }

    /// Attempts to send an item into the Tokio channel synchronously.
    pub fn try_send(&self, item: T) -> Result<(), ChannelError<T>> {
        self.inner.try_send(item).map_err(|err| match err {
            mpsc::error::TrySendError::Full(val) => ChannelError::Full(val),
            mpsc::error::TrySendError::Closed(_val) => ChannelError::Disconnected,
        })
    }
}

/// Asynchronous adapter bridging a `tokio::sync::mpsc::Receiver` to `stitch-channel`.
pub struct TokioMpscReceiver<T> {
    inner: mpsc::Receiver<T>,
}

impl<T> TokioMpscReceiver<T> {
    /// Creates a new adapter wrapping a Tokio MPSC receiver.
    pub fn new(inner: mpsc::Receiver<T>) -> Self {
        Self { inner }
    }

    /// Attempts to receive an item from the Tokio channel synchronously.
    pub fn try_recv(&mut self) -> Result<T, ChannelError<T>> {
        self.inner.try_recv().map_err(|err| match err {
            mpsc::error::TryRecvError::Empty => ChannelError::Empty,
            mpsc::error::TryRecvError::Disconnected => ChannelError::Disconnected,
        })
    }
}

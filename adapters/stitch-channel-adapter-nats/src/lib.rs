//! NATS messaging and streaming bridge for `stitch-channel`.

use stitch_channel_core::error::ChannelError;

/// Trait representing an abstract NATS wire sink.
pub trait NatsSink {
    /// Publish binary payload to a subject.
    fn publish(&mut self, subject: &str, payload: &[u8]) -> Result<(), ChannelError<&[u8]>>;
}

/// A NATS publisher port adapter for stitch-channel.
pub struct NatsPublisherPort<S> {
    subject: &'static str,
    sink: S,
}

impl<S: NatsSink> NatsPublisherPort<S> {
    /// Creates a new NATS publisher port bound to a static subject.
    pub const fn new(subject: &'static str, sink: S) -> Self {
        Self { subject, sink }
    }

    /// Publishes a byte slice via the underlying NATS sink.
    pub fn send(&mut self, payload: &[u8]) -> Result<(), ChannelError<&[u8]>> {
        self.sink.publish(self.subject, payload)
    }
}

//! SMA Terminal integration: writes pipeline items into a channel producer.

use stitch_channel_core::error::ChannelError;
use stitch_channel_core::topology::spsc::Producer;
#[cfg(feature = "stitch")]
use stitch_core::Terminal;

/// Terminal adapter writing into an SPSC channel producer at the bottom of a U-cycle.
pub struct ChannelEgressTerminal<'a, T, const CAP: usize> {
    producer: Producer<'a, T, CAP>,
}

impl<'a, T, const CAP: usize> ChannelEgressTerminal<'a, T, CAP> {
    /// Creates a new egress terminal wrapping a channel producer.
    #[inline(always)]
    pub fn new(producer: Producer<'a, T, CAP>) -> Self {
        Self { producer }
    }
}

#[cfg(feature = "stitch")]
impl<'a, TCtx, T, const CAP: usize> Terminal<TCtx, T, Result<(), ChannelError<T>>, ChannelError<T>>
    for ChannelEgressTerminal<'a, T, CAP>
{
    #[inline(always)]
    fn execute(
        &mut self,
        _ctx: &mut TCtx,
        item: T,
    ) -> Result<Result<(), ChannelError<T>>, ChannelError<T>> {
        match self.producer.try_send(item) {
            Ok(()) => Ok(Ok(())),
            Err(e) => Err(e),
        }
    }
}

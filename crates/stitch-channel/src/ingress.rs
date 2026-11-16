//! SMA Layer integration: passes items down the U-cycle.

#[cfg(feature = "stitch")]
use stitch_core::{FlowControl, Layer};

/// Ingress layer passing intents through the U-cycle.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChannelIngressLayer;

impl ChannelIngressLayer {
    /// Creates a new ingress layer.
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(feature = "stitch")]
impl<TCtx, TIntent, TOutcome, TErr> Layer<TCtx, TIntent, TOutcome, TErr> for ChannelIngressLayer {
    #[inline(always)]
    fn on_enter(&self, _ctx: &mut TCtx, intent: TIntent) -> FlowControl<TIntent, TOutcome, TErr> {
        FlowControl::Proceed(intent)
    }

    #[inline(always)]
    fn on_exit(&self, _ctx: &mut TCtx, _outcome: &mut Result<TOutcome, TErr>) {}
}

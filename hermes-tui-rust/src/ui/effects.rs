//! Aetheric Shaders — Protocol-Linked DSL Effects (Phase 4.2)
//!
//! Manages a `tachyonfx::EffectManager` for lifecycle-tracking effects
//! (coalesce on LLM stream, HSL shift on tool execution). This implementation
//! follows tachyonfx v0.25 patterns and bridges types to Ratatui 0.28.
//!
//! ## Effect Pipeline
//!
//! 1. **LLM Delta Streaming** — `fx::coalesce` over newly appended text
//!    blocks, smoothing the appearance of streaming tokens.
//! 2. **Tool Execution** — `fx::hsl_shift` with a `SweepPattern` sweeping
//!    a bright cyan/green line across the area.
//!
//! All effects are gated behind [`set_low_motion`] for resource-constrained environments.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use tachyonfx::pattern::SweepPattern;
use tachyonfx::{self as txfx, fx, EffectManager as TxFxManager, EffectTimer, Interpolation};

/// Global gate for low-motion mode.
static mut LOW_MOTION: bool = false;

/// Enable or disable low-motion mode.
pub fn set_low_motion(enabled: bool) {
    unsafe {
        LOW_MOTION = enabled;
    }
}

/// Whether effects are currently allowed.
#[inline]
fn effects_enabled() -> bool {
    !unsafe { LOW_MOTION }
}

/// Shader state tied to the TUI event loop.
///
/// Wraps `tachyonfx::EffectManager` and provides methods for triggering
/// protocol-linked effects.
#[derive(Debug)]
pub struct EffectManager {
    inner: TxFxManager<()>,
}

impl Default for EffectManager {
    fn default() -> Self {
        Self {
            inner: TxFxManager::default(),
        }
    }
}

impl EffectManager {
    /// Create a new EffectManager with an empty effect queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigger the LLM-delta coalesce effect (300ms).
    pub fn trigger_stream_effect(&mut self) {
        if !effects_enabled() {
            return;
        }
        let timer = EffectTimer::from_ms(300, Interpolation::Linear);
        self.inner.add_effect(fx::coalesce(timer));
    }

    /// Trigger the tool-execution HSL shift sweep effect (400ms).
    pub fn trigger_tool_effect(&mut self) {
        if !effects_enabled() {
            return;
        }
        let timer = EffectTimer::from_ms(400, Interpolation::SineInOut);

        // v0.25 pattern: hsl_shift(Option<[f32; 3]>, Option<[f32; 3]>, timer)
        // [hue_add, sat_mul, light_mul]
        let effect = fx::hsl_shift(Some([60.0, 1.2, 1.15]), None, timer)
            .with_pattern(SweepPattern::left_to_right(10));

        self.inner.add_effect(effect);
    }

    /// Trigger a "grand" view-transition effect (300ms).
    /// Faster and more visually complex than the previous version.
    pub fn trigger_view_transition(&mut self) {
        if !effects_enabled() {
            return;
        }

        // 1. High-speed chromatic sweep (White/Blue flash)
        let sweep_timer = EffectTimer::from_ms(150, Interpolation::QuadOut);
        let sweep = fx::hsl_shift(Some([200.0, 1.5, 1.5]), None, sweep_timer)
            .with_pattern(SweepPattern::left_to_right(5));
        self.inner.add_effect(sweep);

        // 2. Coalesce reveal (slightly offset)
        let coal_timer = EffectTimer::from_ms(300, Interpolation::CubicOut);
        self.inner.add_effect(fx::coalesce(coal_timer));

        // 3. Glitch-like HSL jitter (very fast)
        let jitter_timer = EffectTimer::from_ms(100, Interpolation::Linear);
        self.inner
            .add_effect(fx::hsl_shift(Some([0.0, 2.0, 0.8]), None, jitter_timer));
    }

    /// Trigger a "Grand Loader" effect for significant operations.
    pub fn trigger_grand_loader(&mut self) {
        if !effects_enabled() {
            return;
        }
        let timer = EffectTimer::from_ms(800, Interpolation::SineInOut);

        // Circular-like HSL rotation sweep
        let effect = fx::hsl_shift(Some([360.0, 1.3, 1.1]), None, timer)
            .with_pattern(SweepPattern::up_to_down(20));

        self.inner.add_effect(effect);
    }

    /// Advance effect timers (called from the demand-driven ticker).
    pub fn advance(&mut self) {
        if !effects_enabled() || !self.inner.is_running() {
            return;
        }

        // We advance the internal state by processing with a dummy buffer if no real
        // buffer is available. In Hermes, this is called during the tick loop.
        let mut buf =
            ratatui_core::buffer::Buffer::empty(ratatui_core::layout::Rect::new(0, 0, 0, 0));
        self.inner.process_effects(
            txfx::Duration::from_millis(16),
            &mut buf,
            ratatui_core::layout::Rect::default(),
        );
    }

    /// Whether any effects are currently running.
    #[inline]
    pub fn is_running(&self) -> bool {
        self.inner.is_running()
    }

    /// Apply the current effects to the frame buffer.
    ///
    /// Bridges Ratatui 0.28 types to tachyonfx's internal ratatui-core 0.1 dependency
    /// using a surgical transmute (safe due to identical memory layout).
    pub fn apply(&mut self, buf: &mut Buffer, area: Rect, delta_ms: u64) {
        if !effects_enabled() || !self.inner.is_running() {
            return;
        }

        let core_buf: &mut ratatui_core::buffer::Buffer = unsafe { std::mem::transmute(buf) };
        let core_area: ratatui_core::layout::Rect = unsafe { std::mem::transmute(area) };

        self.inner.process_effects(
            txfx::Duration::from_millis(delta_ms as u32),
            core_buf,
            core_area,
        );
    }
}

/// Sinusoidal colour offsets for the wave footer.
pub fn wave_palette_shift(phase: f64) -> (Color, Color) {
    let r = (128.0 + (phase * 0.7).sin() * 40.0) as u8;
    let g = (180.0 + (phase * 1.3).cos() * 30.0) as u8;
    let b = (220.0 + (phase * 0.5).sin() * 35.0) as u8;

    let p2 = (phase * 2.0 + 1.2).sin();
    let r2 = (160.0 + p2 * 45.0) as u8;
    let g2 = (200.0 - p2 * 25.0) as u8;
    let b2 = (120.0 + p2 * 40.0) as u8;

    (Color::Rgb(r, g, b), Color::Rgb(r2, g2, b2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effects_enabled_default() {
        assert!(effects_enabled());
    }

    #[test]
    fn test_low_motion_gate() {
        set_low_motion(true);
        assert!(!effects_enabled());
        set_low_motion(false);
        assert!(effects_enabled());
    }

    #[test]
    fn test_wave_palette_shift_is_smooth() {
        let (a, _) = wave_palette_shift(0.0);
        let (c, _) = wave_palette_shift(1.0);
        assert_ne!(format!("{a:?}"), format!("{c:?}"));
    }

    #[test]
    fn test_effect_manager_default() {
        let manager = EffectManager::new();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_trigger_stream() {
        let mut manager = EffectManager::new();
        manager.trigger_stream_effect();
        assert!(manager.is_running());
    }
}

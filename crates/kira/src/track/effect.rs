//! Modifies audio signals.

pub mod delay;
pub mod distortion;
pub mod filter;
pub mod reverb;

use crate::{clock::Clocks, dsp::Frame};

pub trait EffectBuilder {
	type Handle;

	fn into_effect(self) -> (Box<dyn Effect>, Self::Handle);
}

/// Receives input audio from a mixer track and outputs modified audio.
#[allow(unused_variables)]
pub trait Effect: Send + Sync {
	/// Called when the effect is first sent to the renderer.
	fn init(&mut self, sample_rate: u32) {}

	fn on_start_processing(&mut self) {}

	/// Transforms an input [`Frame`].
	/// - `input` is the input audio
	/// - `dt` is the time that's elapsed since the previous round of
	/// processing (in seconds)
	/// - `parameters` contains information about the current value of
	/// parameters. This is an opaque type that's only useful for updating
	/// `CachedValue`s.
	fn process(&mut self, input: Frame, dt: f64, clocks: &mut Clocks) -> Frame;
}

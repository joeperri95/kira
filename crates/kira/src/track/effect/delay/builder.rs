use crate::track::effect::{Effect, EffectBuilder};

use super::{effect::Delay, DelayHandle};

/// Settings for a [`Delay`] effect.
#[non_exhaustive]
pub struct DelayBuilder {
	/// The delay time (in seconds).
	pub delay_time: f64,
	/// The amount of feedback.
	pub feedback: f64,
	/// The amount of audio the delay can store (in seconds).
	/// This affects the maximum delay time.
	pub buffer_length: f64,
	/// Effects that should be applied in the feedback loop.
	pub feedback_effects: Vec<Box<dyn Effect>>,
	/// How much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means
	/// only the dry signal will be heard. `1.0` means
	/// only the wet signal will be heard.
	pub mix: f64,
}

impl DelayBuilder {
	/// Creates a new `DelaySettings` with the default settings.
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets the delay time (in seconds).
	pub fn set_delay_time(&mut self, delay_time: f64) {
		self.delay_time = delay_time;
	}

	/// Sets the amount of feedback.
	pub fn set_feedback(&mut self, feedback: f64) {
		self.feedback = feedback;
	}

	/// Sets the amount of audio the delay can store.
	pub fn set_buffer_length(&mut self, buffer_length: f64) {
		self.buffer_length = buffer_length;
	}

	/// Adds an effect to the feedback loop.
	pub fn add_feedback_effect<B: EffectBuilder>(&mut self, builder: B) -> B::Handle {
		let (effect, handle) = builder.into_effect();
		self.feedback_effects.push(effect);
		handle
	}

	/// Sets how much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means only the dry
	/// signal will be heard. `1.0` means only the wet signal will
	/// be heard.
	pub fn set_mix(&mut self, mix: f64) {
		self.mix = mix;
	}
}

impl Default for DelayBuilder {
	fn default() -> Self {
		Self {
			delay_time: 0.5,
			feedback: 0.5,
			buffer_length: 10.0,
			feedback_effects: vec![],
			mix: 0.5,
		}
	}
}

impl EffectBuilder for DelayBuilder {
	type Handle = DelayHandle;

	fn into_effect(self) -> (Box<dyn Effect>, Self::Handle) {
		let (effect, handle) = Delay::new(self);
		(Box::new(effect), handle)
	}
}

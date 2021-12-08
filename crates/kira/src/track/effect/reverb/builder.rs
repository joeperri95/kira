use crate::track::effect::{Effect, EffectBuilder};

use super::{effect::Reverb, ReverbHandle};

/// Settings for a `Reverb`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct ReverbBuilder {
	/// How much the room reverberates. A higher value will
	/// result in a bigger sounding room. 1.0 gives an infinitely
	/// reverberating room.
	pub feedback: f64,
	/// How quickly high frequencies disappear from the reverberation.
	pub damping: f64,
	/// The stereo width of the reverb effect (0.0 being fully mono,
	/// 1.0 being fully stereo).
	pub stereo_width: f64,
	/// How much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means
	/// only the dry signal will be heard. `1.0` means
	/// only the wet signal will be heard.
	pub mix: f64,
}

impl ReverbBuilder {
	/// Creates a new `ReverbSettings` with the default settings.
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets how much the room reverberates. A higher value will
	/// result in a bigger sounding room. 1.0 gives an infinitely
	/// reverberating room.
	pub fn feedback(self, feedback: f64) -> Self {
		Self { feedback, ..self }
	}

	/// Sets how quickly high frequencies disappear from the reverberation.
	pub fn damping(self, damping: f64) -> Self {
		Self { damping, ..self }
	}

	/// Sets the stereo width of the reverb effect (0.0 being fully mono,
	/// 1.0 being fully stereo).
	pub fn stereo_width(self, stereo_width: f64) -> Self {
		Self {
			stereo_width,
			..self
		}
	}

	/// Sets how much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means only the dry
	/// signal will be heard. `1.0` means only the wet signal will
	/// be heard.
	pub fn mix(self, mix: f64) -> Self {
		Self { mix, ..self }
	}
}

impl Default for ReverbBuilder {
	fn default() -> Self {
		Self {
			feedback: 0.9,
			damping: 0.1,
			stereo_width: 1.0,
			mix: 0.5,
		}
	}
}

impl EffectBuilder for ReverbBuilder {
	type Handle = ReverbHandle;

	fn into_effect(self) -> (Box<dyn Effect>, Self::Handle) {
		let (effect, handle) = Reverb::new(self);
		(Box::new(effect), handle)
	}
}

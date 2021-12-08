use crate::track::effect::{Effect, EffectBuilder};

use super::{effect::Distortion, DistortionHandle, DistortionKind};

/// Settings for a [`Distortion`] effect.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct DistortionBuilder {
	/// The kind of distortion to use.
	pub kind: DistortionKind,
	/// The factor to multiply the signal by before applying
	/// the distortion.
	pub drive: f64,
	/// How much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means
	/// only the dry signal will be heard. `1.0` means
	/// only the wet signal will be heard.
	pub mix: f64,
}

impl DistortionBuilder {
	/// Creates a new `DistortionSettings` with the default settings.
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets the kind of distortion to use.
	pub fn kind(self, kind: DistortionKind) -> Self {
		Self { kind, ..self }
	}

	/// Sets the factor to multiply the signal by before applying
	/// the distortion.
	pub fn drive(self, drive: f64) -> Self {
		Self { drive, ..self }
	}

	/// Sets how much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means only the dry
	/// signal will be heard. `1.0` means only the wet signal will
	/// be heard.
	pub fn mix(self, mix: f64) -> Self {
		Self { mix, ..self }
	}
}

impl Default for DistortionBuilder {
	fn default() -> Self {
		Self {
			kind: Default::default(),
			drive: 1.0,
			mix: 1.0,
		}
	}
}

impl EffectBuilder for DistortionBuilder {
	type Handle = DistortionHandle;

	fn into_effect(self) -> (Box<dyn Effect>, Self::Handle) {
		let (effect, handle) = Distortion::new(self);
		(Box::new(effect), handle)
	}
}

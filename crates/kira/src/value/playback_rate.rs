use crate::parameter::{ParameterHandle, ParameterId};

use super::{AsValue, Mapping, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackRate {
	Factor(f64),
	Semitones(f64),
}

impl PlaybackRate {
	pub fn as_factor(&self) -> f64 {
		match self {
			PlaybackRate::Factor(factor) => *factor,
			PlaybackRate::Semitones(semitones) => semitones_to_factor(*semitones),
		}
	}

	pub fn as_semitones(&self) -> f64 {
		match self {
			PlaybackRate::Factor(factor) => factor_to_semitones(*factor),
			PlaybackRate::Semitones(semitones) => *semitones,
		}
	}
}

impl AsValue for PlaybackRate {
	fn map(input: f64, mapping: Mapping<Self>) -> Self {
		let relative_input =
			(input - mapping.input_range.0) / (mapping.input_range.1 - mapping.input_range.0);
		match mapping.output_range.0 {
			PlaybackRate::Factor(start_factor) => {
				let end_factor = mapping.output_range.1.as_factor();
				let mut output_factor = start_factor + (end_factor - start_factor) * relative_input;
				if mapping.clamp_bottom {
					output_factor = output_factor.max(start_factor);
				}
				if mapping.clamp_top {
					output_factor = output_factor.min(end_factor);
				}
				Self::Factor(output_factor)
			}
			PlaybackRate::Semitones(start_semitones) => {
				let end_semitones = mapping.output_range.1.as_semitones();
				let mut output_semitones =
					start_semitones + (end_semitones - start_semitones) * relative_input;
				if mapping.clamp_bottom {
					output_semitones = output_semitones.max(start_semitones);
				}
				if mapping.clamp_top {
					output_semitones = output_semitones.min(end_semitones);
				}
				Self::Semitones(output_semitones)
			}
		}
	}
}

impl From<PlaybackRate> for Value<PlaybackRate> {
	fn from(playback_rate: PlaybackRate) -> Self {
		Value::Fixed(playback_rate)
	}
}

impl From<f64> for Value<PlaybackRate> {
	fn from(factor: f64) -> Self {
		Value::Fixed(PlaybackRate::Factor(factor))
	}
}

impl From<ParameterId> for Value<PlaybackRate> {
	fn from(id: ParameterId) -> Self {
		Value::Parameter {
			id,
			mapping: Mapping {
				input_range: (0.0, 1.0),
				output_range: (PlaybackRate::Factor(0.0), PlaybackRate::Factor(1.0)),
				clamp_bottom: false,
				clamp_top: false,
			},
		}
	}
}

impl From<&ParameterHandle> for Value<PlaybackRate> {
	fn from(handle: &ParameterHandle) -> Self {
		handle.id().into()
	}
}

fn factor_to_semitones(factor: f64) -> f64 {
	12.0 * factor.log2()
}

fn semitones_to_factor(semitones: f64) -> f64 {
	2.0f64.powf(semitones / 12.0)
}

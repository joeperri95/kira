//! Adds reverberations to a sound.

use crate::{
	dsp::Frame,
	parameter::Parameters,
	track::Effect,
	value::{CachedValue, Value},
};
use all_pass::AllPassFilter;
use comb::CombFilter;

mod all_pass;
mod comb;

const NUM_COMB_FILTERS: usize = 8;
const NUM_ALL_PASS_FILTERS: usize = 4;
const GAIN: f32 = 0.015;
const STEREO_SPREAD: usize = 23;

/// Settings for a `Reverb`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct ReverbSettings {
	/// How much the room reverberates. A higher value will
	/// result in a bigger sounding room. 1.0 gives an infinitely
	/// reverberating room.
	pub feedback: Value,
	/// How quickly high frequencies disappear from the reverberation.
	pub damping: Value,
	/// The stereo width of the reverb effect (0.0 being fully mono,
	/// 1.0 being fully stereo).
	pub stereo_width: Value,
	/// How much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means
	/// only the dry signal will be heard. `1.0` means
	/// only the wet signal will be heard.
	pub mix: Value,
}

impl ReverbSettings {
	/// Creates a new `ReverbSettings` with the default settings.
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets how much the room reverberates. A higher value will
	/// result in a bigger sounding room. 1.0 gives an infinitely
	/// reverberating room.
	pub fn feedback(self, feedback: impl Into<Value>) -> Self {
		Self {
			feedback: feedback.into(),
			..self
		}
	}

	/// Sets how quickly high frequencies disappear from the reverberation.
	pub fn damping(self, damping: impl Into<Value>) -> Self {
		Self {
			damping: damping.into(),
			..self
		}
	}

	/// Sets the stereo width of the reverb effect (0.0 being fully mono,
	/// 1.0 being fully stereo).
	pub fn stereo_width(self, stereo_width: impl Into<Value>) -> Self {
		Self {
			stereo_width: stereo_width.into(),
			..self
		}
	}

	/// Sets how much dry (unprocessed) signal should be blended
	/// with the wet (processed) signal. `0.0` means only the dry
	/// signal will be heard. `1.0` means only the wet signal will
	/// be heard.
	pub fn mix(self, mix: impl Into<Value>) -> Self {
		Self {
			mix: mix.into(),
			..self
		}
	}
}

impl Default for ReverbSettings {
	fn default() -> Self {
		Self {
			feedback: Value::Fixed(0.9),
			damping: Value::Fixed(0.1),
			stereo_width: Value::Fixed(1.0),
			mix: Value::Fixed(0.5),
		}
	}
}

#[derive(Debug)]
enum ReverbState {
	Uninitialized,
	Initialized {
		comb_filters: [(CombFilter, CombFilter); NUM_COMB_FILTERS],
		all_pass_filters: [(AllPassFilter, AllPassFilter); NUM_ALL_PASS_FILTERS],
	},
}

/// A reverb effect. Useul for simulating room tones.
// This code is based on Freeverb by Jezar at Dreampoint, found here:
// http://blog.bjornroche.com/2012/06/freeverb-original-public-domain-code-by.html
pub struct Reverb {
	feedback: CachedValue,
	damping: CachedValue,
	stereo_width: CachedValue,
	mix: CachedValue,
	state: ReverbState,
}

impl Reverb {
	/// Creates a new `Reverb` effect.
	pub fn new(settings: ReverbSettings) -> Self {
		Self {
			feedback: CachedValue::new(-1.0..=1.0, settings.feedback, 0.9),
			damping: CachedValue::new(0.0..=1.0, settings.damping, 0.1),
			stereo_width: CachedValue::new(0.0..=1.0, settings.stereo_width, 1.0),
			mix: CachedValue::new(0.0..=1.0, settings.mix, 0.5),
			state: ReverbState::Uninitialized,
		}
	}
}

impl Effect for Reverb {
	fn init(&mut self, sample_rate: u32) {
		if let ReverbState::Uninitialized = &self.state {
			const REFERENCE_SAMPLE_RATE: u32 = 44100;

			let adjust_buffer_size = |buffer_size: usize| -> usize {
				let sample_rate_factor = (sample_rate as f64) / (REFERENCE_SAMPLE_RATE as f64);
				((buffer_size as f64) * sample_rate_factor) as usize
			};

			self.state = ReverbState::Initialized {
				comb_filters: [
					(
						CombFilter::new(adjust_buffer_size(1116)),
						CombFilter::new(adjust_buffer_size(1116 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1188)),
						CombFilter::new(adjust_buffer_size(1188 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1277)),
						CombFilter::new(adjust_buffer_size(1277 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1356)),
						CombFilter::new(adjust_buffer_size(1356 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1422)),
						CombFilter::new(adjust_buffer_size(1422 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1491)),
						CombFilter::new(adjust_buffer_size(1491 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1557)),
						CombFilter::new(adjust_buffer_size(1557 + STEREO_SPREAD)),
					),
					(
						CombFilter::new(adjust_buffer_size(1617)),
						CombFilter::new(adjust_buffer_size(1617 + STEREO_SPREAD)),
					),
				],
				all_pass_filters: [
					(
						AllPassFilter::new(adjust_buffer_size(556)),
						AllPassFilter::new(adjust_buffer_size(556 + STEREO_SPREAD)),
					),
					(
						AllPassFilter::new(adjust_buffer_size(441)),
						AllPassFilter::new(adjust_buffer_size(441 + STEREO_SPREAD)),
					),
					(
						AllPassFilter::new(adjust_buffer_size(341)),
						AllPassFilter::new(adjust_buffer_size(341 + STEREO_SPREAD)),
					),
					(
						AllPassFilter::new(adjust_buffer_size(225)),
						AllPassFilter::new(adjust_buffer_size(225 + STEREO_SPREAD)),
					),
				],
			}
		} else {
			panic!("Reverb should be in the uninitialized state before init");
		}
	}

	fn process(&mut self, input: Frame, _dt: f64, parameters: &Parameters) -> Frame {
		if let ReverbState::Initialized {
			comb_filters,
			all_pass_filters,
		} = &mut self.state
		{
			self.feedback.update(parameters);
			self.damping.update(parameters);
			self.stereo_width.update(parameters);

			let feedback = self.feedback.get() as f32;
			let damping = self.damping.get() as f32;
			let stereo_width = self.stereo_width.get() as f32;

			let mut output = Frame::ZERO;
			let mono_input = (input.left + input.right) * GAIN;
			// accumulate comb filters in parallel
			for comb_filter in comb_filters {
				output.left += comb_filter.0.process(mono_input, feedback, damping);
				output.right += comb_filter.1.process(mono_input, feedback, damping);
			}
			// feed through all-pass filters in series
			for all_pass_filter in all_pass_filters {
				output.left = all_pass_filter.0.process(output.left);
				output.right = all_pass_filter.1.process(output.right);
			}
			let wet_1 = stereo_width / 2.0 + 0.5;
			let wet_2 = (1.0 - stereo_width) / 2.0;
			let output = Frame::new(
				output.left * wet_1 + output.right * wet_2,
				output.right * wet_1 + output.left * wet_2,
			);
			let mix = self.mix.get() as f32;
			output * mix.sqrt() + input * (1.0 - mix).sqrt()
		} else {
			panic!("Reverb should be initialized before the first process call")
		}
	}
}

use crate::{clock::Clocks, dsp::Frame, parameter::Parameter, track::Effect};
use all_pass::AllPassFilter;
use comb::CombFilter;

use super::{ReverbBuilder, ReverbHandle};

mod all_pass;
mod comb;

const NUM_COMB_FILTERS: usize = 8;
const NUM_ALL_PASS_FILTERS: usize = 4;
const GAIN: f32 = 0.015;
const STEREO_SPREAD: usize = 23;

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
	feedback: Parameter,
	damping: Parameter,
	stereo_width: Parameter,
	mix: Parameter,
	state: ReverbState,
}

impl Reverb {
	/// Creates a new `Reverb` effect.
	pub fn new(builder: ReverbBuilder) -> (Self, ReverbHandle) {
		let (feedback, feedback_handle) = Parameter::new(builder.feedback);
		let (damping, damping_handle) = Parameter::new(builder.damping);
		let (stereo_width, stereo_width_handle) = Parameter::new(builder.stereo_width);
		let (mix, mix_handle) = Parameter::new(builder.mix);
		(
			Self {
				feedback,
				damping,
				stereo_width,
				mix,
				state: ReverbState::Uninitialized,
			},
			ReverbHandle {
				feedback: feedback_handle,
				damping: damping_handle,
				stereo_width: stereo_width_handle,
				mix: mix_handle,
			},
		)
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

	fn on_start_processing(&mut self) {
		self.feedback.on_start_processing();
		self.damping.on_start_processing();
		self.stereo_width.on_start_processing();
	}

	fn process(&mut self, input: Frame, dt: f64, clocks: &mut Clocks) -> Frame {
		if let ReverbState::Initialized {
			comb_filters,
			all_pass_filters,
		} = &mut self.state
		{
			self.feedback.update(dt, clocks);
			self.damping.update(dt, clocks);
			self.stereo_width.update(dt, clocks);

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

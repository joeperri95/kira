use std::f64::consts::PI;

use crate::{clock::Clocks, dsp::Frame, parameter::Parameter, track::effect::Effect};

use super::{FilterBuilder, FilterHandle, FilterMode};

/// An effect that removes frequencies from input audio.
pub(super) struct Filter {
	mode: FilterMode,
	cutoff: Parameter,
	resonance: Parameter,
	mix: Parameter,
	ic1eq: Frame,
	ic2eq: Frame,
}

impl Filter {
	/// Creates a new filter.
	pub fn new(builder: FilterBuilder) -> (Self, FilterHandle) {
		let (cutoff, cutoff_handle) = Parameter::new(builder.cutoff);
		let (resonance, resonance_handle) = Parameter::new(builder.resonance);
		let (mix, mix_handle) = Parameter::new(builder.mix);
		(
			Self {
				mode: builder.mode,
				cutoff,
				resonance,
				mix,
				ic1eq: Frame::ZERO,
				ic2eq: Frame::ZERO,
			},
			FilterHandle {
				cutoff: cutoff_handle,
				resonance: resonance_handle,
				mix: mix_handle,
			},
		)
	}
}

impl Effect for Filter {
	fn on_start_processing(&mut self) {
		self.cutoff.on_start_processing();
		self.resonance.on_start_processing();
		self.mix.on_start_processing();
	}

	fn process(&mut self, input: Frame, dt: f64, clocks: &mut Clocks) -> Frame {
		self.cutoff.update(dt, clocks);
		self.resonance.update(dt, clocks);
		self.mix.update(dt, clocks);

		let sample_rate = 1.0 / dt;
		let g = (PI * (self.cutoff.get() / sample_rate)).tan();
		let k = 2.0 - (1.9 * self.resonance.get().min(1.0).max(0.0));
		let a1 = 1.0 / (1.0 + (g * (g + k)));
		let a2 = g * a1;
		let a3 = g * a2;
		let v3 = input - self.ic2eq;
		let v1 = (self.ic1eq * (a1 as f32)) + (v3 * (a2 as f32));
		let v2 = self.ic2eq + (self.ic1eq * (a2 as f32)) + (v3 * (a3 as f32));
		self.ic1eq = (v1 * 2.0) - self.ic1eq;
		self.ic2eq = (v2 * 2.0) - self.ic2eq;
		let output = match self.mode {
			FilterMode::LowPass => v2,
			FilterMode::BandPass => v1,
			FilterMode::HighPass => input - v1 * (k as f32) - v2,
			FilterMode::Notch => input - v1 * (k as f32),
		};
		let mix = self.mix.get() as f32;
		output * mix.sqrt() + input * (1.0 - mix).sqrt()
	}
}

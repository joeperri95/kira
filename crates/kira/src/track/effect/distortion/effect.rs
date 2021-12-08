use crate::{clock::Clocks, dsp::Frame, parameter::Parameter, track::effect::Effect};

use super::{DistortionBuilder, DistortionHandle, DistortionKind};

/// An effect that modifies an input signal to make it more
/// distorted and noisy.
pub(super) struct Distortion {
	kind: DistortionKind,
	drive: Parameter,
	mix: Parameter,
}

impl Distortion {
	/// Creates a new distortion effect.
	pub fn new(builder: DistortionBuilder) -> (Self, DistortionHandle) {
		let (drive, drive_handle) = Parameter::new(builder.drive);
		let (mix, mix_handle) = Parameter::new(builder.mix);
		(
			Self {
				kind: builder.kind,
				drive,
				mix,
			},
			DistortionHandle {
				drive: drive_handle,
				mix: mix_handle,
			},
		)
	}
}

impl Effect for Distortion {
	fn on_start_processing(&mut self) {
		self.drive.on_start_processing();
		self.mix.on_start_processing();
	}

	fn process(&mut self, input: Frame, dt: f64, clocks: &mut Clocks) -> Frame {
		self.drive.update(dt, clocks);
		self.mix.update(dt, clocks);

		let drive = self.drive.get() as f32;
		let mut output = input * drive;
		output = match self.kind {
			DistortionKind::HardClip => Frame::new(
				output.left.max(-1.0).min(1.0),
				output.right.max(-1.0).min(1.0),
			),
			DistortionKind::SoftClip => Frame::new(
				output.left / (1.0 + output.left.abs()),
				output.right / (1.0 + output.right.abs()),
			),
		};
		output /= drive;

		let mix = self.mix.get() as f32;
		output * mix.sqrt() + input * (1.0 - mix).sqrt()
	}
}

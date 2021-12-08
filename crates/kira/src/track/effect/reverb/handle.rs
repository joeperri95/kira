use crate::{parameter::ParameterHandle, tween::Tween, CommandQueueFull};

pub struct ReverbHandle {
	pub(crate) feedback: ParameterHandle,
	pub(crate) damping: ParameterHandle,
	pub(crate) stereo_width: ParameterHandle,
	pub(crate) mix: ParameterHandle,
}

impl ReverbHandle {
	pub fn feedback(&self) -> f64 {
		self.feedback.get()
	}

	pub fn damping(&self) -> f64 {
		self.damping.get()
	}

	pub fn stereo_width(&self) -> f64 {
		self.stereo_width.get()
	}

	pub fn mix(&self) -> f64 {
		self.mix.get()
	}

	pub fn set_feedback(&mut self, feedback: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.feedback.set(feedback, tween)
	}

	pub fn set_damping(&mut self, damping: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.damping.set(damping, tween)
	}

	pub fn set_stereo_width(
		&mut self,
		stereo_width: f64,
		tween: Tween,
	) -> Result<(), CommandQueueFull> {
		self.stereo_width.set(stereo_width, tween)
	}

	pub fn set_mix(&mut self, mix: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.mix.set(mix, tween)
	}
}

use crate::{parameter::ParameterHandle, tween::Tween, CommandQueueFull};

pub struct DelayHandle {
	pub(crate) delay_time: ParameterHandle,
	pub(crate) feedback: ParameterHandle,
	pub(crate) mix: ParameterHandle,
}

impl DelayHandle {
	pub fn delay_time(&self) -> f64 {
		self.delay_time.get()
	}

	pub fn feedback(&self) -> f64 {
		self.feedback.get()
	}

	pub fn mix(&self) -> f64 {
		self.mix.get()
	}

	pub fn set_delay_time(
		&mut self,
		delay_time: f64,
		tween: Tween,
	) -> Result<(), CommandQueueFull> {
		self.delay_time.set(delay_time, tween)
	}

	pub fn set_feedback(&mut self, feedback: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.feedback.set(feedback, tween)
	}

	pub fn set_mix(&mut self, mix: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.mix.set(mix, tween)
	}
}

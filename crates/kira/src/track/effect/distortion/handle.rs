use crate::{parameter::ParameterHandle, tween::Tween, CommandQueueFull};

pub struct DistortionHandle {
	pub(crate) drive: ParameterHandle,
	pub(crate) mix: ParameterHandle,
}

impl DistortionHandle {
	pub fn drive(&self) -> f64 {
		self.drive.get()
	}

	pub fn mix(&self) -> f64 {
		self.mix.get()
	}

	pub fn set_drive(&mut self, drive: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.drive.set(drive, tween)
	}

	pub fn set_mix(&mut self, mix: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.mix.set(mix, tween)
	}
}

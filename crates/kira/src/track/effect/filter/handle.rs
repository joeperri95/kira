use crate::{parameter::ParameterHandle, tween::Tween, CommandQueueFull};

pub struct FilterHandle {
	pub(super) cutoff: ParameterHandle,
	pub(super) resonance: ParameterHandle,
	pub(super) mix: ParameterHandle,
}

impl FilterHandle {
	pub fn cutoff(&self) -> f64 {
		self.cutoff.get()
	}

	pub fn resonance(&self) -> f64 {
		self.resonance.get()
	}

	pub fn mix(&self) -> f64 {
		self.mix.get()
	}

	pub fn set_cutoff(&mut self, cutoff: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.cutoff.set(cutoff, tween)
	}

	pub fn set_resonance(&mut self, resonance: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.resonance.set(resonance, tween)
	}

	pub fn set_mix(&mut self, mix: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.mix.set(mix, tween)
	}
}

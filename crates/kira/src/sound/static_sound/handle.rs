use std::sync::Arc;

use ringbuf::Producer;

use crate::{parameter::ParameterHandle, tween::Tween, CommandQueueFull};

use super::{sound::Shared, Command, PlaybackState};

/// Controls a static sound.
pub struct StaticSoundHandle {
	pub(super) command_producer: Producer<Command>,
	pub(super) shared: Arc<Shared>,
	pub(super) volume: ParameterHandle,
	pub(super) playback_rate: ParameterHandle,
	pub(super) panning: ParameterHandle,
}

impl StaticSoundHandle {
	/// Returns the current playback state of the sound.
	pub fn state(&self) -> PlaybackState {
		self.shared.state()
	}

	/// Returns the current playback position of the sound (in seconds).
	pub fn position(&self) -> f64 {
		self.shared.position()
	}

	pub fn volume(&self) -> f64 {
		self.volume.get()
	}

	pub fn playback_rate(&self) -> f64 {
		self.playback_rate.get()
	}

	pub fn panning(&self) -> f64 {
		self.panning.get()
	}

	pub fn set_volume(&mut self, volume: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.volume.set(volume, tween)
	}

	pub fn set_playback_rate(
		&mut self,
		playback_rate: f64,
		tween: Tween,
	) -> Result<(), CommandQueueFull> {
		self.playback_rate.set(playback_rate, tween)
	}

	pub fn set_panning(&mut self, panning: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.panning.set(panning, tween)
	}

	/// Fades out the sound to silence with the given tween and then
	/// pauses playback.
	pub fn pause(&mut self, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Pause(tween))
			.map_err(|_| CommandQueueFull)
	}

	/// Resumes playback and fades in the sound from silence
	/// with the given tween.
	pub fn resume(&mut self, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Resume(tween))
			.map_err(|_| CommandQueueFull)
	}

	/// Fades out the sound to silence with the given tween and then
	/// stops playback.
	///
	/// Once the sound is stopped, it cannot be restarted.
	pub fn stop(&mut self, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Stop(tween))
			.map_err(|_| CommandQueueFull)
	}

	/// Sets the playback position to the specified time in seconds.
	pub fn seek_to(&mut self, position: f64) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SeekTo(position))
			.map_err(|_| CommandQueueFull)
	}

	/// Moves the playback position by the specified amount of time in seconds.
	pub fn seek_by(&mut self, amount: f64) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::SeekBy(amount))
			.map_err(|_| CommandQueueFull)
	}
}

use crate::{track::TrackId, tween::Tween, value::Value, LoopBehavior, StartTime};

/// Settings for a static sound.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct StaticSoundSettings {
	/// When the sound should start playing.
	pub start_time: StartTime,
	/// The initial playback position of the sound (in seconds).
	pub start_position: f64,
	/// The volume of the sound.
	pub volume: Value<f64>,
	/// The playback rate of the sound, as a factor of the
	/// normal playback rate.
	///
	/// Changing the playback rate will change both the speed
	/// and the pitch of the sound.
	pub playback_rate: Value<f64>,
	/// The panning of the sound, where 0 is hard left
	/// and 1 is hard right.
	pub panning: Value<f64>,
	/// Whether the sound should play in reverse.
	///
	/// If set to `true`, the start position will be relative
	/// to the end of the sound.
	pub reverse: bool,
	/// The looping behavior of the sound.
	pub loop_behavior: Option<LoopBehavior>,
	/// The mixer track this sound should play on.
	pub track: TrackId,
	/// An optional fade-in from silence.
	pub fade_in_tween: Option<Tween>,
}

impl StaticSoundSettings {
	/// Creates a new [`StaticSoundSettings`] with the default settings.
	pub fn new() -> Self {
		Self {
			start_time: StartTime::default(),
			start_position: 0.0,
			volume: Value::Fixed(1.0),
			playback_rate: Value::Fixed(1.0),
			panning: Value::Fixed(0.5),
			reverse: false,
			loop_behavior: None,
			track: TrackId::Main,
			fade_in_tween: None,
		}
	}

	/// Sets when the sound should start playing.
	pub fn start_time(self, start_time: impl Into<StartTime>) -> Self {
		Self {
			start_time: start_time.into(),
			..self
		}
	}

	/// Sets the initial playback position of the sound (in seconds).
	pub fn start_position(self, start_position: f64) -> Self {
		Self {
			start_position,
			..self
		}
	}

	/// Sets the volume of the sound.
	pub fn volume(self, volume: impl Into<Value<f64>>) -> Self {
		Self {
			volume: volume.into(),
			..self
		}
	}

	/// Sets the playback rate of the sound, as a factor of the
	/// normal playback rate.
	///
	/// Changing the playback rate will change both the speed
	/// and the pitch of the sound.
	pub fn playback_rate(self, playback_rate: impl Into<Value<f64>>) -> Self {
		Self {
			playback_rate: playback_rate.into(),
			..self
		}
	}

	/// Sets the panning of the sound, where 0 is hard left
	/// and 1 is hard right.
	pub fn panning(self, panning: impl Into<Value<f64>>) -> Self {
		Self {
			panning: panning.into(),
			..self
		}
	}

	/// Sets whether the sound should play in reverse.
	pub fn reverse(self, reverse: bool) -> Self {
		Self { reverse, ..self }
	}

	/// Sets the looping behavior of the sound.
	pub fn loop_behavior(self, loop_behavior: impl Into<Option<LoopBehavior>>) -> Self {
		Self {
			loop_behavior: loop_behavior.into(),
			..self
		}
	}

	/// Sets the mixer track this sound should play on.
	pub fn track(self, track: impl Into<TrackId>) -> Self {
		Self {
			track: track.into(),
			..self
		}
	}

	/// Sets the tween used to fade in the sound from silence.
	pub fn fade_in_tween(self, fade_in_tween: impl Into<Option<Tween>>) -> Self {
		Self {
			fade_in_tween: fade_in_tween.into(),
			..self
		}
	}
}

impl Default for StaticSoundSettings {
	fn default() -> Self {
		Self::new()
	}
}

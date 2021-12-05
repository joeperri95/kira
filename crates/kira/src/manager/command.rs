pub mod producer;

use atomic_arena::Key;

use crate::{
	clock::{Clock, ClockId},
	parameter::{Parameter, ParameterId},
	sound::Sound,
	track::{SubTrackId, Track, TrackId},
	tween::Tween,
	value::Value,
};

pub(crate) enum SoundCommand {
	Add(Key, Box<dyn Sound>),
}

pub(crate) enum ParameterCommand {
	Add(ParameterId, Parameter),
	Set {
		id: ParameterId,
		target: f64,
		tween: Tween,
	},
	Pause(ParameterId),
	Resume(ParameterId),
}

pub(crate) enum MixerCommand {
	AddSubTrack(SubTrackId, Track),
	SetTrackVolume(TrackId, Value<f64>),
	SetTrackPanning(TrackId, Value<f64>),
}

pub(crate) enum ClockCommand {
	Add(ClockId, Clock),
	SetInterval(ClockId, Value<f64>),
	Start(ClockId),
	Pause(ClockId),
	Stop(ClockId),
}

pub(crate) enum Command {
	Sound(SoundCommand),
	Parameter(ParameterCommand),
	Mixer(MixerCommand),
	Clock(ClockCommand),
	Pause(Tween),
	Resume(Tween),
}

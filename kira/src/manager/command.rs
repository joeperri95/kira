pub mod producer;

use crate::{parameter::{tween::Tween, Parameter, ParameterId}, sound::{
		instance::{Instance, InstanceId},
		Sound, SoundId,
	}, value::Value};

pub(crate) enum SoundCommand {
	Add(SoundId, Sound),
}

pub(crate) enum InstanceCommand {
	Add(InstanceId, Instance),
	SetVolume(InstanceId, Value),
	SetPlaybackRate(InstanceId, Value),
	SetPanning(InstanceId, Value),
}

pub(crate) enum ParameterCommand {
	Add(ParameterId, Parameter),
	Set(ParameterId, f64),
	Tween {
		id: ParameterId,
		target: f64,
		tween: Tween,
		command_sent_time: u64,
	},
}

pub(crate) enum Command {
	Sound(SoundCommand),
	Instance(InstanceCommand),
	Parameter(ParameterCommand),
}

mod data;
mod handle;
mod settings;
mod sound;

pub use data::*;
pub use handle::*;
pub use settings::*;

use kira::{tween::Tween, value::Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Command {
	SetVolume(Value<f64>),
	SetPlaybackRate(Value<f64>),
	SetPanning(Value<f64>),
	Pause(Tween),
	Resume(Tween),
	Stop(Tween),
	SeekBy(f64),
	SeekTo(f64),
}

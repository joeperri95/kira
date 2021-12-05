//! The [`Value`] enum, which is used as the type for many settings
//! throughout Kira.

mod cached;
mod mapping;

pub use cached::*;
pub use mapping::*;

use crate::parameter::{ParameterHandle, ParameterId};

pub trait AsValue: Sized + Copy {
	fn map(input: f64, mapping: Mapping<Self>) -> Self;
}

impl AsValue for f64 {
	fn map(input: f64, mapping: Mapping<Self>) -> Self {
		let relative_input =
			(input - mapping.input_range.0) / (mapping.input_range.1 - mapping.input_range.0);
		let mut output = mapping.output_range.0
			+ (mapping.output_range.1 - mapping.output_range.0) * relative_input;
		if mapping.clamp_bottom {
			output = output.max(mapping.output_range.0);
		}
		if mapping.clamp_top {
			output = output.min(mapping.output_range.1);
		}
		output
	}
}

impl From<f64> for Value<f64> {
	fn from(value: f64) -> Self {
		Value::Fixed(value)
	}
}

impl From<ParameterId> for Value<f64> {
	fn from(id: ParameterId) -> Self {
		Value::Parameter {
			id,
			mapping: Mapping {
				input_range: (0.0, 1.0),
				output_range: (0.0, 1.0),
				clamp_bottom: false,
				clamp_top: false,
			},
		}
	}
}

impl From<&ParameterHandle> for Value<f64> {
	fn from(handle: &ParameterHandle) -> Self {
		handle.id().into()
	}
}

/// The possible values for a setting.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Value<T: AsValue> {
	/// The setting is fixed to the specified value.
	Fixed(T),
	/// The setting is linked to a parameter with the
	/// given mapping.
	Parameter {
		/// The parameter the setting is linked to.
		id: ParameterId,
		/// The mapping of parameter values to setting values.
		mapping: Mapping<T>,
	},
}

//! Saves the last valid raw value of a [`Value`]. Useful for writing
//! [`Effect`](crate::track::effect::Effect)s and
//! [`Sound`](crate::sound::Sound)s.

use crate::parameter::Parameters;

use super::{AsValue, Value};

/// Holds a [`Value`] and remembers the last valid raw value.
pub struct CachedValue<T: AsValue> {
	value: Value<T>,
	raw_value: T,
}

impl<T: AsValue> CachedValue<T> {
	/// Creates a new [`CachedValue`].
	pub fn new(value: Value<T>, default: T) -> Self {
		Self {
			value,
			raw_value: match value {
				Value::Fixed(value) => value,
				Value::Parameter { .. } => default,
			},
		}
	}

	/// Gets the last valid raw value.
	pub fn get(&self) -> T {
		self.raw_value
	}

	/// Sets the value.
	pub fn set(&mut self, value: Value<T>) {
		self.value = value;
	}

	/// Updates the [`CachedValue`] with the current values of parameters.
	pub fn update(&mut self, parameters: &Parameters) {
		if let Value::Parameter { id, mapping } = self.value {
			if let Some(parameter) = parameters.get(id) {
				self.raw_value = T::map(parameter.value(), mapping);
			}
		}
	}
}

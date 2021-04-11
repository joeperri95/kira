use crate::parameter::Parameter;

pub(crate) mod sealed {
	pub trait AsValue: Copy + From<f64> + Into<f64> {}

	impl<T: Copy + From<f64> + Into<f64>> AsValue for T {}
}

use sealed::AsValue;

#[derive(Clone)]
pub enum Value<T: AsValue> {
	Fixed(T),
	Parameter(Parameter<T>),
}

impl<T: AsValue> Value<T> {
	pub(crate) fn get(&self) -> T {
		match self {
			Value::Fixed(value) => *value,
			Value::Parameter(parameter) => parameter.get(),
		}
	}
}

impl<T: AsValue> From<T> for Value<T> {
	fn from(value: T) -> Self {
		Self::Fixed(value)
	}
}

impl<T: AsValue> From<Parameter<T>> for Value<T> {
	fn from(parameter: Parameter<T>) -> Self {
		Self::Parameter(parameter)
	}
}

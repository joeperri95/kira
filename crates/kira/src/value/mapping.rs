/// A transformation from one range of values to another.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mapping<T> {
	/// The input range of the mapping.
	pub input_range: (f64, f64),
	/// The corresponding output range of the mapping.
	pub output_range: (T, T),
	/// Whether values should be prevented from being
	/// less than the bottom of the output range.
	pub clamp_bottom: bool,
	/// Whether values should be prevented from being
	/// greater than the top of the output range.
	pub clamp_top: bool,
}

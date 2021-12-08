//! Makes a sound harsher and noisier.

mod builder;
mod effect;
mod handle;

pub use builder::*;
pub use handle::*;

/// Different types of distortion effect.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DistortionKind {
	/// The signal will be clamped to the -1.0 to 1.0 range.
	///
	/// This creates a harsh distortion when the signal leaves
	/// the -1.0 to 1.0 range.
	HardClip,
	/// The signal will be kept in the -1.0 to 1.0 range,
	/// and the slope will gradually decrease as it reaches
	/// -1.0 or 1.0.
	///
	/// This creates a smoother distortion that gradually
	/// becomes more prominent as the signal becomes louder.
	SoftClip,
}

impl Default for DistortionKind {
	fn default() -> Self {
		Self::HardClip
	}
}

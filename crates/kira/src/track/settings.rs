use super::{effect::EffectBuilder, routes::TrackRoutes, Effect};

/// Settings for a mixer track.
#[non_exhaustive]
pub struct TrackBuilder {
	/// The volume of the track.
	pub(crate) volume: f64,
	/// The panning of the track, where 0 is hard left
	/// and 1 is hard right.
	pub(crate) panning: f64,
	/// How the output of this track should be routed
	/// to other mixer tracks.
	pub(crate) routes: TrackRoutes,
	/// The effects that should be applied to the input audio
	/// for this track.
	pub(crate) effects: Vec<Box<dyn Effect>>,
}

impl TrackBuilder {
	/// Creates a new [`TrackSettings`] with the default settings.
	pub fn new() -> Self {
		Self {
			volume: 1.0,
			panning: 0.5,
			routes: TrackRoutes::new(),
			effects: vec![],
		}
	}

	/// Sets the volume of the track.
	pub fn set_volume(&mut self, volume: f64) {
		self.volume = volume;
	}

	/// Sets the panning of the track, where 0 is hard left
	/// and 1 is hard right.
	pub fn set_panning(&mut self, panning: f64) {
		self.panning = panning;
	}

	/// Sets how the output of this track should be routed
	/// to other mixer tracks.
	pub fn set_routes(&mut self, routes: TrackRoutes) {
		self.routes = routes;
	}

	/// Adds an effect to the track.
	pub fn add_effect<B: EffectBuilder>(&mut self, builder: B) -> B::Handle {
		let (effect, handle) = builder.into_effect();
		self.effects.push(effect);
		handle
	}
}

impl Default for TrackBuilder {
	fn default() -> Self {
		Self::new()
	}
}

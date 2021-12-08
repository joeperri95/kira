use std::sync::Arc;

use atomic_arena::{Arena, Controller};
use ringbuf::Producer;

use crate::{
	clock::Clocks,
	dsp::Frame,
	manager::{backend::context::Context, command::MixerCommand},
	track::{effect::Effect, SubTrackId, Track, TrackBuilder, TrackId},
};

pub(crate) struct Mixer {
	main_track: Track,
	sub_tracks: Arena<Track>,
	sub_track_ids: Vec<SubTrackId>,
	dummy_routes: Vec<(TrackId, f64)>,
	unused_track_producer: Producer<Track>,
}

impl Mixer {
	pub fn new(
		sub_track_capacity: usize,
		unused_sub_track_producer: Producer<Track>,
		context: &Arc<Context>,
		main_track_effects: Vec<Box<dyn Effect>>,
	) -> Self {
		Self {
			main_track: Track::new(
				{
					let mut settings = TrackBuilder::new();
					settings.effects = main_track_effects;
					settings
				},
				context,
			),
			sub_tracks: Arena::new(sub_track_capacity),
			sub_track_ids: Vec::with_capacity(sub_track_capacity),
			dummy_routes: vec![],
			unused_track_producer: unused_sub_track_producer,
		}
	}

	pub fn sub_track_controller(&self) -> Controller {
		self.sub_tracks.controller()
	}

	pub fn track_mut(&mut self, id: TrackId) -> Option<&mut Track> {
		match id {
			TrackId::Main => Some(&mut self.main_track),
			TrackId::Sub(id) => self.sub_tracks.get_mut(id.0),
		}
	}

	pub fn run_command(&mut self, command: MixerCommand) {
		match command {
			MixerCommand::AddSubTrack(id, track) => {
				self.sub_tracks
					.insert_with_key(id.0, track)
					.expect("Sub-track arena is full");
				self.sub_track_ids.push(id);
			}
			MixerCommand::SetTrackVolume(id, volume) => {
				if let Some(track) = self.track_mut(id) {
					track.set_volume(volume);
				}
			}
			MixerCommand::SetTrackPanning(id, panning) => {
				if let Some(track) = self.track_mut(id) {
					track.set_panning(panning);
				}
			}
		}
	}

	pub fn on_start_processing(&mut self) {
		self.remove_unused_tracks();
		for (_, track) in &mut self.sub_tracks {
			track.on_start_processing();
		}
		self.main_track.on_start_processing();
	}

	fn remove_unused_tracks(&mut self) {
		let mut i = 0;
		while i < self.sub_track_ids.len() && !self.unused_track_producer.is_full() {
			let id = self.sub_track_ids[i];
			let track = &mut self.sub_tracks[id.0];
			if track.shared().is_marked_for_removal() {
				if self
					.unused_track_producer
					.push(
						self.sub_tracks
							.remove(id.0)
							.unwrap_or_else(|| panic!("Sub track with ID {:?} does not exist", id)),
					)
					.is_err()
				{
					panic!("Unused track producer is full")
				}
				self.sub_track_ids.remove(i);
			} else {
				i += 1;
			}
		}
	}

	pub fn process(&mut self, dt: f64, clocks: &mut Clocks) -> Frame {
		// iterate through the sub-tracks newest to oldest
		for id in self.sub_track_ids.iter().rev() {
			// process the track and get its output
			let track = self
				.sub_tracks
				.get_mut(id.0)
				.expect("sub track IDs and sub tracks are out of sync");
			let output = track.process(dt, clocks);
			// temporarily take ownership of its routes. we can't just
			// borrow the routes because then we can't get mutable
			// references to the other tracks
			std::mem::swap(track.routes_mut(), &mut self.dummy_routes);
			// send the output to the destination tracks
			for (id, amount) in &self.dummy_routes {
				let destination_track = match id {
					TrackId::Main => Some(&mut self.main_track),
					TrackId::Sub(id) => self.sub_tracks.get_mut(id.0),
				};
				if let Some(destination_track) = destination_track {
					destination_track.add_input(output * *amount as f32);
				}
			}
			// borrow the track again and give it back its routes
			let track = self
				.sub_tracks
				.get_mut(id.0)
				.expect("sub track IDs and sub tracks are out of sync");
			std::mem::swap(track.routes_mut(), &mut self.dummy_routes);
		}
		self.main_track.process(dt, clocks)
	}
}

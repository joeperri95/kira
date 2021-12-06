use atomic_arena::{Arena, Controller};
use ringbuf::Producer;

use crate::{
	clock::{Clock, ClockId},
	manager::command::ClockCommand,
};

/// Provides access to all existing [`Clock`]s.
///
/// You'll only have access to this if you're writing your own
/// [`Sound`](crate::sound::Sound)s.
pub struct Clocks {
	clocks: Arena<Clock>,
	unused_clock_producer: Producer<Clock>,
}

impl Clocks {
	pub(crate) fn new(capacity: usize, unused_clock_producer: Producer<Clock>) -> Self {
		Self {
			clocks: Arena::new(capacity),
			unused_clock_producer,
		}
	}

	pub(crate) fn controller(&self) -> Controller {
		self.clocks.controller()
	}

	/// Returns a reference to the clock with the given ID (if it exists).
	pub fn get(&self, id: ClockId) -> Option<&Clock> {
		self.clocks.get(id.0)
	}

	pub(crate) fn on_start_processing(&mut self) {
		if self.unused_clock_producer.is_full() {
			return;
		}
		for (_, clock) in self
			.clocks
			.drain_filter(|clock| clock.shared().is_marked_for_removal())
		{
			if self.unused_clock_producer.push(clock).is_err() {
				panic!("Unused clock producer is full")
			}
			if self.unused_clock_producer.is_full() {
				return;
			}
		}
	}

	pub(crate) fn run_command(&mut self, command: ClockCommand) {
		match command {
			ClockCommand::Add(id, clock) => self
				.clocks
				.insert_with_key(id.0, clock)
				.expect("Clock arena is full"),
			ClockCommand::SetInterval(id, interval) => {
				if let Some(clock) = self.clocks.get_mut(id.0) {
					clock.set_interval(interval);
				}
			}
			ClockCommand::Start(id) => {
				if let Some(clock) = self.clocks.get_mut(id.0) {
					clock.start();
				}
			}
			ClockCommand::Pause(id) => {
				if let Some(clock) = self.clocks.get_mut(id.0) {
					clock.pause();
				}
			}
			ClockCommand::Stop(id) => {
				if let Some(clock) = self.clocks.get_mut(id.0) {
					clock.stop();
				}
			}
		}
	}

	pub(crate) fn update(&mut self, dt: f64) {
		for (_, clock) in &mut self.clocks {
			clock.update(dt);
		}
	}
}

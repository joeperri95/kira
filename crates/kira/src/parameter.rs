use std::sync::{
	atomic::{AtomicU64, Ordering},
	Arc,
};

use ringbuf::{Consumer, Producer, RingBuffer};

use crate::{
	clock::Clocks,
	tween::{Tween, Tweenable},
	CommandQueueFull,
};

const COMMAND_CAPACITY: usize = 8;

enum Command {
	Set(f64, Tween),
}

pub struct Parameter {
	tweenable: Tweenable,
	command_consumer: Consumer<Command>,
	shared_value: Arc<AtomicU64>,
}

impl Parameter {
	pub fn new(initial_value: f64) -> (Self, ParameterHandle) {
		let (command_producer, command_consumer) = RingBuffer::new(COMMAND_CAPACITY).split();
		let shared_value = Arc::new(AtomicU64::new(initial_value.to_bits()));
		(
			Self {
				tweenable: Tweenable::new(initial_value),
				command_consumer,
				shared_value: shared_value.clone(),
			},
			ParameterHandle {
				command_producer,
				shared_value,
			},
		)
	}

	pub fn get(&self) -> f64 {
		self.tweenable.get()
	}

	pub fn on_start_processing(&mut self) {
		while let Some(command) = self.command_consumer.pop() {
			match command {
				Command::Set(target, tween) => self.tweenable.set(target, tween),
			}
		}
		self.shared_value
			.store(self.tweenable.get().to_bits(), Ordering::SeqCst);
	}

	pub fn update(&mut self, dt: f64, clocks: &Clocks) {
		self.tweenable.update(dt, clocks);
	}
}

pub struct ParameterHandle {
	command_producer: Producer<Command>,
	shared_value: Arc<AtomicU64>,
}

impl ParameterHandle {
	pub fn get(&self) -> f64 {
		f64::from_bits(self.shared_value.load(Ordering::SeqCst))
	}

	pub fn set(&mut self, target: f64, tween: Tween) -> Result<(), CommandQueueFull> {
		self.command_producer
			.push(Command::Set(target, tween))
			.map_err(|_| CommandQueueFull)
	}
}

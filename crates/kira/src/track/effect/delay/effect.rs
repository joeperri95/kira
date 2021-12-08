use crate::{
	clock::Clocks,
	dsp::{interpolate_frame, Frame},
	parameter::Parameter,
	track::Effect,
};

use super::{DelayBuilder, DelayHandle};

#[derive(Debug, Clone)]
enum DelayState {
	Uninitialized {
		buffer_length: f64,
	},
	Initialized {
		buffer: Vec<Frame>,
		write_position: usize,
	},
}

/// An effect that repeats audio after a certain delay. Useful
/// for creating echo effects.
pub struct Delay {
	delay_time: Parameter,
	feedback: Parameter,
	mix: Parameter,
	state: DelayState,
	feedback_effects: Vec<Box<dyn Effect>>,
}

impl Delay {
	/// Creates a new delay effect.
	pub fn new(builder: DelayBuilder) -> (Self, DelayHandle) {
		let (delay_time, delay_time_handle) = Parameter::new(builder.delay_time);
		let (feedback, feedback_handle) = Parameter::new(builder.feedback);
		let (mix, mix_handle) = Parameter::new(builder.mix);
		(
			Self {
				delay_time,
				feedback,
				mix,
				state: DelayState::Uninitialized {
					buffer_length: builder.buffer_length,
				},
				feedback_effects: builder.feedback_effects,
			},
			DelayHandle {
				delay_time: delay_time_handle,
				feedback: feedback_handle,
				mix: mix_handle,
			},
		)
	}
}

impl Effect for Delay {
	fn init(&mut self, sample_rate: u32) {
		if let DelayState::Uninitialized { buffer_length } = &self.state {
			self.state = DelayState::Initialized {
				buffer: vec![Frame::ZERO; (buffer_length * sample_rate as f64) as usize],
				write_position: 0,
			};
			for effect in &mut self.feedback_effects {
				effect.init(sample_rate);
			}
		} else {
			panic!("The delay should be in the uninitialized state before init")
		}
	}

	fn on_start_processing(&mut self) {
		self.delay_time.on_start_processing();
		self.feedback.on_start_processing();
		self.mix.on_start_processing();
		for effect in &mut self.feedback_effects {
			effect.on_start_processing();
		}
	}

	fn process(&mut self, input: Frame, dt: f64, clocks: &mut Clocks) -> Frame {
		if let DelayState::Initialized {
			buffer,
			write_position,
		} = &mut self.state
		{
			self.delay_time.update(dt, clocks);
			self.feedback.update(dt, clocks);
			self.mix.update(dt, clocks);

			// get the read position (in samples)
			let mut read_position = *write_position as f32 - (self.delay_time.get() / dt) as f32;
			while read_position < 0.0 {
				read_position += buffer.len() as f32;
			}

			// read an interpolated sample
			let current_sample_index = read_position as usize;
			let previous_sample_index = if current_sample_index == 0 {
				buffer.len() - 2
			} else {
				current_sample_index - 1
			};
			let next_sample_index = (current_sample_index + 1) % buffer.len();
			let next_sample_index_2 = (current_sample_index + 2) % buffer.len();
			let fraction = read_position % 1.0;
			let mut output = interpolate_frame(
				buffer[previous_sample_index],
				buffer[current_sample_index],
				buffer[next_sample_index],
				buffer[next_sample_index_2],
				fraction,
			);
			for effect in &mut self.feedback_effects {
				output = effect.process(output, dt, clocks);
			}

			// write output audio to the buffer
			*write_position += 1;
			*write_position %= buffer.len();
			buffer[*write_position] = input + output * self.feedback.get() as f32;

			let mix = self.mix.get() as f32;
			output * mix.sqrt() + input * (1.0 - mix).sqrt()
		} else {
			panic!("The delay should be initialized by the first process call")
		}
	}
}

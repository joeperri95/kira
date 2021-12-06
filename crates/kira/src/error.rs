use std::{
	error::Error,
	fmt::{Display, Formatter},
};

/// Errors that can occur when sending a command to the audio thread.
#[derive(Debug)]
#[non_exhaustive]
pub enum CommandError {
	/// Could not add a sound because the command queue is full.
	CommandQueueFull,
	/// Could not add a sound because a thread panicked while using the command queue.
	MutexPoisoned,
}

impl Display for CommandError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			CommandError::CommandQueueFull => {
				"Could not add a sound because the command queue is full."
			}
			CommandError::MutexPoisoned => {
				"Could not add a sound because a thread panicked while using the command queue."
			}
		})
	}
}

impl Error for CommandError {}

/// An error that occurs when trying to modify something
/// whose command queue is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandQueueFull;

impl Display for CommandQueueFull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Cannot send a command to the audio renderer because the command queue is full")
	}
}

impl Error for CommandQueueFull {}

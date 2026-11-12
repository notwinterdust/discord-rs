//! Voice client state and audio source abstractions.

#![forbid(unsafe_code)]

use std::collections::VecDeque;

/// A source of encoded audio frames.
pub trait AudioSource {
    /// Returns the next frame, or `None` when playback is complete.
    fn next_frame(&mut self) -> Option<Vec<u8>>;
}

/// A transport capable of sending voice frames.
pub trait VoiceTransport {
    /// Sends one encoded frame.
    fn send_frame(&mut self, frame: &[u8]) -> Result<(), String>;

    /// Closes the voice transport.
    fn close(&mut self);
}

/// Voice connection state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VoiceState {
    /// No voice connection exists.
    #[default]
    Disconnected,
    /// The voice session is connecting.
    Connecting,
    /// The voice session is ready.
    Ready,
    /// The voice session is closed.
    Closed,
}

/// A small voice playback controller.
pub struct VoiceClient<T> {
    transport: T,
    state: VoiceState,
    queue: VecDeque<Box<dyn AudioSource>>,
}

impl<T: VoiceTransport> VoiceClient<T> {
    /// Creates a disconnected voice client.
    #[must_use]
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            state: VoiceState::Disconnected,
            queue: VecDeque::new(),
        }
    }

    /// Returns the state.
    #[must_use]
    pub const fn state(&self) -> VoiceState {
        self.state
    }

    /// Marks the client ready.
    pub fn connect(&mut self) {
        self.state = VoiceState::Ready;
    }

    /// Queues an audio source.
    pub fn enqueue(&mut self, source: impl AudioSource + 'static) {
        self.queue.push_back(Box::new(source));
    }

    /// Sends one frame from the first queued source.
    pub fn tick(&mut self) -> Result<bool, String> {
        if self.state != VoiceState::Ready {
            return Ok(false);
        }
        let Some(source) = self.queue.front_mut() else {
            return Ok(false);
        };
        let Some(frame) = source.next_frame() else {
            self.queue.pop_front();
            return Ok(false);
        };
        self.transport.send_frame(&frame)?;
        Ok(true)
    }

    /// Closes the voice client.
    pub fn close(&mut self) {
        self.transport.close();
        self.state = VoiceState::Closed;
        self.queue.clear();
    }
}

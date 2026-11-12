//! Voice client state and audio source abstractions.

#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::net::{SocketAddr, UdpSocket};

/// Trait for audio codecs (PCM to Opus and vice versa).
pub trait AudioCodec {
    fn encode(&mut self, pcm: &[u8]) -> Result<Vec<u8>, String>;
    fn decode(&mut self, encoded: &[u8]) -> Result<Vec<u8>, String>;
}

/// Trait for encrypting and decrypting voice frames.
pub trait VoiceEncrypt {
    fn encrypt(&self, packet: &[u8]) -> Result<Vec<u8>, String>;
    fn decrypt(&self, packet: &[u8]) -> Result<Vec<u8>, String>;
}

/// Voice gateway state update payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceStateUpdate {
    pub guild_id: u64,
    pub channel_id: Option<u64>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

/// A UDP voice socket transport.
pub struct UdpVoiceTransport {
    socket: UdpSocket,
    remote: SocketAddr,
}

impl UdpVoiceTransport {
    pub fn bind(local: SocketAddr, remote: SocketAddr) -> Result<Self, String> {
        let socket = UdpSocket::bind(local).map_err(|err| err.to_string())?;
        Ok(Self { socket, remote })
    }

    pub fn send(&self, packet: &[u8]) -> Result<usize, String> {
        self.socket
            .send_to(packet, self.remote)
            .map_err(|err| err.to_string())
    }

    pub fn receive(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddr), String> {
        self.socket.recv_from(buffer).map_err(|err| err.to_string())
    }
}

/// A voice RTP packet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoicePacket {
    /// RTP sequence number.
    pub sequence: u16,
    /// RTP timestamp.
    pub timestamp: u32,
    /// Encoded payload.
    pub payload: Vec<u8>,
}

impl VoicePacket {
    /// Encodes a minimal standard RTP packet with payload type 0x78 (Opus).
    #[must_use]
    pub fn to_rtp(&self, ssrc: u32) -> Vec<u8> {
        let mut packet = Vec::with_capacity(12 + self.payload.len());
        packet.extend_from_slice(&[0x80, 0x78]);
        packet.extend_from_slice(&self.sequence.to_be_bytes());
        packet.extend_from_slice(&self.timestamp.to_be_bytes());
        packet.extend_from_slice(&ssrc.to_be_bytes());
        packet.extend_from_slice(&self.payload);
        packet
    }
}

/// Negotiated voice session metadata.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VoiceSession {
    /// Voice server endpoint.
    pub endpoint: Option<String>,
    /// Session identifier.
    pub session_id: Option<String>,
    /// Negotiated secret key.
    pub secret_key: Option<Vec<u8>>,
}

impl VoiceSession {
    /// Returns whether the session has completed negotiation.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.endpoint.is_some()
            && self.session_id.is_some()
            && self.secret_key.as_ref().is_some_and(|key| !key.is_empty())
    }
}

impl VoicePacket {
    /// Creates a packet.
    #[must_use]
    pub fn new(sequence: u16, timestamp: u32, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            sequence,
            timestamp,
            payload: payload.into(),
        }
    }
}

/// Voice heartbeat state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Heartbeat {
    /// Last sent nonce.
    pub nonce: u64,
    /// Whether an acknowledgement is pending.
    pub awaiting_ack: bool,
}

impl Heartbeat {
    /// Creates an initial heartbeat.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nonce: 0,
            awaiting_ack: false,
        }
    }

    /// Starts the next heartbeat.
    pub fn send(&mut self) -> u64 {
        self.nonce = self.nonce.wrapping_add(1);
        self.awaiting_ack = true;
        self.nonce
    }

    /// Acknowledges the current heartbeat.
    pub fn acknowledge(&mut self, nonce: u64) -> bool {
        if self.awaiting_ack && self.nonce == nonce {
            self.awaiting_ack = false;
            true
        } else {
            false
        }
    }
}

/// A source of encoded audio frames.
pub trait AudioSource {
    /// Returns the next frame, or `None` when playback is complete.
    fn next_frame(&mut self) -> Option<Vec<u8>>;
}

/// An in-memory audio source.
#[derive(Clone, Debug, Default)]
pub struct BufferedSource {
    frames: VecDeque<Vec<u8>>,
}

impl BufferedSource {
    /// Creates an empty source.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            frames: VecDeque::new(),
        }
    }

    /// Appends a frame.
    pub fn push(&mut self, frame: impl Into<Vec<u8>>) {
        self.frames.push_back(frame.into());
    }
}

impl AudioSource for BufferedSource {
    fn next_frame(&mut self) -> Option<Vec<u8>> {
        self.frames.pop_front()
    }
}

/// Applies a scalar volume to little-endian signed 16-bit PCM frames.
pub struct VolumeSource<S> {
    source: S,
    volume: f32,
}

impl<S> VolumeSource<S> {
    /// Creates a volume-wrapped source.
    #[must_use]
    pub const fn new(source: S, volume: f32) -> Self {
        Self { source, volume }
    }
}

impl<S: AudioSource> AudioSource for VolumeSource<S> {
    fn next_frame(&mut self) -> Option<Vec<u8>> {
        let mut frame = self.source.next_frame()?;
        for sample in frame.chunks_exact_mut(2) {
            let value = i16::from_le_bytes([sample[0], sample[1]]);
            let scaled = (f32::from(value) * self.volume)
                .round()
                .clamp(f32::from(i16::MIN), f32::from(i16::MAX)) as i16;
            sample.copy_from_slice(&scaled.to_le_bytes());
        }
        Some(frame)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockVoiceTransport {
        sent: Vec<Vec<u8>>,
        closed: bool,
    }

    impl VoiceTransport for MockVoiceTransport {
        fn send_frame(&mut self, frame: &[u8]) -> Result<(), String> {
            self.sent.push(frame.to_vec());
            Ok(())
        }
        fn close(&mut self) {
            self.closed = true;
        }
    }

    #[test]
    fn rtp_packet_serialization() {
        let packet = VoicePacket::new(10, 20, vec![1, 2, 3, 4]);
        let rtp = packet.to_rtp(12345);
        assert_eq!(rtp.len(), 16);
        assert_eq!(&rtp[0..2], &[0x80, 0x78]);
        assert_eq!(&rtp[2..4], &10_u16.to_be_bytes());
        assert_eq!(&rtp[4..8], &20_u32.to_be_bytes());
        assert_eq!(&rtp[8..12], &12345_u32.to_be_bytes());
        assert_eq!(&rtp[12..], &[1, 2, 3, 4]);
    }

    #[test]
    fn volume_source_scaling() {
        // Two 16-bit PCM samples: 1000 and -1000
        let pcm_sample: Vec<u8> = vec![
            (1000_i16).to_le_bytes()[0],
            (1000_i16).to_le_bytes()[1],
            (-1000_i16).to_le_bytes()[0],
            (-1000_i16).to_le_bytes()[1],
        ];

        let mut buffered = BufferedSource::new();
        buffered.push(pcm_sample);

        let mut vol_source = VolumeSource::new(buffered, 0.5);
        let scaled = vol_source.next_frame().unwrap();

        let s1 = i16::from_le_bytes([scaled[0], scaled[1]]);
        let s2 = i16::from_le_bytes([scaled[2], scaled[3]]);
        assert_eq!(s1, 500);
        assert_eq!(s2, -500);
    }

    #[test]
    fn voice_client_queue_and_playback_lifecycle() {
        let transport = MockVoiceTransport {
            sent: Vec::new(),
            closed: false,
        };
        let mut client = VoiceClient::new(transport);
        assert_eq!(client.state(), VoiceState::Disconnected);

        let mut src = BufferedSource::new();
        src.push(vec![1, 2, 3]);
        client.enqueue(src);

        // Before connecting, tick does not send
        assert!(!client.tick().unwrap());

        client.connect();
        assert_eq!(client.state(), VoiceState::Ready);

        // Tick sends the frame
        assert!(client.tick().unwrap());
        assert_eq!(client.transport.sent.len(), 1);

        // Subsequent tick pops empty source and finishes
        assert!(!client.tick().unwrap());

        client.close();
        assert_eq!(client.state(), VoiceState::Closed);
        assert!(client.transport.closed);
    }
}

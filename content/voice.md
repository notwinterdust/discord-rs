# Voice Gateway, Audio Pipelines & FFmpeg Streaming

`discord-rust` includes a dedicated [`voice`](#) crate for communicating with Discord's real-time voice infrastructure, and a built-in `FfmpegSource` for streaming audio from files or URLs.

---

## Voice Architecture

Discord voice connections operate on two separate protocols:
1. **Voice Gateway (WebSocket)**: Controls voice channel signaling, encryption handshakes (Select Protocol), speaking state, and heartbeat pings.
2. **Voice Media (UDP)**: High-speed UDP socket transmitting real-time encrypted audio packets framed with the **RTP (Real-time Transport Protocol)** standard.

---

## RTP Packet Framing

Before sending raw audio over UDP, audio frames must be encapsulated into standard RTP packets containing:
- SSRC (Synchronization Source Identifier assigned by Discord).
- Sequence Number (incremented per packet).
- Timestamp (incremented by frame sample size).
- Encoded Opus audio payload.

```rust
use discord_rust::voice::VoicePacket;

// 20ms of Opus-encoded audio
let payload = vec![0xF8, 0xFF, 0xFE];
let packet = VoicePacket::new(/*sequence=*/1, /*timestamp=*/960, payload);

let rtp_bytes = packet.to_rtp(/*ssrc=*/123456789);
// Transmit rtp_bytes over the voice UDP socket!
```

---

## FFmpeg Audio Streaming

`FfmpegSource` spawns an `ffmpeg` child process and streams its output as 48 kHz stereo 16-bit PCM — the exact format Discord's voice pipeline expects.

Each call to `next_frame()` returns exactly `1920` bytes (20 ms of stereo s16le PCM).

### From a File

```rust
use discord_rust::voice::FfmpegSource;

let mut source = FfmpegSource::from_file("audio/song.mp3")
    .expect("ffmpeg not found or file missing");

while let Some(frame) = source.next_frame() {
    // frame is Vec<u8> of exactly 1920 bytes (20ms @ 48kHz stereo s16le)
    // encode to Opus, wrap in VoicePacket, send over UDP
}
```

### From a URL

```rust
let mut source = FfmpegSource::from_url("https://example.com/stream.mp3")
    .expect("ffmpeg spawn failed");

// Streams live from the HTTP URL with reconnection flags set automatically
while let Some(frame) = source.next_frame() {
    // process frame
}
```

### Custom ffmpeg Invocation

```rust
use std::process::{Command, Stdio};
use discord_rust::voice::FfmpegSource;

let child = Command::new("ffmpeg")
    .args(["-re", "-i", "playlist.m3u8",
           "-f", "s16le", "-ac", "2", "-ar", "48000", "pipe:1"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

let mut source = FfmpegSource::from_child(child).unwrap();
```

---

## Audio Source Trait

All audio sources implement the simple `AudioSource` trait:

```rust
pub trait AudioSource {
    fn next_frame(&mut self) -> Option<Vec<u8>>;
}
```

You can implement your own source for synthesized audio, silence padding, or in-memory buffers:

```rust
use discord_rust::voice::{AudioSource, BufferedSource};

let mut buf = BufferedSource::new();
buf.push(vec![0u8; 1920]); // one 20ms silent frame

while let Some(frame) = buf.next_frame() {
    // send frame
}
```

---

## Volume Scaling

Wrap any `AudioSource` with `VolumeSource` for linear gain control:

```rust
use discord_rust::voice::VolumeSource;
use discord_rust::voice::FfmpegSource;

let raw = FfmpegSource::from_file("music.ogg").unwrap();
let mut attenuated = VolumeSource::new(raw, 0.5); // 50% volume

while let Some(frame) = attenuated.next_frame() {
    // frame is at half amplitude, clamped to i16 range
}
```

---

## Pluggable Encryption

Discord requires all voice packets to be encrypted (e.g. `xsalsa20_poly1305` or `aead_aes256_gcm_rtpsize`).

`discord-rust` defines the [`VoiceEncrypt`](#) trait:

```rust
pub trait VoiceEncrypt {
    fn encrypt(&self, packet: &[u8]) -> Result<Vec<u8>, String>;
    fn decrypt(&self, packet: &[u8]) -> Result<Vec<u8>, String>;
}
```

Plug in your preferred cryptographic backend with zero-copy slicing.

---

## Discord Frame Size Constant

```rust
use discord_rust::voice::DISCORD_FRAME_SIZE;

assert_eq!(DISCORD_FRAME_SIZE, 1_920);
// 48,000 samples/s × 0.020 s × 2 channels × 2 bytes/sample = 1,920 bytes
```

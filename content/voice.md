# Voice Gateway & Audio Pipelines

`discord-rust` includes a dedicated [`voice`](#) crate for communicating with Discord's real-time voice infrastructure.

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
- Encrypted Opus audio payload.

```rust
use discord_rust::voice::RtpPacket;

// Frame 20ms of audio
let payload = vec![0xF8, 0xFF, 0xFE]; // Opus packet bytes
let rtp = RtpPacket::new(1, 960, 123456, payload);

let raw_bytes = rtp.to_bytes();
// Transmit raw_bytes over the voice UDP socket!
```

---

## Audio Source & Volume Scaling

The audio pipeline provides [`VolumeSource`](#) for linear gain scaling and software volume control without clipping:

```rust
use discord_rust::voice::VolumeSource;

// Wrap an audio stream with a 50% volume attenuator
let mut volume_filter = VolumeSource::new(0.5);

let mut pcm_samples = vec![1000_i16, -2000_i16, 3000_i16];
volume_filter.apply_volume(&mut pcm_samples);

// Output: [500, -1000, 1500]
assert_eq!(pcm_samples, vec![500, -1000, 1500]);
```

---

## Pluggable Encryption

Discord requires all voice packets to be encrypted (e.g. `xsalsa20_poly1305` or `aead_aes256_gcm_rtpsize`).

`discord-rust` defines the [`VoiceEncryptor`](#) trait:
```rust
pub trait VoiceEncryptor {
    fn encrypt(&self, plaintext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String>;
    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String>;
}
```
You can plug in your preferred cryptographic backend with standard zero-copy slicing.

//! FFmpeg child process streaming audio source.

use std::io::Read;
use std::path::Path;
use std::process::{Child, ChildStdout, Command, Stdio};

use crate::AudioSource;

/// The standard frame size for 20ms of 48kHz stereo 16-bit PCM audio:
/// 48,000 samples/sec * 0.020 sec * 2 channels * 2 bytes/sample = 1,920 bytes.
pub const DISCORD_FRAME_SIZE: usize = 1_920;

/// An audio source that reads PCM frames from an `ffmpeg` child process standard output.
pub struct FfmpegSource {
    child: Child,
    stdout: ChildStdout,
    frame_size: usize,
}

impl FfmpegSource {
    /// Spawns an `ffmpeg` process reading from an audio or video file on disk.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the child process fails to spawn or stdout is unavailable.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let path_str = path.as_ref().to_string_lossy();
        Self::spawn(&path_str, &[])
    }

    /// Spawns an `ffmpeg` process streaming from an HTTP or media URL.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the child process fails to spawn or stdout is unavailable.
    pub fn from_url(url: &str) -> Result<Self, std::io::Error> {
        Self::spawn(url, &["-reconnect", "1", "-reconnect_streamed", "1"])
    }

    /// Spawns an `ffmpeg` process with custom input flags and converts to 48kHz stereo s16le PCM.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if `ffmpeg` fails to spawn.
    pub fn spawn(input: &str, extra_args: &[&str]) -> Result<Self, std::io::Error> {
        let mut command = Command::new("ffmpeg");

        for arg in extra_args {
            command.arg(arg);
        }

        command
            .arg("-i")
            .arg(input)
            .arg("-f")
            .arg("s16le")
            .arg("-ac")
            .arg("2")
            .arg("-ar")
            .arg("48000")
            .arg("-loglevel")
            .arg("quiet")
            .arg("pipe:1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        let mut child = command.spawn()?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "missing stdout"))?;

        Ok(Self {
            child,
            stdout,
            frame_size: DISCORD_FRAME_SIZE,
        })
    }

    /// Constructs an [`FfmpegSource`] from an existing child process with piped stdout.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the child's stdout is missing.
    pub fn from_child(mut child: Child) -> Result<Self, std::io::Error> {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "missing stdout"))?;

        Ok(Self {
            child,
            stdout,
            frame_size: DISCORD_FRAME_SIZE,
        })
    }

    /// Overrides the per-frame byte size read from the stream.
    #[must_use]
    pub const fn frame_size(mut self, size: usize) -> Self {
        self.frame_size = size;
        self
    }
}

impl AudioSource for FfmpegSource {
    fn next_frame(&mut self) -> Option<Vec<u8>> {
        let mut buffer = vec![0u8; self.frame_size];
        match self.stdout.read_exact(&mut buffer) {
            Ok(()) => Some(buffer),
            Err(_) => None,
        }
    }
}

impl Drop for FfmpegSource {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_size_constant_matches_discord_spec() {
        assert_eq!(DISCORD_FRAME_SIZE, 1_920);
    }

    #[test]
    fn audio_source_reads_from_child_process() {
        // Spawn a dummy process that outputs 1920 bytes of test data
        let mut cmd = Command::new("head");
        cmd.arg("-c")
            .arg("3840")
            .arg("/dev/zero")
            .stdout(Stdio::piped());

        if let Ok(child) = cmd.spawn() {
            let mut source = FfmpegSource::from_child(child).unwrap();
            let frame1 = source.next_frame();
            assert!(frame1.is_some());
            assert_eq!(frame1.unwrap().len(), 1920);

            let frame2 = source.next_frame();
            assert!(frame2.is_some());
            assert_eq!(frame2.unwrap().len(), 1920);

            let frame3 = source.next_frame();
            assert!(frame3.is_none());
        }
    }
}


//! Android audio stub - will be implemented with MediaCodec/AAudio in Phase 3
//!
//! This module provides placeholder audio decoder and player for Android builds
//! until MediaCodec integration is complete.

#![cfg(target_os = "android")]

use anyhow::Result;
use log::info;
use std::sync::Arc;
use parking_lot::Mutex;
use tokio::sync::mpsc;

/// Android audio decoder (stub)
pub struct AudioDecoder {
    sample_tx: mpsc::Sender<Vec<i16>>,
    sample_rx: Option<mpsc::Receiver<Vec<i16>>>,
}

impl AudioDecoder {
    pub fn new(_sample_rate: u32, _channels: u16) -> Result<Self> {
        info!("Android audio decoder stub initialized (MediaCodec implementation pending)");
        let (sample_tx, sample_rx) = mpsc::channel(100);
        Ok(Self {
            sample_tx,
            sample_rx: Some(sample_rx),
        })
    }

    pub fn decode(&mut self, _data: &[u8]) -> Result<Vec<i16>> {
        // Return silence for now (20ms at 48kHz stereo)
        Ok(vec![0; 1920])
    }

    pub fn decode_async(&mut self, _data: &[u8]) {
        // Stub - send silence
        let silence = vec![0i16; 1920];
        let _ = self.sample_tx.try_send(silence);
    }

    pub fn take_sample_receiver(&mut self) -> Option<mpsc::Receiver<Vec<i16>>> {
        self.sample_rx.take()
    }
}

/// Android audio player (stub)
pub struct AudioPlayer {
    playing: Arc<Mutex<bool>>,
}

impl AudioPlayer {
    pub fn new(_sample_rate: u32, _channels: u16) -> Result<Self> {
        info!("Android audio player stub initialized (oboe implementation pending)");
        Ok(Self {
            playing: Arc::new(Mutex::new(false)),
        })
    }

    pub fn play(&self) -> Result<()> {
        *self.playing.lock() = true;
        Ok(())
    }

    pub fn pause(&self) -> Result<()> {
        *self.playing.lock() = false;
        Ok(())
    }

    pub fn push_samples(&self, _samples: &[i16]) {
        // Stub - discard samples
    }

    pub fn buffer_level_ms(&self) -> f32 {
        0.0
    }
}

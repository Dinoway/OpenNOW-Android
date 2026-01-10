//! Android audio stub - will be implemented with MediaCodec in Phase 3
//!
//! This module provides a placeholder audio decoder for Android builds
//! until MediaCodec integration is complete.

#![cfg(target_os = "android")]

use anyhow::Result;
use log::info;

/// Android audio decoder (stub)
pub struct AndroidAudioDecoder {
    // Placeholder
}

impl AndroidAudioDecoder {
    pub fn new() -> Result<Self> {
        info!("Android audio decoder stub initialized (MediaCodec implementation pending)");
        Ok(Self {})
    }

    pub fn decode(&mut self, _data: &[u8]) -> Result<Vec<i16>> {
        // Return silence for now
        Ok(vec![0; 960]) // 20ms of silence at 48kHz
    }
}
//! Android-specific entry point and lifecycle management
//!
//! This module handles the Android NativeActivity lifecycle and provides
//! the entry point for the Rust application on Android.

#![cfg(target_os = "android")]

use android_activity::{
    AndroidApp, InputStatus, MainEvent, PollEvent,
};
use log::{info, error};
use std::sync::Arc;
use parking_lot::Mutex;

use crate::app::App;
use crate::gui::Renderer;

/// Android application state
pub struct AndroidAppState {
    app: Arc<Mutex<App>>,
    renderer: Option<Renderer>,
    runtime: tokio::runtime::Handle,
}

/// Android entry point (called by android-activity)
#[no_mangle]
fn android_main(android_app: AndroidApp) {
    // Initialize Android logger (outputs to logcat)
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("OpenNOW"),
    );

    info!("OpenNOW Android v{}", env!("CARGO_PKG_VERSION"));
    info!("Initializing Android application...");

    // Create tokio runtime for async operations
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            error!("Failed to create tokio runtime: {}", e);
            return;
        }
    };

    // Initialize NDK context for JNI calls
    ndk_context::initialize_android_context(
        android_app.vm_as_ptr() as *mut _,
        android_app.activity_as_ptr() as *mut _,
    );

    info!("NDK context initialized");

    // Create app state
    let app = Arc::new(Mutex::new(App::new(runtime.handle().clone())));
    let mut state = AndroidAppState {
        app,
        renderer: None,
        runtime: runtime.handle().clone(),
    };

    // Main event loop
    info!("Entering main event loop...");
    loop {
        let mut should_render = false;

        // Poll events with timeout
        android_app.poll_events(
            Some(std::time::Duration::from_millis(16)), // ~60fps
            |event| {
                match event {
                    PollEvent::Wake => {
                        info!("Event: Wake");
                    }
                    PollEvent::Timeout => {
                        // No events - continue to rendering
                        should_render = true;
                    }
                    PollEvent::Main(main_event) => {
                        info!("MainEvent: {:?}", main_event);
                        match main_event {
                            MainEvent::InitWindow { .. } => {
                                info!("Window initialized - creating renderer");
                                // TODO: Create renderer when winit integration is complete
                                // state.renderer = Some(Renderer::new(...));
                            }
                            MainEvent::TerminateWindow { .. } => {
                                info!("Window terminated - destroying renderer");
                                state.renderer = None;
                            }
                            MainEvent::WindowResized { .. } => {
                                info!("Window resized");
                                // TODO: Handle resize
                            }
                            MainEvent::RedrawNeeded { .. } => {
                                should_render = true;
                            }
                            MainEvent::InputAvailable { .. } => {
                                // TODO: Handle input events
                            }
                            MainEvent::ConfigChanged { .. } => {
                                info!("Configuration changed");
                            }
                            MainEvent::LowMemory => {
                                info!("Low memory warning");
                                // TODO: Reduce memory usage
                            }
                            MainEvent::Start => {
                                info!("Activity started");
                            }
                            MainEvent::Resume { .. } => {
                                info!("Activity resumed");
                                // TODO: Resume streaming if active
                            }
                            MainEvent::SaveState { .. } => {
                                info!("Saving state");
                                // TODO: Save app state
                            }
                            MainEvent::Pause => {
                                info!("Activity paused");
                                // TODO: Pause streaming
                                let mut app = state.app.lock();
                                if app.state == crate::app::AppState::Streaming {
                                    info!("Pausing active stream");
                                    // Don't terminate, just pause
                                }
                            }
                            MainEvent::Stop => {
                                info!("Activity stopped");
                            }
                            MainEvent::Destroy => {
                                info!("Activity destroyed - exiting");
                                return;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            },
        );

        // Render frame if needed
        if should_render {
            if let Some(ref mut renderer) = state.renderer {
                // TODO: Render with existing renderer
                // For now, just update app state
                let mut app = state.app.lock();
                app.update();
            }
        }
    }
}

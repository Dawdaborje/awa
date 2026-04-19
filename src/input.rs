use rdev::{Event, EventType, listen};
use std::sync::mpsc::{self, Receiver};

/// Spawns a background thread that listens for global mouse movement.
/// Returns a receiver that yields (x, y) screen coordinates.
pub fn spawn_mouse_listener() -> Receiver<(f64, f64)> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        listen(move |event: Event| {
            if let EventType::MouseMove { x, y } = event.event_type {
                // Ignore send errors — happens if the main thread exits first
                let _ = tx.send((x, y));
            }
        })
        .expect("failed to start global mouse listener");
    });

    rx
}

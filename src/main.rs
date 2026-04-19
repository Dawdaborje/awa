mod input;
mod mascot;
mod physics;
mod sprite;

use mascot::Mascot;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowLevel},
};

const MASCOT_W: u32 = 128;
const MASCOT_H: u32 = 128;

// pixels 0.13 has Pixels<'surf> tied to the surface texture reference.
// Storing both the window and Pixels in the same struct creates a
// self-referential problem. Box::leak gives the Window a 'static lifetime,
// which is fine — the mascot lives for the entire process anyway.
struct AppReady {
    window: &'static Window,
    pixels: Pixels<'static>,
    mascot: Mascot,
}

struct App {
    mouse_rx: std::sync::mpsc::Receiver<(f64, f64)>,
    ready: Option<AppReady>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("awa")
            .with_inner_size(PhysicalSize::new(MASCOT_W, MASCOT_H))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false);

        // Leak into 'static so Pixels can hold a reference without lifetime headaches
        let window: &'static Window = Box::leak(Box::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        ));
        window.set_window_level(WindowLevel::AlwaysOnTop);

        let surface_texture = SurfaceTexture::new(MASCOT_W, MASCOT_H, window);
        let pixels = Pixels::new(MASCOT_W, MASCOT_H, surface_texture)
            .expect("failed to create pixel buffer");

        let mascot = Mascot::new(MASCOT_W, MASCOT_H);

        self.ready = Some(AppReady {
            window,
            pixels,
            mascot,
        });
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(state) = &mut self.ready else { return };

        while let Ok((mx, my)) = self.mouse_rx.try_recv() {
            state.mascot.physics.set_target(mx as f32, my as f32);
        }

        state.mascot.update();

        let pos = state.mascot.physics.pos;
        state
            .window
            .set_outer_position(winit::dpi::PhysicalPosition::new(
                pos.x as i32 - (MASCOT_W as i32 / 2),
                pos.y as i32 - MASCOT_H as i32,
            ));

        state.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.ready else { return };

        match event {
            WindowEvent::RedrawRequested => {
                state.mascot.draw(state.pixels.frame_mut());
                state.pixels.render().expect("render failed");
            }

            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::MouseInput {
                button: winit::event::MouseButton::Right,
                state: winit::event::ElementState::Pressed,
                ..
            } => {
                state.mascot.cycle_mode();
                log::info!("mode: {:?}", state.mascot.mode);
            }

            _ => {}
        }
    }
}

fn main() {
    env_logger::init();

    let mouse_rx = input::spawn_mouse_listener();

    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        mouse_rx,
        ready: None,
    };
    event_loop.run_app(&mut app).expect("event loop error");
}

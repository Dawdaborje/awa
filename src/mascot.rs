use crate::physics::SpringPhysics;
use std::time::Instant;

/// Awa's personality modes — switch with right-click
#[derive(Debug, Clone, PartialEq)]
pub enum MascotMode {
    Cute,
    Sexy,
    Focus, // for when you're deep in work — minimal animations
}

/// What Awa is currently doing
#[derive(Debug, Clone, PartialEq)]
pub enum MascotState {
    Idle,
    Walking,
    Running,
    Sitting,
    Waving,
}

pub struct Mascot {
    pub physics: SpringPhysics,
    pub mode: MascotMode,
    pub state: MascotState,

    width: u32,
    height: u32,

    last_frame: Instant,
    idle_timer: f32,   // seconds since last significant movement
    speed_smooth: f32, // smoothed speed for state transitions
}

impl Mascot {
    pub fn new(width: u32, height: u32) -> Self {
        // Start in the center of the primary monitor (rough estimate)
        Self {
            physics: SpringPhysics::new(960.0, 540.0),
            mode: MascotMode::Cute,
            state: MascotState::Idle,
            width,
            height,
            last_frame: Instant::now(),
            idle_timer: 0.0,
            speed_smooth: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32().min(0.1);
        self.last_frame = now;

        self.physics.update(dt);
        self.update_state(dt);
    }

    fn update_state(&mut self, dt: f32) {
        let speed = self.physics.vel.length();

        // Smooth speed so state transitions aren't jittery
        self.speed_smooth += (speed - self.speed_smooth) * (dt * 8.0);

        match self.speed_smooth {
            s if s > 400.0 => self.transition(MascotState::Running),
            s if s > 80.0 => self.transition(MascotState::Walking),
            _ => {
                self.idle_timer += dt;
                if self.idle_timer > 5.0 {
                    // Sit down after 5 seconds of no movement
                    self.transition(MascotState::Sitting);
                } else {
                    self.transition(MascotState::Idle);
                }
            }
        }
    }

    fn transition(&mut self, new_state: MascotState) {
        if self.state != new_state {
            log::debug!("state: {:?} -> {:?}", self.state, new_state);
            if new_state != MascotState::Sitting && new_state != MascotState::Idle {
                self.idle_timer = 0.0;
            }
            self.state = new_state;
        }
    }

    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            MascotMode::Cute => MascotMode::Sexy,
            MascotMode::Sexy => MascotMode::Focus,
            MascotMode::Focus => MascotMode::Cute,
        };
    }

    /// Draw the current frame into the pixel buffer.
    ///
    /// Right now this renders a placeholder colored square so you can see
    /// something on screen before you have real sprites. Swap this out for
    /// SpriteSheet::blit_frame() once you have your Aseprite assets.
    pub fn draw(&self, frame: &mut [u8]) {
        // Clear to transparent
        for px in frame.chunks_exact_mut(4) {
            px[0] = 0;
            px[1] = 0;
            px[2] = 0;
            px[3] = 0;
        }

        // Placeholder color per mode so you can see mode switching works
        let color: [u8; 4] = match self.mode {
            MascotMode::Cute => [255, 182, 193, 220], // pink
            MascotMode::Sexy => [180, 80, 220, 220],  // purple
            MascotMode::Focus => [80, 160, 255, 220], // blue
        };

        // Draw a simple rounded blob — replace with sprite blit later
        let cx = (self.width / 2) as i32;
        let cy = (self.height / 2) as i32;
        let r = 40i32;

        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= r * r {
                    let i = (y as usize * self.width as usize + x as usize) * 4;
                    frame[i..i + 4].copy_from_slice(&color);
                }
            }
        }

        // Small indicator dot that shows current state
        let dot_color: [u8; 4] = match self.state {
            MascotState::Idle => [255, 255, 255, 200],
            MascotState::Walking => [100, 255, 100, 200],
            MascotState::Running => [255, 200, 50, 200],
            MascotState::Sitting => [100, 200, 255, 200],
            MascotState::Waving => [255, 100, 100, 200],
        };

        let dot_x = cx;
        let dot_y = cy;
        let dr = 8i32;
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let dx = x - dot_x;
                let dy = y - dot_y;
                if dx * dx + dy * dy <= dr * dr {
                    let i = (y as usize * self.width as usize + x as usize) * 4;
                    frame[i..i + 4].copy_from_slice(&dot_color);
                }
            }
        }
    }
}

/// Simple 2D vector
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

/// Spring-damper physics for smooth, springy following
pub struct SpringPhysics {
    pub pos: Vec2,
    pub vel: Vec2,
    target: Vec2,

    /// How strongly the mascot is pulled toward the target
    stiffness: f32,
    /// How much velocity bleeds off each frame (0..1, closer to 1 = more damping)
    damping: f32,
    /// Distance within which the mascot ignores the mouse (dead zone)
    dead_zone: f32,
}

impl SpringPhysics {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::default(),
            target: Vec2::new(x, y),
            stiffness: 6.0,
            damping: 0.75,
            dead_zone: 120.0, // px — won't follow if mouse is this close
        }
    }

    pub fn set_target(&mut self, x: f32, y: f32) {
        self.target = Vec2::new(x, y);
    }

    /// Call once per frame. `dt` is delta-time in seconds.
    pub fn update(&mut self, dt: f32) {
        let diff = self.target - self.pos;
        let dist = diff.length();

        // Dead zone — don't be annoying
        if dist < self.dead_zone {
            // Gradually slow down rather than hard-stopping
            self.vel = self.vel * (self.damping * 0.8);
            self.pos = self.pos + self.vel * dt;
            return;
        }

        let force = diff * self.stiffness;
        self.vel = (self.vel + force * dt) * self.damping;

        // Cap speed so it doesn't warp across the screen
        let speed = self.vel.length();
        if speed > 800.0 {
            self.vel = self.vel * (800.0 / speed);
        }

        self.pos = self.pos + self.vel * dt;
    }
}

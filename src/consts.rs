pub const WINDOW_WIDTH: f32 = 1024.;
pub const WINDOW_HEIGHT: f32 = 720.;
pub const GROUND_THICKNESS: f32 = 50.; // bottom of screen to top of ground px
pub const GROUND_Y: f32 = GROUND_THICKNESS - WINDOW_HEIGHT / 2.; // px top of ground

pub const GUN_COOLDOWN: f64 = 0.3; // seconds
pub const BULLET_SPEED: f32 = 500.; // px / s

pub const OUT_OF_BOUNDS_X: f32 = WINDOW_WIDTH / 2.0 + 90.;
pub const OUT_OF_BOUNDS_Y: f32 = WINDOW_HEIGHT / 2.0 + 90.;

pub const GRAVITY: f32 = -29.81;

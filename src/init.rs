use super::*;

pub const SIDE: f32 = 800.0;
pub const HALF: f32 = SIDE / 2.0;

const SCALE: f32 = 1.0;
const SCREEN_SIDE: f32 = SIDE / SCALE;
const CENTER: Vec2 = vec2(SCREEN_SIDE / 2.0, SCREEN_SIDE / 2.0);

pub const fn pow(num: f32, exp: usize) -> f32 {
    if exp <= 1 {
        num
    } else {
        num * pow(num, exp - 1)
    }
}

pub fn init_with_camera() -> Camera2D {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    request_new_screen_size(SCREEN_SIDE, SCREEN_SIDE);
    Camera2D {
        target: Vec2::ZERO,
        zoom: 2.0 * Vec2::ONE / vec2(SCREEN_SIDE, SCREEN_SIDE),
        ..Default::default()
    }
}

pub fn world_pos<T: Into<Vec2>>(screen_point: T) -> Vec2 {
    screen_point.into() - CENTER
}

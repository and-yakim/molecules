use super::*;

pub const SIDE: f32 = 800.0;
pub const HALF: f32 = SIDE / 2.0;

pub const fn pow(num: f32, exp: usize) -> f32 {
    if exp <= 1 {
        num
    } else {
        num * pow(num, exp - 1)
    }
}

const BORDER: f32 = 20.0;

pub fn init_with_camera() -> Camera2D {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    request_new_screen_size(SIDE + BORDER, SIDE + BORDER);
    Camera2D {
        target: Vec2::ZERO,
        zoom: 2.0 * Vec2::ONE / vec2(SIDE + BORDER, SIDE + BORDER),
        ..Default::default()
    }
}

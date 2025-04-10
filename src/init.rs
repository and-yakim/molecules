use super::*;

pub const SCREEN_SIDE: f32 = 800.0;

pub const fn pow(num: f32, exp: usize) -> f32 {
    if exp <= 1 {
        num
    } else {
        num * pow(num, exp - 1)
    }
}

pub fn init() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    request_new_screen_size(SCREEN_SIDE, SCREEN_SIDE);
}

pub fn get_camera(target: Vec2, scale: f32) -> Camera2D {
    Camera2D {
        target,
        zoom: 2.0 * vec2(scale, scale) / SCREEN_SIDE,
        ..Default::default()
    }
}

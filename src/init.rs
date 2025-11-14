use super::*;

pub const SCREEN_SIDE: f32 = 800.0;
pub const CENTER: Vec2 = vec2(SCREEN_SIDE / 2.0, SCREEN_SIDE / 2.0);

pub fn default_world_pos(screen_pos: Vec2) -> Vec2 {
    screen_pos - CENTER
}

pub const fn pow(num: f32, exp: usize) -> f32 {
    if exp <= 1 {
        num
    } else {
        num * pow(num, exp - 1)
    }
}

pub fn init_rand() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
}

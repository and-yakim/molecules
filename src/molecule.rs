pub use macroquad::prelude::*;

pub const SIDE: f32 = 800.0;
pub const HALF: f32 = SIDE / 2.0;

pub const RADIUS: f32 = 20.0;

const fn pow(num: f32, exp: usize) -> f32 {
    if exp <= 1 {
        num
    } else {
        num * pow(num, exp - 1)
    }
}

const R2: f32 = pow(RADIUS, 2);
const RC: f32 = RADIUS * 2.5;
const RC2: f32 = pow(RC, 2);

const FC: f32 = -(-1.0 / pow(2.5, 14) + 1.0 / pow(2.5, 8));

pub struct Molecule {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Molecule {
    pub fn new(pos: Vec2, vel: Vec2) -> Molecule {
        Molecule { pos, vel }
    }

    pub fn force(&mut self, mol: &mut Self) {}

    pub fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    pub fn border_check(&mut self) {}

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, RADIUS, BLUE);
    }
}

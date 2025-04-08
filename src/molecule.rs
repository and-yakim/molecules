pub use macroquad::prelude::*;
use std::f32::consts::PI;
pub use std::time;

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

const DIAMETER: f32 = RADIUS * 2.0;

const SIDE_MOLS: usize = (SIDE / DIAMETER) as usize;
pub const LEN: usize = SIDE_MOLS * SIDE_MOLS;

const R2: f32 = pow(RADIUS, 2);
const RC: f32 = RADIUS * 2.5;
const RC2: f32 = pow(RC, 2);

const FC: f32 = -(-1.0 / pow(2.5, 14) + 1.0 / pow(2.5, 8));

#[derive(Clone, Copy, Default)]
pub struct Molecule {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Molecule {
    pub fn new(pos: Vec2, vel: Vec2) -> Molecule {
        Molecule { pos, vel }
    }

    pub fn generate() -> [Self; LEN] {
        let mut arr = [Molecule::default(); LEN];
        let start = vec2(-HALF + RADIUS, -HALF + RADIUS);
        for i in 0..SIDE_MOLS {
            for j in 0..SIDE_MOLS {
                let ampl = (-rand::gen_range::<f32>(0.0, 1.0).ln()).sqrt();
                let angle = rand::gen_range(0.0, 2.0 * PI);
                let vel = vec2(ampl * angle.cos(), ampl * angle.sin());
                let pos = start + vec2(DIAMETER * i as f32, DIAMETER * j as f32);
                arr[SIDE_MOLS * i + j] = Molecule::new(pos, vel);
            }
        }
        arr
    }

    pub fn force(&mut self, mol: &mut Self) {}

    pub fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    pub fn border_check(&mut self) {}

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, RADIUS, DARKGREEN);
    }
}

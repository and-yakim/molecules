use super::init::*;
use super::*;
use std::f32::consts::PI;

pub trait Molecule {
    const RADIUS: f32;
    const DIAMETER: f32 = Self::RADIUS * 2.0;
    const R2: f32 = pow(Self::RADIUS, 2);

    fn r(&self) -> f32 {
        Self::RADIUS
    }

    fn d(&self) -> f32 {
        Self::DIAMETER
    }

    fn r2(&self) -> f32 {
        Self::R2
    }

    fn pos(&self) -> Vec2;

    fn vel(&self) -> Vec2;

    fn move_pos(&mut self);

    fn force(&mut self, mol: &mut Self);

    fn draw(&self) {}
}

#[derive(Clone, Copy, Default)]
pub struct Atom {
    pub pos: Vec2,
    pub vel: Vec2,
}

const FC: f32 = -(-1.0 / pow(2.5, 14) + 1.0 / pow(2.5, 8));

impl Molecule for Atom {
    const RADIUS: f32 = 20.0;

    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn vel(&self) -> Vec2 {
        self.vel
    }

    fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    fn force(&mut self, _mol: &mut Self) {}

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r(), DARKGREEN);
    }
}

impl Atom {
    const RC: f32 = Self::RADIUS * 2.5;
    const RC2: f32 = pow(Self::RC, 2);

    pub fn new(pos: Vec2, vel: Vec2) -> Atom {
        Atom { pos, vel }
    }

    const SIDE_MOLS: usize = (SIDE / Self::DIAMETER) as usize;
    const GEN_LEN: usize = Self::SIDE_MOLS * Self::SIDE_MOLS;
    pub fn generate(cell: f32) -> [Self; Self::GEN_LEN] {
        let mut arr = [Atom::default(); Self::GEN_LEN];
        let start = Vec2::splat(cell);
        for i in 0..Self::SIDE_MOLS {
            for j in 0..Self::SIDE_MOLS {
                let ampl = (-rand::gen_range::<f32>(0.0, 1.0).ln()).sqrt();
                let angle = rand::gen_range(0.0, 2.0 * PI);
                let vel = vec2(ampl * angle.cos(), ampl * angle.sin());
                let pos = start + vec2(Self::DIAMETER * i as f32, Self::DIAMETER * j as f32);
                arr[Self::SIDE_MOLS * i + j] = Atom::new(pos, vel);
            }
        }
        arr
    }

    pub fn border_check(&mut self) {}
}

use super::init::*;
use super::*;
use std::f32::consts::PI;

pub trait Molecule {
    const RADIUS: f32;

    fn pos(&self) -> Vec2;

    const DIAMETER: f32 = Self::RADIUS * 2.0;
    const R2: f32 = pow(Self::RADIUS, 2);

    const RC: f32 = Self::RADIUS * 2.5;
    const RC2: f32 = pow(Self::RC, 2);

    fn r(&self) -> f32 {
        Self::RADIUS
    }
    fn d(&self) -> f32 {
        Self::DIAMETER
    }
    fn r2(&self) -> f32 {
        Self::R2
    }

    fn draw(&self) {
        let pos = self.pos();
        draw_circle(pos.x, pos.y, self.r(), DARKGREEN);
    }
}

const _FC: f32 = -(-1.0 / pow(2.5, 14) + 1.0 / pow(2.5, 8));

pub struct Atom<const R: usize> {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl<const R: usize> Molecule for Atom<R> {
    const RADIUS: f32 = R as f32;

    fn pos(&self) -> Vec2 {
        self.pos
    }
}

impl<const R: usize> Atom<R> {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Atom { pos, vel }
    }

    pub fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    pub fn generate(side: f32) -> Vec<Self> {
        let side_n = (side / Self::DIAMETER) as usize;
        let mut arr = Vec::with_capacity(side_n * side_n);
        let start = Vec2::splat(Self::RC + Self::RADIUS);
        for i in 0..side_n {
            for j in 0..side_n {
                let ampl = (-rand::gen_range::<f32>(0.0, 1.0).ln()).sqrt();
                let angle = rand::gen_range(0.0, 2.0 * PI);
                let vel = vec2(ampl * angle.cos(), ampl * angle.sin());
                let pos = start + vec2(Self::DIAMETER * i as f32, Self::DIAMETER * j as f32);
                arr.push(Self::new(pos, vel));
            }
        }
        arr
    }
}

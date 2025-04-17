use super::*;
pub use crate::init::pow;
use std::f32::consts::PI;

/// +- 1k in f32
pub type FVec2 = Vector2<I11F21>;

pub fn fvec2(x: f32, y: f32) -> FVec2 {
    Vector2::new(I11F21::from_num(x), I11F21::from_num(y))
}

pub fn to_vec2(v: FVec2) -> Vec2 {
    Vec2::new(v.x.to_num::<f32>(), v.y.to_num::<f32>())
}

/// for distance below 32.0
/// or Atom<R < 45.25>
pub fn distance(a: FVec2, b: FVec2) -> I11F21 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

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

const FC: f32 = -(-1.0 / pow(2.5, 14) + 1.0 / pow(2.5, 8));

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

    pub fn get_force(&self, other: &Self) -> Option<Vec2> {
        let diff = self.pos - other.pos;
        let r2 = diff.length_squared();
        if r2 < Self::RC2 {
            let f1 = Self::R2 / r2;
            let f2 = f1.powi(3);
            let df = f2 * f1 * (f2 - 1.0) - FC;
            println!("{df}");
            Some(df * diff.normalize())
        } else {
            None
        }
    }

    pub fn generate(side: f32, offset: Vec2, sparsity: f32) -> Vec<Self> {
        let dist = Self::DIAMETER * sparsity;
        let side_n = (side / dist) as usize;
        let mut arr = Vec::with_capacity(side_n * side_n);
        let start = offset + Vec2::splat(Self::RADIUS);
        for i in 0..side_n {
            for j in 0..side_n {
                let ampl = (-rand::gen_range::<f32>(0.0, 1.0).ln()).sqrt();
                let angle = rand::gen_range(0.0, 2.0 * PI);
                let vel = vec2(ampl * angle.cos(), ampl * angle.sin());
                let pos = start + vec2(dist * i as f32, dist * j as f32);
                arr.push(Self::new(pos, vel));
            }
        }
        arr
    }
}

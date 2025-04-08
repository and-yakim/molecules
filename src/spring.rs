use std::cell::Cell;

use super::init::*;
use super::*;

use ndarray::prelude::*;

const CELL: f32 = 20.0;
const K: f32 = 1.0;

struct Point {
    pos: Vec2,
    vel: Vec2,
}

impl Point {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Point { pos, vel }
    }

    fn from_shape(shape: (usize, usize), offset: Vec2) -> Self {
        let pos = vec2(shape.0 as f32, shape.1 as f32) * CELL;
        Self::new(pos + offset, Vec2::ZERO)
    }

    fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    fn force(&mut self, mol: &mut Self, ext_force: Vec2) {}

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 4.0, GREEN);
    }
}

pub struct SoftBody {
    arr: Array2<Point>,
}

impl SoftBody {
    pub fn new(a: usize, b: usize) -> Self {
        let offset = vec2(-CELL * a as f32 / 2.0, -CELL * b as f32 / 2.0);
        SoftBody {
            arr: Array2::from_shape_fn((a, b), |shape| Point::from_shape(shape, offset)),
        }
    }

    pub fn draw(&self) {
        for point in &self.arr {
            point.draw();
        }
    }
}

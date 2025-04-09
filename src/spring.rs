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

    fn x(&self) -> f32 {
        self.pos.x
    }

    fn y(&self) -> f32 {
        self.pos.y
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
        draw_circle(self.x(), self.y(), 4.0, GREEN);
    }

    fn draw_link(&self, next: &Self) {
        draw_line(self.x(), self.y(), next.x(), next.y(), 2.0, DARKGREEN);
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
        let shape = self.arr.shape();
        for i in 0..(shape[0] - 1) {
            for j in 0..(shape[1] - 1) {
                let curr = &self.arr[[i, j]];
                let next = &self.arr[[i, j + 1]];
                let orto = &self.arr[[i + 1, j]];
                curr.draw_link(next);
                curr.draw_link(orto);
                curr.draw();
            }
            let curr = &self.arr[[i, shape[1] - 1]];
            let orto = &self.arr[[i + 1, shape[1] - 1]];
            curr.draw_link(orto);
            curr.draw();
        }
        for j in 0..(shape[1] - 1) {
            let curr = &self.arr[[shape[0] - 1, j]];
            let next = &self.arr[[shape[0] - 1, j + 1]];
            curr.draw_link(next);
            curr.draw();
        }
        self.arr[[shape[0] - 1, shape[1] - 1]].draw();
    }
}

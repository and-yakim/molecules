#![allow(unused)]
use super::init::*;
use super::*;

use ndarray::prelude::*;

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

    fn from_shape(shape: (usize, usize), offset: Vec2, cell: f32) -> Self {
        let pos = vec2(shape.0 as f32, shape.1 as f32) * cell;
        Self::new(pos + offset, Vec2::ZERO)
    }

    fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    fn add_force(&mut self, force: Vec2) {
        self.vel += force
    }

    fn draw(&self) {
        draw_circle(self.x(), self.y(), 4.0, GREEN);
    }

    fn draw_link(&self, next: &Self) {
        draw_line(self.x(), self.y(), next.x(), next.y(), 2.0, DARKGREEN);
    }
}

pub struct SoftBody {
    arr: Array2<Point>,
    shape: (usize, usize),
    cell: f32,
}

const K: f32 = 0.01;

impl SoftBody {
    pub fn new(shape: (usize, usize), cell: f32) -> Self {
        let offset = vec2(-cell * shape.0 as f32 / 2.0, -cell * shape.1 as f32 / 2.0);
        SoftBody {
            arr: Array2::from_shape_fn(shape, |shape| Point::from_shape(shape, offset, cell)),
            shape,
            cell: cell * 1.1,
        }
    }

    fn force(&mut self, i: [usize; 2], j: [usize; 2]) {
        let diff = self.arr[i].pos - self.arr[j].pos;
        let force = K * (self.cell - diff.length()) * diff.normalize();
        self.arr[i].add_force(force);
        self.arr[j].add_force(-force);
    }

    pub fn update(&mut self) {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                let curr = [i, j];
                let next = [i, j + 1];
                let orto = [i + 1, j];
                self.force(curr, next);
                self.force(curr, orto);
            }
            let curr = [i, self.shape.1 - 1];
            let orto = [i + 1, self.shape.1 - 1];
            self.force(curr, orto);
        }
        for j in 0..(self.shape.1 - 1) {
            let curr = [self.shape.0 - 1, j];
            let next = [self.shape.0 - 1, j + 1];
            self.force(curr, next);
        }
        for point in self.arr.iter_mut() {
            point.move_pos();
        }
    }

    pub fn draw(&self) {
        for win in self.arr.windows([2, 1]) {
            &win[[0, 0]].draw_link(&win[[1, 0]]);
        }
        for win in self.arr.windows([1, 2]) {
            &win[[0, 0]].draw_link(&win[[0, 1]]);
            &win[[0, 0]].draw();
        }
        for i in 0..self.shape.0 {
            &self.arr[[i, self.shape.1 - 1]].draw();
        }
    }
}

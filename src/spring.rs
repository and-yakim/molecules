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

pub fn spring_force(p1: Vec2, p2: Vec2, l0: f32) -> Vec2 {
    let diff = p1 - p2;
    K * (l0 - diff.length()) * diff.normalize()
}

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
        let force = spring_force(self.arr[i].pos, self.arr[j].pos, self.cell);
        self.arr[i].add_force(force);
        self.arr[j].add_force(-force);
    }

    fn iterate<F2, F>(&self, f2: F2, f: F)
    where
        F2: Fn(&Point, &Point),
        F: Fn(&Point),
    {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                let curr = [i, j];
                let next = [i, j + 1];
                let orto = [i + 1, j];
                f2(&self.arr[curr], &self.arr[next]);
                f2(&self.arr[curr], &self.arr[orto]);
                f(&self.arr[curr]);
            }
            let curr = [i, self.shape.1 - 1];
            let orto = [i + 1, self.shape.1 - 1];
            f2(&self.arr[curr], &self.arr[orto]);
            f(&self.arr[curr]);
        }
        for j in 0..(self.shape.1 - 1) {
            let curr = [self.shape.0 - 1, j];
            let next = [self.shape.0 - 1, j + 1];
            f2(&self.arr[curr], &self.arr[next]);
            f(&self.arr[curr]);
        }
        f(&self.arr[[self.shape.0 - 1, self.shape.1 - 1]]);
    }

    fn iterate_mut<F2, F>(&mut self, f2: F2, f: F)
    where
        F2: Fn(&mut Self, [usize; 2], [usize; 2]),
        F: Fn(&mut Point),
    {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                let curr = [i, j];
                let next = [i, j + 1];
                let orto = [i + 1, j];
                f2(self, curr, next);
                f2(self, curr, orto);
                f(&mut self.arr[curr]);
            }
            let curr = [i, self.shape.1 - 1];
            let orto = [i + 1, self.shape.1 - 1];
            f2(self, curr, orto);
            f(&mut self.arr[curr]);
        }
        for j in 0..(self.shape.1 - 1) {
            let curr = [self.shape.0 - 1, j];
            let next = [self.shape.0 - 1, j + 1];
            f2(self, curr, next);
            f(&mut self.arr[curr]);
        }
        f(&mut self.arr[[self.shape.0 - 1, self.shape.1 - 1]]);
    }

    pub fn update(&mut self) {
        self.iterate_mut(Self::force, Point::move_pos);
    }

    pub fn draw(&self) {
        self.iterate(Point::draw_link, Point::draw);
    }
}

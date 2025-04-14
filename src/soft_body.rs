use std::f32::consts::PI;

pub use super::spring::*;
use super::*;

use ndarray::prelude::*;

pub struct RectBody {
    pub arr: Array2<Point>,
    pub shape: (usize, usize),
    cell: f32,
    diag: f32,
}

const K: f32 = 0.2;

impl RectBody {
    pub fn from_shape(shape: (usize, usize), offset: Vec2, cell: f32) -> Point {
        let pos = vec2(shape.0 as f32, shape.1 as f32) * cell;
        Point::new(pos + offset)
    }
    pub fn new(shape: (usize, usize), cell: f32) -> Self {
        let offset = vec2(-cell * shape.0 as f32 / 2.0, -cell * shape.1 as f32 / 2.0);
        RectBody {
            arr: Array2::from_shape_fn(shape, |shape| Self::from_shape(shape, offset, cell)),
            shape,
            cell,
            diag: cell * 2.0f32.sqrt(),
        }
    }

    pub fn force(&mut self, i: [usize; 2], j: [usize; 2], l0: f32) {
        let force = spring_force(self.arr[i].pos, self.arr[j].pos, K, l0);
        self.arr[i].add_force(force);
        self.arr[j].add_force(-force);
    }

    pub fn update(&mut self) {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                self.force([i, j], [i + 1, j], self.cell);
                self.force([i, j], [i, j + 1], self.cell);
                self.force([i, j], [i + 1, j + 1], self.diag);
                self.force([i + 1, j], [i, j + 1], self.diag);
                self.arr[[i, j]].move_pos();
            }
            self.force([i, self.shape.1 - 1], [i + 1, self.shape.1 - 1], self.cell);
            self.arr[[i, self.shape.1 - 1]].move_pos();
        }
        for j in 0..(self.shape.1 - 1) {
            self.force([self.shape.0 - 1, j], [self.shape.0 - 1, j + 1], self.cell);
            self.arr[[self.shape.0 - 1, j]].move_pos();
        }
        self.arr[[self.shape.0 - 1, self.shape.1 - 1]].move_pos();
    }

    pub fn update_with_ext_force(&mut self, force: Vec2) {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                self.force([i, j], [i + 1, j], self.cell);
                self.force([i, j], [i, j + 1], self.cell);
                self.force([i, j], [i + 1, j + 1], self.diag);
                self.force([i + 1, j], [i, j + 1], self.diag);
                self.arr[[i, j]].add_force(force);
                self.arr[[i, j]].move_pos();
            }
            self.force([i, self.shape.1 - 1], [i + 1, self.shape.1 - 1], self.cell);
            self.arr[[i, self.shape.1 - 1]].add_force(force);
            self.arr[[i, self.shape.1 - 1]].move_pos();
        }
        for j in 0..(self.shape.1 - 1) {
            self.force([self.shape.0 - 1, j], [self.shape.0 - 1, j + 1], self.cell);
            self.arr[[self.shape.0 - 1, j]].add_force(force);
            self.arr[[self.shape.0 - 1, j]].move_pos();
        }
        self.arr[[self.shape.0 - 1, self.shape.1 - 1]].add_force(force);
        self.arr[[self.shape.0 - 1, self.shape.1 - 1]].move_pos();
    }

    pub fn get_outer_indexes(&self) -> Vec<[usize; 2]> {
        (0..(self.shape.0 - 1))
            .map(|i| [i, 0])
            .chain((0..(self.shape.1 - 1)).map(|i| [self.shape.0 - 1, i]))
            .chain((1..self.shape.0).map(|i| [i, self.shape.1 - 1]).rev())
            .chain((0..self.shape.1).map(|i| [0, i]).rev())
            .collect()
    }

    pub fn iter_outer<F: Fn([usize; 2], [usize; 2]) -> [usize; 2]>(&self, f: F) {
        let chained = (0..(self.shape.0 - 1))
            .map(|i| [i, 0])
            .chain((0..(self.shape.1 - 1)).map(|i| [self.shape.0 - 1, i]))
            .chain((1..self.shape.0).map(|i| [i, self.shape.1 - 1]).rev())
            .chain((0..self.shape.1).map(|i| [0, i]).rev());
        chained.reduce(f);
    }

    pub fn draw_outer(&self) {
        self.iter_outer(|acc, value| {
            self.arr[acc].draw_link(&self.arr[value]);
            self.arr[acc].draw();
            value
        });
    }

    pub fn draw_points(&self) {
        self.arr.for_each(Point::draw);
    }

    pub fn draw_full(&self) {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                self.arr[[i, j]].draw_link(&self.arr[[i + 1, j]]);
                self.arr[[i, j]].draw_link(&self.arr[[i, j + 1]]);
                self.arr[[i, j]].draw_link(&self.arr[[i + 1, j + 1]]);
                self.arr[[i + 1, j]].draw_link(&self.arr[[i, j + 1]]);
                self.arr[[i, j]].draw();
            }
            self.arr[[i, self.shape.1 - 1]].draw_link(&self.arr[[i + 1, self.shape.1 - 1]]);
            self.arr[[i, self.shape.1 - 1]].draw();
        }
        for j in 0..(self.shape.1 - 1) {
            self.arr[[self.shape.0 - 1, j]].draw_link(&self.arr[[self.shape.0 - 1, j + 1]]);
            self.arr[[self.shape.0 - 1, j]].draw();
        }
        self.arr[[self.shape.0 - 1, self.shape.1 - 1]].draw();
    }
}

pub struct Wheel {
    /// central Point + len outer Points
    pub arr: Vec<Point>,
    pub len: usize,
    radius: f32,
    cell: f32,
}

impl Wheel {
    pub fn new(center: Vec2, radius: f32, len: usize) -> Self {
        let mut arr = Vec::with_capacity(len + 1);
        arr.push(Point::new(center));
        let angle = 2.0 * PI / len as f32;
        let cell = (2.0 * radius * radius * (1.0 - angle.cos())).sqrt();
        for i in 0..len {
            let pos = center + radius * Vec2::from_angle(angle * i as f32);
            arr.push(Point::new(pos));
        }
        Wheel {
            arr,
            len,
            radius,
            cell,
        }
    }

    pub fn center(&self) -> &Point {
        &self.arr[0]
    }

    pub fn center_mut(&mut self) -> &mut Point {
        &mut self.arr[0]
    }

    pub fn force(&mut self, i: usize, j: usize, l0: f32) {
        let force = spring_force(self.arr[i].pos, self.arr[j].pos, K, l0);
        self.arr[i].add_force(force);
        self.arr[j].add_force(-force);
    }

    pub fn update_with_ext_force(&mut self, force: Vec2) {
        for i in 1..self.len {
            self.force(i, 0, self.radius);
            self.force(i, i + 1, self.cell);
        }
        self.force(self.len, 0, self.radius);
        self.force(self.len, 1, self.cell);
        self.arr[0].add_force(force);
        self.arr.iter_mut().for_each(Point::move_pos);
    }

    pub fn iter_outer_indexes(&self) -> std::ops::Range<usize> {
        1..self.len
    }

    pub fn draw_points(&self) {
        self.arr.iter().for_each(Point::draw);
    }

    pub fn draw_full(&self) {
        for i in 1..self.len {
            self.arr[i].draw_link(&self.arr[0]);
            self.arr[i].draw_link(&self.arr[i + 1]);
        }
        self.arr[self.len].draw_link(&self.arr[0]);
        self.arr[self.len].draw_link(&self.arr[1]);
        self.draw_points();
    }
}

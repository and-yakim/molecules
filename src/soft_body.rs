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
        Point::new(pos + offset, Vec2::ZERO)
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

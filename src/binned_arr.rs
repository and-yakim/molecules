use super::init::*;
use super::*;

use ndarray::prelude::*;

/// Starts from (cell, cell)
pub struct BinnedArr<T> {
    arr: Array2<Vec<T>>,
    side: usize,
    cell: f32,
}

impl<T> BinnedArr<T> {
    pub fn new(side: usize, cell: f32, n: usize) -> Self {
        let estimate = n / (side * side) * 2;
        BinnedArr {
            arr: Array2::from_shape_fn((side, side), |_| Vec::with_capacity(estimate)),
            side,
            cell,
        }
    }

    pub fn get_coords(&self, pos: Vec2) -> [usize; 2] {
        [
            (pos.x / self.cell).floor() as usize - 1,
            (pos.y / self.cell).floor() as usize - 1,
        ]
    }

    pub fn add(&mut self, pos: Vec2, value: T) {
        let coords = self.get_coords(pos);
        self.arr[coords].push(value);
    }

    pub fn clear(&mut self) {
        self.arr.iter_mut().for_each(Vec::clear);
    }

    pub fn get_camera(&self) -> Camera2D {
        let target = Vec2::splat(self.cell * (self.side as f32 / 2.0 + 1.0));
        let scale = SCREEN_SIDE / (self.cell * self.side as f32);
        get_camera(target, scale)
    }

    pub fn draw(&self) {
        for i in 0..self.side {
            for j in 0..self.side {
                draw_rectangle_lines(
                    self.cell + i as f32 * self.cell,
                    self.cell + j as f32 * self.cell,
                    self.cell,
                    self.cell,
                    2.0,
                    RED,
                );
            }
        }
    }
}

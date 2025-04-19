use super::*;

pub use ndarray::prelude::*;

/// Starts from (cell, cell)
pub struct BinnedArr<T> {
    pub arr: Array2<Vec<T>>,
    pub size: Fixed,
    pub side: usize,
    pub cell: Fixed,
}

impl<T> BinnedArr<T> {
    pub fn new(size: Fixed, cell: Fixed, n: usize) -> Self {
        let side = (size / cell).to_num();
        assert!(side > 2); // for corner_coords
        let estimate = n / (side * side) + 1;
        BinnedArr {
            arr: Array2::from_shape_fn((side, side), |_| Vec::with_capacity(estimate)),
            size,
            side,
            cell,
        }
    }

    pub fn get_coords(&self, pos: FVec2) -> [usize; 2] {
        // println!("c {} {}", pos[0], pos[0].to_bits());
        [
            (pos.x / self.cell).floor().to_num::<usize>() - 1,
            (pos.y / self.cell).floor().to_num::<usize>() - 1,
        ]
    }

    pub fn add(&mut self, pos: FVec2, value: T) {
        let coords = self.get_coords(pos);
        self.arr[coords].push(value);
    }

    pub fn clear(&mut self) {
        self.arr.iter_mut().for_each(Vec::clear);
    }

    // replace with n + 2
    fn next_rem(&self, i: usize) -> usize {
        (i + 1) % self.side
    }
    fn corner_coords(&self, coords: [usize; 2]) -> [[usize; 2]; 4] {
        let next_i = self.next_rem(coords[0]);
        let next_j = self.next_rem(coords[1]);
        [
            [next_i, coords[1]],
            [next_i, next_j],
            [coords[0], next_j],
            [self.next_rem(coords[0] + self.side - 2), next_j],
        ]
    }

    pub fn update_by_fn<F: FnMut(&T, &T)>(&self, coords: [usize; 2], mut f: F) {
        self.arr[coords].iter().enumerate().for_each(|(i, a)| {
            self.arr[coords]
                .iter()
                .skip(i + 1)
                .for_each(|other| f(a, other));
            self.corner_coords(coords).into_iter().for_each(|corner| {
                self.arr[corner].iter().for_each(|other| f(a, other));
            });
        });
    }

    pub fn get_camera(&self) -> Camera2D {
        Camera2D {
            target: Vec2::splat(self.cell.to_num::<f32>() * (self.side as f32 / 2.0 + 1.0)),
            zoom: 2.0 * Vec2::ONE / (self.cell.to_num::<f32>() * self.side as f32),
            ..Default::default()
        }
    }

    pub fn draw(&self) {
        let cell = self.cell.to_num();
        for i in 0..self.side {
            for j in 0..self.side {
                draw_rectangle_lines(
                    (i + 1) as f32 * cell,
                    (j + 1) as f32 * cell,
                    cell,
                    cell,
                    2.0,
                    RED,
                );
            }
        }
    }
}

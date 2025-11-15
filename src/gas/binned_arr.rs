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
    pub fn new(size: Fixed, cell: Fixed) -> Self {
        let side = (size / cell).to_num();
        assert!(side > 2); // for corner_coords
        let estimate = 4; // most memory efficient
        BinnedArr {
            arr: Array2::from_shape_fn((side, side), |_| Vec::with_capacity(estimate)),
            size,
            side,
            cell,
        }
    }

    pub fn get_coords(&self, pos: FVec2) -> [usize; 2] {
        [
            ((pos.x - self.cell) / self.cell).floor().to_num::<usize>(),
            ((pos.y - self.cell) / self.cell).floor().to_num::<usize>(),
        ]
    }

    pub fn add(&mut self, pos: FVec2, value: T) {
        let coords = self.get_coords(pos);
        self.arr[coords].push(value);
    }

    pub fn clear(&mut self) {
        self.arr.iter_mut().for_each(Vec::clear);
    }

    pub fn get_camera(&self, width: f32) -> Camera2D {
        let zoom = width / self.size.to_num::<f32>();
        let offset = -zoom * self.cell.to_num::<f32>();
        Camera2D {
            offset: math::Vector2::new(offset, offset),
            zoom: zoom,
            ..Default::default()
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        let cell = self.cell.to_num();
        for i in 0..self.side {
            for j in 0..self.side {
                d.draw_rectangle_lines_ex(
                    math::Rectangle {
                        x: (i + 1) as f32 * cell,
                        y: (j + 1) as f32 * cell,
                        width: cell,
                        height: cell,
                    },
                    2.0,
                    Color::RED,
                );
            }
        }
    }
}

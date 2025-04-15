use std::vec;

pub use super::spring::*;
use super::*;

pub fn round_to_triangular_grid(pos: Vec2, cell: f32) -> Vec2 {
    let row = (pos.y / (cell * 3.0f32.sqrt() / 2.0)).round();
    let offset = (row as i32 % 2) as f32 * cell * 0.5;
    Vec2::new(
        ((pos.x - offset) / cell).round() * cell + offset,
        row * (cell * 3.0f32.sqrt() / 2.0),
    )
}

pub struct SpringMesh {
    arr: Vec<Point>,
    links: Vec<(usize, usize)>,
    pub cell: f32,
}

impl SpringMesh {
    pub fn new(pos: Vec2, cell: f32) -> Self {
        SpringMesh {
            arr: vec![Point::new(pos)],
            links: Vec::new(),
            cell,
        }
    }

    pub fn add(&mut self, pos: Vec2) {
        self.arr.push(Point::new(pos));
    }

    pub fn draw(&self) {
        self.arr.iter().for_each(Point::draw);
        self.links.iter().for_each(|link| {
            self.arr[link.0].draw_link(&self.arr[link.1]);
        });
    }
}

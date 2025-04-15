use std::vec;

pub use super::spring::*;
use super::*;

pub struct SpringMesh {
    arr: Vec<Point>,
    values: Vec<(usize, usize)>,
    cell: f32,
}

impl SpringMesh {
    pub fn new(pos: Vec2, cell: f32) -> Self {
        SpringMesh {
            arr: vec![Point::new(pos)],
            values: Vec::new(),
            cell,
        }
    }

    pub fn draw(&self) {}
}

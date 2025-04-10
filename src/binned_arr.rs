#![allow(unused)]
use super::init::*;
use super::*;

use ndarray::prelude::*;

fn get_coords(pos: Vec2, cell: f32) -> IVec2 {
    ivec2((pos.x / cell).round() as i32, (pos.y / cell).round() as i32)
}

struct BinnedArr<T> {
    pub arr: Array2<Vec<T>>,
    pub side: usize,
    pub cell: f32,
}

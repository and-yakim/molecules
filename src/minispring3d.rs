#![allow(unused)]
use super::init::*;
use super::*;

use ndarray::prelude::*;

struct Value {
    x: f32,
    y: f32,
    z: f32,
}

struct Point {
    pos: Value,
    vel: Value,
}

pub struct SoftBody3D {
    arr: Array3<Point>,
    center: [usize; 3],
    corner: [usize; 3],
    global: Vec3,
}

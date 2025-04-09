use super::init::*;
use super::*;

use ndarray::prelude::*;

struct Value {
    x: f16,
    y: f16,
    z: f16,
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

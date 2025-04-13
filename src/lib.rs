pub use macroquad::prelude::*;
pub use std::time;

pub const DEBUG_RED: Color = Color::new(RED.r, RED.g, RED.b, 0.5);

pub mod binned_arr;
pub mod init;
pub mod molecule;
pub mod spring;

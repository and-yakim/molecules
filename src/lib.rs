pub use macroquad::prelude::*;
pub use std::time;

pub const DEBUG_RED: Color = Color { a: 0.5, ..RED };

pub mod binned_arr;
pub mod init;
pub mod molecule;
pub mod soft_body;
pub mod spring;

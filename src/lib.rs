pub use macroquad::prelude::*;
pub use std::time;

pub const DEBUG_RED: Color = Color { a: 0.5, ..RED };

pub mod gas;
pub mod init;
pub mod soft_body;
pub mod spring;

pub use init::*;

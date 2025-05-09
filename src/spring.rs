#![allow(unused)]
use super::*;

use ndarray::prelude::*;

#[derive(Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Point {
    pub fn new(pos: Vec2) -> Self {
        Point {
            pos,
            vel: Vec2::ZERO,
        }
    }

    pub fn x(&self) -> f32 {
        self.pos.x
    }

    pub fn y(&self) -> f32 {
        self.pos.y
    }

    pub fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.vel += force
    }

    pub fn add_force_fn<F: Fn(&Self) -> Vec2>(&mut self, f: F) {
        let force = f(&self);
        self.vel += force
    }

    pub fn apply_spring_force(&mut self, pos: Vec2, k: f32, l0: f32) {
        let force = spring_force(self.pos, pos, k, l0);
        self.add_force(force);
    }

    pub fn draw(&self) {
        draw_circle(self.x(), self.y(), 4.0, GREEN);
    }

    pub fn draw_link(&self, next: &Self) {
        draw_line(self.x(), self.y(), next.x(), next.y(), 2.0, DARKGREEN);
    }

    pub fn draw_link_pos(&self, pos: Vec2) {
        draw_line(self.x(), self.y(), pos.x, pos.y, 2.0, DARKGREEN);
    }
}

pub fn spring_force(p1: Vec2, p2: Vec2, k: f32, l0: f32) -> Vec2 {
    let diff = p1 - p2;
    k * (l0 - diff.length()) * diff.normalize()
}

pub struct Point3D {
    pos: Vec3,
    vel: Vec3,
}

pub struct SoftBody3D {
    arr: Array3<Point3D>,
    center: [usize; 3],
    corner: [usize; 3],
    global: Vec3,
}

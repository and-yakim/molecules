use molecules::init::*;
use molecules::spring::*;
use molecules::*;

use ndarray::prelude::*;

struct SoftBody {
    pub arr: Array2<Point>,
    pub shape: (usize, usize),
    cell: f32,
    diag: f32,
}

const K: f32 = 0.1;

impl SoftBody {
    fn from_shape(shape: (usize, usize), offset: Vec2, cell: f32) -> Point {
        let pos = vec2(shape.0 as f32, shape.1 as f32) * cell;
        Point::new(pos + offset, Vec2::ZERO)
    }
    fn new(shape: (usize, usize), cell: f32) -> Self {
        let offset = vec2(-cell * shape.0 as f32 / 2.0, -cell * shape.1 as f32 / 2.0);
        SoftBody {
            arr: Array2::from_shape_fn(shape, |shape| Self::from_shape(shape, offset, cell)),
            shape,
            cell,
            diag: cell * 2.0f32.sqrt(),
        }
    }

    fn force(&mut self, i: [usize; 2], j: [usize; 2], l0: f32) {
        let force = spring_force(self.arr[i].pos, self.arr[j].pos, K, l0);
        self.arr[i].add_force(force);
        self.arr[j].add_force(-force);
    }

    fn update(&mut self) {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                self.force([i, j], [i + 1, j], self.cell);
                self.force([i, j], [i, j + 1], self.cell);
                self.force([i, j], [i + 1, j + 1], self.diag);
                self.force([i + 1, j], [i, j + 1], self.diag);
                self.arr[[i, j]].move_pos();
            }
            self.force([i, self.shape.1 - 1], [i + 1, self.shape.1 - 1], self.cell);
            self.arr[[i, self.shape.1 - 1]].move_pos();
        }
        for j in 0..(self.shape.1 - 1) {
            self.force([self.shape.0 - 1, j], [self.shape.0 - 1, j + 1], self.cell);
            self.arr[[self.shape.0 - 1, j]].move_pos();
        }
        self.arr[[self.shape.0 - 1, self.shape.1 - 1]].move_pos();
    }

    fn draw_full(&self) {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                self.arr[[i, j]].draw_link(&self.arr[[i + 1, j]]);
                self.arr[[i, j]].draw_link(&self.arr[[i, j + 1]]);
                self.arr[[i, j]].draw_link(&self.arr[[i + 1, j + 1]]);
                self.arr[[i + 1, j]].draw_link(&self.arr[[i, j + 1]]);
                self.arr[[i, j]].draw();
            }
            self.arr[[i, self.shape.1 - 1]].draw_link(&self.arr[[i + 1, self.shape.1 - 1]]);
            self.arr[[i, self.shape.1 - 1]].draw();
        }
        for j in 0..(self.shape.1 - 1) {
            self.arr[[self.shape.0 - 1, j]].draw_link(&self.arr[[self.shape.0 - 1, j + 1]]);
            self.arr[[self.shape.0 - 1, j]].draw();
        }
        self.arr[[self.shape.0 - 1, self.shape.1 - 1]].draw();
    }
}

#[macroquad::main("Soft body")]
async fn main() {
    init();
    let camera = get_camera(Vec2::ZERO, 1.0);

    let mut body = SoftBody::new((5, 5), 50.0);

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        if is_key_down(KeyCode::Space) {
            let pos = default_world_pos(mouse_position().into());
            body.arr[[0, 0]].apply_spring_force(pos, K / 100.0, 0.0);
            body.arr[[0, 0]].draw_link_pos(pos);
        }

        body.update();
        body.draw_full();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}

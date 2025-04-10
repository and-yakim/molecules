use molecules::init::*;
use molecules::spring::*;
use molecules::*;

use ndarray::prelude::*;

struct DummyLattice {
    pub arr: Array2<Point>,
    pub shape: (usize, usize),
    cell: f32,
}

const K: f32 = 0.05;

impl DummyLattice {
    fn from_shape(shape: (usize, usize), offset: Vec2, cell: f32) -> Point {
        let pos = vec2(shape.0 as f32, shape.1 as f32) * cell;
        Point::new(pos + offset, Vec2::ZERO)
    }
    fn new(shape: (usize, usize), cell: f32) -> Self {
        let offset = vec2(-cell * shape.0 as f32 / 2.0, -cell * shape.1 as f32 / 2.0);
        DummyLattice {
            arr: Array2::from_shape_fn(shape, |shape| Self::from_shape(shape, offset, cell)),
            shape,
            cell: cell * 1.1,
        }
    }

    fn force(&mut self, i: [usize; 2], j: [usize; 2]) {
        let force = spring_force(self.arr[i].pos, self.arr[j].pos, K, self.cell);
        self.arr[i].add_force(force);
        self.arr[j].add_force(-force);
    }

    fn iterate<F2, F>(&self, f2: F2, f: F)
    where
        F2: Fn(&Point, &Point),
        F: Fn(&Point),
    {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                let curr = [i, j];
                let next = [i, j + 1];
                let orto = [i + 1, j];
                f2(&self.arr[curr], &self.arr[next]);
                f2(&self.arr[curr], &self.arr[orto]);
                f(&self.arr[curr]);
            }
            let curr = [i, self.shape.1 - 1];
            let orto = [i + 1, self.shape.1 - 1];
            f2(&self.arr[curr], &self.arr[orto]);
            f(&self.arr[curr]);
        }
        for j in 0..(self.shape.1 - 1) {
            let curr = [self.shape.0 - 1, j];
            let next = [self.shape.0 - 1, j + 1];
            f2(&self.arr[curr], &self.arr[next]);
            f(&self.arr[curr]);
        }
        f(&self.arr[[self.shape.0 - 1, self.shape.1 - 1]]);
    }

    fn iterate_mut<F2, F>(&mut self, f2: F2, f: F)
    where
        F2: Fn(&mut Self, [usize; 2], [usize; 2]),
        F: Fn(&mut Point),
    {
        for i in 0..(self.shape.0 - 1) {
            for j in 0..(self.shape.1 - 1) {
                let curr = [i, j];
                let next = [i, j + 1];
                let orto = [i + 1, j];
                f2(self, curr, next);
                f2(self, curr, orto);
                f(&mut self.arr[curr]);
            }
            let curr = [i, self.shape.1 - 1];
            let orto = [i + 1, self.shape.1 - 1];
            f2(self, curr, orto);
            f(&mut self.arr[curr]);
        }
        for j in 0..(self.shape.1 - 1) {
            let curr = [self.shape.0 - 1, j];
            let next = [self.shape.0 - 1, j + 1];
            f2(self, curr, next);
            f(&mut self.arr[curr]);
        }
        f(&mut self.arr[[self.shape.0 - 1, self.shape.1 - 1]]);
    }

    fn update(&mut self) {
        self.iterate_mut(Self::force, Point::move_pos);
    }

    fn draw(&self) {
        self.iterate(Point::draw_link, Point::draw);
    }
}

#[macroquad::main("Dummy lattice")]
async fn main() {
    init();
    let camera = get_camera(Vec2::ZERO, 1.0);

    let mut body = DummyLattice::new((10, 10), 20.0);

    loop {
        clear_background(DARKGRAY);
        set_camera(&camera);

        body.update();
        body.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}

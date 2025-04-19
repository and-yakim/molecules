pub use super::binned_arr::*;
pub use super::molecule::*;
use super::*;

type Particle = Atom<4>; // max 4
const CELL: Fixed = Particle::RC;
const SIZE: Fixed = fmul(CELL, 10); // 1k max (~1023 with 2 * CELL)

const ACTUAL_SIZE: i32 = SIZE.to_bits() >> 21;

pub struct Gas {
    pub value: Vec<Particle>,
    pub system: BinnedArr<usize>,
}

impl Gas {
    pub fn new(sparcity: f32) -> Gas {
        let value = Particle::generate(SIZE, FVec2::new(CELL, CELL), sparcity);
        let system = BinnedArr::<usize>::new(SIZE, CELL, value.len());
        Gas { value, system }
    }

    pub fn refresh_sys(&mut self) {
        self.system.clear();
        for (i, mol) in self.value.iter().enumerate() {
            self.system.add(mol.pos, i);
        }
    }

    pub fn force_gas(&mut self) {
        for i in 0..self.system.side {
            for j in 0..self.system.side {
                self.system.update_by_fn([i, j], |x, y| {
                    if let Some(force) = self.value[*x].get_force(&self.value[*y]) {
                        self.value[*x].vel += force;
                        self.value[*y].vel -= force;
                    }
                });
            }
        }
    }

    /// [CELL, CELL + SIZE)
    fn wrap_range(num: Fixed) -> Fixed {
        ((num - CELL) % SIZE + SIZE) % SIZE + CELL
    }
    pub fn fix_bounds(&mut self) {
        for k in 0..self.system.side {
            self.system.arr[[0, k]]
                .iter()
                .chain(self.system.arr[[self.system.side - 1, k]].iter())
                .for_each(|i| {
                    self.value[*i].pos.x = Self::wrap_range(self.value[*i].pos.x);
                    // println!("X {}\n", self.value[*i].pos.x);
                });
            self.system.arr[[k, 0]]
                .iter()
                .chain(self.system.arr[[k, self.system.side - 1]].iter())
                .for_each(|i| {
                    self.value[*i].pos.y = Self::wrap_range(self.value[*i].pos.y);
                });
        }
    }

    pub fn move_gas(&mut self) {
        for mol in self.value.iter_mut() {
            mol.move_pos();
        }
        self.fix_bounds();
    }

    pub fn draw(&self) {
        self.value.iter().for_each(Atom::draw);
    }
}

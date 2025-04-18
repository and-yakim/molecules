pub use super::binned_arr::*;
pub use super::molecule::*;
use super::*;

type Particle = Atom<4>; // max 4
const SIZE: f32 = 1000.0; // max 1000.0
const CELL: f32 = Particle::RC;

pub struct Gas {
    pub value: Vec<Particle>,
    pub system: BinnedArr<usize>,
}

impl Gas {
    pub fn new(sparcity: f32) -> Gas {
        let value = Particle::generate(SIZE, Vec2::splat(CELL), sparcity);
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
    fn wrap_range(num: f32) -> f32 {
        ((num - CELL) % SIZE + SIZE) % SIZE + CELL
    }
    pub fn move_gas(&mut self) {
        for mol in self.value.iter_mut() {
            mol.move_pos();
        }

        for k in 0..self.system.side {
            self.system.arr[[k, 0]]
                .iter()
                .chain(self.system.arr[[k, self.system.side - 1]].iter())
                .for_each(|i| {
                    self.value[*i].pos.x = Self::wrap_range(self.value[*i].pos.x);
                });
            self.system.arr[[0, k]]
                .iter()
                .chain(self.system.arr[[self.system.side - 1, k]].iter())
                .for_each(|i| {
                    self.value[*i].pos.y = Self::wrap_range(self.value[*i].pos.y);
                });
        }
    }

    pub fn draw(&self) {
        self.value.iter().for_each(Molecule::draw);
    }
}

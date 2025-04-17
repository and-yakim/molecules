pub use super::binned_arr::*;
pub use super::molecule::*;
use super::*;

type Particle = Atom<4>;
const SIZE: f32 = 1000.0;
const CELL: f32 = Particle::RC;
const FAR_BORDER: f32 = CELL + SIZE;

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

    pub fn move_gas(&mut self) {
        for mol in self.value.iter_mut() {
            mol.move_pos();
        }

        for n in 0..self.system.side {
            self.system.arr[[n, 0]].iter().for_each(|i| {
                if self.value[*i].pos.x < self.system.cell {
                    self.value[*i].pos.x += SIZE;
                }
            });
            self.system.arr[[n, self.system.side - 1]]
                .iter()
                .for_each(|i| {
                    if self.value[*i].pos.x > FAR_BORDER {
                        self.value[*i].pos.x -= SIZE;
                    }
                });
            self.system.arr[[0, n]].iter().for_each(|i| {
                if self.value[*i].pos.y < self.system.cell {
                    self.value[*i].pos.y += SIZE;
                }
            });
            self.system.arr[[self.system.side - 1, n]]
                .iter()
                .for_each(|i| {
                    if self.value[*i].pos.y > FAR_BORDER {
                        self.value[*i].pos.y -= SIZE;
                    }
                });
        }
    }

    pub fn draw(&self) {
        self.value.iter().for_each(Molecule::draw);
    }
}

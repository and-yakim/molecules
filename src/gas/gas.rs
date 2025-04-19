pub use super::binned_arr::*;
pub use super::molecule::*;
use super::*;

type Particle = Atom<4>; // max 4
const CELL: Fixed = Particle::RC;
const SIZE: Fixed = fmul(CELL, 20); // 1k max (~1023 with 2 * CELL)
const _ACTUAL_SIZE: i32 = SIZE.to_bits() >> 21;

const fn corner_coords(coords: [usize; 2]) -> [[usize; 2]; 4] {
    let next_i = coords[0] + 1;
    let next_j = coords[1] + 1;
    let prev_j = coords[1] - 1;
    [
        [coords[0], next_j],
        [next_i, next_j],
        [next_i, coords[1]],
        [next_i, prev_j],
    ]
}

const OFFSET_TOP: FVec2 = FVec2::new(Fixed::ZERO, fmul(SIZE, -1));
const OFFSET_BOTTOM: FVec2 = FVec2::new(Fixed::ZERO, SIZE);
const OFFSET_TOP_RIGHT: FVec2 = FVec2::new(SIZE, fmul(SIZE, -1));
const OFFSET_BOTTOM_RIGHT: FVec2 = FVec2::new(SIZE, SIZE);

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

    fn _apply_force(&mut self, x: usize, y: usize) {
        if let Some(force) = self.value[x].get_force(&self.value[y]) {
            self.value[x].vel += force;
            self.value[y].vel -= force;
        }
    }

    pub fn force_gas(&mut self) {
        for i in 0..(self.system.side - 1) {
            for j in 1..(self.system.side - 1) {
                for coords in corner_coords([i, j]) {
                    // default
                }
            }
        }
        for i in 0..(self.system.side - 1) {
            // j = 0                                                OFFSET_TOP
            // j = self.system.side - 1                             OFFSET_BOTTOM
        }
        // [i, j] = [self.system.side - 1, 0]                       OFFSET_TOP_RIGHT
        // [i, j] = [self.system.side - 1, self.system.side - 1]    OFFSET_BOTTOM_RIGHT
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

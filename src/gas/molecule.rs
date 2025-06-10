use super::*;
use std::f32::consts::PI;

const fn pow_2_over_5(n: u32) -> i32 {
    let base_bits = (2 << FRAC_BITS) / 5;
    let mut result_bits = base_bits;
    let mut i = 1;

    while i < n {
        let product = (result_bits as i64) * (base_bits as i64);
        result_bits = ((product + (1 << (FRAC_BITS - 1))) >> FRAC_BITS) as i32;
        i += 1;
    }
    result_bits
}

const FC_: Fixed = Fixed::from_bits(pow_2_over_5(14) - pow_2_over_5(8)); // -0.00066
const COEF: Fixed = to_fixed(1);
const FC: Fixed = fmulf(COEF, FC_);

pub struct Atom<const R: usize> {
    pub pos: FVec2,
    pub vel: FVec2,
}

impl<const R: usize> Atom<R> {
    pub const RADIUS: Fixed = usize_to_fixed(R);
    pub const DIAMETER: Fixed = fmul(Self::RADIUS, 2);
    const R2: Fixed = fmulf(Self::RADIUS, Self::RADIUS);

    pub const RC: Fixed = Fixed::from_bits((Self::RADIUS.to_bits() * 5) / 2);
    const RC2: Fixed = fmulf(Self::RC, Self::RC);

    pub fn draw(&self) {
        draw_circle(
            self.pos.x.to_num(),
            self.pos.y.to_num(),
            Self::RADIUS.to_num(),
            DARKGREEN,
        );
    }
    pub fn draw_offset(&self, offset: FVec2) {
        let new_pos = self.pos + offset;
        draw_circle(
            new_pos.x.to_num(),
            new_pos.y.to_num(),
            Self::RADIUS.to_num(),
            DARKGREEN,
        );
    }

    pub fn new(pos: FVec2, vel: FVec2) -> Self {
        Atom { pos, vel }
    }

    pub fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    fn diff_to_force(diff: FVec2) -> Option<FVec2> {
        let r2 = flength2(diff);
        if r2 < Self::RC2 {
            let f1 = Self::R2 / r2;
            let f2 = f1 * f1 * f1;
            let df = COEF * f2 * f1 * (f2 - Fixed::ONE) - FC;
            Some(diff * df)
        } else {
            None
        }
    }

    pub fn get_force(&self, other: &Self) -> Option<FVec2> {
        // let diff = ;
        // unsafe {
        //     let a = load_fvec2x2(vectors.as_ptr());
        //     let b = load_fvec2x2(vectors.as_ptr().add(2));

        //     let sum = fvec2x2_add(a, b);
        //     let product = fvec2x2_mul(a, b);
        //     let quotient = fvec2x2_div(a, b);

        //     store_fvec2x2(vectors.as_mut_ptr(), sum);
        // }
        Self::diff_to_force(self.pos - other.pos)
    }

    pub fn get_force_2(&self, other: &Self, offset: FVec2) -> Option<FVec2> {
        Self::diff_to_force(self.pos - (other.pos + offset))
    }

    pub fn generate(side: Fixed, offset: FVec2, sparsity: f32) -> Vec<Self> {
        let dist = Self::DIAMETER * Fixed::from_num(sparsity);
        let side_n = (side / dist).to_num();
        let mut arr = Vec::with_capacity((side_n * side_n) as usize);
        let start = offset + FVec2::new(Self::RADIUS, Self::RADIUS);
        for i in 0..side_n {
            for j in 0..side_n {
                let ampl = (-rand::gen_range::<f32>(0.0, 1.0).ln()).sqrt() / 10.0;
                let angle = rand::gen_range(0.0, 2.0 * PI);
                let vel = fvec2(ampl * angle.cos(), ampl * angle.sin());
                let pos = start + FVec2::new(fmul(dist, i), fmul(dist, j));
                arr.push(Self::new(pos, vel));
            }
        }
        arr
    }
}

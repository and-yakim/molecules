use super::*;

/// +- 1k in f32
pub type Fixed = I11F21;
pub type FVec2 = Vector2<Fixed>;

pub const FRAC_BITS: i32 = 21;

pub const fn to_fixed(value: i32) -> Fixed {
    Fixed::from_bits(value << FRAC_BITS)
}

pub const fn usize_to_fixed(value: usize) -> Fixed {
    to_fixed(value as i32)
}

pub fn fvec2(x: f32, y: f32) -> FVec2 {
    FVec2::new(Fixed::from_num(x), Fixed::from_num(y))
}

pub fn to_fvec2(v: Vec2) -> FVec2 {
    FVec2::new(Fixed::from_num(v.x), Fixed::from_num(v.y))
}

pub fn to_vec2(v: FVec2) -> Vec2 {
    Vec2::new(v.x.to_num(), v.y.to_num())
}

/// for distance below 32.0
/// or Atom<R < 4.525>
pub fn fdistance(a: FVec2, b: FVec2) -> Fixed {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn flength2(v: FVec2) -> Fixed {
    v.x * v.x + v.y * v.y
}

// for Atom<R> constants

pub const fn fadd(a: Fixed, b: i32) -> Fixed {
    let fb = (b as i32) << FRAC_BITS;
    Fixed::from_bits(a.to_bits() + fb)
}

pub const fn fsubf(a: Fixed, b: Fixed) -> Fixed {
    Fixed::from_bits(a.to_bits() - b.to_bits())
}

pub const fn fmul(a: Fixed, b: i32) -> Fixed {
    Fixed::from_bits(a.to_bits() * b)
}

pub const fn fmulf(a: Fixed, b: Fixed) -> Fixed {
    Fixed::from_bits(((a.to_bits() as i64 * b.to_bits() as i64) >> FRAC_BITS) as i32)
}

pub const fn fdiv(a: Fixed, b: i32) -> Fixed {
    Fixed::from_bits((a.to_bits() as i64 / b as i64) as i32)
}

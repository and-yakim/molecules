use super::*;
use std::arch::aarch64::*;

// 128-bit vector containing 2x FVec2 (4x I16F16)
pub type FVec2x2 = int32x4_t;

#[target_feature(enable = "neon")]
pub unsafe fn fvec2x2_add(a: FVec2x2, b: FVec2x2) -> FVec2x2 {
    vaddq_s32(a, b)
}

#[target_feature(enable = "neon")]
pub unsafe fn fvec2x2_mul(a: FVec2x2, b: FVec2x2) -> FVec2x2 {
    let product = vmulq_s32(a, b);
    vshrq_n_s32(product, 16)
}

#[target_feature(enable = "neon")]
pub unsafe fn fvec2x2_div(a: FVec2x2, b: FVec2x2) -> FVec2x2 {
    // Convert to float32x4_t for division
    let a_scaled = vshlq_n_s32(a, 16);
    let a_f32 = vcvtq_f32_s32(a_scaled);
    let b_f32 = vcvtq_f32_s32(b);

    // Fast reciprocal approximation + Newton-Raphson
    let recip_b = vrecpeq_f32(b_f32);
    let recip_b = vmulq_f32(recip_b, vrecpsq_f32(recip_b, b_f32));
    let recip_b = vmulq_f32(recip_b, vrecpsq_f32(recip_b, b_f32));

    // Multiply and convert back
    vcvtq_s32_f32(vmulq_f32(a_f32, recip_b))
}

// Helper functions for loading/storing
#[target_feature(enable = "neon")]
pub unsafe fn load_fvec2x2(ptr: *const I16F16) -> FVec2x2 {
    vld1q_s32(ptr as *const i32)
}

#[target_feature(enable = "neon")]
pub unsafe fn store_fvec2x2(ptr: *mut I16F16, val: FVec2x2) {
    vst1q_s32(ptr as *mut i32, val);
}

/*
let mut vectors = [
    I16F16::from_num(1.5), I16F16::from_num(2.0),  // Vector 1
    I16F16::from_num(3.5), I16F16::from_num(4.0)   // Vector 2
];

unsafe {
    let a = load_fvec2x2(vectors.as_ptr());
    let b = load_fvec2x2(vectors.as_ptr().add(2));

    let sum = fvec2x2_add(a, b);
    let product = fvec2x2_mul(a, b);
    let quotient = fvec2x2_div(a, b);

    store_fvec2x2(vectors.as_mut_ptr(), sum);
}
*/

use std::mem;
use std::ptr;

// 512-bit vector (8x FVec2, 16x I16F16)
#[derive(Clone, Copy)]
pub struct FVec2x8([int32x4_t; 4]);

#[target_feature(enable = "neon")]
pub unsafe fn fvec2x8_add(a: FVec2x8, b: FVec2x8) -> FVec2x8 {
    FVec2x8([
        vaddq_s32(a.0[0], b.0[0]),
        vaddq_s32(a.0[1], b.0[1]),
        vaddq_s32(a.0[2], b.0[2]),
        vaddq_s32(a.0[3], b.0[3]),
    ])
}

#[target_feature(enable = "neon")]
pub unsafe fn fvec2x8_mul(a: FVec2x8, b: FVec2x8) -> FVec2x8 {
    FVec2x8([
        fvec2x2_mul(a.0[0], b.0[0]),
        fvec2x2_mul(a.0[1], b.0[1]),
        fvec2x2_mul(a.0[2], b.0[2]),
        fvec2x2_mul(a.0[3], b.0[3]),
    ])
}

#[target_feature(enable = "neon")]
pub unsafe fn fvec2x8_div(a: FVec2x8, b: FVec2x8) -> FVec2x8 {
    FVec2x8([
        fvec2x2_div(a.0[0], b.0[0]),
        fvec2x2_div(a.0[1], b.0[1]),
        fvec2x2_div(a.0[2], b.0[2]),
        fvec2x2_div(a.0[3], b.0[3]),
    ])
}

// Loading/storing 512-bit vectors
#[target_feature(enable = "neon")]
pub unsafe fn load_fvec2x8(ptr: *const I16F16) -> FVec2x8 {
    FVec2x8([
        vld1q_s32(ptr.add(0) as *const i32),
        vld1q_s32(ptr.add(4) as *const i32),
        vld1q_s32(ptr.add(8) as *const i32),
        vld1q_s32(ptr.add(12) as *const i32),
    ])
}

#[target_feature(enable = "neon")]
pub unsafe fn store_fvec2x8(ptr: *mut I16F16, val: FVec2x8) {
    vst1q_s32(ptr.add(0) as *mut i32, val.0[0]);
    vst1q_s32(ptr.add(4) as *mut i32, val.0[1]);
    vst1q_s32(ptr.add(8) as *mut i32, val.0[2]);
    vst1q_s32(ptr.add(12) as *mut i32, val.0[3]);
}

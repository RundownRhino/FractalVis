pub mod structs;
pub use structs::*;
pub mod utils;

pub mod mandelbrot;

pub mod newton;
pub mod polynomial;

pub mod visualization;

use std::mem;

use mandelbrot::{calculate_mandelbrot, calculate_mandelbrot_colored};
use num_complex::Complex;

type F = f64;
type C = Complex<F>;

#[repr(C)]
pub struct FFIVec<T> {
    pub ptr: *mut T,
    pub len: u64,
    pub cap: u64,
}
impl<T> From<Vec<T>> for FFIVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::new(vec)
    }
}
impl<T> FFIVec<T> {
    pub fn new(mut x: Vec<T>) -> Self {
        let s = Self {
            ptr: x.as_mut_ptr(),
            len: x.len() as u64,
            cap: x.capacity() as u64,
        };
        mem::forget(x);
        s
    }
}

#[no_mangle]
pub extern "C" fn calculate_mandelbrot_vec(
    x_min: F,
    x_max: F,
    y_min: F,
    y_max: F,
    width: u32,
    height: u32,
    max_iters: u32,
    horison: F,
    shades_max: u8,
) -> FFIVec<u8> {
    let arr = calculate_mandelbrot(
        x_min, x_max, y_min, y_max, width, height, max_iters, horison, shades_max,
    );
    arr.into_raw_vec().into()
}

/// Returns an RGB image as a vector.
#[no_mangle]
pub extern "C" fn calculate_mandelbrot_vec_colored(
    x_min: F,
    x_max: F,
    y_min: F,
    y_max: F,
    width: u32,
    height: u32,
    max_iters: u32,
    horison: F,
    from_angle: f32,
    to_angle: f32,
    saturation: f32,
) -> FFIVec<u8> {
    let arr = calculate_mandelbrot_colored(
        x_min, x_max, y_min, y_max, width, height, max_iters, horison, from_angle, to_angle,
        saturation,
    );
    arr.into_raw_vec().into()
}

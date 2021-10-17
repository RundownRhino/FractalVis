pub mod structs;
pub use structs::*;
pub mod utils;

pub mod ffi;
use ffi::*;

pub mod mandelbrot;

pub mod newton;
pub mod polynomial;

pub mod visualization;

use mandelbrot::{calculate_mandelbrot, calculate_mandelbrot_colored};
use num_complex::Complex;

use crate::newton::calculate_newton;

type F = f64;
type C = Complex<F>;

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
        Viewport::new(x_min, x_max, y_min, y_max, width, height),
        max_iters,
        horison,
        shades_max,
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
        Viewport::new(x_min, x_max, y_min, y_max, width, height),
        max_iters,
        horison,
        ColorSettings::new(from_angle, to_angle, saturation),
    );
    arr.into_raw_vec().into()
}

/// Returns an RGB image as a vector.
#[no_mangle]
pub extern "C" fn calculate_newton_roots_of_unity_vec(
    x_min: F,
    x_max: F,
    y_min: F,
    y_max: F,
    width: u32,
    height: u32,

    max_iters: u32,
    horison: F,
    k: u32,

    from_angle: f32,
    to_angle: f32,
    saturation: f32,
) -> FFIVec<u8> {
    let roots: Vec<_> = (0..k)
        .map(|j| {
            Complex::exp(Complex::new(
                0.0,
                j as f64 / k as f64 * std::f64::consts::TAU,
            ))
        })
        .collect();
    let arr = calculate_newton(
        Viewport::new(x_min, x_max, y_min, y_max, width, height),
        ColorSettings::new(from_angle, to_angle, saturation),
        &roots,
        max_iters,
        horison,
    );
    arr.into_raw_vec().into()
}

pub mod utils;

pub mod mandelbrot;

pub mod newton;
pub mod polynomial;

pub mod visualization;

use std::mem;

use mandelbrot::{calculate_mandelbrot, calculate_mandelbrot_colored};
use num_complex::Complex;
use palette::{encoding::Srgb, Hsv, LinSrgb};

type F = f64;
type C = Complex<F>;

#[derive(Clone, Copy)]
pub struct Viewport {
    x_min: F,
    x_max: F,
    y_min: F,
    y_max: F,
    width: u32,
    height: u32,
}
impl Viewport {
    pub fn new(x_min: F, x_max: F, y_min: F, y_max: F, width: u32, height: u32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
        }
    }

    fn transform(&self, i: usize, j: usize) -> (F, F) {
        let y = self.y_min + (i as F) * ((self.y_max - self.y_min) as F / self.height as F);
        let x = self.x_min + (j as F) * ((self.x_max - self.x_min) as F / self.width as F);
        (x, y)
    }
}

#[derive(Clone, Copy)]
pub struct ColorSettings {
    from_angle: f32,
    to_angle: f32,
    saturation: f32,
}
impl ColorSettings {
    pub fn new(from_angle: f32, to_angle: f32, saturation: f32) -> Self {
        Self {
            from_angle,
            to_angle,
            saturation,
        }
    }

    /// val: from 0.0 to 1.0
    /// Return value: hue in degrees, from 0.0 to 360.0
    fn hue_in_degs(&self, val: f32) -> f32 {
        (val * (self.to_angle - self.from_angle) + self.from_angle) % 360f32
    }
    fn color_from(&self, val: f32) -> LinSrgb<u8> {
        let color: f32 = self.hue_in_degs(val);
        let pxl: Hsv<Srgb, f32> = Hsv::from_components((color, self.saturation, 1f32));
        let float_rgb = LinSrgb::from(pxl);
        let rgb: LinSrgb<u8> = float_rgb.into_format();
        rgb
    }
}

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

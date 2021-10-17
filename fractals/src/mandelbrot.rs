use crate::{C, F};

use ndarray::{Array2, Array3};
use ndarray::{Axis, Zip};
use num_complex::Complex;
use palette::{encoding::Srgb, Hsv, LinSrgb};

#[inline]
fn mandelbrot_point(x: F, y: F, max_iters: u32, horison_sq: F) -> (bool, u32) {
    let c: C = Complex::new(x, y);
    let mut z: C = Complex::default();
    let mut converged = true;
    let mut iter: u32 = 0;
    while iter < max_iters {
        z = z.powi(2) + c;
        if z.norm_sqr() >= horison_sq {
            converged = false;
            break;
        }
        iter += 1;
    }
    (converged, iter)
}

pub fn calculate_mandelbrot(
    x_min: F,
    x_max: F,
    y_min: F,
    y_max: F,
    width: u32,
    height: u32,
    max_iters: u32,
    horison: F,
    shades_max: u8,
) -> Array2<u8> {
    assert!(shades_max >= 1);
    let horison_sq = horison.powi(2);
    let fun = |(i, j), el: &mut u8| {
        let y = y_min + (i as F) * ((y_max - y_min) as F / height as F);
        let x = x_min + (j as F) * ((x_max - x_min) as F / width as F);
        let (converged, iter) = mandelbrot_point(x, y, max_iters, horison_sq);
        if !converged {
            let color: u8 = (iter as u8 % shades_max) * (255 / shades_max);
            *el = color;
        }
    };
    let mut img: Array2<u8> = Array2::zeros((height as usize, width as usize));
    Zip::indexed(&mut img).par_for_each(fun);
    img
}

/// Returns an RGB image array, converted from colors made in HSV.
/// from_angle and to_angle determine the span of hue as iterations to
/// divergence change from 0 to max_iters.
pub fn calculate_mandelbrot_colored(
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
) -> Array3<u8> {
    assert!((0.0..=1.0f32).contains(&saturation));
    let horison_sq = horison.powi(2);
    let mut img: Array3<u8> = Array3::zeros((height as usize, width as usize, 3));
    Zip::indexed(img.lanes_mut(Axis(2))).par_for_each(|(i, j), mut el| {
        let y = y_min + (i as F) * ((y_max - y_min) as F / height as F);
        let x = x_min + (j as F) * ((x_max - x_min) as F / width as F);
        let (converged, iter) = mandelbrot_point(x, y, max_iters, horison_sq);
        if !converged {
            let color: f32 =
                ((iter as f32 / max_iters as f32) * (to_angle - from_angle) + from_angle) % 360f32; // hue in degrees
            let pxl: Hsv<Srgb, f32> = Hsv::from_components((color, saturation, 1f32));
            let float_rgb = LinSrgb::from(pxl);
            let rgb: LinSrgb<u8> = float_rgb.into_format();
            el[0] = rgb.red;
            el[1] = rgb.green;
            el[2] = rgb.blue;
        }
    });
    img
}

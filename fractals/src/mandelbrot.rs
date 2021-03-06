use crate::{ColorSettings, Viewport, C, F};

use ndarray::{Array2, Array3};
use ndarray::{Axis, Zip};

#[inline]
fn mandelbrot_point(x: F, y: F, max_iters: u32, horison_sq: F) -> (bool, u32) {
    let c: C = C::new(x, y);
    let mut z: C = C::default();
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
    viewport: Viewport,
    max_iters: u32,
    horison: F,
    shades_max: u8,
) -> Array2<u8> {
    assert!(shades_max >= 1);
    let horison_sq = horison.powi(2);
    let fun = |(i, j), el: &mut u8| {
        let (x, y) = viewport.transform(i, j);
        let (converged, iter) = mandelbrot_point(x, y, max_iters, horison_sq);
        if !converged {
            let color: u8 = (iter as u8 % shades_max) * (255 / shades_max);
            *el = color;
        }
    };
    let mut img: Array2<u8> = Array2::zeros((viewport.height as usize, viewport.width as usize));
    Zip::indexed(&mut img).par_for_each(fun);
    img
}

/// Returns an RGB image array, converted from colors made in HSV.
/// from_angle and to_angle determine the span of hue as iterations to
/// divergence change from 0 to max_iters.
pub fn calculate_mandelbrot_colored(
    viewport: Viewport,
    max_iters: u32,
    horison: F,
    color_settings: ColorSettings,
) -> Array3<u8> {
    assert!((0.0..=1.0f32).contains(&color_settings.saturation));
    let horison_sq = horison.powi(2);
    let mut img: Array3<u8> = Array3::zeros((viewport.height as usize, viewport.width as usize, 3));
    Zip::indexed(img.lanes_mut(Axis(2))).par_for_each(|(i, j), mut el| {
        let (x, y) = viewport.transform(i, j);
        let (converged, iter) = mandelbrot_point(x, y, max_iters, horison_sq);
        if !converged {
            let rgb = color_settings.color_from(iter as f32 / max_iters as f32);
            el[0] = rgb.red;
            el[1] = rgb.green;
            el[2] = rgb.blue;
        }
    });
    img
}

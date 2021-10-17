use crate::polynomial::Polynomial;
use crate::{ColorSettings, Viewport, C, F};
use palette::LinSrgb;

use ndarray::{Array3, Axis, Zip};

#[inline]
/// Performs up to max_iters iterations of the Newton's method on the polynomial, starting from z,
/// and returns the root the algorithm converged to, if any.
/// horison_sq is the minimal squared distance to a root to consider converged.
fn newton_point(
    poly: &Polynomial<C>,
    derivative: &Polynomial<C>,
    roots: &[C],
    z: C,
    max_iters: u32,
    horison_sq: F,
) -> Option<usize> {
    let mut cur = z;
    for _iter in 0..max_iters {
        let (root_i, _root_z, dist) = roots
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x, (x - cur).norm_sqr()))
            .min_by(|(_, _, dist1), (_, _, dist2)| dist1.partial_cmp(dist2).unwrap())
            .unwrap();
        if dist < horison_sq {
            return Some(root_i);
        }
        cur = cur - poly.evaluate(cur) / derivative.evaluate(cur);
        if cur.is_nan() {
            return None;
        }
    }
    None
}
pub fn root_color(roots: &[C], root_i: usize, color_settings: ColorSettings) -> LinSrgb<u8> {
    color_settings.color_from((root_i + 1) as f32 / roots.len() as f32)
}

pub fn calculate_newton(
    viewport: Viewport,
    color_settings: ColorSettings,
    roots: &[C],
    max_iters: u32,
    horison: F,
) -> Array3<u8> {
    let horison_sq = horison.powi(2);
    let poly = Polynomial::from_roots(roots);
    let derivative = poly.derivative();

    let mut img: Array3<u8> = Array3::zeros((viewport.height as usize, viewport.width as usize, 3));
    Zip::indexed(img.lanes_mut(Axis(2))).par_for_each(|(i, j), mut el| {
        let (x, y) = viewport.transform(i, j);
        let root = newton_point(
            &poly,
            &derivative,
            roots,
            C::new(x, y),
            max_iters,
            horison_sq,
        );
        let rgb = match root {
            Some(root_i) => root_color(roots, root_i, color_settings),
            None => LinSrgb::from_components((0, 0, 0)),
        };
        el[0] = rgb.red;
        el[1] = rgb.green;
        el[2] = rgb.blue;
    });
    img
}

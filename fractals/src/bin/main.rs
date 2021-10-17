use fractal_calculator::{newton::calculate_newton, visualization::array_to_image_rgb};
use fractal_calculator::{ColorSettings, Viewport};
use num_complex::Complex;

use std::time::Instant;

fn main() {
    let x_min = -2.0;
    let x_max = 2.0;
    let y_min = -2.0;
    let y_max = 2.0;
    let size_exp = 13;
    let width: u32 = 2u32.pow(size_exp); // 5s on debug for 10, 38s on release for 14.
    let height: u32 = width;
    let max_iters = 100;
    let horison = 1e-6;
    let from_angle = 0f32;
    let to_angle = 300f32; //magenta
    let saturation = 1.0f32;

    let k = 7;
    let roots: Vec<_> = (0..k)
        .map(|j| {
            Complex::exp(Complex::new(
                0.0,
                j as f64 / k as f64 * std::f64::consts::TAU,
            ))
        })
        .collect();
    //println!("{:?}", roots);
    let out_path = format!(r"output/newton/newton_colored_k{},size{}.png", k, size_exp);

    let start = Instant::now();
    let arr = calculate_newton(
        Viewport::new(x_min, x_max, y_min, y_max, width, height),
        ColorSettings::new(to_angle, from_angle, saturation),
        &roots,
        max_iters,
        horison,
    );
    println!(
        "Time for calculating fractal: {}s",
        start.elapsed().as_secs_f32()
    );
    let start2 = Instant::now();
    let img = array_to_image_rgb(arr);
    println!(
        "Time for converting to image: {}s",
        start2.elapsed().as_secs_f32()
    );
    let start3 = Instant::now();
    img.save_with_format(out_path, image::ImageFormat::Png)
        .unwrap();
    println!(
        "Time for saving to file: {}s",
        start3.elapsed().as_secs_f32()
    );
    println!("Total time:{}s", start.elapsed().as_secs_f32());
}

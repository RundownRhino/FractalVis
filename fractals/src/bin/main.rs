use fractal_calculator::{
    mandelbrot::calculate_mandelbrot_colored, visualization::array_to_image_rgb,
};

use std::time::Instant;

fn main() {
    let x_min = -2.0;
    let x_max = 1.0;
    let y_min = -2.0;
    let y_max = 2.0;
    let width: u32 = 2u32.pow(13);
    let height: u32 = width;
    let max_iters = 100;
    let horison = 2.0;
    let from_angle = 0f32;
    let to_angle = 300f32; //magenta
    let saturation = 1.0f32;

    let start = Instant::now();
    let arr = calculate_mandelbrot_colored(
        x_min, x_max, y_min, y_max, width, height, max_iters, horison, from_angle, to_angle,
        saturation,
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
    img.save_with_format(r"output/mandelbrot_colored.png", image::ImageFormat::Png)
        .unwrap();
    println!(
        "Time for saving to file: {}s",
        start3.elapsed().as_secs_f32()
    );
    println!("Total time:{}s", start.elapsed().as_secs_f32());
}

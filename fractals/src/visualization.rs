use image::{GrayImage, RgbImage, RgbaImage};
use ndarray::{self, Array2, Array3};

pub fn array_to_image(arr: Array3<u8>) -> RgbImage {
    //https://stackoverflow.com/questions/56762026/how-to-save-ndarray-in-rust-as-image
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}
pub fn array_to_image_rgba(arr: Array3<u8>) -> RgbaImage {
    assert!(arr.is_standard_layout());

    let (height, width, chan) = arr.dim();
    assert_eq!(chan, 4);
    let raw = arr.into_raw_vec();

    RgbaImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}
pub fn array_to_image_rgb(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, chan) = arr.dim();
    assert_eq!(chan, 3);
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}
pub fn array_to_grayscale_image(arr: Array2<u8>) -> GrayImage {
    assert!(arr.is_standard_layout());

    let (height, width) = arr.dim();
    let raw = arr.into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

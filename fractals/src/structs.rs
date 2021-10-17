use palette::{encoding::Srgb, Hsv, LinSrgb};

use crate::F;

#[derive(Clone, Copy)]
pub struct Viewport {
    pub x_min: F,
    pub x_max: F,
    pub y_min: F,
    pub y_max: F,
    pub width: u32,
    pub height: u32,
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

    pub fn transform(&self, i: usize, j: usize) -> (F, F) {
        let y = self.y_min + (i as F) * ((self.y_max - self.y_min) as F / self.height as F);
        let x = self.x_min + (j as F) * ((self.x_max - self.x_min) as F / self.width as F);
        (x, y)
    }
}

#[derive(Clone, Copy)]
pub struct ColorSettings {
    pub from_angle: f32,
    pub to_angle: f32,
    pub saturation: f32,
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
    pub fn hue_in_degs(&self, val: f32) -> f32 {
        (val * (self.to_angle - self.from_angle) + self.from_angle) % 360f32
    }
    pub fn color_from(&self, val: f32) -> LinSrgb<u8> {
        let color: f32 = self.hue_in_degs(val);
        let pxl: Hsv<Srgb, f32> = Hsv::from_components((color, self.saturation, 1f32));
        let float_rgb = LinSrgb::from(pxl);
        let rgb: LinSrgb<u8> = float_rgb.into_format();
        rgb
    }
}

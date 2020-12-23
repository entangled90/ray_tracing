use std::io::Write;
use std::iter::Sum;
use std::ops::Add;

use super::geom::*;

#[derive(Clone, Debug)]
pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Color {
        Color::new(Vec3::new(r, g, b))
    }
    pub fn new(vec: Vec3) -> Color {
        Color { rgb: vec }
    }
    pub fn zero() -> Color {
        Color::new(Vec3::iso(0.0))
    }
    pub fn write<W>(&self, w: &mut W, scale: f32) -> std::io::Result<()>
    where
        W: Write,
    {
        let r = (scale * self.rgb.x).sqrt();
        let g = (scale * self.rgb.y).sqrt();
        let b = (scale * self.rgb.z).sqrt();
        w.write_fmt(format_args!(
            "{} {} {}\n",
            Color::scale_color_component(r),
            Color::scale_color_component(g),
            Color::scale_color_component(b)
        ))?;
        Ok(())
    }

    fn scale_color_component(c: f32) -> u8 {
        (256.0 * Color::clamp_color(c)) as u8
    }

    fn clamp_color(x: f32) -> f32 {
        Color::clamp(x, 0.0, 0.999)
    }

    fn clamp(x: f32, min: f32, max: f32) -> f32 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, w: Color) -> Color {
        Color::new(&self.rgb + &w.rgb)
    }
}

impl Sum for Color {
    fn sum<I>(iter: I) -> Color
    where
        I: Iterator<Item = Color>,
    {
        let mut zero = Vec3::iso(0.0);
        for el in iter {
            zero += el.rgb;
        }
        Color::new(zero)
    }
}

mod ray_tracing;

use std::io::{stderr, stdout, Stdout, Write};

use std::error::Error;
use std::result::Result;

use crate::ray_tracing::geom::*;
use crate::ray_tracing::ray::*;

const aspect_ratio: f64 = 16.0 / 9.0;
const image_width: u32 = 400;
const image_heigth: u32 = (image_width as f64 / aspect_ratio) as u32;

const viewport_height: f64 = 2.0;
const viewport_width: f64 = aspect_ratio * viewport_height;
const focal_length: f64 = 1.0;


const horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
const vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let mut out_handle = stdout.lock();
    let stderr = stderr();
    let mut err_handle = stderr.lock();
    let origin: Point = Point(Vec3::new(0.0, 0.0, 0.0));

    let lower_left_corner = origin.0.clone()
        - horizontal.scalar_div(2.0)
        - vertical.scalar_div(2.0)
        - Vec3::new(0.0, 0.0, focal_length);

    out_handle.write(format!("P3\n{} {}\n{}\n", image_width, image_heigth, 255).as_bytes())?;
    for j in (0..image_heigth).rev() {
        err_handle.write_fmt(format_args!("Scanlines remaining: {}\n", j))?;
        for i in 0..image_width {
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_heigth as f64 - 1.0);
            let r = Ray::new(
                origin.clone(),
                Point(
                    &(&(&lower_left_corner + &horizontal.scalar_mul(u)) + &vertical.scalar_mul(v)) - &origin.0,
                ),
            );
            r.color().write(&mut out_handle)?;
        }
    }
    err_handle.write("Done!\n".as_bytes())?;
    Ok(())
}


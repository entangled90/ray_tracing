mod ray_tracing;

use std::io::{stderr, stdout, Write};

use std::error::Error;
use std::result::Result;

use crate::ray_tracing::geom::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::rand::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGTH: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let mut out_handle = stdout.lock();
    let stderr = stderr();
    let mut err_handle = stderr.lock();
    let origin: Point = Point(Vec3::new(0.0, 0.0, 0.0));
    let random = Random::new();

    let lower_left_corner = origin.0.clone()
        - HORIZONTAL.scalar_div(2.0)
        - VERTICAL.scalar_div(2.0)
        - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    out_handle.write_all(format!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGTH, 255).as_bytes())?;
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point(Vec3::new(0.0, 0.0, -1.0)),
        radius: 0.5,
    }));

    world.add(Box::new(Sphere {
        center: Point(Vec3::new(0.0, -100.5, -1.0)),
        radius: 100.0,
    }));


    for j in (0..IMAGE_HEIGTH).rev() {
        err_handle.write_fmt(format_args!("Scanlines remaining: {}\n", j))?;
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let v = (j as f64) / (IMAGE_HEIGTH as f64 - 1.0);
            let r = Ray::new(
                origin.clone(),
                Point(
                    &(&(&lower_left_corner + &HORIZONTAL.scalar_mul(u)) + &VERTICAL.scalar_mul(v))
                        - &origin.0,
                ),
            );
            r.color(&world).write(&mut out_handle)?;
        }
    }
    err_handle.write_all("Done!\n".as_bytes())?;
    Ok(())
}

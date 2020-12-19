mod ray_tracing;

use std::io::{stderr, stdout, Write};
use std::rc::Rc;

use std::error::Error;
use std::result::Result;

use crate::ray_tracing::camera::*;
use crate::ray_tracing::geom::*;
use crate::ray_tracing::rand::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::material::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: f64 = 400f64;
const IMAGE_HEIGTH: f64 = IMAGE_WIDTH / ASPECT_RATIO;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let mut out_handle = stdout.lock();
    let stderr = stderr();
    let mut err_handle = stderr.lock();
    let mut random = Random::new();
    let samples_per_pixel = 100u32;
    let max_depth = 50;
    let camera = Camera::new();
    out_handle.write_all(format!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGTH, 255).as_bytes())?;

    let material_ground: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new_rgb(0.8, 0.8, 0.0))));
    let material_center: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new_rgb(0.7, 0.3, 0.3))));
    let material_left: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Color::new_rgb(0.8, 0.8, 0.8), 0.3)));
    let material_right: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Color::new_rgb(0.8, 0.6, 0.2), 1.0)));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point(Vec3::new(0.0, 0.0, -1.0)),
        radius: 0.5,
        material: material_center.clone(),
    }));

    world.add(Box::new(Sphere {
        center: Point(Vec3::new(0.0, -100.5, -1.0)),
        radius: 100.0,
        material: material_ground.clone(),
    }));

    world.add(Box::new(Sphere {
        center: Point(Vec3::new(-1.0, 0.0, -1.0)),
        radius: 0.5,
        material: material_left.clone(),
    }));
    world.add(Box::new(Sphere {
        center: Point(Vec3::new(1.0, 0.0, -1.0)),
        radius: 0.5,
        material: material_right.clone(),
    }));

    let inverse_height = 1.0 / (IMAGE_HEIGTH - 1.0);
    let inverse_width = 1.0 / (IMAGE_WIDTH - 1.0);
    for j in (0..IMAGE_HEIGTH as u32).rev() {
        err_handle.write_fmt(format_args!("Scanlines remaining: {}\n", j))?;
        for i in 0..IMAGE_WIDTH as u32 {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random.random_double()) * inverse_width;
                let v = (j as f64 + random.random_double()) * inverse_height;
                let ray = camera.ray(u, v);
                color.rgb += ray.color(&world, max_depth, &mut random).rgb;
            }
            color.write(&mut out_handle, samples_per_pixel)?
        }
    }
    err_handle.write_all("Done!\n".as_bytes())?;
    Ok(())
}

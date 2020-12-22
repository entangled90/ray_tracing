mod ray_tracing;

use std::io::{stderr, stdout, Write};

use std::error::Error;
use std::result::Result;

use crate::ray_tracing::camera::*;
use crate::ray_tracing::geom::*;
use crate::ray_tracing::material::*;
use crate::ray_tracing::object::*;
use crate::ray_tracing::rand::*;
use crate::ray_tracing::ray::*;
use crate::ray_tracing::color::*;

use rayon::prelude::*;

const ASPECT_RATIO: f32 = 3.0 / 2.0;
const IMAGE_WIDTH: f32 = 1200f32;
const IMAGE_HEIGTH: f32 = IMAGE_WIDTH / ASPECT_RATIO;

fn init_camera() -> Camera {
    let look_from = Point(Vec3::new(13.0, 2.0, 3.0));
    let look_at = Point(Vec3::new(0.0, 0.0, 0.0));
    let view_up = Point(Vec3::new(0.0, 1.0, 0.0));
    let focus_dist = 10.0;
    let aperture = 0.01;
    Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    )
}

fn random_world() -> HittableList {
    let mut world = HittableList::new();
    let mut random = Random::default();
    let material_ground = Material::new_lambertian(Color::new_rgb(0.5, 0.5, 0.5));
    world.add(Object::Sphere {
        center: Point(Vec3::new(0.0, -1000.0, 0.0)),
        radius: 1000.0,
        material: material_ground,
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random.random_double();
            let center = Point(Vec3::new(
                a as f32 + 0.9 * random.random_double(),
                0.2,
                b as f32 + 0.9 * random.random_double(),
            ));

            if (&center.0 - &Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Material = match choose_mat {
                    //diffuse
                    m if m < 0.8 => {
                        let albedo =
                            Vec3::random(&mut random).index_wise_mul(&Vec3::random(&mut random));
                        Material::new_lambertian(Color::new(albedo))
                    }
                    // metal
                    m if m < 0.95 => {
                        let albedo = Vec3::random_in(&mut random, 0.5, 1.0);
                        let fuzz = random.random_double();
                        Material::new_metal(Color::new(albedo), fuzz)
                    }
                    _ => Material::new_dielectric(1.5),
                };
                world.add(Object::Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }

    world.add(Object::Sphere {
        center: Point(Vec3::new(0.0, 1.0, 0.0)),
        radius: 1.0,
        material: Material::new_dielectric(1.5),
    });
    world.add(Object::Sphere {
        center: Point(Vec3::new(-4.0, 1.0, 0.0)),
        radius: 1.0,
        material: Material::new_lambertian(Color::new(Vec3::new(0.4, 0.2, 0.1))),
    });
    world.add(Object::Sphere {
        center: Point(Vec3::new(4.0, 1.0, 0.0)),
        radius: 1.0,
        material: Material::new_metal(Color::new(Vec3::new(0.7, 0.6, 0.5)), 0.0),
    });

    world
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let mut out_handle = stdout.lock();
    let stderr = stderr();
    let mut err_handle = stderr.lock();
    let samples_per_pixel = 500u32;
    let samples_per_pixel_f = samples_per_pixel as f32;

    let max_depth = 50;
    let camera = init_camera();
    // let mut random = Random::default();

    out_handle.write_all(format!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGTH, 255).as_bytes())?;

    let world = random_world();
    let inverse_height = 1.0 / (IMAGE_HEIGTH - 1.0);
    let inverse_width = 1.0 / (IMAGE_WIDTH - 1.0);
    let colors_matrix: Vec<Vec<Color>> = (0..IMAGE_HEIGTH as u32)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut random = Random::default();
            // err_handle
            //     .write_fmt(format_args!("Scanlines remaining: {}\n", j))
            //     .unwrap();
            (0..IMAGE_WIDTH as u32).map(|i| {
                (0..samples_per_pixel)
                    .map(|_| {
                        let u = (i as f32 + (random.random_double())) * inverse_width;
                        let v = (j as f32 + (random.random_double())) * inverse_height;
                        let ray = camera.ray(u, v, &mut random);
                        ray.color(&world, max_depth, &mut random)
                    })
                    .sum()
            }).collect()
        })
        .collect();

    let scale = 1.0 / samples_per_pixel_f;

    for colors in colors_matrix.iter() {
        for color in colors {
            color.write(&mut out_handle, scale)?
        }
    }
    err_handle.write_all(b"Done!\n")?;
    Ok(())
}

mod ray_tracing;

use std::io::{stderr, stdout, Write};
use std::rc::Rc;

use std::error::Error;
use std::result::Result;

use crate::ray_tracing::camera::*;
use crate::ray_tracing::geom::*;
use crate::ray_tracing::material::*;
use crate::ray_tracing::rand::*;
use crate::ray_tracing::ray::*;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: f64 = 1200f64;
const IMAGE_HEIGTH: f64 = IMAGE_WIDTH / ASPECT_RATIO;

fn init_camera() -> Camera {
    let look_from = Point(Vec3::new(13.0, 2.0, 3.0));
    let look_at = Point(Vec3::new(0.0, 0.0, 0.0));
    let view_up = Point(Vec3::new(0.0, 1.0, 0.0));
    let focus_dist = 10.0;
    let aperture = 0.01;
    let random = Random::default();
    Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_dist,
        random,
    )
}

fn random_world() -> HittableList {
    let mut world = HittableList::new();
    let mut random = Random::default();
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5)));
    // let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new_rgb(0.1, 0.2, 0.5)));
    // let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    // let material_right: Rc<dyn Material> = Rc::new(Metal::new(Color::new_rgb(0.8, 0.6, 0.2), 0.0));
    world.add(Box::new(Sphere {
        center: Point(Vec3::new(0.0, -1000.0, 0.0)),
        radius: 1000.0,
        material: material_ground,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random.random_double();
            let center = Point(Vec3::new(
                a as f64 + 0.9 * random.random_double(),
                0.2,
                b as f64 + 0.9 * random.random_double(),
            ));

            if (&center.0 - &Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<dyn Material> = match choose_mat {
                    //diffuse
                    m if m < 0.8 => {
                        let albedo = Vec3::random(&mut random).index_wise_mul(&Vec3::random(&mut random));
                        Rc::new(Lambertian::new(Color::new(albedo)))
                    }
                    // metal
                    m if m < 0.95 => {
                        let albedo = Vec3::random_in(&mut random, 0.5, 1.0);
                        let fuzz = random.random_double();
                        Rc::new(Metal::new(Color::new(albedo), fuzz))
                    }
                    _ => Rc::new(Dielectric::new(1.5)),
                };
                world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }));
            }
        }
    }

    world.add(Box::new(Sphere {
        center: Point(Vec3::new(0.0, 1.0, 0.0)),
        radius: 1.0,
        material: Rc::new(Dielectric::new(1.5)),
    }));
    world.add(Box::new(Sphere {
        center: Point(Vec3::new(-4.0, 1.0,0.0)),
        radius: 1.0,
        material: Rc::new(Lambertian::new(Color::new(Vec3::new(0.4, 0.2, 0.1)))),
    }));
    world.add(Box::new(Sphere {
        center: Point(Vec3::new(4.0, 1.0, 0.0)),
        radius: 1.0,
        material: Rc::new(Metal::new(Color::new(Vec3::new(0.7, 0.6, 0.5)), 0.0)),
    }));

    world
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let mut out_handle = stdout.lock();
    let stderr = stderr();
    let mut err_handle = stderr.lock();
    let samples_per_pixel = 100u32;
    let samples_per_pixel_f = samples_per_pixel as f64;

    let max_depth = 50;
    let mut camera = init_camera();
    let mut random = Random::default();

    out_handle.write_all(format!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGTH, 255).as_bytes())?;

    let world = random_world();

    let inverse_height = 1.0 / (IMAGE_HEIGTH - 1.0);
    let inverse_width = 1.0 / (IMAGE_WIDTH - 1.0);
    for j in (0..IMAGE_HEIGTH as u32).rev() {
        err_handle.write_fmt(format_args!("Scanlines remaining: {}\n", j))?;
        for i in 0..IMAGE_WIDTH as u32 {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + (random.random_double())) * inverse_width;
                let v = (j as f64 + (random.random_double())) * inverse_height;
                let ray = camera.ray(u, v);
                color.rgb += ray.color(&world, max_depth, &mut random).rgb;
            }
            color.write(&mut out_handle, samples_per_pixel_f)?
        }
    }
    err_handle.write_all(b"Done!\n")?;
    Ok(())
}

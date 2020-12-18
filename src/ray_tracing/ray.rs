use std::io::Write;

use super::geom::*;

pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn new(vec: Vec3) -> Color {
        Color { rgb: vec }
    }
    pub fn zero() -> Color {
        Color::new(Vec3::iso(0.0))
    }
    pub fn write<W>(&self, w: &mut W, samples_per_pixel: u32) -> std::io::Result<()>
    where
        W: Write,
    {
        let scale = 1.0 / samples_per_pixel as f64;

        w.write_fmt(format_args!(
            "{} {} {}\n",
           (256.0 * Color::clamp_color(self.rgb.x * scale)) as u32,
           (256.0 * Color::clamp_color(self.rgb.y * scale)) as u32,
           (256.0 * Color::clamp_color(self.rgb.z * scale)) as u32
        ))?;
        Ok(())
    }

    fn clamp_color(x: f64) -> f64 {
        Color::clamp(x, 0.0, 0.999)
    }

    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Ray {
    pub origin: &'static Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: &'static Point, direction: Point) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        return Point(&self.origin.0 + &self.direction.0.scalar_mul(t));
    }

    pub fn color(&self, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(&self, 0.0, INFINITY) {
            Color {
                rgb: (rec.normal.0 + (Vec3::iso(1.0))).scalar_mul(0.5),
            }
        } else {
            let t = 0.5 * (self.direction.0.unit_norm().y + 1.0);
            Color {
                rgb: Ray::VEC_ISO_1.scalar_mul(1.0 - t) + Ray::VEC_COLOR.scalar_mul(t),
            }
        }
    }

    const VEC_COLOR: Vec3 = Vec3::new(0.5, 0.7, 1.0);
    const VEC_ISO_1: Vec3 = Vec3::new(1.0, 1.0, 1.0);
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub t: f64,
    // pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point, t: f64, outward_normal: Point, ray: &Ray) -> HitRecord {
        let front_face = HitRecord::is_front_face(&outward_normal, ray);
        let normal = if front_face {
            outward_normal.clone()
        } else {
            Point(-&outward_normal.0)
        };
        HitRecord {
            p,
            normal,
            t,
            // front_face
        }
    }

    // static
    fn is_front_face(outward_normal: &Point, ray: &Ray) -> bool {
        ray.direction.0.dot(&outward_normal.0) < 0.0
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittables: Vec::with_capacity(64),
        }
    }
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.hittables {
            // note closest_so_far is used as t_max
            match object.hit(ray, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    temp_rec = Some(rec);
                }
                None => (),
            }
        }
        temp_rec
    }
}

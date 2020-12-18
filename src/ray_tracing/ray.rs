use std::io::Write;

use super::geom::*;

pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        w.write_fmt(format_args!(
            "{} {} {}\n",
            (255.999 * self.rgb.x) as u32,
            (255.999 * self.rgb.y) as u32,
            (255.999 * self.rgb.z) as u32
        ))?;
        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Ray {
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

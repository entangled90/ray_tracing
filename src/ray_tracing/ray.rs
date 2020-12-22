
use crate::Object;

use super::geom::*;
use super::material::*;
use super::rand::*;
use super::color::*;




#[derive(PartialEq, Debug, Clone)]
pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: Point,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: Point) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        Point(&self.origin.0 + &self.direction.0.scalar_mul(t))
    }

    pub fn color(&self, world: &HittableList, depth: u32, r: &mut Random) -> Color {
        if depth == 0 {
            Color::zero()
        } else if let Some(rec) = world.hit(&self, 0.001, INFINITY) {
            match rec.material.scatter(self, &rec, r) {
                Some((color, ray_out)) => Color::new(
                    ray_out
                        .color(world, depth - 1, r)
                        .rgb
                        .index_wise_mul(&color.rgb),
                ),
                None => Color::zero(),
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

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Point,
    pub material: &'a Material,
    pub t: f32,
    pub front_face: bool,
}

impl <'a> HitRecord<'a> {
    pub fn new(
        p: Point,
        t: f32,
        outward_normal: Point,
        material: &'a Material,
        ray: &Ray,
    ) -> HitRecord<'a> {
        let front_face = HitRecord::is_front_face(&outward_normal, ray);
        let normal = if front_face {
            outward_normal
        } else {
            Point(-&outward_normal.0)
        };
        HitRecord{
            p,
            normal,
            t,
            material,
            front_face,
        }
    }

    // static
    fn is_front_face(outward_normal: &Point, ray: &Ray) -> bool {
        ray.direction.0.dot(&outward_normal.0) < 0.0
    }
}


pub struct HittableList {
    pub hittables: Vec<Object>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittables: Vec::with_capacity(64),
        }
    }
    pub fn add(&mut self, hittable: Object) {
        self.hittables.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.hittables {
            // note closest_so_far is used as t_max
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}

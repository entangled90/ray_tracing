use std::iter::Sum;
use crate::Object;
use std::ops::Add;
use std::io::Write;

use super::geom::*;
use super::material::*;
use super::rand::*;

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

impl Sum for Color{
    fn sum<I>(iter: I) -> Color 
    where I: Iterator<Item = Color> { 
        let mut zero = Vec3::iso(0.0);
        for el in iter{
            zero += el.rgb;
        }
        Color::new(zero)
 }
}


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

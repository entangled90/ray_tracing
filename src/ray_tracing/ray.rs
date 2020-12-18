use std::io::Write;
use std::rc::Rc;

use super::geom::*;
use super::rand::*;

pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Color {
        Color::new(Vec3::new(r, g, b))
    }
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

    fn scale_color_component(c: f64) -> u8 {
        (256.0 * Color::clamp_color(c)) as u8
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
pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: Point,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: Point) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        return Point(&self.origin.0 + &self.direction.0.scalar_mul(t));
    }

    pub fn color(&self, world: &HittableList, depth: u32, r: &mut Random) -> Color {
        if depth <= 0 {
            Color::zero()
        } else {
            if let Some(rec) = world.hit(&self, 0.001, INFINITY) {
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
    }

    const VEC_COLOR: Vec3 = Vec3::new(0.5, 0.7, 1.0);
    const VEC_ISO_1: Vec3 = Vec3::new(1.0, 1.0, 1.0);
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub material: Rc<Box<dyn Material>>,
    pub t: f64,
    // pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point,
        t: f64,
        outward_normal: Point,
        material: Rc<Box<dyn Material>>,
        ray: &Ray,
    ) -> HitRecord {
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
            material, // front_face
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

pub trait Material {
    fn scatter<'a>(
        &self,
        ray_in: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Rc<Color>, Ray<'a>)>;
}

pub struct Lambertian {
    albedo: Rc<Color>,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo: Rc::new(albedo),
        }
    }
}
impl Material for Lambertian {
    fn scatter<'a>(
        &self,
        _: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Rc<Color>, Ray<'a>)> {
        let mut scatter_direction = &hit_record.normal.0 + &Vec3::random_unit_vector(r);
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal.0.clone();
        }
        Some((
            self.albedo.clone(),
            Ray::new(&hit_record.p, Point(scatter_direction)),
        ))
    }
}

pub struct Metal {
    albedo: Rc<Color>,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal {
            albedo: Rc::new(albedo),
        }
    }
}

impl Material for Metal {
    fn scatter<'a>(
        &self,
        ray_in: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Rc<Color>, Ray<'a>)> {
        let mut reflected = ray_in.direction.0.unit_norm().reflect(&hit_record.normal.0);
        let ray_out = Ray::new(&hit_record.p, Point(reflected));
        if ray_out.direction.0.dot(&hit_record.normal.0) > 0.0 {
            Some((self.albedo.clone(), ray_out))
        } else {
            None
        }
    }
}

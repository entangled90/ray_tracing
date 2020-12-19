use std::rc::Rc;
use super::ray::*;
use super::geom::*;
use super::rand::*;

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
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: Rc::new(albedo),
            fuzz
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
        let ray_out = Ray::new(&hit_record.p, Point(reflected + Vec3::random_in_unit_sphere(r).scalar_mul(self.fuzz)));
        if ray_out.direction.0.dot(&hit_record.normal.0) > 0.0 {
            Some((self.albedo.clone(), ray_out))
        } else {
            None
        }
    }
}

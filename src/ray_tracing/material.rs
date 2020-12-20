use std::sync::Arc;
use super::geom::*;
use super::rand::*;
use super::ray::*;

pub trait Material: Send + Sync{
    fn scatter<'a>(
        &self,
        ray_in: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Arc<Color>, Ray<'a>)>;
}

pub struct Lambertian {
    albedo: Arc<Color>,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(albedo),
        }
    }
}
impl Material for Lambertian {
    fn scatter<'a>(
        &self,
        _: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Arc<Color>, Ray<'a>)> {
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
    albedo: Arc<Color>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: Arc::new(albedo),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter<'a>(
        &self,
        ray_in: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Arc<Color>, Ray<'a>)> {
        let reflected = ray_in.direction.0.unit_norm().reflect(&hit_record.normal.0);
        let ray_out = Ray::new(
            &hit_record.p,
            Point(reflected + Vec3::random_in_unit_sphere(r).scalar_mul(self.fuzz)),
        );
        if ray_out.direction.0.dot(&hit_record.normal.0) > 0.0 {
            Some((self.albedo.clone(), ray_out))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f64,
    attenuation: Arc<Color>,
}
impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {
            refractive_index,
            attenuation: Arc::new(Color::new_rgb(1.0, 1.0, 1.0)),
        }
    }

    pub fn reflectance(refractive_index: f64, cosine: f64) -> f64 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter<'a>(
        &self,
        ray_in: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(Arc<Color>, Ray<'a>)> {
        let refractive_ratio = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray_in.direction.0.unit_norm();

        let cos_theta = (-unit_direction.dot(&hit_record.normal.0)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refractive_ratio * sin_theta > 1.0;

        let ray_out = if cannot_refract
            || Dielectric::reflectance(refractive_ratio, cos_theta) > r.random_double()
        {
            unit_direction.reflect(&hit_record.normal.0)
        } else {
            unit_direction.refract(&hit_record.normal.0, refractive_ratio)
        };

        assert_eq!((ray_out.length_squared() - 1.0).abs() < 1e7, true);
        Some((
            self.attenuation.clone(),
            Ray::new(&hit_record.p, Point(ray_out)),
        ))
    }
}

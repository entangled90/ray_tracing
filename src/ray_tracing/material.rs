use super::geom::*;
use super::rand::*;
use super::ray::*;
use Material::*;

pub enum Material{
    Lambertian {
        albedo: Color,
    },
    Metal {
        albedo: Color,
        fuzz: f32,
    },
    Dielectric {
        refractive_index: f32,
        attenuation: Color,
    }

}
impl Material{
    pub fn new_dielectric(refractive_index: f32) -> Material {
        Dielectric {
            refractive_index,
            attenuation: Color::new_rgb(1.0, 1.0, 1.0),
        }
    }
    pub fn new_lambertian(albedo: Color) -> Material {
        Lambertian {
            albedo,
        }
    }

    pub fn new_metal(albedo: Color, fuzz: f32) -> Material {
        Metal {
            albedo,
            fuzz,
        }
    }

    pub fn scatter<'a>(
        &'a self,
        ray_in: &'a Ray,
        hit_record: &'a HitRecord,
        r: &mut Random,
    ) -> Option<(&'a Color, Ray<'a>)>{
        match self{
            Lambertian{albedo} => {
                let mut scatter_direction = &hit_record.normal.0 + &Vec3::random_unit_vector(r);
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal.0.clone();
        }
        Some((
            &albedo,
            Ray::new(&hit_record.p, Point(scatter_direction)),
        ))
            },
            Metal{albedo, fuzz} =>{
                {
                    let reflected = ray_in.direction.0.unit_norm().reflect(&hit_record.normal.0);
                    let ray_out = Ray::new(
                        &hit_record.p,
                        Point(reflected + Vec3::random_in_unit_sphere(r).scalar_mul(*fuzz)),
                    );
                    if ray_out.direction.0.dot(&hit_record.normal.0) > 0.0 {
                        Some((&albedo, ray_out))
                    } else {
                        None
                    }
                }
            },
            Dielectric{refractive_index, attenuation} => {
                let refractive_ratio = if hit_record.front_face {
                    1.0 / refractive_index
                } else {
                    *refractive_index
                };
                let unit_direction = ray_in.direction.0.unit_norm();
        
                let cos_theta = (-unit_direction.dot(&hit_record.normal.0)).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refractive_ratio * sin_theta > 1.0;
        
                let ray_out = if cannot_refract
                    || Material::reflectance(refractive_ratio, cos_theta) > r.random_double()
                {
                    unit_direction.reflect(&hit_record.normal.0)
                } else {
                    unit_direction.refract(&hit_record.normal.0, refractive_ratio)
                };
        
                Some((
                    &attenuation,
                    Ray::new(&hit_record.p, Point(ray_out)),
                ))
            }
        }
    }


    pub fn reflectance(refractive_index: f32, cosine: f32) -> f32 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

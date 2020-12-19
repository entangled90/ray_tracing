use std::rc::Rc;
use super::rand::Random;
use super::material::*;
use crate::HitRecord;
use crate::Hittable;
use crate::Ray;
use std::ops::*;

pub const PI: f64 = 3.1415926535897932385;

pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn iso(v: f64) -> Vec3 {
        Vec3 { x: v, y: v, z: v }
    }

    pub fn random(r: &mut Random) -> Vec3 {
        Vec3::new(r.random_double(), r.random_double(), r.random_double())
    }

    pub fn random_in(r: &mut Random, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            r.random_double_in(min, max),
            r.random_double_in(min, max),
            r.random_double_in(min, max),
        )
    }

    pub fn random_in_unit_sphere(r: &mut Random) -> Vec3 {
        loop {
            let p = Vec3::random_in(r, -1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(r: &mut Random) -> Vec3 {
        Vec3::random_in_unit_sphere(r).unit_norm()
    }
    const NEAR_ZERO: f64 = 1e-8;

    pub fn is_near_zero(&self) -> bool {
        Vec3::component_is_near_zero(self.x)
            && Vec3::component_is_near_zero(self.y)
            && Vec3::component_is_near_zero(self.z)
    }

    fn component_is_near_zero(x: f64) -> bool {
        x.abs() < Vec3::NEAR_ZERO
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3{
        self - &normal.scalar_mul(2.0 * self.dot(normal))
    }

    pub fn as_slice(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.as_slice().iter().map(|c| c.powi(2)).sum::<f64>()
    }

    pub fn scalar_mul(&self, mult: f64) -> Vec3 {
        Vec3 {
            x: self.x * mult,
            y: self.y * mult,
            z: self.z * mult,
        }
    }

    pub fn index_wise_mul(&self, v: &Vec3) -> Vec3{
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
    pub fn scalar_div(&self, mult: f64) -> Vec3 {
        self.scalar_mul(1.0 / mult)
    }

    pub fn dot(&self, w: &Vec3) -> f64 {
        self.x * w.x + self.y * w.y + self.z * w.z
    }

    pub fn cross(&self, w: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * w.z - self.z - w.y,
            y: self.z * w.x - self.x - w.z,
            z: self.x * w.y - self.y - w.x,
        }
    }

    pub fn unit_norm(&self) -> Vec3 {
        self.clone().scalar_div(self.length())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, w: Vec3) {
        self.x += w.x;
        self.y += w.y;
        self.z += w.z;
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, w: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, w: Vec3) -> Vec3 {
        &self + &w
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, w: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, w: Vec3) -> Vec3 {
        &self - &w
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, w: Vec3) {
        self.x *= w.x;
        self.y *= w.y;
        self.z *= w.z;
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, w: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * w.x,
            y: self.y * w.y,
            z: self.z * w.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, w: Vec3) -> Vec3 {
        &self * &w
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, w: Vec3) {
        self.x /= w.x;
        self.y /= w.y;
        self.z /= w.z;
    }
}

impl Neg for &Vec3 {
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    type Output = Vec3;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point(pub Vec3);

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Rc<Box<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin.0 - &self.center.0;
        let a = ray.direction.0.length_squared();
        let half_b = &oc.dot(&ray.direction.0);
        let c = &oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            let discr_sqrt = discriminant.sqrt();
            let mut root = (-half_b - discr_sqrt) / a;
            if root < t_min || root > t_max {
                root = (-half_b + discr_sqrt) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }
            let t = root;
            let p = ray.at(t);
            let normal = Point((&p.0 - &self.center.0).scalar_div(self.radius));
            Some(HitRecord::new(p, t, normal, self.material.clone(), &ray))
        }
    }
}

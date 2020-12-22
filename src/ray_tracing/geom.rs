use super::rand::Random;

use std::ops::*;

pub const PI: f32 = 3.141_592_653_589_793;

pub const INFINITY: f32 = f32::INFINITY;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const fn iso(v: f32) -> Vec3 {
        Vec3 { x: v, y: v, z: v }
    }

    pub fn random(r: &mut Random) -> Vec3 {
        Vec3::new(r.random_double(), r.random_double(), r.random_double())
    }

    pub fn random_in(r: &mut Random, min: f32, max: f32) -> Vec3 {
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

    pub fn random_in_unit_disk(r: &mut Random) -> Vec3 {
        loop {
            let p = Vec3::new(
                r.random_double_in(-1.0, 1.0),
                r.random_double_in(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(r: &mut Random) -> Vec3 {
        Vec3::random_in_unit_sphere(r).unit_norm()
    }
    const NEAR_ZERO: f32 = 1e-8;

    pub fn is_near_zero(&self) -> bool {
        Vec3::component_is_near_zero(self.x)
            && Vec3::component_is_near_zero(self.y)
            && Vec3::component_is_near_zero(self.z)
    }

    fn component_is_near_zero(x: f32) -> bool {
        x.abs() < Vec3::NEAR_ZERO
    }

    pub fn as_slice(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn scalar_mul(&self, mult: f32) -> Vec3 {
        Vec3 {
            x: self.x * mult,
            y: self.y * mult,
            z: self.z * mult,
        }
    }

    pub fn index_wise_mul(&self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
    pub fn scalar_div(&self, mult: f32) -> Vec3 {
        self.scalar_mul(1.0 / mult)
    }

    pub fn dot(&self, w: &Vec3) -> f32 {
        self.x * w.x + self.y * w.y + self.z * w.z
    }

    pub fn cross(&self, w: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * w.z - self.z * w.y,
            y: self.z * w.x - self.x * w.z,
            z: self.x * w.y - self.y * w.x,
        }
    }

    pub fn unit_norm(&self) -> Vec3 {
        self.scalar_div(self.length())
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - &normal.scalar_mul(2.0 * self.dot(normal))
    }

    pub fn refract(&self, normal: &Vec3, eta_ratio: f32) -> Vec3 {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let out_perp = (self + &normal.scalar_mul(cos_theta)).scalar_mul(eta_ratio);
        let out_parallel = normal.scalar_mul(-((1.0 - out_perp.length_squared().abs()).sqrt()));
        out_perp + out_parallel
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

impl Deref for Point {
    type Target = Vec3;
    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(x.cross(&y), z);
    }
}

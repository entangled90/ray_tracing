use crate::Ray;
use std::ops::*;

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
    pub fn as_slice(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.as_slice().iter().map(|c| *c * *c).sum::<f64>()
    }

    pub fn scalar_mul(&self, mult: f64) -> Vec3 {
        Vec3 {
            x: self.x * mult,
            y: self.y * mult,
            z: self.z * mult,
        }
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


pub struct Sphere{
    pub center: Point,
    pub radius: f64
}

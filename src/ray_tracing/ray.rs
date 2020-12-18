use std::io::{stderr, stdout, Stdout, Write};

use super::geom::*;
use std::error::Error;
use std::result::Result;

pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn write<W>(&self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        w.write(
            format!(
                "{} {} {}\n",
                (255.999 * self.rgb.x) as u32,
                (255.999 * self.rgb.y) as u32,
                (255.999 * self.rgb.z) as u32
            )
            .as_bytes(),
        )
        .map(|_| ())
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

    pub fn color(&self) -> Color {
        let sphere = Sphere{center: Point(Vec3::new(0.0,0.0, -1.0)), radius: 0.5};
        if self.hits(&sphere) {
            return Color{rgb: Vec3::new(1.0, 0.0, 0.0)};
        }
        let unit = self.direction.0.unit_norm();
        let t = 0.5 * (unit.y + 1.0);
        Color {
            rgb: Vec3::new(1.0, 1.0, 1.0).scalar_mul(1.0 - t)
                + Vec3::new(0.5, 0.7, 1.0).scalar_mul(t),
        }
    }

    pub fn hits(&self, sphere: &Sphere) -> bool {
        let oc = &self.origin.0 - &sphere.center.0;
        let a = self.direction.0.length_squared();
        let b = 2.0 * &oc.dot(&self.direction.0);
        let c = &oc.length_squared() - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}

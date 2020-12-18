use super::geom::*;
use super::ray::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const HORIZONTAL: Point = Point(Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0));
const VERTICAL: Point = Point(Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0));
const ORIGIN: Point = Point(Vec3::new(0.0, 0.0, 0.0));

pub struct Camera {
    origin: &'static Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left_corner = Point(
            ORIGIN.0.clone()
                - HORIZONTAL.0.scalar_div(2.0)
                - VERTICAL.0.scalar_div(2.0)
                - Vec3::new(0.0, 0.0, FOCAL_LENGTH),
        );

        Camera {
            origin: &ORIGIN,
            lower_left_corner,
            horizontal: HORIZONTAL,
            vertical: VERTICAL,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            Point(
                &(&(&self.lower_left_corner.0 + &self.horizontal.0.scalar_mul(u))
                    + &self.vertical.0.scalar_mul(v))
                    - &self.origin.0,
            ),
        )
    }
}

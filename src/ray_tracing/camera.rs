use super::geom::*;
use super::ray::*;
use crate::Random;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
    u: Point,
    v: Point,
    w: Point,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        view_up: Point,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let w = (&look_from.0 - &look_at.0).unit_norm();
        let u = view_up.0.cross(&w).unit_norm();
        let v = w.cross(&u);

        let origin = look_from;
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let horizontal: Point = Point(u.scalar_mul(viewport_width * focus_dist));
        let vertical: Point = Point(v.scalar_mul(viewport_height * focus_dist));
        let lower_left_corner = Point(
            &origin.0
                - &horizontal.0.scalar_div(2.0)
                - vertical.0.scalar_div(2.0)
                - w.scalar_mul(focus_dist),
        );
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u: Point(u),
            v: Point(v),
            w: Point(w),
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, s: f32, t: f32, r: &mut Random) -> Ray {
        let rd = Vec3::random_in_unit_disk(r).scalar_mul(self.lens_radius);
        let offset = self.u.0.scalar_mul(rd.x) + self.v.0.scalar_mul(rd.y);
        Ray::new(
            &self.origin,
            Point(
                &(&(&(&self.lower_left_corner.0 + &self.horizontal.0.scalar_mul(s))
                    + &self.vertical.0.scalar_mul(t))
                    - &self.origin.0)
                    - &offset,
            ),
        )
    }
}

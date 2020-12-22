use crate::HitRecord;
use crate::Material;
use crate::Point;
use crate::Ray;
use Object::*;

pub enum Object {
    Sphere {
        center: Point,
        radius: f32,
        material: Material,
        moving_component: Option<MovingComponent>,
    },
}

pub struct MovingComponent {
    pub center_0: Point,
    pub center_1: Point,
    pub time_0: f32,
    pub time_1: f32,
}

impl Object {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Sphere {
                radius, material, ..
            } => {
                let oc = &ray.origin.0 - &self.center_at(ray.time).0;
                let a = ray.direction.0.length_squared();
                let half_b = &oc.dot(&ray.direction.0);
                let c = oc.length_squared() - radius.powi(2);
                let discriminant = half_b * half_b - a * c;
                if discriminant < 0.0 {
                    None
                } else {
                    let discr_sqrt = discriminant.sqrt();
                    // first root
                    let mut root = (-half_b - discr_sqrt) / a;
                    if root < t_min || root > t_max {
                        // try second root
                        root = (-half_b + discr_sqrt) / a;
                        if root < t_min || root > t_max {
                            return None;
                        }
                    }
                    let t = root;
                    let p = ray.at(t);
                    let normal = Point((&p.0 - &self.center_at(ray.time).0).scalar_div(*radius));
                    Some(HitRecord::new(p, t, normal, &material, &ray))
                }
            }
        }
    }

    pub fn center_at(&self, t: f32) -> Point {
        match self {
            Sphere {
                moving_component,
                center,
                ..
            } => match moving_component {
                Some(MovingComponent {
                    center_0,
                    center_1,
                    time_0,
                    time_1,
                }) => Point(
                    &center_0.0
                        + &(&center_1.0 - &center_0.0).scalar_mul((t - time_0) / (time_1 - time_0)),
                ),
                None => center.clone(),
            },
        }
    }
}

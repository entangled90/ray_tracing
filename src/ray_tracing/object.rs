use core::cmp::Ordering;

use crate::ray_tracing::bounding_box::AaBoundingBox;
use crate::HitRecord;
use crate::Material;
use crate::Point;
use crate::Random;
use crate::Ray;
use crate::Vec3;

use std::clone::Clone;

use Object::*;

#[derive(Debug, Clone)]
pub enum Object {
    Sphere {
        center: Point,
        radius: f32,
        material: Material,
        moving_component: Option<MovingComponent>,
    },
    HittableList {
        hittables: Vec<Object>,
    },
    BVHNode {
        left: Box<Object>,
        right: Box<Object>,
        bounding_box: AaBoundingBox,
    },
}

#[derive(Debug, Clone)]
pub struct MovingComponent {
    pub center_0: Point,
    pub center_1: Point,
    pub time_0: f32,
    pub time_1: f32,
}

impl MovingComponent {
    pub fn center_at(&self, t: f32) -> Point {
        Point(
            &self.center_0.0
                + &(&self.center_1.0 - &self.center_0.0)
                    .scalar_mul((t - self.time_0) / (self.time_1 - self.time_0)),
        )
    }
}

impl Object {
    pub fn new_bvh_node(objects: &mut [Object], t_0: f32, t_1: f32) -> Object {
        let mut random = Random::default();
        let axis = random.random_int_in(0, 2);
        let len = objects.len();
        let comparator = |a: &Object, b: &Object| {
            let a_bb = a.bounding_box(t_0, t_1);
            let b_bb = b.bounding_box(t_0, t_1);
            if a_bb.is_none() || b_bb.is_none() {
                Ordering::Less
            } else {
                a_bb.unwrap()
                    .minimum
                    .0
                    .get(axis)
                    .partial_cmp(&b_bb.unwrap().minimum.0.get(axis))
                    .unwrap()
            }
        };

        let (left, right) = match len {
            1 => (objects[0].clone(), objects[0].clone()),
            j if j > 1 => {
                objects.sort_by(comparator);
                if len == 2 {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    let mid = objects.len() / 2;
                    let left = Object::new_bvh_node(&mut objects[0..mid], t_0, t_1);
                    let right = Object::new_bvh_node(&mut objects[mid..len], t_0, t_1);
                    (left, right)
                }
            }
            _ => panic!("AAAA"),
        };
        let bounding_box = left
            .bounding_box(t_0, t_1)
            .unwrap()
            .surrounding_box(right.bounding_box(t_0, t_1).unwrap());
        BVHNode {
            left: Box::new(left),
            right: Box::new(right),
            bounding_box,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Sphere {
                center,
                radius,
                material,
                moving_component,
                ..
            } => {
                let center_timed: Point = moving_component
                    .as_ref()
                    .map_or_else(|| center.clone(), |c| c.center_at(ray.time));
                let oc = &ray.origin.0 - &center_timed;
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
                    let normal = Point((&p.0 - &center_timed).scalar_div(*radius));
                    Some(HitRecord::new(p, t, normal, &material, &ray))
                }
            }
            HittableList { hittables } => {
                {
                    let mut temp_rec: Option<HitRecord> = None;
                    let mut closest_so_far = t_max;
                    for object in hittables {
                        // note closest_so_far is used as t_max
                        if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                            closest_so_far = rec.t;
                            temp_rec = Some(rec);
                        }
                    }
                    temp_rec
                }
            }
            BVHNode {
                left,
                right,
                bounding_box,
            } => {
                if bounding_box.hit(ray, t_min, t_max) {
                    match left.hit(ray, t_min, t_max) {
                        Some(left_ray) => right.hit(ray, t_min, left_ray.t),
                        None => right.hit(ray, t_min, t_max),
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<AaBoundingBox> {
        match self {
            Sphere {
                center,
                radius,
                moving_component,
                ..
            } => match moving_component {
                None => Some(AaBoundingBox::new(
                    Point(&center.0 - &Vec3::iso(*radius)),
                    Point(&center.0 + &Vec3::iso(*radius)),
                )),
                Some(moving) => {
                    let box_1 = AaBoundingBox::new(
                        Point(moving.center_at(time_0).0 - Vec3::iso(*radius)),
                        Point(moving.center_at(time_0).0 + Vec3::iso(*radius)),
                    );
                    let box_2 = AaBoundingBox::new(
                        Point(moving.center_at(time_1).0 - Vec3::iso(*radius)),
                        Point(moving.center_at(time_1).0 + Vec3::iso(*radius)),
                    );
                    Some(box_1.surrounding_box(box_2))
                }
            },
            HittableList { hittables } => {
                let mut result: Option<AaBoundingBox> = None;
                for obj in hittables {
                    match obj.bounding_box(time_0, time_1) {
                        Some(bb) => {
                            result = match result.take() {
                                Some(previous_bb) => Some(previous_bb.surrounding_box(bb)),
                                None => Some(bb),
                            };
                        }
                        // object has no bounding box, cannot construct a bigger one
                        None => return None,
                    }
                }
                result
            }
            BVHNode { bounding_box, .. } => Some(bounding_box.clone()),
        }
    }
}
